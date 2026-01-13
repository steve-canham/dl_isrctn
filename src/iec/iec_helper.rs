/*
use std::sync::LazyLock;
use regex::Regex;
use std::collections::HashMap;
//use log::info;


pub static NUMBDOT_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();

    map.insert("numdot4", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());  // numeric sub-sub-sub-heading. N.n.n.n
    map.insert("numdot3", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());           // numeric sub-sub-heading. N.n.n 
    map.insert("numdot2", Regex::new(r"^d{1,2}\.\d{1,2}\.").unwrap());                   // numeric sub-heading. N.n. 
    map.insert("numdotnumspc", Regex::new(r"^\d{1,2}\.\d{1,2}\s").unwrap());             // numeric sub-heading space (without final period) N.n
    map.insert("numdotnumalcap", Regex::new(r"^\d{1,2}\.\d{1,2}[A-Z]").unwrap());        // number-dot-number cap letter  - Cap is usually from text (no space)
    map.insert("numdotspc", Regex::new(r"^\d{1,2}\.\s").unwrap());                       // number period and space / tab 1. , 2.    
    map.insert("numdotrtpar", Regex::new(r"^\d{1,2}\.\)").unwrap());                     // number followed by dot and right bracket  1.), 2.)
    map.insert("numdot", Regex::new(r"^\d{1,2}\.").unwrap());                            // number period only - can give false positives
  
    map
});


pub static NUMB_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();

    map.insert("numal", Regex::new(r"^\d{1,2}[a-z]{1} ").unwrap());                      // number plus letter and space Na, Nb
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
 
*/








pub struct IECLine
{
    pub seq_num: i32,
    pub type_id: i32,
    pub split_type: String,
    pub leader: String,
    pub indent_level: i32,
    pub indent_seq_num: i32,
    pub sequence_string: String,
    pub text: String,
}

impl IECLine {

    pub fn new(seq_num: i32, type_id: i32, split_type: &String, text: &String) -> Self {

        IECLine { 
            seq_num: seq_num,
            type_id: type_id,
            split_type: split_type.to_string(),
            leader: "".to_string(),
            indent_level: 0,
            indent_seq_num: 0,
            sequence_string: "".to_string(),
            text: text.to_string(),
        }
    }

}



#[allow(dead_code)]
pub struct TypePars
{
    pub sd_sid: String,
    pub type_base: String,
    pub type_id: i32,
    pub post_crit: i32,
    pub grp_hdr: i32,
    pub no_sep: i32,
    pub type_name: String,
    pub post_crit_name: String,
    pub grp_hdr_name: String,
    pub no_sep_name: String,
    pub sequence_start: String,
}

#[allow(dead_code)]
impl TypePars {

    pub fn new(sd_sid: &String, input_type: &str) -> Self {
        
        let t =  match input_type
        {
            "inclusion" => 1,
            "exclusion" => 2,
            "eligibility" => 3,
            _ => 4
        };

        let ss = match t
        {
            1 => "n.".to_string(),
            2 => "e.".to_string(),
            3 => "g.".to_string(),
            _ => "??".to_string(),
        };

        TypePars { 
            sd_sid: sd_sid.to_string(),
            type_base: input_type.to_string(),
            type_id: t,
            post_crit: 200 + t,
            grp_hdr: 300 + t,
            no_sep: 1000 + t,
            type_name: format!("{} criterion", input_type),
            post_crit_name: format!("{} supplementary statement", input_type),
            grp_hdr_name: format!("{} criteria group heading", input_type),
            no_sep_name: format!("{} criteria with no separator", input_type),
            sequence_start: ss,
        }
    }


    pub fn get_type_name(&self, type_id: i32) -> String {

        if type_id > 1000 {
            self.no_sep_name.clone()
        }
        else if type_id > 300 {
            self.grp_hdr_name.clone()
        }
        else if type_id > 200 {
            self.post_crit_name.clone()
        }
        else {
            self.type_name.clone()
        }
    }

}
    

// The levels vector (Vec<Level>) stores the different indent levels,
// and the names of the leader types associated with them, as used within 
// a set of criteria. The lavel of the line is given by the position of 
// the name in the vector. The L1 leader is at pos(0). L2 leader at pos1 etc.
// The current_seq_num field gives the current sequence number within the level of
// the line

#[allow(dead_code)]
pub struct Level
{
    pub level_name: String,
    pub current_seq_num: i32,
}


#[allow(dead_code)]
impl Level {

    pub fn new(level_name: &String, level_num: i32) -> Self {
        Level { 
            level_name: level_name.to_string(),
            current_seq_num: level_num,
        }
    }
}


pub fn get_level(hdr_name: &String, levels: &mut Vec<Level>) -> usize {

    if levels.len() == 1   // as on initial use - there is a dummy 'none', 0 entry already present.
    {
        levels.push(Level::new(hdr_name, 0));
        return 1;
    }

    // See if the level header has been used - if so
    // return level, if not add and return new level
    
    let mut found_level = 0;
    for i in 0..levels.len() {
        
        if hdr_name == &levels[i].level_name
        {
            found_level = i;
            break;
        }
    }

    if found_level == 0 {
        levels.push(Level::new(hdr_name, 0));
        found_level = levels.len() - 1;
    }

    found_level
}
    


pub fn coalesce_very_short_lines(input_lines: &Vec<&str>) -> Vec<String>
{
    // Function deals with a rare but possible problem with very short lines.
    // May be, or include, 'or' or 'and', or be the result of a spurious CR (e.g. immediately
    // after a line number). 
    
    // In general therefore add such very small lines (vsl) to the preceding normal line. 
    // vsl(s) that include a number should, however, precede the next normal line.
    // (a single vsl with a number as the last line will be added to the penultimate line).
    // vsl initial line(s) should all be prefixed to the first normal line.
    // See tests for the various combinatioons possible and their outputs.
    // N.B. Lines are already trimmed, in calling procedure.
   
    let mut checked_lines:Vec<String> = Vec::new();
    let mut start_pos = 0;

    // Do first line

    if input_lines[0].len() >= 4 {
        checked_lines.push(input_lines[0].to_string());   // Usual situation
        start_pos = 1;
    }
    else {

        // Need to loop until a normal length line is found, adding the short lines in succession

        let mut i = 0;
        let mut first_line_done = false;
        let mut first_line = "".to_string();

        while !first_line_done {

            let s = input_lines[i];
            let slen = s.len();
            first_line = format!("{} {}", first_line.clone(), s);
            
            if slen >= 4 {
                checked_lines.push(first_line.trim().to_string());  

                first_line_done = true;
                start_pos = i + 1;
            }
            i += 1;
        }
    }    

    // Do remaining lines.
    // Actions here dependent if very short line has a digit in it or not.
    
    let mut j = start_pos;
    let mut line_building = false;
    let mut partial_line = "".to_string();

    while j < input_lines.len() {

        let s = input_lines[j].to_string();
        
        if s.len() >= 4
        {   
            if !line_building {
                checked_lines.push(s);  // simple transfer of lines to result set

            }
            else {  // Use the 'normal' length s to finish off the line being built.

                let completed_line = format!("{} {}", partial_line, s);
                checked_lines.push(completed_line.trim().to_string()); 

                line_building = false;    // Reset tracking variables
                partial_line = "".to_string();

            }
        }
        else {
            
            // We have a very short line (vsl), If last line, and not in 'line building' mode
            // simply add vsl to preceding line, whatever its type
            // Otherwise add to current partial line and push the now completed final line.

            if j == input_lines.len() - 1 {

                if line_building {

                    let completed_line = format!("{} {}", partial_line, s);
                    checked_lines.push(completed_line.trim().to_string()); 
                }
                else {
                    let last_pos = checked_lines.len() - 1;
                    checked_lines[last_pos] = format!("{} {}", checked_lines[last_pos], s); // add to preceding line, already transferred 
                }
            }
            else {
                if s.chars().any(|c| c.is_numeric()) {

                    // For a vsl with a digit need to go into 'line building' mode
                    // start the new partial line off as the new (or growing) partial line

                    partial_line = format!("{} {}", partial_line, s);
                    line_building = true;
                }
                else {

                    // for vsl without a digit, add to the preceding transferred line
                    // unless line building, when add to the partial line instead

                    if line_building {
                        partial_line = format!("{} {}", partial_line, s);
                    }
                    else {
                        let last_pos = checked_lines.len() - 1;
                        checked_lines[last_pos] = format!("{} {}", checked_lines[last_pos], s); // add to preceding line, already transferred 
                    }
                }
            }
        }

        j += 1;

    }
             
    checked_lines
}


pub fn trim_internal_iec_headers(s: &String) -> Option<String> {

    if s.len() < 4 {
        None
    }
    else {
        let s_low = s.trim().to_lowercase();
        if s_low == "inclusion:" || s_low == "included:" || s_low =="exclusion:" || s_low == "excluded:"
        {
            None
        }
        else {

            let mut st = s.to_string();
            st = st.replace("key inclusion criteria", "");
            st = st.replace("inclusion criteria include", "");
            st = st.replace("key exclusion criteria", "");
            st = st.replace("exclusion criteria include", "");
            st = st.replace("key criteria", "");
            st = st.replace("inclusion criteria", "");
            st = st.replace("exclusion criteria", "");

            st = st.trim_matches(&[':', ' ']).to_string();
            
            if !st.is_empty() {
                Some(st)
            }
            else {
                None
            }
        }
    }
}



pub fn check_if_all_lines_end_consistently(in_lines: &Vec<IECLine>, allowance: usize)  -> bool {

    let mut valid_end_chars = 0;
    for ln in in_lines
    {
        let end_char = &ln.text.chars().next_back().unwrap();  // always at least one char
        if *end_char == '.' || *end_char == ';' || *end_char ==  ','
        {
            valid_end_chars += 1;
        }
    }
    valid_end_chars >= in_lines.len() - allowance
}

pub fn check_if_all_lines_start_with_caps(in_lines: &Vec<IECLine>, allowance: usize)  -> bool {

    let mut valid_start_chars = 0;
    for ln in in_lines
    {
        let start_char = &ln.text.chars().next().unwrap();  // always at least one char
        if start_char.is_uppercase()
        {
            valid_start_chars += 1;
        }
    }
    valid_start_chars >= in_lines.len() - allowance
}

pub fn check_if_all_lines_start_with_lower_case(in_lines: &Vec<IECLine>, allowance: usize)  -> bool {

    let mut valid_start_chars = 0;
    for ln in in_lines
    {
        let start_char = &ln.text.chars().next().unwrap();  // always at least one char
        if start_char.is_lowercase()
        {
            valid_start_chars += 1;
        }
    }
    valid_start_chars >= in_lines.len() - allowance
}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn check_iec_coalesce_single_very_short_line() {

        // normal size lines

        let mut lines: Vec<&str> = vec!["line 1 stuff", "line 2 stuff", "line 3 stuff", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff".to_string(), "line 2 stuff".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string()]);


        // short line with digits - middle of list, start and end of list

        let mut lines: Vec<&str> = vec!["line 1 stuff", "l2", "line 3 stuff", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff".to_string(), "l2 line 3 stuff".to_string(), "line 4 stuff".to_string()]);

        let mut lines: Vec<&str> = vec!["l1", "line 2 stuff", "line 3 stuff", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["l1 line 2 stuff".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string()]);

        let mut lines: Vec<&str> = vec!["line 1 stuff", "line 2 stuff", "line 3 stuff", "l4"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff".to_string(), "line 2 stuff".to_string(), "line 3 stuff l4".to_string()]);


        // short line without digits - middle of list, start and end of list

        let mut lines: Vec<&str> = vec!["line 1 stuff", "lab", "line 3 stuff", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff lab".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string()]);
        
        let mut lines: Vec<&str> = vec!["lab", "line 2 stuff", "line 3 stuff", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["lab line 2 stuff".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string()]);

        let mut lines: Vec<&str> = vec!["line 1 stuff", "line 2 stuff", "line 3 stuff", "lab"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff".to_string(), "line 2 stuff".to_string(), "line 3 stuff lab".to_string()]);
        
    }

    #[test]
    fn check_iec_coalesce_pair_very_short_lines() {
             
        // 2 no digit sls in middle of list
        
        let mut lines: Vec<&str> = vec!["line 0 stuff t1", "line 1 stuff", "lab", "lob", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t1".to_string(), "line 1 stuff lab lob".to_string(), "line 4 stuff".to_string(), "line 5 stuff".to_string()]);

        // 2 no digit sls at start of list

        let mut lines: Vec<&str> = vec!["lab", "lob", "line 2 stuff t2", "line 3 stuff", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["lab lob line 2 stuff t2".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string(), "line 5 stuff".to_string()]);
 
        // 2 no digit sls at end of list

        let mut lines: Vec<&str> = vec!["line 0 stuff t3", "line 1 stuff", "line 2 stuff", "line 3 stuff", "lab", "lob"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t3".to_string(), "line 1 stuff".to_string(), "line 2 stuff".to_string(), "line 3 stuff lab lob".to_string()]);

        /****************************************/

        // 2 digit sls in middle of list

        let mut lines: Vec<&str> = vec!["line 0 stuff t4", "line 1 stuff", "l2", "l3", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t4".to_string(), "line 1 stuff".to_string(), "l2 l3 line 4 stuff".to_string(), "line 5 stuff".to_string()]);

        // 2 digit sls at start of list

        let mut lines: Vec<&str> = vec!["l1", "l2", "line 2 stuff t5", "line 3 stuff", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["l1 l2 line 2 stuff t5".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string(), "line 5 stuff".to_string()]);
 
        // 2 digit sls at end of list

        let mut lines: Vec<&str> = vec!["line 0 stuff t6", "line 1 stuff", "line 2 stuff", "line 3 stuff", "l4", "l5"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t6".to_string(), "line 1 stuff".to_string(), "line 2 stuff".to_string(), "line 3 stuff".to_string(), "l4 l5".to_string()]);

        /****************************************/

        // no digit sl, digit sl, in middle of list

        let mut lines: Vec<&str> = vec!["line 0 stuff t7", "line 1 stuff", "lab", "l3", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t7".to_string(), "line 1 stuff lab".to_string(), "l3 line 4 stuff".to_string(), "line 5 stuff".to_string()]);

        // no digit sl, digit sl, at start of list

        let mut lines: Vec<&str> = vec!["lab", "l2", "line 2 stuff t8", "line 3 stuff", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["lab l2 line 2 stuff t8".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string(), "line 5 stuff".to_string()]);

        // no digit sl, digit sl, at end of list
        
        let mut lines: Vec<&str> = vec!["line 0 stuff t9", "line 1 stuff", "line 2 stuff", "line 3 stuff", "lab", "l5"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t9".to_string(), "line 1 stuff".to_string(), "line 2 stuff".to_string(), "line 3 stuff lab l5".to_string()]);


        /****************************************/

        // digit sl, no digit sl, in middle of list
        
        let mut lines: Vec<&str> = vec!["line 0 stuff t10", "line 1 stuff", "l2", "lab", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t10".to_string(), "line 1 stuff".to_string(), "l2 lab line 4 stuff".to_string(), "line 5 stuff".to_string()]);

        // digit sl,no digit sl, at start of list

        let mut lines: Vec<&str> = vec!["l1", "lab", "line 2 stuff t11", "line 3 stuff", "line 4 stuff", "line 5 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["l1 lab line 2 stuff t11".to_string(), "line 3 stuff".to_string(), "line 4 stuff".to_string(), "line 5 stuff".to_string()]);

        // digit sl, no digit sl, at end of list

        let mut lines: Vec<&str> = vec!["line 0 stuff t12", "line 1 stuff", "line 2 stuff", "line 3 stuff", "l5", "lab"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 0 stuff t12".to_string(), "line 1 stuff".to_string(), "line 2 stuff".to_string(), "line 3 stuff".to_string(), "l5 lab".to_string()]);


        /****************************************/

        // middle two of 4

        let mut lines: Vec<&str> = vec!["line 1 stuff", "lab", "l3", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff lab".to_string(), "l3 line 4 stuff".to_string()]);

        let mut lines: Vec<&str> = vec!["line 1 stuff", "l3", "lab", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff".to_string(), "l3 lab line 4 stuff".to_string()]);

        let mut lines: Vec<&str> = vec!["line 1 stuff", "lab", "lob", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff lab lob".to_string(), "line 4 stuff".to_string()]);

        let mut lines: Vec<&str> = vec!["line 1 stuff", "l2", "l3", "line 4 stuff"];
        assert_eq!(coalesce_very_short_lines(&mut lines), vec!["line 1 stuff".to_string(), "l2 l3 line 4 stuff".to_string()]);

    }

}