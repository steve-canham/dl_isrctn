use std::sync::LazyLock;
use regex::Regex;
use std::collections::HashMap;
use super::iec_helper::*;
//use log::info;

pub struct RegexResults {
    pub tag: String,
    pub tag_name: String,
    pub regex: String, 
    pub new_line: String,

}

pub static NUMBDOT_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();

    map.insert("numdot4", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());  // numeric sub-sub-sub-heading. N.n.n.n
    map.insert("numdot3", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());           // numeric sub-sub-heading. N.n.n 
    map.insert("numdot2", Regex::new(r"^\d{1,2}\.\d{1,2}\.").unwrap());                   // numeric sub-heading. N.n. 
    map.insert("numdotnumspc", Regex::new(r"^\d{1,2}\.\d{1,2}\s").unwrap());             // numeric sub-heading space (without final period) N.n
    map.insert("numdotnumalcap", Regex::new(r"^\d{1,2}\.\d{1,2}[A-Z]").unwrap());        // number-dot-number cap letter  - Cap is usually from text (no space)
    map.insert("numdotspc", Regex::new(r"^\d{1,2}\.\s").unwrap());                       // number period and space / tab 1. , 2.    
    map.insert("numdotrtpar", Regex::new(r"^\d{1,2}\.\)").unwrap());                     // number followed by dot and right bracket  1.), 2.)
    map.insert("numdot", Regex::new(r"^\d{1,2}\.").unwrap());                            // number period only - can give false positives
  
    map
});


pub static NUMB_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();
    map.insert("numaldot", Regex::new(r"^\d{1,2}[a-z]{1}\.").unwrap());                  // number plus letter and space Na., Nb. 
    map.insert("numal", Regex::new(r"^\d{1,2}[a-z]{1} ").unwrap());                      // number plus letter and space Na , Nb 
    map.insert("numrtpardot", Regex::new(r"^\d{1,2}\)\.").unwrap());                     // number followed by right bracket and dot 1)., 2).
    map.insert("numrtpar", Regex::new(r"^\d{1,2}\)").unwrap());                          // number followed by right bracket 1), 2)
    map.insert("numcol", Regex::new(r"^\d{1,2}\:").unwrap());                            // number followed by colon 1:, 2:
 
    map.insert("numrtbr", Regex::new(r"^\d{1,2}\]").unwrap());                           // numbers with right square bracket   1], 2]
    map.insert("numdshnumpar", Regex::new(r"^\d{1,2}\-\d{1,2}\)").unwrap());             // numbers and following dash, then following number right bracket  1-1), 1-2)
    map.insert("numdshpar", Regex::new(r"^\d{1,2}\-\)").unwrap());                       // numbers and following dash, right bracket  1-), 2-)
    map.insert("numdsh", Regex::new(r"^\d{1,2}\-").unwrap());                            // numbers and following dash  1-, 2-
    map.insert("numslash", Regex::new(r"^\d{1,2}\/").unwrap());                          // numbers and following slash  1/, 2/
    map.insert("numtab", Regex::new(r"^\d{1,2}\t").unwrap());                            // number followed by tab, 1\t, 2\t
       
    map.insert("num3spc", Regex::new(r"^(1|2)\d{2}\.?\s").unwrap());                     // 3 numbers between 100 and 300 followed by dot / space
    map.insert("numspc", Regex::new(r"^\d{1,2}\s").unwrap());                            // number space only - can give false positives            
    map.insert("numalcap", Regex::new(r"^\d{1,2}[A-Z]{1} ").unwrap());                   // number-cap letter  - might give false positives    
    map.insert("numal", Regex::new(r"^\d{1,2}[a-z]{1} ").unwrap());                      // number plus letter and space Na, Nb

    map
});

pub static ALPH_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();
  
    map.insert("aldottab", Regex::new(r"^[a-z]\.\t").unwrap());                          // alpha-period followed by tab   a.\t, b.\t
    map.insert("aldot", Regex::new(r"^[a-z]\.").unwrap());                               // alpha period. a., b.
    map.insert("alcapdot", Regex::new(r"^[A-Z]\.").unwrap());                            // alpha caps period. A., B.
    map.insert("alinpar", Regex::new(r"^\([a-z]\)").unwrap());                           // alpha in parentheses. (a), (b)
    map.insert("alrpar", Regex::new(r"^[a-z]\)").unwrap());                              // alpha with right bracket. a), b)
           
    map.insert("ospc", Regex::new(r"^o ").unwrap());                                     // open 'o' bullet followed by space, o , o
    map.insert("otab", Regex::new(r"^o\t").unwrap());                                    // open 'o' bullet followed by tab  o\t, o\t
    
    map.insert("romrtpar", Regex::new(r"^x{0,3}(|ix|iv|v?i{0,3})\)").unwrap());          // roman numerals right brackets    i), ii)
    map.insert("romdot", Regex::new(r"^x{0,3}(|ix|iv|v?i{0,3})\.").unwrap());            // roman numerals dots   i., ii.

    map.insert("e_num", Regex::new(r"^(E|e)\s?\d{1,2}").unwrap());                       // exclusion as E or e numbers, optional space E 01, E 02
    map.insert("i_num", Regex::new(r"^(I|i)\s?\d{1,2}").unwrap());                       // inclusion as I or i numbers, optional space i1, i2
   
    map
});

pub static OTH_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();
 
    map.insert("alinpar", Regex::new(r"^\([a-z]\)").unwrap());                           // alpha in parentheses. (a), (b)
    map.insert("numinpar", Regex::new(r"^\(\d{1,2}\)").unwrap());                        // bracketed numbers (1), (2)
    map.insert("numinbrs", Regex::new(r"^\[\d{1,2}\]").unwrap());                        // numbers in square brackets   [1], [2]
       
    map.insert("buls1", Regex::new(r"^[\u{2022},\u{2023},\u{25E6},\u{2043},\u{2219}]").unwrap());  // various bullets 1  
    map.insert("buls2", Regex::new(r"^[\u{2212},\u{2666},\u{00B7},\u{F0B7}]").unwrap());           // various bullets 2
    map.insert("bultab", Regex::new(r"^\u{F0A7}\t").unwrap());                             // ? bullet and tab     
    map.insert("dshtab", Regex::new(r"^-\t").unwrap());                                  // hyphen followed by tab, -\t, -\t 
    map.insert("strtab", Regex::new(r"^\*\t").unwrap());                                 // asterisk followed by tab    *\t, *\t      
     
    map.insert("rominpar", Regex::new(r"^\(x{0,3}(|ix|iv|v?i{0,3})\)").unwrap());        // roman numerals double bracket   (i), (ii)
   
    map.insert("dshonly", Regex::new(r"^-").unwrap());                                   // dash only   -, -
    map.insert("dblstr", Regex::new(r"^\*\*").unwrap());                                 // two asterisks   **, **
    map.insert("stronly", Regex::new(r"^\*").unwrap());                                  // asterisk only   *, *
    map.insert("semcolonly", Regex::new(r"^;").unwrap());                                // semi-colon only   ;, ; 
    map.insert("qmkonly", Regex::new(r"^\?").unwrap());                                  // question mark only   ?, ?  
    map.insert("invqm", Regex::new(r"^¿").unwrap());                                     // inverted question mark only   ¿, ¿
 
    map
});
 


pub fn test_against_numdot_res(this_line: &String) ->  Option<RegexResults>{

    // Ordering of these 'numdot' REs is important and not easily done from the hashmap
    // where theyare stored. Therefore each one called individually in the 
    // required order.

    if let Some(s1) = test_numdot_re("numdotspc", this_line) {
        Some(s1)
    }
    else if let Some(s2) = test_numdot_re("numdotnumspc", this_line) {
        Some(s2)
    }
    else if let Some(s3) = test_numdot_re("numdot4", this_line) {
        Some(s3)
    }
    else if let Some(s4) = test_numdot_re("numdot3", this_line) {
        Some(s4)
    }
    else if let Some(s5) = test_numdot_re("numdot2", this_line) {
        Some(s5)
    }
    else if let Some(s6) = test_numdot_re("numdotrtpar", this_line) {
        Some(s6)
    }
    else if let Some(s7) = test_numdot_re("numdotnumalcap", this_line) {
        Some(s7)
    }   
    else if let Some(s8) = test_numdot_re("numdot", this_line) {
        Some(s8)
    }     
    else {
        None
    }
   
}


fn test_numdot_re(tag_name: &str, this_line: &String) ->  Option<RegexResults> {

    let re = NUMBDOT_MAP.get(tag_name).unwrap();
    if let Some(c) = re.captures(this_line) {

        let tag = c.get_match().as_str();
        let regex = re.to_string();

        match tag_name {

            "none" => None,

            _ => Some (RegexResults {
            tag: tag.to_string(),
            tag_name: tag_name.to_string(),
            regex: regex, 
            new_line: this_line[tag.len()..].to_string(),
        }) 
        }
    }
    else {

        None
    }


}


pub fn test_against_numeric_res(this_line: &String) ->  Option<RegexResults>{

    let mut tag = "";
    let mut tag_name = "none";
    let mut regex = "".to_string(); 

    for r in NUMB_MAP.iter() {

        let re = r.1;
        if let Some(c) = re.captures(&this_line) {
            tag = c.get_match().as_str();
            tag_name = *r.0; 
            regex = re.to_string();

            break;
        }
    }

   // Beware false positives

    if tag_name == "numdsh"
    {
        // regex_pattern = @"^\d{1,2}\-", number followed by a dash
        // may need to be put back together if the first character of the text is also
        // a number - indicates that this is a numeric range (e.g. of age, weight)

        let rest_of_text = this_line[tag.len()..].trim().to_string();
        if rest_of_text.first_char().is_digit(10)
        {
            tag_name = "none"; // not really a match for anything
            tag = "";
            regex = "".to_string();
        }
    }

    match tag_name {

        "none" => None,

        _ => Some (RegexResults {
        tag: tag.to_string(),
        tag_name: tag_name.to_string(),
        regex: regex, 
        new_line: this_line[tag.len()..].to_string(),
    }) 
}



}


pub fn test_against_alpha_res(this_line: &String) ->  Option<RegexResults> {

    let mut tag_name = "none";
    let mut tag = "";
    let mut regex = "".to_string();

    for r in ALPH_MAP.iter() {

        let re = r.1;
        if let Some(c) = re.captures(&this_line) {

            tag = c.get_match().as_str();
            tag_name = *r.0; 
            regex = re.to_string();

            break;
        }
    }

    match tag_name {

        "none" => None,

        _ => Some (RegexResults {
        tag: tag.to_string(),
        tag_name: tag_name.to_string(),
        regex: regex, 
        new_line: this_line[tag.len()..].to_string(),
        }) 
    }
}

// may need to do some checking / corrections before coming out of the loop
pub fn test_against_other_res(this_line: &String) ->  Option<RegexResults>{

    let mut tag_name = "none";
    let mut tag = "";
    let mut regex = "".to_string();

    for r in OTH_MAP.iter() {

        let re = r.1;
        if let Some(c) = re.captures(&this_line) {

            tag = c.get_match().as_str();
            tag_name = *r.0; 
            regex = re.to_string();


            break;
        }
    }

    // may need to do some checking / corrections before coming out of the loop

    match tag_name {

        "none" => None,

        _ => Some (RegexResults {
        tag: tag.to_string(),
        tag_name: tag_name.to_string(),
        regex: regex, 
        new_line: this_line[tag.len()..].to_string(),
        }) 

    }


   // if ldr_name == "none" {None} else {Some((leader.to_string(), ldr_name.to_string(), regex))}

}


