use std::sync::LazyLock;
use regex::Regex;
use std::collections::HashMap;
use crate::iec::iec_helper::StringExtensions;

use super::iec_structs::*;
// use log::info;

pub struct RegexResults {
    pub tag: String,
    pub tag_type: String,
    pub text: String,

}

pub static IE_RE_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();

    map.insert("numdot4", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());  // numeric sub-sub-sub-heading. N.n.n.n(.)
    map.insert("numdot3", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());           // numeric sub-sub-heading. N.n.n(.) 
    map.insert("numdot2", Regex::new(r"^\d{1,2}\.\d{1,2}").unwrap());                    // numeric sub-heading. N.n(.) 
    map.insert("numdotspc", Regex::new(r"^\d{1,2}\.\s").unwrap());                       // number period and space / tab 1. , 2.   

    map.insert("numdotnumalcap", Regex::new(r"^\d{1,2}\.\d{1,2}[A-Z]").unwrap());        // number-dot-number cap letter  N.nA- Cap is usually from text (no space)
    map.insert("numdotrtpar", Regex::new(r"^\d{1,2}\.\)").unwrap());                     // number followed by dot and right bracket  1.), 2.)
    map.insert("numdot", Regex::new(r"^\d{1,2}\.").unwrap());                            // number period only - can give false positives

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
    map.insert("numalcap", Regex::new(r"^\d{1,2}[A-Z]{1} ").unwrap());                   // number-cap letter nA, nB - might give false positives    
    map.insert("numal", Regex::new(r"^\d{1,2}[a-z]{1} ").unwrap());                      // number plus letter and space Na, Nb
    
    map.insert("aldottab", Regex::new(r"^[a-z]\.\s").unwrap());                          // alpha-period followed by tab or space  a.\t, b.\t
    map.insert("aldot", Regex::new(r"^[a-z]\.").unwrap());                               // alpha period. a., b.
    map.insert("alcapdot", Regex::new(r"^[A-Z]\.").unwrap());                            // alpha caps period. A., B.
    map.insert("alinpar", Regex::new(r"^\([a-z]\)").unwrap());                           // alpha in parentheses. (a), (b)
    map.insert("alrpar", Regex::new(r"^[a-z]\)").unwrap());                              // alpha with right bracket. a), b)
    map.insert("ospc", Regex::new(r"^o\s").unwrap());                                    // open 'o' bullet followed by space or tab, o , o
        
    map.insert("romrtpar", Regex::new(r"^x{0,3}(|ix|iv|v?i{0,3})\)").unwrap());          // roman numerals right brackets    i), ii)
    map.insert("romdot", Regex::new(r"^x{0,3}(|ix|iv|v?i{0,3})\.").unwrap());            // roman numerals dot   i., ii.
    map.insert("romcapdot", Regex::new(r"^X{0,3}(|IX|IV|V?I{0,3})\.").unwrap());         // capital roman numerals dot   I., II.
    
    map.insert("e_num", Regex::new(r"^(E|e)\s?\d{1,2}").unwrap());                       // exclusion as E or e numbers, optional space E 01, E 02
    map.insert("i_num", Regex::new(r"^(I|i)\s?\d{1,2}").unwrap());                       // inclusion as I or i numbers, optional space i1, i2
    map.insert("numinpar", Regex::new(r"^\(\d{1,2}\)").unwrap());                        // bracketed numbers (1), (2)
    map.insert("numinbrs", Regex::new(r"^\[\d{1,2}\]").unwrap());                        // numbers in square brackets   [1], [2]
       
    map.insert("buls1", Regex::new(r"^[\u{2022},\u{2023},\u{25E6},\u{2043},\u{2219}]").unwrap());  // various bullets 1  
    map.insert("buls2", Regex::new(r"^[\u{2212},\u{2666},\u{00B7},\u{F0B7}]").unwrap());           // various bullets 2
    map.insert("bultab", Regex::new(r"^\u{F0A7}\s").unwrap());                             // ? bullet and space or tab     
    map.insert("dshtab", Regex::new(r"^-\s").unwrap());                                  // hyphen followed by space or tab, -\t, -\t 
    map.insert("strtab", Regex::new(r"^\*\t").unwrap());                                 // asterisk followed by space or tab  *\t, *\t      
    map.insert("rominpar", Regex::new(r"^\(x{0,3}(|ix|iv|v?i{0,3})\)").unwrap());        // roman numerals double bracket   (i), (ii)
   
    map.insert("dshonly", Regex::new(r"^-").unwrap());                                   // dash only   -, -
    map.insert("stronly", Regex::new(r"^\*{1,3}").unwrap());                                  // asterisk only   *, *
    map.insert("semcolonly", Regex::new(r"^;").unwrap());                                // semi-colon only   ;, ; 
    map.insert("qmkonly", Regex::new(r"^\?").unwrap());                                  // question mark only   ?, ?  
    map.insert("invqm", Regex::new(r"^¿").unwrap());                                     // inverted question mark only   ¿, ¿
  
    map
});

pub fn test_re(tag_type: &str, this_line: &String) ->  Option<RegexResults> {

    let re = IE_RE_MAP.get(tag_type).unwrap();  // should always be present
    if let Some(c) = re.captures(this_line) {
        let tag = c.get_match().as_str();

        match tag_type {
            "none" => None,
            _ => {
                let init_rr = Some (RegexResults {
                    tag: tag.to_string(),
                    tag_type: tag_type.to_string(),
                    text: this_line[tag.len()..].trim().to_string(),
                });
                init_rr
            }
        }
    }
    else {
        None
    }
}

pub fn test_against_numdot_res(index: usize, this_line: &String, prev_tag_type: &String, prev_level: i32, _tagged_lines: &Vec<CLine>, levels: &mut Vec<String>) ->  Option<CLine> {

    // Ordering of these 'numdot' REs is important and not easily done from the hashmap
    // where they are stored. Therefore each one called individually in the 
    // required order.

    // For some tag types final periods may be omiited (inconsistently) in a proportion of the lines.

    let raw_result = 
    if let Some(mut s1) = test_re("numdotspc", this_line) { 
        
        // number-dot space - safer to make the tag just numdot
        // as this allows variations in the sapcing to be dealt with by the system
                
        s1.tag_type = "numdot".to_string();
        Some(s1) 
    }               

    else if let Some(mut s4) = test_re("numdot4", this_line) {   // numeric sub-sub-sub-heading. N.n.n.n, may have additional final '.'
        
        if s4.text.starts_with('.') {
            s4.text = s4.text.trim_start_matches(&['.', ' ']).to_string();
        }
        Some(s4) 
    }            
    else if let Some(mut s3) = test_re("numdot3", this_line) {   // numeric sub-sub-heading. N.n.n, may have additional final '.' 
        
        if s3.text.starts_with('.') {
            s3.text = s3.text.trim_start_matches(&['.', ' ']).to_string();
        }
        Some(s3) 
    }            
    else if let Some(mut s2) = test_re("numdot2", this_line) {    // numeric sub-heading. N.n, may have additional final '.' 
        if s2.text.starts_with('.') {
            s2.text = s2.text.trim_start_matches(&['.', ' ']).to_string();
        }
        
        Some(s2) 
    }           
    else if let Some(s6) = test_re("numdotrtpar", this_line) { Some(s6) }        // number followed by dot and right bracket  1.), 2.)
    else if let Some(s7) = test_re("numdotnumalcap", this_line) {     // number-dot-number cap letter  - Cap is usually from text (no space)
        Some(s7) 
    }    
    else if let Some(s8) = test_re("numdot", this_line,) { Some(s8) }            // number period only - can give false positives
    else {
        None
    };


    let processed_result = match raw_result {
        
        Some (r) => {
            
            // get level - necessary to properly compare with previous lines

            let (level, new_level) = if &r.tag_type != prev_tag_type {get_level(&r.tag_type, levels)} else {(prev_level, false)};
            
            let mut genuine = true; // as the starting point
            let mut proc_tag_type = r.tag_type.clone();
            let mut proc_tag = r.tag.clone();  
            let mut proc_text = r.text.clone();  

            if &r.tag_type == "numdotnumalcap" {

                // number-dot-number cap letter  - Cap is usually from text (no space)
                // Change the tag name, tag and text details accordingly
                
                proc_tag_type = "numdot2".to_string();

                let proc_tag2 = proc_tag.clone();
                let tag_letter = proc_tag2.last_char();
                proc_tag.pop();
                proc_text = format!("{}{}", tag_letter, proc_text);

            }
            
            // The n.n tag type may be a genuine number in front of a quantity 

            if &r.tag_type == "numdot2" {
                
                if prev_tag_type == "numdot2" 
                    || (prev_tag_type == "numdot" && proc_tag.ends_with(".1"))
                    || (prev_tag_type == "none" && proc_tag.ends_with(".1"))
                {
                    genuine = true;   // continuation of numdot2 or the first in a numdot2 sequence
                }
                else {
                    genuine = false;
                }

                // if genuine appears false what is this line? 
                // Could be a number, X.Y, followed by a quantity (in which case a header type line)
                // Or could be a n. tag, follwe by a number Y... as the first character in the string

                if !genuine
                {
                    let low_text = r.text.trim().to_lowercase();
                    if low_text.starts_with("secs")
                        || low_text.starts_with("second")
                        || low_text.starts_with("mins")
                        || low_text.starts_with("minute")
                        || low_text.starts_with("hour")
                        || low_text.starts_with("day")
                        || low_text.starts_with("week")
                        || low_text.starts_with("month")
                        || low_text.starts_with("year")
                        || low_text.starts_with("mg")
                        || low_text.starts_with("ml")
                        || low_text.starts_with("kg")
                        || low_text.starts_with("g/")
                        || low_text.starts_with("cm")
                        || low_text.starts_with("patient")
                        || low_text.starts_with("subject")
                    {
                        // appears to be a header or (more likely) a split line part
                    }
                    else
                    {
                        // change the tag namne, tag and text to refflect a numdot type

                        proc_tag_type = "numdot".to_string();
                        let proc_tag2 = proc_tag.clone();
                        let tag_parts: Vec<&str> = proc_tag2.split('.').collect();

                        proc_tag = tag_parts[0].to_string();
                        proc_text = format!("{} {}", tag_parts[1], proc_text);
                    }

                }

            }
            

            if !genuine {

                // remove the level if one has just been added to the levels vector
                if new_level {
                    levels.pop();   
                }             
                None
            }
            else {
                Some(CLine {
                    seq_num: (index + 1) as i32,
                    tag: proc_tag,
                    tag_type: proc_tag_type,
                    indent_level: level,
                    text: proc_text,
                })
            }
        },
        None => None,
    };

    processed_result
     
}



pub fn test_against_numeric_res(index: usize, this_line: &String, prev_tag_type: &String, prev_level: i32, tagged_lines: &Vec<CLine>, levels: &mut Vec<String>) ->  Option<CLine> {
    
    let raw_result = 
    if let Some(s1) = test_re("numaldot", this_line) { Some(s1) }
    else if let Some(s2) = test_re("numal", this_line) { Some(s2) }
    else if let Some(s3) = test_re("numrtpardot", this_line) { Some(s3) }
    else if let Some(s4) = test_re("numrtpar", this_line) { Some(s4) }
    else if let Some(s5) = test_re("numcol", this_line) { Some(s5) }
    
    else if let Some(s6) = test_re("numrtbr", this_line) { Some(s6) }
    else if let Some(s7) = test_re("numdshnumpar", this_line) { Some(s7) }   
    else if let Some(s8) = test_re("numdshpar",  this_line) { Some(s8) }   
    else if let Some(s9) = test_re("numdsh", this_line) { 

        // may need to be put back together if the first character of the text is also
        // a number - indicates that this is a numeric range (e.g. of age, weight)
        if s9.text.first_char().is_digit(10)
        {
            None
        }
        else {
            Some(s9)
        }
    }
    else if let Some(s10) = test_re("numslash", this_line) { Some(s10) }
    else if let Some(s11) = test_re("numtab", this_line) { Some(s11) }   

    else if let Some(mut s12) = test_re("num3spc", this_line) { 

        s12.text = s12.text.trim_start_matches(&['.', ' ']).to_string();   // text is shorn of any whitespace, periods
        s12.tag = s12.tag.trim_matches(&['.', ' ']).to_string();     // tag is now just the 3 numerals

        Some(s12) 
    }   
    else if let Some(s13) = test_re("numspc", this_line) { Some(s13) }
    else if let Some(s14) = test_re("numalcap", this_line) { 
        
        // e.g. 1A , 2A, 2B, May be a false positive, especially if letter is A
        // i.e. the 'A' is the start of a non-tag line.
        // For the moment leave as is and see if problems arise
        
        Some(s14) 
    
    }
    else if let Some(s15) = test_re("numal", this_line) { Some(s15) }
    else {
        None
    };

    let processed_result = match raw_result {
        
        Some (r) => {
            
            // get level - necessary to properly compare with previous lines

            let (level, new_level) = if &r.tag_type != prev_tag_type {get_level(&r.tag_type, levels)} else {(prev_level, false)};

            let mut genuine = true; // as the starting point
            let proc_tag_type = r.tag_type.clone();
            let proc_tag = r.tag.clone().trim().to_string();
            let this_tag_num: i32 = proc_tag.parse().unwrap_or_else(|_| 0);

            if &r.tag_type == "numspc" {

                // May need to be ignored if a number appears out of sequence
                // Also the case if number is followed by a time period or unit
                // - in which case almost always part of the line above 

                if (index == 0 || (index == 1 && prev_level == 1)) && this_tag_num != 1
                {
                    genuine = false; // probably, but the converse may not be true
                }

                let low_text = r.text.trim().to_lowercase();
                if low_text.starts_with("secs")
                    || low_text.starts_with("second")
                    || low_text.starts_with("mins")
                    || low_text.starts_with("minute")
                    || low_text.starts_with("hour")
                    || low_text.starts_with("day")
                    || low_text.starts_with("week")
                    || low_text.starts_with("month")
                    || low_text.starts_with("year")
                    || low_text.starts_with("mg")
                    || low_text.starts_with("ml")
                    || low_text.starts_with("kg")
                    || low_text.starts_with("g/")
                    || low_text.starts_with("cm")
                    || low_text.starts_with("patient")
                    || low_text.starts_with("subject")
                {
                    // appears to be a header or (more likely) a split line part

                    genuine = false; 
                }

                if this_tag_num > 1 {
                    if tag_in_numeric_sequence(this_tag_num, tagged_lines, level) {
                        genuine = true;   // continuation of numdot2 or the first in a numdot2 sequence
                    }
                    else {
                        genuine = false;
                    }
                }

                if this_tag_num == 1  {

                        // presumed genuine unless made false above

                }
            }

            if r.tag_type == "num3spc".to_string() {
               
               // Regex is @"^(1|2)\d{1,2}\.?\s?" - tag type will be ^(1|2)\d{1,2}
               // Can be a spurious CR followed by a number, a.g. after an equals sign
               // or before a unit. Should normally also be part of a sequence.
           
               let low_text = r.text.trim().to_lowercase();
               if low_text.starts_with("mg") || low_text.starts_with("cm")
                  || low_text.starts_with("kg") || low_text.starts_with("secs")
                  || low_text.starts_with("patients") ||  low_text.starts_with("min")
                  || low_text.starts_with("days") ||  low_text.starts_with("subjects")
                {
                    genuine = false;
                }

                if index > 0
                {
                    let prev_line = &tagged_lines[index - 1].text;
                    if prev_line.last_char() == '=' || prev_line.last_char() == '>' || prev_line.last_char() == '<'
                    {
                        genuine = false;
                    }
                    else
                    {
                         if r.tag != "101" && r.tag != "201"
                         {
                            let prevln1same = prev_tag_type == "num3spc";
                            let mut prevln2same = false; 
                            if index > 1
                            {
                                let prevldr2 = tagged_lines[index - 2].tag.clone();
                                if prevldr2 ==  "num3spc".to_string()
                                {
                                    prevln2same = true;
                                }
                            }
                            if !(prevln1same || prevln2same)
                            {
                                // line seems to be isolated in the sequence
                                genuine = false;
                            }
                        }
                    }

                }
             
            }

            if genuine {

                Some(CLine {
                    seq_num: (index + 1) as i32,
                    tag: r.tag,
                    tag_type: proc_tag_type,
                    indent_level: level,
                    text: r.text,
                })
            }
            else {
                // remove the level if one has just been added to the levels vector
                if new_level {
                    levels.pop();   
                }             
                None
            }
        },
        None => None,
    };

    processed_result
    
}


pub fn test_against_alpha_res(index: usize, this_line: &String, prev_tag_type: &String, prev_level: i32, tagged_lines: &Vec<CLine>, levels: &mut Vec<String>) -> Option<CLine> {
        
    let raw_result = 
    
    if let Some(si) = test_re("romdot", this_line) {  // roman numerals dot   i., ii.
        Some(si) 
    } 
    else if let Some(si) = test_re("romrtpar", this_line)  {   // roman numerals and right paranthesis
        Some(si) 
    }  
    else if let Some(si) = test_re("romcapdot", this_line)  {  // capital roman numerals dot   I., II.
        Some(si)
    }   

    else if let Some(s1) = test_re("aldottab", this_line)  { Some(s1) }      // alpha-period followed by tab or space   a.\t, b.\t
    
    else if let Some(s2) = test_re("aldot", this_line)  {     // alpha period. a., b.
        if s2.tag == "e." && s2.text.starts_with("g.") {   // very rare but can occur - a false match that needs to be recognised as a non-tag
            None
        }
        else {
            Some(s2) 
        }
    }        
    
    else if let Some(s3) = test_re("alcapdot", this_line)  {   // alpha caps period. A., B.
        if s3.tag == "N." && s3.text.starts_with("B.") {   // very rare but can occur - a false match that needs to be recognised as a non-tag
            None
        }
        else {
            Some(s3) 
        }
    }   

    else if let Some(s5) = test_re("alrpar", this_line) { Some(s5) }        // alpha with right bracket. a), b)
    else if let Some(s6) = test_re("ospc", this_line)  { Some(s6) }          // open 'o' bullet followed by space or tab, o , o

    else if let Some(s10) = test_re("e_num", this_line)  { Some(s10) }       // exclusion as E or e numbers, optional space E 01, E 02
    else if let Some(s11) = test_re("i_num", this_line)  { Some(s11) }        // inclusion as I or i numbers, optional space i1, i2 
    else {
        None
    };
    
    let processed_result = match raw_result {
        
        Some (r) => {
            
            // get level - necessary to properly compare with previous lines

            let (mut level, _new_level) = if &r.tag_type != prev_tag_type {get_level(&r.tag_type, levels)} else {(prev_level, false)};

            // Some roman numeral tags may in fact be part of an alpha sequence - check sequence
            // They would be in as a sub-criterion at a level 1 more than they should be.

            let mut proc_tag_type = r.tag_type.clone();  // default
  
            if r.tag_type.starts_with("rom") {
                if r.tag == "i." || r.tag == "i)" || r.tag == "I."
                || r.tag == "v." || r.tag == "v)" || r.tag == "V."
                || r.tag == "x." || r.tag == "x)" || r.tag == "X." {
                    if tag_in_sequence(&r.tag, tagged_lines, level) {
                        proc_tag_type = "aldot".to_string();
                        level = level - 1;
                    }
                }
            }

            if r.tag == "i." && r.text.starts_with("e.") { // very rare but can occur - a false match that needs to be recognised as a non-tag
                None
            }
            else {
                Some(CLine {
                    seq_num: (index + 1) as i32,
                    tag: r.tag,
                    tag_type: proc_tag_type,
                    indent_level: level,
                    text: r.text,
                })
            }
        },
        None => None,
    };

    processed_result
}
 

pub fn test_against_other_res(index: usize, this_line: &String, prev_tag_type: &String, prev_level: i32, tagged_lines: &Vec<CLine>, levels: &mut Vec<String>) ->  Option<CLine> {

    let raw_result = 
    if let Some(s9) = test_re("rominpar", this_line)  { Some(s9) }      // roman numerals double bracket   (i), (ii)

    else if let Some(s1) = test_re("alinpar", this_line)   { Some(s1) }           // alpha in parentheses. (a), (b)
    else if let Some(s2) = test_re("numinpar", this_line)  { Some(s2) }      // bracketed numbers (1), (2)
    else if let Some(s3) = test_re("numinbrs", this_line)  { Some(s3) }      // numbers in square brackets   [1], [2]
    
    else if let Some(s4) = test_re("buls1", this_line)  { Some(s4) }         // various bullets 1  
    else if let Some(s5) = test_re("buls2", this_line)  { Some(s5) }         // various bullets 2  
    else if let Some(s6) = test_re("bultab", this_line)  { Some(s6) }        // ? bullet and space or tab     
    else if let Some(s7) = test_re("dshtab", this_line) { Some(s7) }        // hyphen followed by space or tab, -\t, -\t 
    else if let Some(s8) = test_re("strtab", this_line)  { Some(s8) }        // asterisk followed by space or tab  *\t, *\t      

    else if let Some(s10) = test_re("dshonly", this_line)  { Some(s10) }     // dash only   -, -
    else if let Some(s4) = test_re("stronly", this_line)  { Some(s4) }       // asterisk only   *, *
    else if let Some(s5) = test_re("semcolonly", this_line)  { Some(s5) }    // semi-colon only   ;, ; 
    else if let Some(s6) = test_re("qmkonly", this_line)  { Some(s6) }       // question mark only   ?, ?  
    else if let Some(s7) = test_re("invqm", this_line)  { Some(s7) }         // inverted question mark only   ¿, ¿

    else {
        None
    };

    let processed_result = match raw_result {
        
        Some (r) => {
            
            // get level - necessary to properly compare with previous lines

            let (mut level, _new_level) = if &r.tag_type != prev_tag_type {get_level(&r.tag_type, levels)} else {(prev_level, false)};

            // Some roman numeral tags may in fact be part of an alpha sequence - check sequence
            // They would be in as a sub-criterion at a level 1 more than they should be.

            let mut proc_tag_type = r.tag_type.clone();  // default
  
            if r.tag_type.starts_with("rom") {
                if r.tag == "(i)" || r.tag == "(v)" || r.tag == "(x)" {
                    if tag_in_bracketed_sequence(&r.tag, tagged_lines, level) {
                        proc_tag_type = "alinpar".to_string();
                        level = level - 1;
                    }
                }
            }
           
            Some(CLine {
                seq_num: (index + 1) as i32,
                tag: r.tag,
                tag_type: proc_tag_type,
                indent_level: level,
                text: r.text,
            })

        },
        None => None,
    };

    processed_result

}


fn get_level(tag_type: &String, levels: &mut Vec<String>) -> (i32, bool) {
            
    // The tag style has changed - therefore use the get_level function to obtain the 
    // appropriate indent level for the new tag type. 
    
    // This function adds the tag type to the levels vector, if it is not already present 
    // in that vector - the level returned being that of the new entry. Otherwise it 
    // will simply return the associated level number.
    
    let mut new_level = true;
    let mut found_level = 0;
    for i in 2..levels.len() {
        
        if tag_type == &levels[i]
        {
            new_level = false;
            found_level = i;
            break;
        }
    }

    if found_level == 0 {     // tag not found
        levels.push(tag_type.to_string());
        found_level = levels.len() - 1;
    }
   
    // if level = 2, (and is not the first such entry) we have 'returned to a 
    // 'top level' tag. The rest of the levels array needs to be cleared so that 
    // identification of lower level tags is kept 'local' to an individual top level 
    // element, and built up as necessary for each top level element.

    if found_level == 2 && levels.len() > 3    // Remove all but the first 2 (levels 0 and 1) entries from the levels vector
    {
        for _ in levels.drain(3..) {}
    }

    (found_level as i32, new_level)

}


fn tag_in_sequence(tag: &String, tagged_lines: &Vec<CLine>, current_indent_level: i32) -> bool {

    // Rolls back through the lines tagged so far (if any) to see if the most recent line with the same indent level
    // has the form equivalent to the preceeding character, i.e. if the parameter passed appears to be in a sequence
    
    let mut in_sequence = false;
    if tagged_lines.len() > 0 {

        let first_char = tag.first_char();
        let target_char = ((first_char as u8) - 1) as char;  // going up one letter in the ascii table
        let max_i = tagged_lines.len() - 1;

        for i in (0..max_i).rev() {
            if tagged_lines[i].indent_level == current_indent_level - 1 
                && tagged_lines[i].tag.starts_with(target_char) {
                in_sequence = true;
                break;
            }
        } 
    }
    in_sequence
}


fn tag_in_bracketed_sequence(tag: &String, tagged_lines: &Vec<CLine>, current_indent_level: i32) -> bool {

    // Rolls back through the lines tagged so far (if any) to see if the most recent line with the same indent level
    // has the form equivalent to the preceeding character, i.e. if the parameter passed appears to be in a sequence
    
    let mut in_sequence = false;
    if tagged_lines.len() > 0 {

        let first_char = tag.nth_char(1);
        let target_char = ((first_char as u8) - 1) as char;  // going up one letter in the ascii table
        let target_string = format!("({}", target_char);
        let max_i = tagged_lines.len() - 1;

        for i in (0..max_i).rev() {
            if tagged_lines[i].indent_level == current_indent_level - 1
                && tagged_lines[i].tag.starts_with(&target_string) {
                in_sequence = true;
                break;
            }
        } 
    }
    in_sequence
}


fn tag_in_numeric_sequence(tag_num: i32, tagged_lines: &Vec<CLine>, current_indent_level: i32) -> bool {

    // Rolls back through the lines tagged so far (if any) to see if the most recent line with the same indent level
    // has the form equivalent to the preceeding character, i.e. if the parameter passed appears to be in a sequence
    
    let mut in_sequence = false;
    if tagged_lines.len() > 0 {
        let target_num = tag_num - 1;  // the number before
        let max_i = tagged_lines.len() - 1;

        for i in (0..max_i).rev() {
            if tagged_lines[i].indent_level == current_indent_level {
                    let test_tag = tagged_lines[i].tag.trim().to_string();
                    let test_tag_num: i32 = test_tag.parse().unwrap_or_else(|_| 0);
                    if test_tag_num == target_num {
                    in_sequence = true;
                    break;
                }
            }
        } 
    }
    in_sequence
}