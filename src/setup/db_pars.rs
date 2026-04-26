use crate::err::AppError;
use crate::setup::config_reader::DB_PARS;
use std::time::Duration;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use sqlx::ConnectOptions;

pub fn fetch_db_conn_string(db: &str) -> Result<String, AppError> {
    let db_pars = DB_PARS.get()
        .ok_or_else(|| AppError::MissingDBParameters())?;

    let db_name = match db {
        "source" => &db_pars.source_db,
        "context" => &db_pars.context_db,
        "monitor" => &db_pars.monitor_db,
        _ => "",   // should never occur
    };

    Ok(format!("postgres://{}:{}@{}:{}/{}",
        db_pars.db_user, db_pars.db_password, db_pars.db_host, db_pars.db_port, db_name))
}


pub async fn get_db_pool(db: &str) -> Result<PgPool, AppError> {

    // Use DB name to get the connection string
    // Use the string to set up a connection options object and change
    // the time threshold for warnings. Set up a DB pool option and
    // connect using the connection options object.

    let db_conn_string = fetch_db_conn_string(&db)?;

    let mut opts: PgConnectOptions = db_conn_string.parse()
        .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db), e))
}
