
pub mod setup;
pub mod err;
mod download;

use download::data_access::{get_next_download_id, update_dl_event_record};
use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;

#[derive(Clone)]
pub struct DownloadResult {
    pub num_checked: i32,
    pub num_downloaded: i32,
    pub num_added: i32,
}

impl DownloadResult {
    pub fn new() -> Self {
        DownloadResult {  
        num_checked: 0,
        num_downloaded: 0,
        num_added: 0,
        }
   }

   pub fn add(&self, other: DownloadResult ) -> Self {
        DownloadResult {  
            num_checked: self.num_checked + other.num_checked,
            num_downloaded: self.num_downloaded + other.num_downloaded,
            num_added: self.num_added + other.num_added,
        }
    }
}

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    let cli_pars: cli_reader::CliPars;
    cli_pars = cli_reader::fetch_valid_arguments(args)?;
    
    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
                              
    let params = setup::get_params(cli_pars, &config_string)?;

    setup::establish_log(&params)?;
    let mon_pool = setup::get_mon_db_pool().await?;  // pool for the monitoring db
    let src_pool = setup::get_src_db_pool().await?;  // pool for the source specific db

    // Download type is constant - reading data from a set of csv files.
    // First recreate the sd schema tables, get Id of this download,
    // then import the data into the sd tables
    // before updating the download record.

    setup::create_tables::create_tables(&src_pool).await?;

    let dl_id = get_next_download_id(&mon_pool).await?;
    let res = download::process_files(&params.csv_data_path, &params.json_data_path, dl_id, &src_pool).await?;
    update_dl_event_record (dl_id, 1, res, &mon_pool).await?;
    
    Ok(())  
}



