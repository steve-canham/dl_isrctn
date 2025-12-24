mod xml_model;
mod processor;
pub mod data_access;
pub mod gen_helper;
mod helper;

//use std::collections::HashMap;
//use std::path::PathBuf;
use crate::{AppError};
use super::setup::InitParams;
use super::DownloadResult;
use chrono::{NaiveDate, Days};
use xml_model::{AllTrials, FullTrial};

//use data_access::{update_who_study_mon, add_new_single_file_record, 
// add_contents_record, store_who_summary};
//use file_models::WHOLine;
//use super::setup::config_reader::fetch_src_db_name;
//use std::fs;
//use std::io::BufReader;
//use std::fs::File;
//use csv::ReaderBuilder;
//use std::io::Write;
//use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};
//use log::info;


pub async fn process_data(params: &InitParams, _dl_id:i32, _mon_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

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

    let base = params.base_url.clone();
    let mut sd = params.start_date;
    let res= DownloadResult::new();


    while sd < params.end_date  {
         
        let mut ed = sd.checked_add_days(Days::new(4)).unwrap();  // unwrap should be safe!
        if ed >  params.end_date {
            ed = params.end_date  // ensure does not go beyond end of range
        }

        let record_num = get_record_count(&base, &sd, &ed).await?;

        if record_num > 0 {
            if record_num > 100 {

                // Split the (up to) 4 days up into single days

                let mut d = sd;
                while d < ed {
                    res.add(process_single_day(&base, &d).await?);
                    d = d.checked_add_days(Days::new(1)).unwrap();
                }
            }
            else {

                // Process all records.

                res.add(process_batch(&base, &sd, &ed).await?);
            }
        }
        sd = ed;   // make the start date the old end date
    }

    Ok(res)

}


async fn get_record_count(base_url: &String, start_date: &NaiveDate, end_date: &NaiveDate) -> Result<i32, AppError>
{
    let start_date_param = start_date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);

    let end_date_param = end_date.format("%Y-%m-%d").to_string();
    let query_end_param = format!("lastEdited%20LT%20{}T23:59:59%20&limit=", end_date_param);

    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, 1);

    let response = reqwest::get(url.clone()).await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
    let xml_content = response.text().await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
   
    let files: AllTrials = quick_xml::de::from_str(&xml_content)
        .map_err(|e| AppError::QuickXMLError(url.clone(), e))?;
   
    Ok(files.total_count)
}


async fn process_single_day(base_url: &String, date: &NaiveDate) -> Result<DownloadResult, AppError>
{
    let start_date_param = date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);

    let end_date_param = date.format("%Y-%m-%d").to_string();
    let query_end_param = format!("lastEdited%20LT%20{}T23:59:59%20&limit=", end_date_param);

    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, 1);

    let response = reqwest::get(url.clone()).await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
    let xml_content = response.text().await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
   
    let studies: AllTrials = quick_xml::de::from_str(&xml_content)
        .map_err(|e| AppError::QuickXMLError(url.clone(), e))?;

    let res = process_studies(studies.full_trials).await?;
   
    Ok(res)
}


async fn process_batch(base_url: &String, date: &NaiveDate, end_date: &NaiveDate) -> Result<DownloadResult, AppError>
{
    let start_date_param = date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);

    let end_date_param = end_date.format("%Y-%m-%d").to_string();
    let query_end_param = format!("lastEdited%20LT%20{}T23:59:59%20&limit=", end_date_param);

    let url = format!("{}{}{}{}", base_url, query_start_param, query_end_param, 1);

    let response = reqwest::get(url.clone()).await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
    let xml_content = response.text().await
        .map_err(|e| AppError::ReqwestError(url.clone(), e))?;
   
    let studies: AllTrials = quick_xml::de::from_str(&xml_content)
        .map_err(|e| AppError::QuickXMLError(url.clone(), e))?;
    let res = process_studies(studies.full_trials).await?;
   
    Ok(res)
}


async fn process_studies(_studies: Vec<FullTrial>) -> Result<DownloadResult, AppError> {

    let res = DownloadResult::new();

        /* 
        // iterate through the vector of FullTrials
        For each, call the processor.rs routine that goes through the xml derived structure
        // and which produces a mmuch more mdr compliant model

        // That includes tidying up many of the fields, removing spaces and carriage treturns...

        // Once that model has been returned

        // Write it out as a json file
        // Also - optionally - update the database with it, as a new or updated version of 
        // the various tables.

        // This allows the isrctn database to be updated in situ 
        // (though there is still a lot of coding to be applied after each update)






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
        Ok(res)
    }

    // Writes out the file with the correct name to the correct folder, as indented json.
    // Called from the DownloadBatch function.
    // Returns the full file path as constructed, or an 'error' string if an exception occurred.

/* 
async fn  WriteOutFile(Study s, string sd_sid, string file_base)
{
    string file_name = sd_sid + ".json";
    string full_path = Path.Combine(file_base, file_name);
    try
    {
        await using FileStream jsonStream = File.Create(full_path);
        await JsonSerializer.SerializeAsync(jsonStream, s, _json_options);
        await jsonStream.DisposeAsync();
        return full_path;
    }
    catch (Exception e)
    {
        _loggingHelper.LogLine("Error in trying to save file at " + full_path + ":: " + e.Message);
        return "error";
    }
}
*/

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
