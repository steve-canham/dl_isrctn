mod file_models;
mod processor;
pub mod data_access;
pub mod gen_helper;
mod who_helper;

use std::collections::HashMap;
use std::path::PathBuf;
use crate::{AppError, DownloadResult};
use data_access::{update_who_study_mon, add_new_single_file_record, 
    add_contents_record, store_who_summary};
use file_models::WHOLine;
use super::setup::config_reader::fetch_src_db_name;
use std::fs;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use std::io::Write;
use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};
use log::info;


pub async fn process_files(file_path: &PathBuf, json_path: &PathBuf, dl_id:i32, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    // Set up source file, csv reader, counters, hash table.
    
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_reader);
    
    let mut file_res = DownloadResult::new();
    let mut source_tots: HashMap<i32, i32> = HashMap::new();
    info!("");
    info!("Processing file {:?}", file_path);

    for result in csv_rdr.deserialize() {

        file_res.num_checked +=1;
        if file_res.num_checked % 5000 == 0 {
            info!("{} records checked", file_res.num_checked);
        }

        let who_line: WHOLine = match result {
             Ok(w) => w,
             Err(e) => return Err(AppError::CsvError(e)),
        };
               
        let rec_summ = match processor::summarise_line(&who_line, file_res.num_checked)
        {
            Some(r) => r,
            None => continue,   // some sort of problem occured - should have been loggged
        };
        
        // Adjust running source totals (even if file not processed further)

        let source_id = rec_summ.source_id;
        source_tots.entry(source_id).and_modify(|n| *n += 1).or_insert(1);

        // assemble variables from summary record, allows them to be used 
        // later even if the whole record has already been moved for storage 

        let sid = rec_summ.sd_sid.clone();
        let date_of_rec = rec_summ.date_last_rev;
        let study_type = rec_summ.study_type;
        let study_status = rec_summ.study_status;
        let remote_url = rec_summ.remote_url.clone();
        let idents = rec_summ.secondary_ids.clone();
        let countries = rec_summ.country_list.clone();
        
        store_who_summary(rec_summ, pool).await?;             // add or update summary database record

        let mut full_path = PathBuf::from ("");
        let db_name = fetch_src_db_name()?;

        if source_id != 100120  && source_id != 100126 {           // file production not necessary for these sources
  
            match processor::process_line(who_line, source_id, &sid,         // get full version of WHO record
                study_type, study_status, &remote_url, idents, countries)
            {
                Some (rec) => {
                    let file_folder: PathBuf = [json_path, &PathBuf::from(&db_name)].iter().collect();
                    if !folder_exists(&file_folder) {
                        fs::create_dir_all(&file_folder)?;
                    }
                    let file_name = format!("{}.json", &sid);
                    full_path = [file_folder, PathBuf::from(&file_name)].iter().collect();
        
                    // Write the JSON string to a file - see if it is a new download, or an existing one.
                    
                    let json_string = to_string_pretty(&rec).unwrap();
                    let mut file = File::create(&full_path)?;
                    file.write_all(json_string.as_bytes())?;
                },

                None => continue,  // some sort of problem occured - should have been loggged
            }
        }

        // Update database and res accordingly.
        let added = update_who_study_mon(&db_name, &sid, &remote_url, dl_id,
                        &date_of_rec, &full_path, pool).await?;

        file_res.num_downloaded +=1;
        if added {
            file_res.num_added +=1;
        } 
    }

    info!("{} records checked in total for this file", file_res.num_checked);
    info!("---------------------------------------------------");

    // Update database with single file details and 
    // return the aggregate figures in the res struct ... 

    add_new_single_file_record(dl_id, file_path, &file_res, pool).await?;
    add_contents_record(file_path, &mut source_tots, pool).await?;

    Ok(file_res)

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
