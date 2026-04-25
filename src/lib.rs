
pub mod setup;
pub mod err;
pub mod base_types;
pub mod recording;
mod download;
mod import;
mod data_models;
mod helpers;
mod iec;

use crate::base_types::{DownloadType, ImportType, EncodingType};
use crate::recording::events::EventRepo;
use setup::cli_reader;
use setup::db_pars::get_db_pool;
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;
use chrono::NaiveDate;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    // Obtain starting parameters from the CLi and the config file.
    // See the Readme.md file for details of the various types of download availaable, and
    // the types of import and coding, and the flags and parameters associated with them.

    let cli_pars = cli_reader::fetch_valid_arguments(args)?;
    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
    let mut params = setup::get_params(cli_pars, &config_string)?;

    // Set up the access to the monitoring database using an 'events' database repository
    // object. This is necesary for the following two steps, which involve DB access.

    let mon_pool = get_db_pool("monitor").await?;  // pool for the events db
    let events = recording::events::EventRepo::new(mon_pool.clone());  // events repo object

    // Obtain the source name, (will stop execution if the source id cannot be matched).
    // Then for the 'download recent' types, try and find a start date if one was missing.

    params.source_name = get_source_name(&events, params.source_id).await?;
    if params.download_type == DownloadType::Recent
        && params.start_date == NaiveDate::from_ymd_opt(1900, 1, 1) {
        params.start_date = get_start_date_from_db(&events, params.source_id).await?;
    }

    // If reached here we are good to go. Establish log and then carry out download,
    // and / or import and / or coding as directed by the starting parameters.

    setup::establish_log(&params)?;
    if params.download_type != DownloadType::None {   // a download requested

        let dl_id = events.get_next_download_id(params.source_id, &params.download_type).await?;
        let dl_res = download::download_data(&params, dl_id).await?;
        events.update_dl_event_record (dl_id, dl_res, &params).await?;
    }
    if params.import_type != ImportType::None {     // an import requested

        let imp_id = events.get_next_import_id(&params.import_type).await?;
        let imp_res = import::import_data(&params.import_type, imp_id).await?;
        events.update_imp_event_record (imp_id, imp_res).await?;
    }
    if params.encoding_type != EncodingType::None {     // coding requested



    }

    Ok(())
}


async fn get_source_name(events: &EventRepo, source_id: i32) -> Result<String, AppError> {

    match events.get_source_name (source_id).await {
        Some(srce_name) => Ok(srce_name),
        None => Err(AppError::MissingProgramParameter("valid source id".to_string())),
    }
}

async fn get_start_date_from_db(events: &EventRepo, source_id: i32) -> Result<Option<NaiveDate>, AppError> {

    match events.get_last_dl_recent_type_date (source_id).await {
        Some(start_date) => Ok(Some(start_date)),
        None => Err(AppError::MissingProgramParameter("valid start date".to_string())),
    }
}
