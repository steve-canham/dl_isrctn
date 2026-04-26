use crate::base_types::*;
use crate::AppError;
use crate::helpers::date_extensions::*;
use sqlx::{Pool, Postgres};
use chrono::{Utc, NaiveDate};

pub struct EventRepo {
    pub pool: Pool<Postgres>,
}

impl EventRepo{
    pub fn new(pool: Pool<Postgres>) -> Self {
        EventRepo {
            pool: pool,
        }
    }

    pub async fn get_source_name (&self, source_id: i32) -> Option<String> {

        let sql = format!(r#"SELECT repo_name FROM src.parameters
                            where id = {}"#, source_id);
        sqlx::query_scalar(&sql).fetch_optional(&self.pool)
                        .await.map_err(|e| AppError::SqlxError(e, sql.to_string())).ok()?
    }


    pub async fn get_next_download_id(&self, source_id: i32, dl_type: &DownloadType) -> Result<i32, AppError>{

        let sql = "select coalesce(max(id), 10001) from evs.dl_events ";
        let last_id: i32 = sqlx::query_scalar(sql).fetch_one(&self.pool)
                          .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        let new_id = last_id + 1;

        // Create the new record (to be updated later).
        //
        let now = Utc::now();
        let sql = "Insert into evs.dl_events(id, source_id, dl_type, time_started) values ($1, $2, $3, $4)";
        sqlx::query(sql).bind(new_id).bind(source_id).bind(dl_type.to_string()).bind(now)
                .execute(&self.pool)
                .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

        Ok(new_id)
    }

    pub async fn update_dl_event_record (&self, dl_id: i32, dl_res: DownloadResult, params: &InitParams) ->  Result<bool, AppError> {

        let par_1 = params.start_date.as_string_opt();
        let par_2 = params.end_date.as_string_opt();
        let now = Utc::now();
        let sql = r#"Update evs.dl_events set
                 time_ended = $2,
                 num_records_checked = $3,
                 num_records_downloaded = $4,
                 num_records_added = $5,
                 par1 = $6,
                 par2 = $7,
                 filefolder_path = $8
                 where id = $1"#;
        let res = sqlx::query(sql).bind(dl_id).bind(now)
                .bind(dl_res.num_checked).bind(dl_res.num_downloaded).bind(dl_res.num_added)
                .bind(par_1).bind(par_2).bind(params.json_data_path.to_string_lossy())
                .execute(&self.pool)
                .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        Ok(res.rows_affected() == 1)
    }

    pub async fn get_last_dl_recent_type_date (&self, source_id: i32) -> Option<NaiveDate> {

            let sql = format!(r#"SELECT max(time_ended)::date FROM evs.dl_events
                    where source_id = {} and dl_type = 'Recently updated'"#, source_id);
            sqlx::query_scalar(&sql).fetch_optional(&self.pool)
                            .await.map_err(|e| AppError::SqlxError(e, sql.to_string())).ok()?
    }


    pub async fn get_next_import_id(&self, import_type: &ImportType) -> Result<i32, AppError>{

        let sql = "select coalesce(max(id), 10001) from evs.imp_events ";
        let last_id: i32 = sqlx::query_scalar(sql).fetch_one(&self.pool)
                          .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        let new_id = last_id + 1;

        // Create the new record (to be updated later).

        let now = Utc::now();
        let sql = "Insert into evs.imp_events(id, source_id, imp_type, time_started) values ($1, $2, $3, $4)";
        sqlx::query(sql).bind(new_id).bind(100126).bind(import_type.to_string()).bind(now)
                .execute(&self.pool)
                .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

        Ok(new_id)
    }

    pub async fn update_imp_event_record (&self,imp_event_id: i32, imp_res: ImportResult) ->  Result<bool, AppError> {

        let now = Utc::now();
        let sql = r#"Update evs.imp_events set
                 time_ended = $2,
                 num_records_available = $3,
                 num_records_imported = $4,
                 earliest_dl_date = $6,
                 latest_dl_date = $6
                 where id = $1"#;
        let res = sqlx::query(sql).bind(imp_event_id).bind(now)
              .bind(imp_res.num_available).bind(imp_res.num_imported)
              .bind(imp_res.earliest_dl_date).bind(imp_res.latest_dl_date)
              .execute(&self.pool)
              .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
        Ok(res.rows_affected() == 1)
    }
}
