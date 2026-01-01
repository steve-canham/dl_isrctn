pub mod data_access;

//use crate::data_models::db_models;
//use crate::data_models::json_models;
use crate::AppError;
use sqlx::{Pool, Postgres};
//use log::info;

pub async fn import_data(_import_type: &String, _imp_event_id:i32, _src_pool: &Pool<Postgres>) -> Result<i64, AppError> {


    Ok(0)
}
