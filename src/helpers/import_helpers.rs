use std::sync::LazyLock;
use regex::Regex;

use chrono::{Local, NaiveDate};

pub fn option_from_count<T>(v: Vec<T>) -> Option<Vec<T>> {
    match v.len() {
        0 => None,
        _ => Some(v),
    }
}


pub fn split_date_string (ds: Option<String>) -> (Option<i32>, Option<i32>, Option<String>) {

    static RE_ISO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap());
    match ds {
        Some(d) => {
            if RE_ISO.is_match(&d) {     // Check in the form of "YYYY-MM-DD" 

                let y: i32 = d[0..4].to_string().parse().unwrap_or(0);
                let m: u32 = d[5..7].to_string().parse().unwrap_or(0);
                let d: u32 = d[8..].to_string().parse().unwrap_or(0);

                if let Some (dt) = NaiveDate::from_ymd_opt(y, m, d) {
                    let today = Local::now().date_naive();
                    let d_type = if dt < today {"a".to_string()} else {"e".to_string()};
            
                    (Some(y), Some(m as i32), Some(d_type))
                }
                else {
                    (None, None, None)
                }
            }
            else {
                (None, None, None)
            }
        }, 
        None => (None, None, None),
    }
}


pub fn date_from_iso_string(ds: Option<String>) -> Option<NaiveDate> {

    static RE_ISO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap());
    match ds {
        Some(d) => {
            if RE_ISO.is_match(&d) {  

                let y: i32 = d[0..4].to_string().parse().unwrap_or(0);
                let m: u32 = d[5..7].to_string().parse().unwrap_or(0);
                let d: u32 = d[8..].to_string().parse().unwrap_or(0);

                if let Some (dt) = NaiveDate::from_ymd_opt(y, m, d) 
                {
                    Some(dt)
                }
                else {
                    None
                }
            }
            else {
                None
            }
        },
        None => None,

    }


}

pub fn get_study_type (st: &Option<String>) -> i32 {

    match st {
        Some(t) => {
              match t.to_ascii_lowercase().as_str() {
                "interventional" => 11,
                "observational" => 12,
                "observational patient registry" => 13,
                "patient registry" => 13,
                "expanded access" => 14,
                "funded programme" => 15,
                "diagnostic test " => 16,
                _ => 99,
            }
        }, 
        None => 0,
    }
}


pub fn get_study_status (st: &Option<String>) -> i32 {

    match st {
        Some(t) => {
              match t.to_ascii_lowercase().as_str() {
                "not yet recruiting" => 10,
                "withdrawn" => 12,
                "recruiting" => 15,
                "enrolling by invitation" => 16,
                "suspended" => 19,
                "ongoing, recruitment status unclear" => 22,
                "ongoing, no longer recruiting" => 25,
                "terminated" => 28,
                "completed" => 30,
                "not applicable" => 98,
                _ => 99,
            }
        }, 
        None => 0,
    }
}


pub fn get_age_units (au: &Option<String>) -> Option<i32> {

    match au {
        Some(d) => {
              match d.to_ascii_lowercase().as_str() {
                "years" => Some(17),
                "months" => Some(16),
                "weeks" => Some(15),
                "days" => Some(14),
                "hours" => Some(13),
                "minutes" => Some(112),
                _ => None,
            }
        }, 
        None => None,
    }
}

