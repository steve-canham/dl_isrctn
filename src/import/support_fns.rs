use std::sync::LazyLock;
use regex::Regex;

use chrono::{Local, NaiveDate};

//use crate::helpers::name_extensions;

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



pub fn split_age_string(age: &Option<String>) -> (Option<f64>, Option<String>, f64) {
     
    if let Some(a) = age {
        let age_parts: Vec<&str> = a.split(' ').collect();

        let num: Option<f64> = match age_parts[0].trim().parse() {
            Ok(num) => Some(num),
            Err(_) => None
        };
        let units:Option<&str> = match age_parts[1].to_lowercase().trim() {
            "year" => Some("y"),
            "years" => Some("y"),
            "month" => Some("m"),
            "months" => Some("m"),
            "week" => Some("w"),
            "weeks" => Some("w"),
            "day" => Some("d"),
            "days" => Some("d"),
            "hour" => Some("h"),
            "hours" => Some("h"),
            _ => None,
        };
        let mut num_days: f64 = 0.0;
        if let Some(n) = num {
            num_days = match units {
                Some("y") => n * 365.25,
                Some("m") => n * 30.5,
                Some("w") => n * 7.0,
                Some("d") => n,
                Some("h") => 1.0,
                None => 0.0,
                _ => 0.0,
            };
        }
        let units_as_string: Option<String> = match units {
            Some(u) => Some(u.to_string()),
            None => None,
        };
        (num, units_as_string, num_days)
    }
    else {
        (None, None, 0.0)
    }

}



pub fn get_full_name(given_name: Option<String>, family_name: Option<String>) -> Option<String>{
    
    let giv_n = given_name.unwrap_or_else(||"".to_string());
    let fam_n = family_name.unwrap_or_else(||"".to_string());

    if giv_n == "" && fam_n == "" {
        None
    }
    else {
        Some(format!("{} {}", giv_n, fam_n).trim().to_string())
    }
}


pub fn  get_cr_numbered_strings(input: &String) -> Option<Vec<&str>> {

    static RE_CRNUM_SPLITTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n\d{1,2}\.").unwrap());
    
    let res: Vec<&str> = RE_CRNUM_SPLITTER.split(input)
                  .map(|p| p.trim())
                  .collect();

    let mut result: Vec<&str> = Vec::new();
    if res.len() > 0 {
        for mut r in res {
            if r.starts_with("1.") { r = &r[2..];}
            result.push(r.trim());
        }
    }

    match result.len() {
        0 => None,
        _ => Some(result)
    }
}


pub fn  get_numbered_strings(input: &String) -> Option<Vec<&str>> {

    static RE_NUM_SPLITTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d{1,2}\. ").unwrap());
    
    let result: Vec<&str> = RE_NUM_SPLITTER.split(input)
                        .map(|p| p.trim())
                        .filter(| t| *t != "")
                        .collect();

    match result.len() {
        0 => None,
        _ => Some(result)
    }
}


pub fn  get_comma_delim_strings(input: &String, min_width: usize) -> Vec<String> {

    // The problem is that such strings may often include brackets
    // that themselves contain commas
    // Therefore a need to do a run through the string chars, replacing 'meaningful' commas
    // with '||' and then split on the '||'s.


    // loop over string chars.

    let mut in_brackets = false;
    let mut new_s = "".to_string();

    for c in input.chars() {
        
        match c {
            '(' => {
                in_brackets = true;  new_s.push(c);},
            ')' => {
                in_brackets = false;  new_s.push(c);},
            ',' => {
                    if !in_brackets {
                        new_s.push('|');
                        new_s.push('|');
                    }
                    else {
                        new_s.push(c);
                    }
                },
            _ => new_s.push(c),
        }
    }

    let res: Vec<&str> = new_s.split("||")
                         .map(|p| p.trim())
                         .collect();

    // In addition, somne comma delimited portions are small and in fact are extensions
    // of the item before...

    if res.len() > 1 {
        let mut modified_res: Vec<String> = Vec::new();
        let mut skip_res1 = false;

        for i in 0..res.len() {
            let mut new_string = res[i].to_string();
            if res[i].len() < min_width
            {
                if i == 0    // add to res[1], and skip over res[1]
                {
                    new_string = format!("{}, {}", res[0], res[1]);
                    modified_res.push(new_string.trim().to_string());
                    skip_res1 = true;
                }
                else         // add to the previous item and replace that item
                {
                    new_string = format!("{}, {}", res[i-1], res[i]);  
                    modified_res.pop();
                    modified_res.push(new_string.trim().to_string());
                }
            }
            else {
                if i != 1 || (i == 1 && skip_res1 == false) {
                    modified_res.push(new_string.trim().to_string());
                }
            }

        }
        modified_res
   
    }
    else {
        vec![new_s.to_string()]   // no split, just return input
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    //use crate::helpers::string_extensions::*;

    #[test]
    fn check_get_cr_numbered_strings() {

        let input = &"1. Item 1\n2. item 2. \n3.item3".to_string();
        assert_eq!(get_cr_numbered_strings(input), Some(vec!["Item 1", "item 2.", "item3"]));
    }

    #[test]
    fn check_get_numbered_strings() {

        let input = &"1. Item 1; 2. item 2; 3. item3".to_string();
        assert_eq!(get_numbered_strings(input), Some(vec!["Item 1;", "item 2;", "item3"]));
    }

    #[test]
    fn check_get_comma_delim_strings() {

        let input = &"Item 1, Item 2, Item 3 ".to_string();
        assert_eq!(get_comma_delim_strings(input, 4), vec!["Item 1", "Item 2", "Item 3"]);

        let input = &"Item 1, Item 2 (some stuff, some other stuff), Item 3 ".to_string();
        assert_eq!(get_comma_delim_strings(input, 4), vec!["Item 1", "Item 2 (some stuff, some other stuff)", "Item 3"]);

        let input = &"foo, Item 1, Item 2 (some stuff, some other stuff), Item 3, bar ".to_string();
        assert_eq!(get_comma_delim_strings(input, 4), vec!["foo, Item 1", "Item 2 (some stuff, some other stuff)", "Item 3, bar"]);

    }

    #[test]
    fn check_split_age_strings() {

        let input = &Some("18 Years".to_string());
        let num_days = 18.0 * 365.25;
        assert_eq!(split_age_string(input), (Some(18.0), Some("y".to_string()), num_days));

        let input = &Some("75 Years".to_string());
        let num_days = 75.0 * 365.25;
        assert_eq!(split_age_string(input), (Some(75.0), Some("y".to_string()), num_days));

        let input = &Some("6 Month".to_string());
        assert_eq!(split_age_string(input), (Some(6.0), Some("m".to_string()), 183.0));

        let input = &Some("6.72 Month".to_string());
        let num_days = 6.72 * 30.5;
        assert_eq!(split_age_string(input), (Some(6.72), Some("m".to_string()), num_days));

        let input = &Some("1 day".to_string());
        assert_eq!(split_age_string(input), (Some(1.0), Some("d".to_string()), 1.0));

    }
}