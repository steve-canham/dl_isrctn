use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use crate::err::AppError;
use chrono::Utc;

#[allow(dead_code)]
pub async fn update_isrctn_mon(sd_sid: &String, remote_url: &String, dl_id: i32,
                     record_date: &Option<String>, full_path: &PathBuf, src_pool: &Pool<Postgres>) -> Result<bool, AppError> {

        let mut added = false;          // indicates if will be a new record or update of an existing one
        let now = Utc::now();
        let local_path = full_path.to_str().unwrap();  // assumes utf-8 characters
        
        let sql = format!("SELECT EXISTS(SELECT 1 from mn.source_data where sd_sid = '{}')", sd_sid); 
        let mon_record_exists = sqlx::query_scalar(&sql).fetch_one(src_pool).await
                        .map_err(|e| AppError::SqlxError(e, sql))?;

        if mon_record_exists {   // Row already exists - update with new details.
            
            let sql = r#"Update mn.source_data set 
                        remote_url = $2,
                        last_revised = $3::timestamp,
                        local_path = $4,
                        last_dl_id = $5,
                        last_downloaded = $6
                        where sd_sid = $1;"#;
            sqlx::query(&sql).bind(sd_sid).bind(remote_url).bind(record_date)    
            .bind(local_path).bind(dl_id).bind(now).execute(src_pool).await
                    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;       
        }
        else {   // Create as a new record.
            
            let sql = r#"Insert into mn.source_data(sd_sid, remote_url, last_revised,
	                    local_path, last_dl_id, last_downloaded) values ($1, $2, $3::timestamp, $4, $5, $6)"#;
            sqlx::query(&sql).bind(sd_sid).bind(remote_url).bind(record_date)    
            .bind(local_path).bind(dl_id).bind(now).execute(src_pool).await
                    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;     
            added = true;  
        }
        Ok(added)
}


pub async fn get_next_import_id(_import_type: &String, mon_pool: &Pool<Postgres>) -> Result<i32, AppError>{

    let sql = "select max(id) from evs.dl_events ";
    let last_id: i32 = sqlx::query_scalar(sql).fetch_one(mon_pool)
                      .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    let new_id = last_id + 1;
    
    // Create the new record (to be updated later).

    let now = Utc::now();
    let sql = "Insert into evs.dl_events(id, source_id, time_started) values ($1, $2, $3)";
    sqlx::query(sql).bind(new_id).bind(100126).bind(now).execute(mon_pool)
             .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(new_id)
}

pub async fn update_imp_event_record (import_type: &String, imp_ev_id: i32, num_imported: i64, mon_pool: &Pool<Postgres>) ->  Result<bool, AppError> {
     
    let now = Utc::now();
    let sql = r#"Update evs.dl_events set 
             num_records_checked = $1,
             num_records_downloaded = $2,
             num_records_added = $3,
             time_ended = $4,
             type_id = $5
             where id = $6"#;
    let res = sqlx::query(sql).bind(num_imported)
          .bind(now).bind(import_type).bind(imp_ev_id).execute(mon_pool)
             .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}
