/***************************************************************************
 * Establishes the log for the programme's operation using log and log4rs, 
 * and includes various helper functions.
 * Once established the log file appears to be accessible to any log
 * statement within the rest of the program (after 'use log:: ...').
 ***************************************************************************/

use chrono::Local;
use std::path::PathBuf;
use crate::base_types::*;
use crate::AppError;

use log::{info, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};


pub fn setup_log (data_folder: &PathBuf) -> Result<log4rs::Handle, AppError> {
    let log_file_path = get_log_file_path(data_folder);
    config_log (&log_file_path)
}

fn get_log_file_path(data_folder: &PathBuf) -> PathBuf {
    
    let datetime_string = Local::now().format("%m-%d %H%M%S").to_string();
    let log_file_name = format!("ISRCTN DL {} ", datetime_string);
    [data_folder, &PathBuf::from(&log_file_name)].iter().collect()
    
}

fn config_log (log_file_path: &PathBuf) -> Result<log4rs::Handle, AppError> {
    
    // Initially establish a pattern for each log line.

    let log_pattern = "{d(%d/%m %H:%M:%S)}  {h({l})}  {({M}.{L}):>38.48}:  {m}\n";

    // Define a stderr logger, as one of the 'logging' sinks or 'appender's.

    let stderr = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(log_pattern)))
        .target(Target::Stderr).build();

    // Define a second logging sink or 'appender' - to a log file (provided path will place it in the current data folder).

    let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new(log_pattern)))
            .build(log_file_path)
            .map_err(|e| AppError::IoWriteErrorWithPath(e, log_file_path.to_owned()))?;
    
    // Configure and build log4rs instance, using the two appenders described above

    let config = Config::builder()
        .appender(Appender::builder()
                .build("logfile", Box::new(logfile)),)
        .appender(Appender::builder()
                .build("stderr", Box::new(stderr)),)
        .build(Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Info),
        )
        .map_err(|e| AppError::LogSetupError("Error when creating log4rs configuration".to_string(), e.to_string()))?;

    log4rs::init_config(config)
        .map_err(|e| AppError::LogSetupError("Error when creating log4rs handle".to_string(), e.to_string()))

}


pub fn log_startup_params (ip : &InitParams) {
    
    // Called at the end of set up to record the input parameters
    
    info!("PROGRAM START");
    info!("");
    info!("************************************");
    info!("");
    info!("base_url: {:?}", ip.base_url);
    info!("json data path: {:?}", ip.json_data_path);
    info!("log folder path: {:?}", ip.log_folder_path);

    info!("download data: {:?}", ip.download_type.to_string());
    info!("import data: {:?}", ip.import_type.to_string());
    info!("encoding data: {:?}", ip.encoding_type.to_string());
    let sd = if ip.start_date == None {"none".to_string()} else {ip.start_date.unwrap().format("%Y-%m-%d").to_string()};
    info!("start date: {}", sd);
    let ed = if ip.end_date == None {"none".to_string()} else {ip.end_date.unwrap().format("%Y-%m-%d").to_string()};
    info!("start date: {}", ed);

    info!("");
    info!("************************************");
    info!("");
}
