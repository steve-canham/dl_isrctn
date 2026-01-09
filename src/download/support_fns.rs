use std::sync::LazyLock;
use regex::Regex;

pub fn count_option<T>(v: Vec<T>) -> Option<Vec<T>> {
    match v.len() {
        0 => None,
        _ => Some(v),
    }
}


pub fn split_identifier(id: &String) -> Vec<String> {
     
    // Attempts to split a string on commas but only at appropriate places

    let mut this_id = id.to_string();

    // As an initial step replace semi-colons with a split marker

    this_id = this_id.replace(";", "||");

    // Then try to get the commas replaced when they immediately follow a common id type

    static RE_IRASC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"IRAS:? ?\d{6,7},").unwrap());
    static RE_CPMSC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CPMS:? ?\d{5},").unwrap());
    static RE_NIHRC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NIHR:? ?\d{6},").unwrap());
    static RE_HTAC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"HTA \d{2}/\d{2,3}/\d{2,3}, ").unwrap());
   
    match RE_IRASC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no iras match at all
    }
    
    match RE_CPMSC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no cpms match at all
    }

    match RE_NIHRC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no nihr match at all
    }

    match RE_HTAC.captures(&this_id) {
        Some(s) => {
            this_id = this_id.replace(&s[0], &s[0].replace(",", "||"))
        },
        None => {},  // no hta match at all
    }

    // Then replace remaining commas if they preceed common combined entities

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

    // Again, try to replace remaining commas if they preceed common combined entities

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


pub fn classify_identifier(identifier_value: String) -> (i32, String, String) {

    // Attempts to identify the type of some of the more common / distinctive (non trial registry) identifiers

    static RE_IRAS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"IRAS:? ?\d{6,7}").unwrap());
    static RE_CPMS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CPMS:? ?\d{5}").unwrap());
    static RE_NIHR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NIHR:? ?\d{6}").unwrap());
    static RE_HTA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"HTA \d{2}/\d{2,3}/\d{2,3}").unwrap());
    static RE_NTR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NTR:? ?\d{2,6}").unwrap());
    static RE_CCMO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NL\d{5}.\d{3}.\d{2}").unwrap());
    static RE_CIV: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CIV-\d{2}-\d{2}-\d{6}").unwrap());
    static RE_ANSM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d{4}-A\d{5}-\d{2}").unwrap());
    
    match RE_IRAS.captures(&identifier_value) {
        Some(s) => {
            let new_value = s[0].to_string().replace("IRAS", "").replace(":", "").trim().to_string();
            (303, "IRAS ID".to_string(), new_value)
        },
        None => {
            match RE_CPMS.captures(&identifier_value) {
                Some(s) => {
                    let new_value = s[0].to_string().replace("CPMS", "").replace(":", "").trim().to_string();
                    (304, "CPMS ID".to_string(), new_value)
                },
                None => {
                    match RE_NIHR.captures(&identifier_value) {
                        Some(s) => {
                            let new_value = s[0].to_string().replace("NIHR", "").replace(":", "").trim().to_string();
                            (416, "NIHR ID".to_string(), new_value)
                        },
                        None => {
                            match RE_HTA.captures(&identifier_value) {
                                Some(s) => {
                                    let new_value = s[0].to_string();
                                    (417, "HTA ID".to_string(), new_value)
                                },
                                None => {
                                    match RE_NTR.captures(&identifier_value) {
                                        Some(s) => {
                                            let new_value = s[0].to_string();
                                            (181, "Obsolete NTR ID".to_string(), new_value)
                                        },
                                        None =>  {
                                            match RE_CCMO.captures(&identifier_value) {
                                                Some(s) => {
                                                    let new_value = s[0].to_string();
                                                    (801, "CCMO ethics ID".to_string(), new_value)
                                                },
                                                None => {
                                                    match RE_CIV.captures(&identifier_value) {
                                                        Some(s) => {
                                                            let new_value = s[0].to_string();
                                                            (186, "Eudamed CIV ID".to_string(), new_value)
                                                        },
                                                        None => {
                                                            match RE_ANSM.captures(&identifier_value) {
                                                                Some(s) => {
                                                                    let new_value = s[0].to_string();
                                                                    (301, "ANSM (ID-RCB number)".to_string(), new_value)
                                                                },
                                                                None => (502, "???".to_string(), identifier_value)
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
                    }
                },  
            }
        },  
    }
   

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_can_identify_iras_number() {
        let (type_id, type_string, id) = classify_identifier("IRAS 123456".to_string());

        assert_eq!(type_id, 303);
        assert_eq!(type_string, "IRAS ID".to_string());
        assert_eq!(id, "123456".to_string());
    } 
    
    #[test]
    fn check_can_identify_cpms_number() {
        let (type_id, type_string, id) = classify_identifier("CPMS 12345".to_string());
    
        assert_eq!(type_id, 304);
        assert_eq!(type_string, "CPMS ID".to_string());
        assert_eq!(id, "12345".to_string());
    } 
    
    #[test]
    fn check_can_identify_cpms_number_with_colon() {
        let (type_id, type_string, id) = classify_identifier("CPMS:54321".to_string());
    
        assert_eq!(type_id, 304);
        assert_eq!(type_string, "CPMS ID".to_string());
        assert_eq!(id, "54321".to_string());
    } 
    
    #[test]
    fn check_can_identify_nihr() {
        let (type_id, type_string, id) = classify_identifier("NIHR123456".to_string());
    
        assert_eq!(type_id, 416);
        assert_eq!(type_string, "NIHR ID".to_string());
        assert_eq!(id, "123456".to_string());
    } 


    
    #[test]
    fn check_can_identify_hta_number() {
        let (type_id, type_string, id) = classify_identifier("Some HTA 12/123/123 id".to_string());
    
        assert_eq!(type_id, 417);
        assert_eq!(type_string, "HTA ID".to_string());
        assert_eq!(id, "HTA 12/123/123".to_string());
    } 

    
    #[test]
    fn check_can_identify_ccmo_number() {
        let (type_id, type_string, id) = classify_identifier("ccmo -- NL12345.789.34".to_string());
    
        assert_eq!(type_id, 801);
        assert_eq!(type_string, "CCMO ethics ID".to_string());
        assert_eq!(id, "NL12345.789.34".to_string());
    } 

     #[test]
    fn check_can_identify_eudamed_number() {
        let (type_id, type_string, id) = classify_identifier("Eudamed: CIV-12-34-654321".to_string());
    
        assert_eq!(type_id, 186);
        assert_eq!(type_string, "Eudamed CIV ID".to_string());
        assert_eq!(id, "CIV-12-34-654321".to_string());
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
