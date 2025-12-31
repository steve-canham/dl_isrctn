use std::sync::LazyLock;
use regex::Regex;
use super::json_models::Identifier;

pub fn count_option<T>(v: Vec<T>) -> Option<Vec<T>> {
    match v.len() {
        0 => None,
        _ => Some(v),
    }
}


pub fn split_identifier(id: &String) -> Vec<String> {
     
    // Attempts to split a string on commas but only at appropriate places

    let mut this_id = id.to_string();

    // As an initial stage try to get the commas replaced when they immediately follow a common id type

    static RE_IRASC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"IRAS:? ?\d{6,7},").unwrap());
    static RE_CPMSC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CPMS:? ?\d{5},").unwrap());
    static RE_NIHRC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NIHR:? ?\d{6},").unwrap());
    static RE_HTAC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"HTA \d{2}/\d{2,3}/\d{2,3}, ").unwrap());
   
    match RE_IRASC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no eu match at all
    }
    
    match RE_CPMSC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no eu match at all
    }

    match RE_NIHRC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no eu match at all
    }

    match RE_HTAC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no eu match at all
    }

    // Then replace remaining commas if they preceeed common combined entities

    if this_id.contains("," ) {   //still
        if this_id.contains(", NIHR") { this_id = this_id.replace(", NIHR", "||NIHR"); }
        if this_id.contains(", CPMS") { this_id = this_id.replace(", CPMS", "||CPMS"); }
        if this_id.contains(", IRAS") { this_id = this_id.replace(", IRAS", "||IRAS"); }
        if this_id.contains(", HTA") {  this_id = this_id.replace(", HTA", "||HTA"); }
        if this_id.contains(", NIHR") { this_id = this_id.replace(", NIHR", "||NIHR"); }
        if this_id.contains(", CDRC") { this_id = this_id.replace(", CDRC", "||CDRC"); }
        if this_id.contains(", CIV-") { this_id = this_id.replace(", CIV-", "||CIV-"); }
        if this_id.contains(", MR") { this_id = this_id.replace(", MR", "||MR"); }
    }

    // Again, try to replace remaining commas if they preceeed common combined entities

    if this_id.contains("," ) {   // still
        if this_id.contains(", Sponsor") { this_id = this_id.replace(", Sponsor", "||Sponsor"); }
        if this_id.contains(", sponsor") { this_id = this_id.replace(", sponsor", "||sponsor"); }
        if this_id.contains(", Protocol") {this_id = this_id.replace(", Protocol", "||Protocol"); }
        if this_id.contains(", protocol") {this_id = this_id.replace(", protocol", "||protocol"); }
        if this_id.contains(", Grant") { this_id = this_id.replace(", Grant", "||Grant"); }
        if this_id.contains(", grant") { this_id = this_id.replace(", grant", "||grant"); }

        if this_id.contains(", Quotient") {this_id = this_id.replace(", Quotient", "||Quotient"); }
        if this_id.contains(", Trial") {this_id = this_id.replace(", Trial", "||Quotient"); }   
    }

    let split_ids: Vec<&str> = this_id.split("||").collect();
    split_ids.iter().map(|&s| s.trim().to_string()).collect()


}


pub fn classify_identifier(ident: Identifier) -> Identifier {

    // Attempts to identify the type of some of the more common / distinctive (non trial registry) identifiers
     
    static RE_IRAS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"IRAS:? ?\d{6,7}").unwrap());
    static RE_CPMS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CPMS:? ?\d{5}").unwrap());
    static RE_NIHR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NIHR:? ?\d{6}").unwrap());
    static RE_HTA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"HTA \d{2}/\d{2,3}/\d{2,3}").unwrap());
    static RE_CCMO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NL\d{5}.\d{3}.\d{2}").unwrap());
    static RE_CIV: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CIV-\d{2}-\d{2}-\d{6}").unwrap());

    let old_value = ident.identifier_value.clone();
    let new_ident = match RE_IRAS.captures(&old_value) {
        Some(s) => {
            let new_value = s[0].to_string().replace("IRAS", "").replace(":", "").trim().to_string();
            Identifier::new(303, "IRAS ID".to_string(), new_value)
        },
        None => {
            match RE_CPMS.captures(&old_value) {
                Some(s) => {
                    let new_value = s[0].to_string().replace("CPMS", "").replace(":", "").trim().to_string();
                    Identifier::new(304, "CPMS ID".to_string(), new_value)
                },
                None => {
                    match RE_NIHR.captures(&old_value) {
                        Some(s) => {
                            let new_value = s[0].to_string().replace("NIHR", "").replace(":", "").trim().to_string();
                            Identifier::new(416, "NIHR ID".to_string(), new_value)
                        },
                        None => {
                            match RE_HTA.captures(&old_value) {
                                Some(s) => {
                                    let new_value = s[0].to_string();
                                    Identifier::new(417, "HTA ID".to_string(), new_value)
                                },
                                None => {
                                    match RE_CCMO.captures(&old_value) {
                                        Some(s) => {
                                            let new_value = s[0].to_string();
                                            Identifier::new(801, "CCMO ID".to_string(), new_value)
                                        },
                                        None => {
                                            match RE_CIV.captures(&old_value) {
                                                Some(s) => {
                                                    let new_value = s[0].to_string();
                                                    Identifier::new(186, "Eudamed ID".to_string(), new_value)
                                                },
                                                None => {  // revert to the original
                                                    Identifier::new(ident.identifier_type_id, ident.identifier_type, ident.identifier_value)
                                                },  
                                            }
                                        },  
                                    }
                                },  
                            }
                        },  
                    }
                },  
            }
        },  
    };
    
    new_ident

}


#[allow(dead_code)]
pub trait StringExtensions {
    fn tidy(&self) -> Option<String>;
    fn replace_unicodes(&self) -> Option<String>;
    fn replace_tags_and_unicodes(&self) -> Option<String>;
    fn regularise_hyphens(&self) -> Option<String>;
    fn compress_spaces(&self) -> Option<String>;
}

pub trait OptionStringExtensions {
    fn as_text_opt(&self) -> Option<String>;
    fn as_filtered_text_opt(&self) -> Option<String>;
    fn as_date_opt(&self) -> Option<String>;
    fn as_datetime_opt(&self) -> Option<String>;
    fn as_float_opt(&self) -> Option<f32>;
    fn as_bool_opt(&self) -> Option<bool>;
}

// Extensions for Option<String>, largely specific to 
// the ISRCTN data derived from deserialisation of its XML.

// The XML deserialises to Option<String> because most elements
// and attributes are optional, and may be empty or completely missing.
// The generated json also has to support Options, both to make missing 
// data clearer, and for it to be more easily transferred to a database.
// It is useful, however, to introduce different types as appropriate, 
// (e.g. Option<bool>, Option<f32>), and also to put dates into 
// appropriate levels of accuracy, by truncating the over precise 
// ISO strings. In the json dates are still strings, but
// in a form more easily convertable to the correct DB type.

impl OptionStringExtensions for Option<String> {

    fn as_text_opt(&self) -> Option<String> {
         match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } else {
                        Some(st.to_string())
                    }
                },
            None => None
        }
    }

    // Filtering here is to translate 'n/a', 'null' or 'nil'
    // type entries with None. the options used are ISRCTN specific -
    // other choices might be necessary in other systems.

    fn as_filtered_text_opt(&self) -> Option<String> {
        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        let stl = st.to_ascii_lowercase();
                        if stl == "n/a" || stl == "na" || stl == "no" 
                        || stl.starts_with("nil ") || stl.starts_with("not ") {
                            None
                        }
                        else {
                            Some(st.to_string())
                        }
                    }
                },
            None => None
        }
    }

    // dates are kept as strings but truncated to the 
    // short ISO YYYY-MM-DD format. It is assumed that the
    // fields using this extension are written as long ISO dates.
    // It may be that a Regexp check shopuld be added to ensure 
    // that this is the case.

    fn as_date_opt(&self) -> Option<String> {
        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        let st2 = st.to_string();
                        if st2.len() > 10 {
                            let date_string = &st2[0..10];
                            Some(date_string.to_string())
                        }
                        else {
                            None
                        }
                    }
                },
                None => None
        }
    }

    // dates are kept as strings but truncated to the 
    // ISO YYY-MM-DDThh:mm::ss format. It is assumed that the
    // fields using this extension are written as long ISO dates.
    // It may be that a Regexp check shopuld be added to ensure 
    // that this is the case.

    fn as_datetime_opt(&self) -> Option<String> {
        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        let st2 = st.to_string();
                        if st2.len() > 19 {
                            let date_string = &st2[0..19];
                            Some(date_string.to_string())
                        }
                        else {
                            None
                        }
                    }
                },
                None => None
        }
    }
    

    fn as_float_opt(&self) -> Option<f32> {
        match self {
            Some(s) => { 
                let st = s.trim();
                if st == "" 
                {
                    None
                } 
                else 
                {
                    match st.parse::<f32>() 
                    {
                        Ok(n) => Some(n),
                        Err(_e) => None
                    }
                }
            },
            None => None,
        }
    }


    fn as_bool_opt(&self) -> Option<bool> {
        match self {
            Some(s) => { 
                let st = s.trim();
                if st == "" 
                {
                    None
                } 
                else {
                    let stl = st.to_ascii_lowercase();
                    if stl == "true" || stl == "yes" {
                        Some(true)
                    }
                    else if stl == "false" || stl == "no" {
                        Some(false)
                    }
                    else {
                        None
                    }
                }
            },
            None => None
        }
    }

}


impl StringExtensions for String {
    
    fn tidy(&self) -> Option<String> {
        
        let quoteless = self.trim_matches('"');
        if quoteless.to_ascii_lowercase() == "null" || quoteless.trim() == ""
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
                Some(trimmed.to_owned())
            }
        }
    }


    fn replace_unicodes(&self) -> Option<String> {
        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            None
        }
        else {
            let trimmed: &str;
            if self.starts_with("\"") {
                let complex_trim = |c| c == ' ' || c == ';' || c == '"';
                trimmed = self.trim_matches(complex_trim);
            }
            else {
                let complex_trim = |c| c == ' ' || c == ';';
                trimmed = self.trim_matches(complex_trim);
            }
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
        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            None
        }
        else {
            let trimmed: &str;
            if self.starts_with("\"") {
                let complex_trim = |c| c == ' ' || c == ';' || c == '"';
                trimmed = self.trim_matches(complex_trim);
            }
            else {
                let complex_trim = |c| c == ' ' || c == ';';
                trimmed = self.trim_matches(complex_trim);
            }
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

//  let new_string = teststring.replace(" ", "\u{00A0}");

    fn regularise_hyphens(&self) -> Option<String> {

        let quoteless = self.trim_matches('"');
        if quoteless.to_ascii_lowercase() == "null" || quoteless.trim() == ""
        {
            None
        }
        else {
            let mut output_string = quoteless.replace("\u{2010}", "-"); 
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_can_identify_iras_number() {

        let old_identifier  = Identifier::new(502, "Sponsor's id (presumed)".to_string(), "IRAS 123456".to_string());
        let new_identfier = classify_identifier(old_identifier);
    
        assert_eq!(new_identfier.identifier_type_id, 303);
        assert_eq!(new_identfier.identifier_type, "IRAS ID".to_string());
        assert_eq!(new_identfier.identifier_value, "123456".to_string());
    } 
    
    #[test]
    fn check_can_identify_cpms_number() {

        let old_identifier  = Identifier::new(502, "Sponsor's id (presumed)".to_string(), "CPMS 12345".to_string());
        let new_identfier = classify_identifier(old_identifier);
    
        assert_eq!(new_identfier.identifier_type_id, 304);
        assert_eq!(new_identfier.identifier_type, "CPMS ID".to_string());
        assert_eq!(new_identfier.identifier_value, "12345".to_string());
    } 
    
    #[test]
    fn check_can_identify_cpms_number_with_colon() {

        let old_identifier  = Identifier::new(502, "Sponsor's id (presumed)".to_string(), "CPMS:54321".to_string());
        let new_identfier = classify_identifier(old_identifier);
    
        assert_eq!(new_identfier.identifier_type_id, 304);
        assert_eq!(new_identfier.identifier_type, "CPMS ID".to_string());
        assert_eq!(new_identfier.identifier_value, "54321".to_string());
    } 
    
    #[test]
    fn check_can_identify_nihr() {

        let old_identifier  = Identifier::new(990, "Other Id (provenance not supplied)".to_string(), "NIHR123456".to_string());
        let new_identfier = classify_identifier(old_identifier);
    
        assert_eq!(new_identfier.identifier_type_id, 416);
        assert_eq!(new_identfier.identifier_type, "NIHR ID".to_string());
        assert_eq!(new_identfier.identifier_value, "123456".to_string());
    } 


    
    #[test]
    fn check_can_identify_hta_number() {

        let old_identifier  = Identifier::new(990, "Other Id (provenance not supplied)".to_string(), "Some HTA 12/123/123 id".to_string());
        let new_identfier = classify_identifier(old_identifier);
    
        assert_eq!(new_identfier.identifier_type_id, 417);
        assert_eq!(new_identfier.identifier_type, "HTA ID".to_string());
        assert_eq!(new_identfier.identifier_value, "HTA 12/123/123".to_string());
    } 

    
    #[test]
    fn check_can_identify_ccmo_number() {

        let old_identifier  = Identifier::new(502, "Sponsor's id (presumed)".to_string(), "ccmo -- NL12345.789.34".to_string());
        let new_identfier = classify_identifier(old_identifier);
    
        assert_eq!(new_identfier.identifier_type_id, 801);
        assert_eq!(new_identfier.identifier_type, "CCMO ID".to_string());
        assert_eq!(new_identfier.identifier_value, "NL12345.789.34".to_string());
    } 

     #[test]
    fn check_can_identify_eudamed_number() {

        let old_identifier  = Identifier::new(502, "Sponsor's id (presumed)".to_string(),  "Eudamed: CIV-12-34-654321".to_string());
        let new_identfier = classify_identifier(old_identifier);
    
        assert_eq!(new_identfier.identifier_type_id, 186);
        assert_eq!(new_identfier.identifier_type, "Eudamed ID".to_string());
        assert_eq!(new_identfier.identifier_value, "CIV-12-34-654321".to_string());
    } 


    #[test]
    fn check_can_split_string_1() {

        let input_string  = "IRAS 123456, CPMS 12345".to_string();
        let terms = split_identifier(&input_string);
      
        assert_eq!(terms[0], "IRAS 123456".to_string());
        assert_eq!(terms[1], "CPMS 12345".to_string());
    } 

    #[test]
    fn check_can_split_string_2() {

        let input_string  = "Something else, Quotient number 123".to_string();
        let terms = split_identifier(&input_string);
      
        assert_eq!(terms[0], "Something else".to_string());
        assert_eq!(terms[1], "Quotient number 123".to_string());
    } 

    #[test]
    fn check_can_split_string_3() {

        let input_string  = "NIHR:987654, strange grant 45A, Sponsor # EEE".to_string();
        let terms = split_identifier(&input_string);
      
        assert_eq!(terms[0], "NIHR:987654".to_string());
        assert_eq!(terms[1], "strange grant 45A".to_string());
        assert_eq!(terms[2], "Sponsor # EEE".to_string());
    } 

    #[test]
    fn check_can_split_string_4() {

        let input_string  = "strange grant 45A and the rest".to_string();
        let terms = split_identifier(&input_string);
      
        assert_eq!(terms[0], "strange grant 45A and the rest".to_string());
    } 

    #[test]
    fn check_can_split_string_5() {

        let input_string  = "NIHR:987654, rlgbfdldb, Sponsor # EEE, version 22".to_string();
        let terms = split_identifier(&input_string);
      
        assert_eq!(terms[0], "NIHR:987654".to_string());
        assert_eq!(terms[1], "rlgbfdldb".to_string());
        assert_eq!(terms[2], "Sponsor # EEE, version 22".to_string());
    } 

    #[test]
    fn check_can_split_string_6() {

        let input_string  = "IRAS 123456, rlgbfdldb, protocol v 3, dated yesterday".to_string();
        let terms = split_identifier(&input_string);
      
        assert_eq!(terms[0], "IRAS 123456".to_string());
        assert_eq!(terms[1], "rlgbfdldb".to_string());
        assert_eq!(terms[2], "protocol v 3, dated yesterday".to_string());
    } 

}
