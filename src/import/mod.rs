pub mod monitoring;
mod processor;
mod db_sd_tables;
use std::fs;
use std::path::PathBuf;

//use crate::data_models::db_models;
use crate::data_models::json_models::*;
use crate::AppError;
use sqlx::{Pool, Postgres};
use crate::base_types::{ImportType, ImportResult};
use chrono::Local;
use log::info;


#[derive(sqlx::FromRow)]
struct FilePath {
    sd_sid: String,
    local_path: String,
}

pub async fn import_data(import_type: &ImportType, _imp_event_id:i32, src_pool: &Pool<Postgres>) -> Result<ImportResult, AppError> {

    // First recreate the staging database tables

    db_sd_tables::build_sd_tables(src_pool).await?;

    let count_sql = match import_type {
        ImportType::Recent => {
            r#"select count(*) from mn.source_data
            where last_imported is null 
            or last_downloaded > last_imported"#
        },
        ImportType::All => {
            "select count(*) from mn.source_data;"
        }
        ImportType::None => ""
    };

    let num_files: i64 = sqlx::query_scalar(count_sql).fetch_one(src_pool).await
                    .map_err(|e| AppError::SqlxError(e, count_sql.to_string()))?;

    for n in (0..num_files).step_by(1000) { 

        let file_sql = match import_type {
            ImportType::Recent => {
                    format!(r#"select sd_sid, local_path from mn.source_data
                    where last_imported is null 
                    or last_downloaded > last_imported
                    ORDER BY sd_sid
                    offset {} limit 1000"#, n)
            },
            ImportType::All  => {
                    format!(r#"select sd_sid, local_path from mn.source_data
                    ORDER BY sd_sid
                    offset {} limit 1000"#, n)
            }
            ImportType::None => "".to_string()
        };

        let file_list: Vec<FilePath> = sqlx::query_as(&file_sql).fetch_all(src_pool).await
                        .map_err(|e| AppError::SqlxError(e, file_sql))?;      
        let mut i = 0;
        for path in file_list {
            
            // Deserialise the file beiung referneced and pass for processing

            let p= PathBuf::from(&path.local_path);
            let json_data = fs::read_to_string(&p)?;
            let s: Study = serde_json::from_str(&json_data)?;
            processor::process_study_json(&path.sd_sid, s)?;

            i += 1;
            if i> 4 { break;}
        } 

        if n > 2000 {
            break;
        }

    }

    info!("number of files found: {}",  num_files);

    Ok(ImportResult {
        num_available: num_files,
        num_imported: 0,
        earliest_dl_date: Local::now().date_naive(),
        latest_dl_date: Local::now().date_naive(),
    })
}
