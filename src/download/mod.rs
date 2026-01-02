mod processor;
pub mod monitoring;
pub mod isrctn_helper;

use crate::data_models::xml_models;
use crate::data_models::json_models;

use crate::AppError;
use crate::base_types::*;

use chrono::{NaiveDate, Days};
use xml_models::{AllTrials, FullTrial, TrialsCount};
use quick_xml::de;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use serde_json::to_string_pretty;
use std::{thread, time};
use rand::prelude::*;
use sqlx::{Pool, Postgres};
use log::info;

pub async fn download_data(params: &InitParams, dl_id:i32, src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    // The base url, json file folder, log folder, and start and end dates have
    // already been checked as being present and reasonable.

    // The download process downloads all records with last edited dates >= start date and < end date.
    // Records last edited on the start date are included, but the full set only includes studies
    // last edited up to the (end date - 1). This avoids duplications when stepping through the
    // full set in the API. It also means that the best time for regular downloading is in the very 
    // early morning (European time) as this means a minimal number of records are missed.
    
    // Each period is broken up into periods of 4 days. There does not appear to be a way to rank or 
    // order results and select from within a returned set, so record sets are returned 'as is'.
    // If the number of available records for a selected period is > 100 records the call is 
    // broken down into calls for individual days. 
    
    let base_url = params.base_url.clone();
    let mut sd = params.start_date;
    let mut res = DownloadResult::new();
    
    while sd < params.end_date  {

        // For each pass, set end date to be 4 days later than start date. 
        // If that goes past the overall end date set end date back to the overall end date.
         
        let mut ed = sd.checked_add_days(Days::new(4)).unwrap();  // unwrap should be safe!
        if ed >  params.end_date {
            ed = params.end_date  // ensure does not go beyond end of range
        }

        // Establish api url for these dates.

        let start_date_param = sd.format("%Y-%m-%d").to_string();
        let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);
        let end_date_param = ed.format("%Y-%m-%d").to_string();
        let query_end_param = format!("lastEdited%20LT%20{}T00:00:00%20&limit=", end_date_param);
        let dated_url = format!("{}{}{}", base_url, query_start_param, query_end_param);

        // Initially just get record count.

        let url = format!("{}{}", dated_url, 1);
        let record_num = get_study_count(&url).await?;

        // If over 100 records split processing to by day, else process all.
        
        if record_num > 0 {
            if record_num > 100 {    // Split the (up to) 4 days up into single days

                let mut d = sd;
                while d < ed {
                    let this_res = process_single_day(params, &d, dl_id, src_pool).await?;
                    info!("For single day {}, records checked:{}", d, this_res.num_checked);
                    res = res + this_res;
                    d = d.checked_add_days(Days::new(1)).unwrap();
                }
            }
            else {    // Process all records.
                                
                let url = format!("{}{}", dated_url, 100);
                let studies: AllTrials = get_studies(&url).await?;
                let this_res = process_studies(params, studies.full_trials, dl_id, src_pool).await?;
                info!("For period GE {}, to LT {}, records checked:{}", start_date_param, end_date_param, this_res.num_checked);
                res = res + this_res;
            }
        }
        else {
            info!("For period GE {}, to LT {}, no records found", start_date_param, end_date_param);
        }

        sd = ed;    // make the start date the old end date
         
    }
    
    info!("{} records checked in total. {} Files written ({} of them new)", res.num_checked, res.num_downloaded, res.num_added);

    Ok(res)

}


async fn process_single_day(params: &InitParams, date: &NaiveDate, dl_id: i32, src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError>
{
    let date_param = date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", date_param);
    let query_end_param = format!("lastEdited%20LT%20{}T23:59:59%20&limit=", date_param);

    // See how many records there are this day.

    let base_url = params.base_url.clone();
    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, 1);
    let limit = get_study_count(&url).await?;

    if limit > 0 {

        // Get the full set of records (i.e. set limit to be all the records aavailable)...

        let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, limit);
        let studies: AllTrials = get_studies(&url).await?;
        let res = process_studies(params, studies.full_trials, dl_id, src_pool).await?;

        Ok(res) 
    }
    else {
        Ok(DownloadResult::new())
    }
}


async fn get_study_count(url: &String) -> Result<i32, AppError> {

    let response = reqwest::get(url.clone()).await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;

    // Add a pause after any api access - random value between 0.5 and 1.5 seconds...
    
    let mut rng = rand::rng();
    let num = &rng.random_range(1..=1000);
    let millis = 500 + num;
    let pause = time::Duration::from_millis(millis);
    thread::sleep(pause);

    // Extract api text and deserialise it to the xml model

    let xml_content = response.text().await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
   
    let trials_count: TrialsCount = de::from_str(&xml_content)
        .map_err(|e| AppError::QuickXMLError(url.clone(), e))?;

    Ok(trials_count.total_count)

}


async fn get_studies(url: &String) -> Result<AllTrials, AppError> {

    let response = reqwest::get(url.clone()).await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;

    // Add a pause after any api access - random value between 0.5 and 1.5 seconds...
    
    let mut rng = rand::rng();
    let num = &rng.random_range(1..=1000);
    let millis = 500 + num;
    let pause = time::Duration::from_millis(millis);
    thread::sleep(pause);

    // Extract api text and deserialise it to the xml model

    let xml_content = response.text().await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
   
    de::from_str(&xml_content)
        .map_err(|e| AppError::QuickXMLError(url.clone(), e))

}





async fn process_studies(params: &InitParams, studies: Vec<FullTrial>, dl_id: i32, src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let mut res = DownloadResult::new();

        // iterate through studies, the vector of FullTrials
        // For each, call the process_studyroutine that goes through the xml 
        // derived structure and which produces a much more mdr compliant model
        // That includes tidying up many of the fields, removing spaces and carriage returns...

        // Once that model has been returned Write it out as a json file
        // Also - optionally - update the database with it, as a new or updated version of 
        // the various tables. (Probably not at this stage). This would allows the 
        // isrctn database to be updated in situ, (though there is still a lot of 
        // coding to be applied after each update)

        for s in studies {

            res.num_checked += 1;
            let t= processor::process_study(s)?;

            let sd_sid = &t.sd_sid;
            let record_date = &t.registration.last_updated;
            let remote_url = format!("https://www.isrctn.com/{}", sd_sid.clone());


            let mut yr = "pre-2007".to_string();  // default value
            if let Some(s) = &t.registration.date_id_assigned {
                let reg_year_string = s[0..4].to_string();
                if reg_year_string.as_str() > "2006" 
                {
                    yr = reg_year_string;
                }
            }
            let json_folder = &params.json_data_path;
            let file_folder: PathBuf = [json_folder, &PathBuf::from(&yr)].iter().collect(); 
            if !folder_exists(&file_folder) {
                fs::create_dir_all(&file_folder)?;
            }
                             
            let full_path = write_out_file(&t, &file_folder).await?;

            let added = monitoring::update_isrctn_mon(sd_sid, &remote_url, dl_id,
                     &record_date, &full_path, src_pool).await?;

            res.num_downloaded += 1;
            if added {
                res.num_added +=1;
            }
        }

        Ok(res)
}


pub async fn write_out_file(t: &json_models::Study, json_folder: &PathBuf) -> Result<PathBuf, AppError> {

    // Writes out the file with the correct name to the correct folder, as indented json.
    // Called from the process_studies function.
    // Returns the full file path as constructed.
 
    let file_name = format!("{}.json", t.sd_sid);
    let file_path: PathBuf= [json_folder, &PathBuf::from(&file_name)].iter().collect();
    let json_string = to_string_pretty(&t).unwrap();

    let mut file = fs::File::create(&file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(file_path)
}


fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}


/*

// Routines below used temporarily for correctinmg some downloads after code change
// Retained in case similar use case arises in the future

pub async fn correct_data(params: &InitParams, src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {
    
    // get the dataset of individual ids to correct
    // In this instance the correcvtion is of studies with incorrect IDs, that were
    // 'tagged' in the database by insertring specific values in the last_aggregation_id field

    #[derive(sqlx::FromRow)]
    struct SdSid {
        sd_sid: String,
    }

    let sql = r#"select sd_sid from mn.source_data
            where last_aggregation_id in (12, 13) and last_dl_id < 101970
            ORDER BY sd_sid"#;

    let ids: Vec<SdSid> = sqlx::query_as(&sql).fetch_all(src_pool).await
                    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;       
    
    let dl_id = 101970;  // static, indicates successful re-processing, change if re-used
    let mut res = DownloadResult::new();
    let mut n = 0;

    for id in ids {

        //for each id...construct the single trial url

        let url = format!("https://www.isrctn.com/api/trial/{}/format/default", id.sd_sid);
        info!("{}", url);
        n += 1;

        // call it to get and process the data
        // that should also change the details in mn.source_data

        let study: FullTrial = get_study(&url).await?;
        let studies = vec![study];
        res = res + process_studies(params, studies, dl_id, src_pool).await?;

        if n > 100 {  // just to limit numbers per batch
            break;
        }
    }
    Ok(res)
}


async fn get_study(url: &String) -> Result<FullTrial, AppError> {

    let response = reqwest::get(url.clone()).await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;

    // Add a pause after any api access - random value between 0.5 and 1.5 seconds...
    
    let mut rng = rand::rng();
    let num = &rng.random_range(1..=1000);
    let millis = 800 + num;   
    let pause = time::Duration::from_millis(millis);
    thread::sleep(pause);

    // Extract api text and deserialise it to the xml model

    let xml_content = response.text().await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
   
    de::from_str(&xml_content)
        .map_err(|e| AppError::QuickXMLError(url.clone(), e))

}


*/