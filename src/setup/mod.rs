/**********************************************************************************
The setup module, and the get_params function in this file in particular,
orchestrates the collection and fusion of parameters as provided in
1) a config toml file, and
2) command line arguments.
The module also checks the parameters for completeness. If possible, defaults are
used to stand in for mising parameters. If not possible the program stops with
a message explaining the problem.
The module also provides a database connection pool on demand.
***********************************************************************************/

pub mod cli_reader;
pub mod config_reader;
pub mod log_helper;
pub mod db_pars;

use std::fs;
use std::sync::OnceLock;
use crate::err::AppError;
//use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use std::path::PathBuf;
//use std::time::Duration;
//use sqlx::ConnectOptions;
use config_reader::Config;
use cli_reader::CliPars;
use crate::base_types::InitParams;

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn get_params(cli_pars: CliPars, config_string: &String) -> Result<InitParams, AppError> {

    let config_file: Config = config_reader::populate_config_vars(&config_string)?;

    let base_url = config_file.data.api_base_url;
    let source_id = config_file.data.source_id;
    let json_data_path = config_file.folders.json_data_path;

    if !folder_exists(&json_data_path) {
        fs::create_dir_all(&json_data_path)?;
    }

    let log_folder_path = config_file.folders.log_folder_path;
    if !folder_exists(&log_folder_path) {
        fs::create_dir_all(&log_folder_path)?;
    }

    Ok(InitParams {
        source_id: source_id,
        source_name: "".to_string(),
        api_base_url: base_url,
        json_data_path: json_data_path,
        log_folder_path: log_folder_path,
        download_type: cli_pars.download_type,
        import_type: cli_pars.import_type,
        encoding_type: cli_pars.encoding_type,
        start_date: cli_pars.start_date,
        end_date:cli_pars.end_date,
        is_test: cli_pars.is_test,
    })

}

fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false,
        Err(_e) => false,
    };
    res
}


pub fn establish_log(params: &InitParams) -> Result<(), AppError> {

    if !log_set_up() {  // can be called more than once in context of integration tests
        log_helper::setup_log(&params.log_folder_path)?;
        LOG_RUNNING.set(true).unwrap(); // should always work
        log_helper::log_startup_params(&params);
    }
    Ok(())
}

pub fn log_set_up() -> bool {
    match LOG_RUNNING.get() {
        Some(_) => true,
        None => false,
    }
}


// Tests
#[cfg(test)]
//use super::*;

mod tests {

    use super::*;
    use std::ffi::OsString;
    use chrono::{NaiveDate, Utc};
    use crate::base_types::{DownloadType, ImportType};

    #[test]
    fn check_results_with_min_download_params() {
        let config = r#"
[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="pg_user"
db_password="foo"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-s", "2020-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();
        let res = get_params(cli_pars, &config_string).unwrap();
        let today = Utc::now().date_naive();

        assert_eq!(res.api_base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.source_id, 100126);
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/isrctn"));
        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::Recent);
        assert_eq!(res.start_date, Some(NaiveDate::from_ymd_opt(2020, 12, 4).unwrap()));
        assert_eq!(res.end_date, Some(today));
    }


    #[test]
    fn check_results_with_year_download_params() {
        let config = r#"
[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="pg_user"
db_password="foo"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-y", "-s", "2024"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();
        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.api_base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.source_id, 100126);
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/isrctn"));
        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::ByYear);
        assert_eq!(res.start_date, Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
        assert_eq!(res.end_date, Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));

    }


    #[test]
    fn check_results_with_import_recent_params() {
        let config = r#"
[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="pg_user"
db_password="foo"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-i"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();
        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.api_base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.source_id, 100126);
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/isrctn"));
        assert_eq!(res.import_type, ImportType::Recent);
        assert_eq!(res.download_type, DownloadType::None);
        assert_eq!(res.start_date, None);
        assert_eq!(res.end_date, None);
    }

}
