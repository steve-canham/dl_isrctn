
use std::sync::OnceLock;
use toml;
use serde::Deserialize;
use crate::err::AppError;
use std::path::PathBuf;


#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub api: Option<TomlAPIPars>, 
    pub folders: Option<TomlFolderPars>, 
    pub database: Option<TomlDBPars>,
}

#[derive(Debug, Deserialize)]
pub struct TomlAPIPars {
    pub base_url: Option<String>,
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
    pub mon_db_name: Option<String>,
    pub src_db_name: Option<String>,
}

pub struct Config {
    pub api: APIPars, 
    pub folders: FolderPars, 
    pub db_pars: DBPars,
}

pub struct APIPars {
    pub base_url: String,
}

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
    pub mon_db_name: String,
    pub src_db_name: String,
}

pub static DB_PARS: OnceLock<DBPars> = OnceLock::new();

pub fn populate_config_vars(config_string: &String) -> Result<Config, AppError> {

    let toml_config = toml::from_str::<TomlConfig>(&config_string)
        .map_err(|_| {AppError::ConfigurationError("Unable to parse config file.".to_string(),
                                       "File (app_config.toml) may be malformed.".to_string())})?;

    let toml_api = match toml_config.api {
        Some (a) => a,
        None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
            "Cannot find a section called '[api]'.".to_string()))},
    };

    let toml_database = match toml_config.database {
        Some(d) => d,
        None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
            "Cannot find a section called '[database]'.".to_string()))},
    };

    let toml_folders = match toml_config.folders {
        Some(f) => f,
        None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
           "Cannot find a section called '[folders]'.".to_string()))},
    };
       
    let config_api = verify_api_parameters(toml_api)?;
    let config_folders = verify_folder_parameters(toml_folders)?;
    let config_db_pars = verify_db_parameters(toml_database)?;

    let _ = DB_PARS.set(config_db_pars.clone());

    Ok(Config{
        api: config_api,
        folders: config_folders,
        db_pars: config_db_pars,
    })
}


fn verify_api_parameters(toml_api: TomlAPIPars) -> Result<APIPars, AppError> {

    let base_url = check_essential_string (toml_api.base_url, "api base url", "base_url")?;

    Ok(APIPars {
        base_url: base_url,
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
       
    let db_host = check_defaulted_string (toml_database.db_host, "DB host", "localhost", "localhost");
            
    let db_port_as_string = check_defaulted_string (toml_database.db_port, "DB port", "5432", "5432");
    let db_port: usize = db_port_as_string.parse().unwrap_or_else(|_| 5432);

    let mon_db_name = check_defaulted_string (toml_database.mon_db_name, "Mon DB name", "mon", "mon");
    let src_db_name = check_defaulted_string (toml_database.src_db_name, "Src DB name", "isrctn", "isrctn");

    Ok(DBPars {
        db_host,
        db_user,
        db_password,
        db_port,
        mon_db_name,
        src_db_name
    })
}


fn check_essential_string (src_name: Option<String>, value_name: &str, config_name: &str) -> Result<String, AppError> {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        return Result::Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
        format!("Cannot find a value for {} ({}).", value_name, config_name)))
    }
    else {
        Ok(s)
    }
}


fn check_defaulted_string (src_name: Option<String>, value_name: &str, default_name: &str, default:  &str) -> String {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        println!("No value found for the {} in config file - using the provided default value ('{}') instead.", 
        value_name, default_name);
        default.to_owned()
    }
    else {
       s
    }
}

pub fn fetch_mon_db_name() -> Result<String, AppError> {
    let db_pars = match DB_PARS.get() {
         Some(dbp) => dbp,
         None => {
            return Result::Err(AppError::MissingDBParameters());
        },
    };
    Ok(db_pars.mon_db_name.clone())
}

pub fn fetch_src_db_name() -> Result<String, AppError> {
    let db_pars = match DB_PARS.get() {
         Some(dbp) => dbp,
         None => {
            return Result::Err(AppError::MissingDBParameters());
        },
    };
    Ok(db_pars.src_db_name.clone())
}

pub fn fetch_db_conn_string(db_name: &String) -> Result<String, AppError> {
    let db_pars = match DB_PARS.get() {
         Some(dbp) => dbp,
         None => {
            return Result::Err(AppError::MissingDBParameters());
        },
    };
    
    Ok(format!("postgres://{}:{}@{}:{}/{}", 
    db_pars.db_user, db_pars.db_password, db_pars.db_host, db_pars.db_port, db_name))
}



#[cfg(test)]
mod tests {
    use super::*;

    // Ensure the parameters are being correctly extracted from the config file string
    
    #[test]
    fn check_config_with_all_params_present() {

        let config = r#"

[api]
base_url = "https://www.isrctn.com/api/query/format/default?q="

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR/MDR_Logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="isrctn"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.api.base_url, "https://www.isrctn.com/api/query/format/default?q=");
        assert_eq!(res.folders.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/isrctn"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("/home/steve/Data/MDR/MDR_Logs/isrctn"));

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.mon_db_name, "mon");
        assert_eq!(res.db_pars.src_db_name, "isrctn");
    }
    

    #[test]
    #[should_panic]
    fn check_panics_if_missing_base_url() {

        let config = r#"
[api]
base_url = ""

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR/MDR_Logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="isrctn"

"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_panics_if_missing_json_folder () {

        let config = r#"

[api]
base_url = "https://www.isrctn.com/api/query/format/default?q="

[folders]
json_data_path=""
log_folder_path="/home/steve/Data/MDR/MDR_Logs/isrctn"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="isrctn"

"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_panics_if_missing_log_folder () {

        let config = r#"

[api]
base_url = "https://www.isrctn.com/api/query/format/default?q="

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path=""

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="isrctn"

"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }
    

    


    #[test]
    #[should_panic]
    fn check_missing_user_name_panics() {

        let config = r#"

[api]
base_url = "https://www.isrctn.com/api/query/format/default?q="

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR/MDR_Logs/isrctn"

[database]
db_host="localhost"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="isrctn"

"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    fn check_db_defaults_are_supplied() {

        let config = r#"

[api]
base_url = "https://www.isrctn.com/api/query/format/default?q="

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR/MDR_Logs/isrctn"

[database]
db_user="user_name"
db_password="password"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.mon_db_name, "mon");
        assert_eq!(res.db_pars.src_db_name, "isrctn");
    }


#[test]
    fn missing_port_gets_default() {

        let config = r#"

[api]
base_url = "https://www.isrctn.com/api/query/format/default?q="

[folders]
json_data_path="/home/steve/Data/MDR json files/isrctn"
log_folder_path="/home/steve/Data/MDR/MDR_Logs/isrctn" 

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
mon_db_name="mon"
src_db_name="anz"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.mon_db_name, "mon");
        assert_eq!(res.db_pars.src_db_name, "anz");
   }

}
  

