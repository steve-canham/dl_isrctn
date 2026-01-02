use std::ops::Add;
use chrono::NaiveDate;
use std::path::PathBuf;

pub struct InitParams {
    pub base_url: String,
    pub json_data_path: PathBuf,
    pub log_folder_path: PathBuf,
    pub import_type: ImportType,
    pub dl_type: DownloadType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}


#[derive(PartialEq, Debug)]
pub enum DownloadType {
    Recent,
    BetweenDates,
    ByYear,
    None
}

impl DownloadType {
    pub fn to_string(&self) -> String {
        match self { 
            DownloadType::Recent => "Recently updated".to_string(), 
            DownloadType::BetweenDates => "Updated between dates".to_string(), 
            DownloadType::ByYear => "Updated in year".to_string(), 
            DownloadType::None => "None".to_string(), 
        }
    }
}

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
}

impl Add for DownloadResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{  
            num_checked: self.num_checked + other.num_checked,
            num_downloaded: self.num_downloaded + other.num_downloaded,
            num_added: self.num_added + other.num_added,
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum ImportType {
    None,
    Recent,
    All,
}

impl ImportType {
    pub fn to_string(&self) -> String {
        match self { 
            ImportType::None => "None".to_string(), 
            ImportType::Recent => "Recently downloaded".to_string(), 
            ImportType::All => "All files".to_string(), 
        }
    }
}

pub struct ImportResult {
    pub num_available: i64,
    pub num_imported: i64,
    pub earliest_dl_date: NaiveDate,
    pub latest_dl_date: NaiveDate,
}
