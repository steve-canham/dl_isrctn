use sqlx::{Pool, Postgres};
use crate::err::AppError;
use chrono::{Utc};
use super::{ImportType, ImportResult};

#[allow(dead_code)]
pub async fn update_isrctn_mon(sd_sid: &String, imp_event_id: i32, src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Row already exists - update with new details.

    let now = Utc::now();
    let sql = r#"Update mn.source_data set 
                last_import_id = $2,
                last_imported = $3
                where sd_sid = $1;"#;
    sqlx::query(&sql).bind(sd_sid).bind(imp_event_id).bind(now)
        .execute(src_pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;       
        
    Ok(())
}


pub async fn get_next_import_id(import_type: &ImportType, mon_pool: &Pool<Postgres>) -> Result<i32, AppError>{

    let sql = "select max(id) from evs.import_events ";
    let last_id: i32 = sqlx::query_scalar(sql).fetch_one(mon_pool)
                      .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    let new_id = last_id + 1;
    
    // Create the new record (to be updated later).

    let now = Utc::now();
    let sql = "Insert into evs.imp_events(id, source_id, imp_type, time_started) values ($1, $2, $3, $4)";
    sqlx::query(sql).bind(new_id).bind(100126).bind(import_type.to_string()).bind(now)
            .execute(mon_pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(new_id)
}

pub async fn update_imp_event_record (imp_event_id: i32, imp_res: ImportResult, mon_pool: &Pool<Postgres>) ->  Result<bool, AppError> {
     
    let now = Utc::now();
    let sql = r#"Update evs.imp_events set 
             time_ended = $2
             num_records_available = $3,
             num_records_imported = $4,
             earliest_dl_date = $6,
             latest_dl_date = $6
             where id = $1"#;
    let res = sqlx::query(sql).bind(imp_event_id).bind(now)
          .bind(imp_res.num_available).bind(imp_res.num_imported)
          .bind(imp_res.earliest_dl_date).bind(imp_res.latest_dl_date)
          .execute(mon_pool)
          .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}

