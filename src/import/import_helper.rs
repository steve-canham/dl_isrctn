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
                    let d_type = if dt > today {"a".to_string()} else {"e".to_string()};
            
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


#[allow(dead_code)]
pub trait StringExtensions {
    fn tidy(&self) -> Option<String>;
    fn replace_unicodes(&self) -> Option<String>;
    fn replace_tags_and_unicodes(&self) -> Option<String>;
    fn regularise_hyphens(&self) -> Option<String>;
    fn compress_spaces(&self) -> Option<String>;
    fn replace_apostrophes(&self) -> Option<String>;
}

impl StringExtensions for String {
        
    fn replace_apostrophes(&self) -> Option<String> {
    
        let quoteless = self.trim_matches('"');
        let lower = quoteless.to_lowercase();
        if lower == "null" || lower == "n/a"
        || lower == "na" || lower == "none"
        {
            None
        }
        else {

            let mut a_name = quoteless.replace("&#44;", ","); // unusual but it can occur
            a_name = a_name.replace("&#39;", "'"); // unusual but it can occur

            if a_name.contains('\'') {

                // Do a blanket replacement of apostrophes to RSQs.
                // Then deal with situations where a LSQ applies

                a_name = a_name.replace("'", "’");
                
                if a_name.starts_with('’') {
                    let mut chars = a_name.chars();
                    chars.next();
                    a_name = format!("‘{}", chars.as_str());
                }

                a_name = a_name.replace(" ’", " ‘");
                a_name = a_name.replace("(’", "(‘");
            }

            Some(a_name)
           
        }
    }
    


    fn tidy(&self) -> Option<String> {

        let quoteless = self.trim_matches('"');
        let lower = quoteless.to_lowercase();
        if lower == "null" || lower == "n/a"
        || lower == "na" || lower == "none"
        {
            None
        }
        else {
            let complex_trim = |c| c == ' ' || c == ';';
            let trimmed = quoteless.trim_matches(complex_trim);
            if trimmed == "" {
                None
            }
            else {
                Some(trimmed.to_string())
            }
        }
    }


    fn replace_unicodes(&self) -> Option<String> {

        let quoteless = self.trim_matches('"');
        let lower = quoteless.to_lowercase();
        if lower == "null" || lower == "n/a"
        || lower == "na" || lower == "none"
        {
            None
        }
        else {
            let complex_trim = |c| c == ' ' || c == ';';
            let trimmed = quoteless.trim_matches(complex_trim);
            if trimmed == "" {
                None
            }
            else {
                let mut output = trimmed.to_owned();
                output = output.replace("&#32;", " ").replace("&#37;", "%");
                output = output.replace("#gt;", ">").replace("#lt;", "<");       
                output = output.replace("&#39;", "’").replace("&rsquo;", "’");
                output = output.replace("&quot;", "'");
                output = output.replace("&gt;", ">").replace("&lt;", "<");
                output = output.replace("&amp;", "&");
                Some(output)
            }
        }
    }


    fn replace_tags_and_unicodes(&self) -> Option<String> {
        let quoteless = self.trim_matches('"');
        let lower = quoteless.to_lowercase();
        if lower == "null" || lower == "n/a"
        || lower == "na" || lower == "none"
        {
            None
        }
        else {
            let complex_trim = |c| c == ' ' || c == ';';
            let trimmed = quoteless.trim_matches(complex_trim);
            if trimmed == "" {
                None
            }
            else {
                let mut output = trimmed.to_owned();
                
                output = output.replace("<p>", "\n");
                output = output.replace("<br>", "\n");
                output = output.replace("<br/>", "\n");
                output = output.replace("<br />", "\n");
                output = output.replace("\n\n", "\n").replace("\n \n", "\n");
                output = output.replace(",,", ",");
                output = output.replace("</p>", "");

                output = output.replace("&#32;", " ").replace("&#37;", "%");
                output = output.replace("#gt;", ">").replace("#lt;", "<");       
                output = output.replace("&#39;", "’").replace("&rsquo;", "’");
                output = output.replace("&quot;", "'");
                output = output.replace("&gt;", ">").replace("&lt;", "<");
                output = output.replace("&amp;", "&");
                Some(output)
            }
        }
    }


    fn regularise_hyphens(&self) -> Option<String> {

        // assumed this call is immediately after a 'tidy' call, either
        // on a string or string option, so only basic null check required

        if self.trim() == "" {
            None
        }
        else {

            let mut output_string = self.replace("\u{2010}", "-"); 
            output_string = output_string.replace("\u{2011}", "-"); 
            output_string = output_string.replace("\u{2012}", "-"); 
            output_string = output_string.replace("\u{2013}", "-"); 
            output_string = output_string.replace("\u{2212}", "-"); 

            Some(output_string)
        }

    }
    

    fn compress_spaces(&self) -> Option<String> {
    
       let trimmed = self.trim();
       if trimmed == "NULL" ||  trimmed == "null" ||  trimmed == "\"NULL\"" ||  trimmed == "\"null\""
            ||  trimmed == ""
        {
            None
        }
        else {
            
            let mut output_string = trimmed.replace("\r\n", "\n");    // regularise endings
            output_string = output_string.replace("\r", "\n");

            while output_string.contains("  ")
            {
                output_string = output_string.replace("  ", " ");
            }

            output_string = output_string.replace("\n:\n", ":\n");
            output_string = output_string.replace("\n ", "\n");
            while output_string.contains("\n\n")
            {
                output_string = output_string.replace("\n\n", "\n");
            }

            let result = output_string.trim();
            Some(result.to_string())
        }
    }
}
