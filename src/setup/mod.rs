/**********************************************************************************
The setup module, and the get_params function in this file in particular, 
orchestrates the collection and fusion of parameters as provided in 
1) a config toml file, and 
2) command line arguments. 
Where a parameter may be given in either the config file or command line, the 
command line version always over-writes anything from the file.
The module also checks the parameters for completeness (those required will vary, 
depending on the activity specified). If possible, defaults are used to stand in for 
mising parameters. If not possible the program stops with a message explaining the 
problem.
The module also provides a database connection pool on demand.
***********************************************************************************/

pub mod cli_reader;
pub mod config_reader;
pub mod log_helper;
pub mod create_tables;

use std::fs;
use std::sync::OnceLock;
use crate::err::AppError;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use std::path::PathBuf;
use std::time::Duration;
use sqlx::ConnectOptions;
use config_reader::Config;
use cli_reader::CliPars;
use std::ffi::OsStr;

pub struct InitParams {
    pub csv_data_path: PathBuf,
    pub json_data_path: PathBuf,
    pub log_folder_path: PathBuf,
    pub importing: bool,
    pub transforming: bool,
}

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn get_params(cli_pars: CliPars, config_string: &String) -> Result<InitParams, AppError> {

    let config_file: Config = config_reader::populate_config_vars(&config_string)?; 
    let folder_pars = config_file.folders;  
    let csv_data_path = folder_pars.csv_data_path;
    let json_data_path = folder_pars.json_data_path; 

    if !folder_exists(&json_data_path) {
        fs::create_dir_all(&json_data_path)?;
    }

    let log_folder_path = folder_pars.log_folder_path;  
    if !folder_exists(&log_folder_path) {
        fs::create_dir_all(&log_folder_path)?;
    }
    
    Ok(InitParams {
        csv_data_path: csv_data_path,
        json_data_path: json_data_path,
        log_folder_path: log_folder_path,
        importing: cli_pars.importing,
        transforming: cli_pars.transforming,
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


pub async fn get_mon_db_pool() -> Result<PgPool, AppError> {  

    // Establish DB name and thence the connection string
    // (done as two separate steps to allow for future development).
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = match config_reader::fetch_mon_db_name() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
}


pub async fn get_src_db_pool() -> Result<PgPool, AppError> {  

    // Establish DB name and thence the connection string
    // (done as two separate steps to allow for future development).
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = match config_reader::fetch_src_db_name() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
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


pub fn get_files_to_process(data_folder: &PathBuf, last_file: &String) -> Result<Vec<String>, AppError> {
    
    let last_file_as_buf = PathBuf::from(last_file);
    let last_file_as_osstr: &OsStr = last_file_as_buf.as_os_str();

    // Get list of csv files in the source folder.
    // 1) Filter out all those directory entries which couldn't be read.
    // 2) Map the directory entries to paths
    // 3) Filter out all paths with extensions other than `csv`

    let csv_paths = std::fs::read_dir(data_folder)?    // Read_dir provides an list of Result<DirEntry, Error>
             .filter_map(|res| res.ok())                   // The list now just the valid DirEntries (.ok generates an option
                                                           // ) but filter_map includes only those with a Some() value
             .map(|dir_entry| dir_entry.path())            // Mapped across to the paths included in the DirEntries
             .filter_map(|path| {
                if path.extension().map_or(false, |ext| ext == "csv") {     //  filter_map filters on Some()) values as 
                                                                            //  generated by the closure. In the closure, the map_or function
                                                                            //  generates true if the path has a .csv extension,
                                                                            //  false otherwise, which determines if the if branch 
                                                                            //  is followed (for Some(path)) or the else.       
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();      
 
    // Generates a collection of PathBufs.
    // Iterate over and compare with the last_file.
    
    let files = csv_paths.iter()
            .filter_map(|p| p.file_name())
            .filter_map(|f| 
                     if f >  last_file_as_osstr {
                        Some(f)
                     }
                    else {
                        None
                    })
            .map(|f| f.to_str().unwrap().to_string())   // assumes utf-8 characters
            .collect::<Vec<_>>();
   
    Ok(files)
}


// Tests
#[cfg(test)]

mod tests {

    use super::*;
    use std::ffi::OsString;
    #[test]
    fn check_results_with_no_params() {
        let config = r#"
[folders]
csv_data_path="/home/steve/Data/MDR source data/ANZCTR"
json_data_path="/home/steve/Data/MDR json files/anz"
log_folder_path="/home/steve/Data/MDR/MDR_Logs/anz"

[database]
db_host="localhost"
db_user="pg_user"
db_password="foo"
db_port="5432"
mon_db_name="mon"
src_db_name="anz"
        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();
        
        assert_eq!(res.csv_data_path, PathBuf::from("E:/MDR source data/WHO/data"));
        assert_eq!(res.json_data_path, PathBuf::from("E:/MDR source files"));
        assert_eq!(res.log_folder_path, PathBuf::from("E:/MDR/MDR Logs"));
    }

    #[test]
    fn check_cli_vars_overwrite_config_values() {
        let config = r#"
[folders]
csv_data_path="/home/steve/Data/MDR source data/ANZCTR"
json_data_path="/home/steve/Data/MDR json files/anz"
log_folder_path="/home/steve/Data/MDR/MDR_Logs/anz"

[database]
db_host="localhost"
db_user="pg_user"
db_password="foo"
db_port="5432"
mon_db_name="mon"
src_db_name="anz"
        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "503", "-f", "dummy who file.csv"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.csv_data_path, PathBuf::from("E:/MDR source data/WHO/data"));
        assert_eq!(res.json_data_path, PathBuf::from("E:/MDR source files"));
        assert_eq!(res.log_folder_path, PathBuf::from("E:/MDR/MDR Logs"));
    }
}