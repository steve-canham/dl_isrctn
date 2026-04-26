use std::sync::OnceLock;
use toml;
use serde::Deserialize;
use crate::err::AppError;
use std::path::PathBuf;
use log::info;


#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub data: Option<TomlDataPars>,
    pub folders: Option<TomlFolderPars>,
    pub database: Option<TomlDBPars>,
}

#[derive(Debug, Deserialize)]
pub struct TomlDataPars {
    pub api_base_url: Option<String>,
    pub source_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TomlFolderPars {
    pub json_data_path: Option<String>,
    pub log_folder_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TomlDBPars {
    pub db_host: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_port: Option<String>,
    pub source_db: Option<String>,
    pub monitor_db: Option<String>,
    pub context_db: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub data: DataPars,
    pub folders: FolderPars,
    pub db_pars: DBPars,
}

#[derive(Debug, Clone)]
pub struct DataPars {
    pub api_base_url: String,
    pub source_id: i32,
}

#[derive(Debug, Clone)]
pub struct FolderPars {
    pub json_data_path: PathBuf,
    pub log_folder_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DBPars {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_port: usize,
    pub source_db: String,
    pub monitor_db: String,
    pub context_db: String,
}

pub static DB_PARS: OnceLock<DBPars> = OnceLock::new();

pub fn populate_config_vars(config_string: &String) -> Result<Config, AppError> {

    let toml_config = toml::from_str::<TomlConfig>(&config_string)
        .map_err(|_| {AppError::ConfigurationError("Unable to parse config file.".to_string(),
                                       "File (app_config.toml) may be malformed.".to_string())})?;
 
    let toml_data = toml_config.data
        .ok_or_else(|| AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
            "Cannot find a section called '[data]'.".to_string()))?;
    
    let toml_database = toml_config.database
        .ok_or_else(|| AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
            "Cannot find a section called '[database]'.".to_string()))?;

    let toml_folders = toml_config.folders
        .ok_or_else(|| AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
            "Cannot find a section called '[folders]'.".to_string()))?;

    let config_data = verify_data_parameters(toml_data)?;
    let config_folders = verify_folder_parameters(toml_folders)?;
    let config_db_pars = verify_db_parameters(toml_database)?;

    let _ = DB_PARS.set(config_db_pars.clone());

    Ok(Config{
        data: config_data,
        folders: config_folders,
        db_pars: config_db_pars,
    })
}


fn verify_data_parameters(toml_api: TomlDataPars) -> Result<DataPars, AppError> {

    let base_url = check_essential_string (toml_api.api_base_url, "api base url", "base_url")?;
    let source_id_as_string = check_essential_string (toml_api.source_id, "source id", "source_id")?;
    let source_id: i32 = source_id_as_string.parse().unwrap_or_else(|_| 0);   // zero detected later

    Ok(DataPars {
        api_base_url: base_url,
        source_id: source_id,
    })
}

fn verify_folder_parameters(toml_folders: TomlFolderPars) -> Result<FolderPars, AppError> {

    let json_data_path_string = check_essential_string (toml_folders.json_data_path, "json outputs parents folder", "json_data_path")?;
    let log_folder_path_string = check_essential_string (toml_folders.log_folder_path, "log folder", "log_folder_path")?;

    Ok(FolderPars {
        json_data_path: PathBuf::from(json_data_path_string),
        log_folder_path: PathBuf::from(log_folder_path_string),
    })
}

fn verify_db_parameters(toml_database: TomlDBPars) -> Result<DBPars, AppError> {

    // Check user name and password first as there are no defaults for these values.
    // They must therefore be present.

    let db_user = check_essential_string (toml_database.db_user, "database user name", "db_user")?;
    let db_password = check_essential_string (toml_database.db_password, "database user password", "db_password")?;
    let db_host = check_defaulted_string (toml_database.db_host, "DB host", "localhost");
    let db_port_as_string = check_defaulted_string (toml_database.db_port, "DB port", "5432");
    let db_port: usize = db_port_as_string.parse().unwrap_or_else(|_| 5432);

    let source_db = check_defaulted_string (toml_database.source_db, "Source specific DB name", "isrctn");
    let monitor_db = check_defaulted_string (toml_database.monitor_db, "Monitoring DB name", "mon");
    let context_db = check_defaulted_string (toml_database.context_db, "Context DB name", "cxt");

    Ok(DBPars {
        db_host,
        db_user,
        db_password,
        db_port,
        source_db,
        monitor_db,
        context_db,
    })
}

fn check_essential_string (src_name: Option<String>, value_name: &str, config_name: &str) -> Result<String, AppError> {

    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
        format!("Cannot find a value for {} ({}).", value_name, config_name)))
    }
    else {
        Ok(s)
    }
}

fn check_defaulted_string (src_name: Option<String>, value_name: &str, default:  &str) -> String {
    match src_name {
        Some(s) if !s.trim().is_empty() => s,
        _ => {
            info!("No value found for the {} in config file - using the provided default value ('{}') instead.",
                value_name, default);
            default.to_owned()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Ensure the parameters are being correctly extracted from the config file string

    #[test]
    fn check_config_with_all_params_present() {

        let config = r#"
[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.data.api_base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.data.source_id, 100126);
        assert_eq!(res.folders.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/isrctn"));

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.source_db, "isrctn");
        assert_eq!(res.db_pars.monitor_db, "mon");
        assert_eq!(res.db_pars.context_db, "cxt");
    }

    #[test]
    fn check_non_numeric_source_id_gives_0() {

        let config = r#"
[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "???"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.data.api_base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.data.source_id, 0);
        assert_eq!(res.folders.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/isrctn"));

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.source_db, "isrctn");
        assert_eq!(res.db_pars.monitor_db, "mon");
        assert_eq!(res.db_pars.context_db, "cxt");
    }

    #[test]
    #[should_panic]
    fn check_panics_if_missing_base_url() {

        let config = r#"
[data]
api_base_url = ""
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_panics_if_missing_json_folder () {

        let config = r#"

[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path=""
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_panics_if_missing_log_folder () {

        let config = r#"

[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path=""

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_missing_user_name_panics() {

        let config = r#"

[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_password="password"
db_port="5432"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    fn check_db_defaults_are_supplied() {

        let config = r#"

[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_user="user_name"
db_password="password"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.data.api_base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.data.source_id, 100126);
        assert_eq!(res.folders.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/isrctn"));

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.source_db, "isrctn");
        assert_eq!(res.db_pars.monitor_db, "mon");
        assert_eq!(res.db_pars.context_db, "cxt");
    }


#[test]
    fn missing_port_gets_default() {

        let config = r#"

[data]
api_base_url = "https://www.isrctn.com/api/query/format/default?q="
source_id = "100126"

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
source_db="isrctn"
monitor_db="mon"
context_db="cxt"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.data.api_base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.data.source_id, 100126);
        assert_eq!(res.folders.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/isrctn"));

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.source_db, "isrctn");
        assert_eq!(res.db_pars.monitor_db, "mon");
        assert_eq!(res.db_pars.context_db, "cxt");
   }

}
