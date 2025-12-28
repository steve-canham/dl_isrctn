mod xml_models;
mod json_models;
mod processor;
pub mod data_access;
pub mod isrctn_helper;

use std::path::PathBuf;
use crate::AppError;
use super::setup::InitParams;
use super::DownloadResult;
use chrono::{NaiveDate, Days};
use xml_models::{AllTrials, FullTrial};
use quick_xml::de;
use std::fs::File;
use std::io::Write;
use serde_json::to_string_pretty;
use std::{thread, time};
use rand::prelude::*;
use sqlx::{Pool, Postgres};
use log::info;


pub async fn process_data(params: &InitParams, dl_id:i32, src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    // The base url, json file folder, log folder, and start and end dates have
    // already been checked as being present and reasonable.

    // The download process downloads all records with last edited dates >= start date and < end date.
    // Records last edited on the start date are included, but the full set only includes studies
    // last edited up to the (end date - 1). This avoids duplications when stepping through the
    // full set in the API. It also means that the best time for regular downloading is in the very 
    // early morning (European time) as this means a minimal number of records are missed.
    
    // Period is broken up into periods of 4 days. If the total for that period > 100
    // those 4 days are done as single days. If a single day is > 100 the limit is raised to that amount.
    // A normal weekly update may therefore involve only two calls to the API.
    
    // Construct the outer loop and call the downloading process within it.

    //let base = params.base_url.clone();
    //let folder_path = params.json_data_path.clone();

    let mut sd = params.start_date;
    let mut res = DownloadResult::new();
    
    while sd < params.end_date  {

        // For each pass, set end date to be 4 days later than start date. 
        // If that goes past the overall end date set end date back to the overall end date.
         
        let mut ed = sd.checked_add_days(Days::new(4)).unwrap();  // unwrap should be safe!
        if ed >  params.end_date {
            ed = params.end_date  // ensure does not go beyond end of range
        }

        let record_num = get_record_count(params, &sd, &ed).await?;

        if record_num > 0 {
            if record_num > 100 {    // Split the (up to) 4 days up into single days

                let mut d = sd;
                while d < ed {
                    res = res + process_single_day(params, &d, dl_id, src_pool).await?;
                    d = d.checked_add_days(Days::new(1)).unwrap();
                }
            }
            else {   // Process all records.
                res = res + process_batch(params, &sd, &ed, dl_id, src_pool).await?;
            }
        }
        sd = ed;   // make the start date the old end date
    }
    Ok(res)

}


async fn get_record_count(params: &InitParams, start_date: &NaiveDate, end_date: &NaiveDate) -> Result<i32, AppError>
{
    let start_date_param = start_date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);

    let end_date_param = end_date.format("%Y-%m-%d").to_string();
    let query_end_param = format!("lastEdited%20LT%20{}T23:59:59%20&limit=", end_date_param);

    let base_url = params.base_url.clone();
    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, 1);
    let studies: AllTrials = get_studies(&url).await?;

    Ok(studies.total_count)
}


async fn process_single_day(params: &InitParams, date: &NaiveDate, dl_id: i32, src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError>
{
    let start_date_param = date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);

    let end_date_param = date.format("%Y-%m-%d").to_string();
    let query_end_param = format!("lastEdited%20LT%20{}T23:59:59%20&limit=", end_date_param);

    // See how many records there are this day.

    let base_url = params.base_url.clone();
    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, 1);
    let studies: AllTrials = get_studies(&url).await?;
    let limit = studies.total_count;

    // Get the full set of records...

    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, limit);
    let studies: AllTrials = get_studies(&url).await?;
    let res = process_studies(params, studies.full_trials, dl_id, src_pool).await?;

    info!("For single day: {} : Numbers checked:{}", start_date_param, res.num_checked);
    Ok(res)
}


async fn process_batch(params: &InitParams, date: &NaiveDate, end_date: &NaiveDate, dl_id: i32, src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError>
{
    let start_date_param = date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);

    let end_date_param = end_date.format("%Y-%m-%d").to_string();
    let query_end_param = format!("lastEdited%20LT%20{}T23:59:59%20&limit=", end_date_param);
    
    //  Default study number limit is 10, over-ride to ensure all records obtained.

    let base_url = params.base_url.clone();
    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, 100);
    let studies: AllTrials = get_studies(&url).await?;
    let res = process_studies(params, studies.full_trials, dl_id, src_pool).await?;

    info!("For period: {} (inc) to {} (exc): Numbers checked:{}", start_date_param, end_date_param, res.num_checked);
    Ok(res)
}


async fn get_studies(url: &String) -> Result<AllTrials, AppError> {

    let response = reqwest::get(url.clone()).await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;

    // Add a pause after any access - random value between 0.5 and 1.5 seconds...
    
    let mut rng = rand::rng();
    let num = &rng.random_range(1..=1000);
    let millis = 500 + num;
    let pause = time::Duration::from_millis(millis);
    thread::sleep(pause);

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
                        
            let full_path = write_out_file(&t, &params.json_data_path).await?;

            let added = data_access::update_isrctn_mon(sd_sid, &remote_url, dl_id,
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
    // Called from the DownloadBatch function.
    // Returns the full file path as constructed, or an 'error' string if an exception occurred.
 
    // Write the JSON string to a file.

    let file_name = format!("{}.json", t.sd_sid);
    let file_path: PathBuf= [json_folder, &PathBuf::from(&file_name)].iter().collect();

    let json_string = to_string_pretty(&t).unwrap();
    let mut file = File::create(&file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(file_path)

}
      

        /* 
        ISRCTN_Processor isrctn_processor = new();
        int number_returned = result.totalCount;
        if (number_returned > 0 && result.fullTrials?.Any() is true) 
        { 
            foreach (FullTrial f in result.fullTrials)
            {
                res.num_checked++;
                Study? s = await isrctn_processor.GetFullDetails(f, _loggingHelper);
                if (s is not null)
                {
                    string full_path = await WriteOutFile(s, s.sd_sid, file_base);
                    if (full_path != "error")
                    {
                        string remote_url = "https://www.isrctn.com/" + s.sd_sid;
                        DateTime? last_updated = s.lastUpdated?.FetchDateTimeFromISO();
                        bool added = _monDataLayer.UpdateStudyDownloadLog(source_id, s.sd_sid, remote_url, saf_id,
                                                last_updated, full_path);
                        res.num_downloaded++;
                        if (added) res.num_added++;
                    }
                }
            }
        }
        */



    // Writes out the file with the correct name to the correct folder, as indented json.
    // Called from the DownloadBatch function.
    // Returns the full file path as constructed, or an 'error' string if an exception occurred.


/* 
fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
    */
