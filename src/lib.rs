
pub mod setup;
pub mod err;
pub mod base_types;
mod download;
mod import;
mod data_models;
mod helpers;
mod iec;

use download::monitoring::{get_next_download_id, update_dl_event_record, get_last_dl_recent_type_date};
use import::monitoring::{get_next_import_id, update_imp_event_record};
use crate::base_types::{DownloadType, ImportType, EncodingType};
use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;
use chrono::NaiveDate;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    // The dl_isrctn program uses the API of the ISRCTN web site (https://www.isrctn.com/)
    // to download data about the trials registered on the site.
    // That data is then used to construct json files, that are stored locally, and 
    // which can then later be used by the import and coding processes to construct a 
    // database of the data.

    // There are four types of download.
    // 'Recent' (-r in the CLI) identifies and downloads studies edited since a cut-off date, 
    // usually from the previous week (i.e., the date of the most recent download). It must be 
    // accompanied by a date parameter in ISO format (e.g. -s "2025-10-18"), or be able to obtain such
    // a parameter from the database record of previous downloads of the same type.
   
    // 'UdBetweenDates' (-b in the CLI) downloads all records that were last edited
    // between two dates. 
    // 'CrBetweenDates' (-c in the CLI) downloads all records that were created (more exactly,)
    // applied for inclusion in ISRCTN) between two dates. 

    // 'ByYear' (-y in the CLI) can be used to download all studies that applied for inclusion 
    // to ISRTCN in a specified year, and is designed for bulk download scenarios, such as 
    // rebuilding the whole ISRCTN dataset from scratch.

    // All procedures need a start and end date, but in the case of type 'Recent' the
    // end date is taken as the current date, and the case of 'ByYear' the dates are the first
    // date of the year, and the first date of the following year.

    // Imports can be of recently downloaded files, i.e. since the last import (-i in the CLI)
    // or can be of all downloaded files (-I in the CLI).

    // Coding can be of just uncoded data, or be a recoding of all data
    
    let cli_pars = cli_reader::fetch_valid_arguments(args)?;

    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
        let mut params = setup::get_params(cli_pars, &config_string)?;

    let src_pool = setup::get_db_pool("db").await?; // pool for the source specific db
    let mon_pool = setup::get_db_pool("mon").await?;  // pool for the monitoring db

    // Here, possibly modify start date for a 'recent' download type
    // If date not given in CLI it may be available from the DB....
    // Check to see if this is the case - if not post an error and stop program
    // If date present use it as the start date parameter...

    if params.download_type == DownloadType::Recent 
        && params.start_date == NaiveDate::from_ymd_opt(1900, 1, 1) {

        if let Some(nd) = get_last_dl_recent_type_date (100126, &mon_pool).await {
            params.start_date = Some(nd);
        }
        else {
            return Result::Err(AppError::MissingProgramParameter("valid start date".to_string()));
        }
    }

    setup::establish_log(&params)?;

    if params.download_type != DownloadType::None {

        // a download requested

        let dl_id = get_next_download_id(100126, &params.download_type, &mon_pool).await?;
        let dl_res = download::download_data(&params, dl_id, &src_pool).await?;
        update_dl_event_record (dl_id, dl_res, &params, &mon_pool).await?;
    }


    if params.import_type != ImportType::None {
        
        // an import requested

        let imp_id = get_next_import_id(&params.import_type, &mon_pool).await?;
        let imp_res = import::import_data(&params.import_type, imp_id, &src_pool).await?;
        update_imp_event_record (imp_id, imp_res, &mon_pool).await?;
    }


    if params.encoding_type != EncodingType::None {
        
        // coding requested

    }
    
    Ok(())  
}



