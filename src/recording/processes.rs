
use crate::AppError;
use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use chrono::Utc;


pub async fn update_mon_table(sd_sid: &String, remote_url: &String, dl_id: i32,
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
