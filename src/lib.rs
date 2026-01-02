
pub mod setup;
pub mod err;
pub mod base_types;
mod download;
mod import;
mod data_models;

use download::monitoring::{get_next_download_id, update_dl_event_record};
use import::monitoring::{get_next_import_id, update_imp_event_record};
use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;

use crate::base_types::DownloadType;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    // The dl_isrctn program uses the API of the ISRCTN web site (https://www.isrctn.com/)
    // to download data about the trials registered on the site.
    // That data is then used to construct json files, that are stored locally, and 
    // which can then later be used to construct a database of the data.

    // There are three types of download.
    // Type 111 (-t 111 in the CLI) identifies and downloads studies edited since a cut-off date, 
    // usually from the previous week (i.e., the date of the most recent download). It must be 
    // accompanied by a date parameter in ISO format (e.g. -s "2025-10-18")
   
    // Type 115 (-t 115 in the CLI) downloads all records that were last edited
    // between two dates. Running against this type in batches allows all ISRCTN records to be
    // re-downloaded, if and when necessary. Calling -t 115 requires two date
    // parmameters, for the start and end dates respectively, e.g. 
    // -s "2023-10-01" -e "2023-10-31"

    // Both procedures need a start and end date, but in the case of type 111 the
    // end date is taken as the current date.

    // Type 117 (-t 117 in the CLI) can be used to download all records for a specified year,
    // and is designed for bulk download scenarios. It takes a single parameter (e.g. -y 2009),
    // and constructs start and end dates for that year, calling the -t 115 routine with those dates.
    // It therefore wraps the -t 115 download type.
    
    let cli_pars: cli_reader::CliPars;
    cli_pars = cli_reader::fetch_valid_arguments(args)?;
    
    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
                              
    let params = setup::get_params(cli_pars, &config_string)?;

    setup::establish_log(&params)?;
    let mon_pool = setup::get_mon_db_pool().await?;  // pool for the monitoring db
    let src_pool = setup::get_src_db_pool().await?;  // pool for the source specific db

    if params.dl_type != DownloadType::None {

        // a download reuested

        let dl_id = get_next_download_id(&params.dl_type, &mon_pool).await?;
        let dl_res = download::download_data(&params, dl_id, &src_pool).await?;
        update_dl_event_record (dl_id, dl_res, &params, &mon_pool).await?;
    }
    else {
        
        // an import requested
        let imp_id = get_next_import_id(&params.import_type, &mon_pool).await?;
        let imp_res = import::import_data(&params.import_type, imp_id, &src_pool).await?;
        update_imp_event_record (imp_id, imp_res, &mon_pool).await?;
    }
    
    Ok(())  
}



