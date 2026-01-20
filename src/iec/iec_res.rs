use std::sync::LazyLock;
use regex::Regex;
use std::collections::HashMap;
//use log::info;

pub struct RegexResults {
    pub tag: String,
    pub tag_name: String,
    pub new_line: String,

}

pub static IE_RE_MAP: LazyLock<HashMap<&'static str, Regex>> = LazyLock::new(||{
    
    let mut map = HashMap::new();

    map.insert("numdot4", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());  // numeric sub-sub-sub-heading. N.n.n.n
    map.insert("numdot3", Regex::new(r"^\d{1,2}\.\d{1,2}\.\d{1,2}").unwrap());           // numeric sub-sub-heading. N.n.n 
    map.insert("numdot2", Regex::new(r"^\d{1,2}\.\d{1,2}\.").unwrap());                  // numeric sub-heading. N.n. 
    map.insert("numdotnumspc", Regex::new(r"^\d{1,2}\.\d{1,2}\s").unwrap());             // numeric sub-heading space (without final period) N.n
    map.insert("numdotnumalcap", Regex::new(r"^\d{1,2}\.\d{1,2}[A-Z]").unwrap());        // number-dot-number cap letter  - Cap is usually from text (no space)
    map.insert("numdotspc", Regex::new(r"^\d{1,2}\.\s").unwrap());                       // number period and space / tab 1. , 2.    
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
    map.insert("numalcap", Regex::new(r"^\d{1,2}[A-Z]{1} ").unwrap());                   // number-cap letter  - might give false positives    
    map.insert("numal", Regex::new(r"^\d{1,2}[a-z]{1} ").unwrap());                      // number plus letter and space Na, Nb

    map.insert("aldottab", Regex::new(r"^[a-z]\.\t").unwrap());                          // alpha-period followed by tab   a.\t, b.\t
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
    map.insert("dblstr", Regex::new(r"^\*\*").unwrap());                                 // two asterisks   **, **
    map.insert("stronly", Regex::new(r"^\*").unwrap());                                  // asterisk only   *, *
    map.insert("semcolonly", Regex::new(r"^;").unwrap());                                // semi-colon only   ;, ; 
    map.insert("qmkonly", Regex::new(r"^\?").unwrap());                                  // question mark only   ?, ?  
    map.insert("invqm", Regex::new(r"^¿").unwrap());                                     // inverted question mark only   ¿, ¿
  
    map
});

pub fn test_re(tag_name: &str, this_line: &String) ->  Option<RegexResults> {

    let re = IE_RE_MAP.get(tag_name).unwrap();  // should always be present
    if let Some(c) = re.captures(this_line) {
        let tag = c.get_match().as_str();

        match tag_name {
            "none" => None,
            _ => Some (RegexResults {
                tag: tag.to_string(),
                tag_name: tag_name.to_string(),
                new_line: this_line[tag.len()..].trim().to_string(),
            }) 
        }
    }
    else {
        None
    }
}

pub fn test_against_numdot_res(this_line: &String) ->  Option<RegexResults>{

    // Ordering of these 'numdot' REs is important and not easily done from the hashmap
    // where they are stored. Therefore each one called individually in the 
    // required order.

    let raw_result = 
    if let Some(s1) = test_re("numdotspc", this_line) { Some(s1) }               // number period and space / tab 1. , 2.   
    else if let Some(s2) = test_re("numdotnumspc", this_line) { Some(s2) }       // numeric sub-heading space (without final period) N.n
    else if let Some(s3) = test_re("numdot4", this_line) { Some(s3) }            // numeric sub-sub-sub-heading. N.n.n.n
    else if let Some(s4) = test_re("numdot3", this_line) { Some(s4) }            // numeric sub-sub-heading. N.n.n 
    else if let Some(s5) = test_re("numdot2", this_line) { Some(s5) }            // numeric sub-heading. N.n. 
    else if let Some(s6) = test_re("numdotrtpar", this_line) { Some(s6) }        // number followed by dot and right bracket  1.), 2.)
    else if let Some(s7) = test_re("numdotnumalcap", this_line) { Some(s7) }     // number-dot-number cap letter  - Cap is usually from text (no space)
    else if let Some(s8) = test_re("numdot", this_line) { Some(s8) }             // number period only - can give false positives
    else {
        None
    };

    // Guard against ambiguity, especially as often final periods may be omiited
    // in a small proportion of the lines

    raw_result
      
}



pub fn test_against_numeric_res(this_line: &String) ->  Option<RegexResults>{

    let raw_result = 
    if let Some(s1) = test_re("numaldot", this_line) { Some(s1) }
    else if let Some(s2) = test_re("numal", this_line) { Some(s2) }
    else if let Some(s3) = test_re("numrtpardot", this_line) { Some(s3) }
    else if let Some(s4) = test_re("numrtpar", this_line) { Some(s4) }
    else if let Some(s5) = test_re("numcol", this_line) { Some(s5) }
    
    else if let Some(s6) = test_re("numrtbr", this_line) { Some(s6) }
    else if let Some(s7) = test_re("numdshnumpar", this_line) { Some(s7) }   
    else if let Some(s8) = test_re("numdshpar", this_line) { Some(s8) }   
    else if let Some(s9) = test_re("numdsh", this_line) { Some(s9) }
    else if let Some(s10) = test_re("numslash", this_line) { Some(s10) }
    else if let Some(s11) = test_re("numtab", this_line) { Some(s11) }   

    else if let Some(s12) = test_re("num3spc", this_line) { Some(s12) }   
    else if let Some(s13) = test_re("numspc", this_line) { Some(s13) }
    else if let Some(s14) = test_re("numalcap", this_line) { Some(s14) }
    else if let Some(s15) = test_re("numal", this_line) { Some(s15) }
    else {
        None
    };

    // Guard against ambiguity

    raw_result 
    

/* 
    let mut tag = "";
    let mut tag_name = "none";
    let mut regex = "".to_string(); 

    for r in IE_RE_MAP.iter() {

        let re = r.1;
        if let Some(c) = re.captures(&this_line) {
            tag = c.get_match().as_str();
            tag_name = *r.0; 
            regex = re.to_string();

            break;
        }
    }
*/
/*
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

 */

}

/* 
fn tag_in_sequence(c: char, tagged_lines: &Vec<IECLine>, current_indent_level: i32) -> bool {

    // Rolls back through the lines tagged so far (if any)
    // to see if the most recent line with the same indent level
    // has the form equivalent to the preceeding character
    // if the parameter passed appears to be in a sequence
    
    let mut in_sequence = false;
    if tagged_lines.len() > 0 {
        for i in (0..tagged_lines.len()).rev() {
            if tagged_lines[i].indent_level == current_indent_level {

            } 
        }
    }
    in_sequence

}
*/

pub fn test_against_alpha_res(this_line: &String) ->  Option<RegexResults> {
    
    /* 



    if let Some(mut si) = test_re("romrtpar", this_line) {   // roman numerals right brackets    i), ii)
        if si.tag == "i)" && tag_in_sequence('i', tagged_lines) {
            si.tag_name = "alrpar".to_string();
        }
        if si.tag == "v)" && tag_in_sequence('v', tagged_lines)  {
            si.tag_name = "alrpar".to_string();
        }
        if si.tag == "x)" && tag_in_sequence('x', tagged_lines)  {
            si.tag_name = "alrpar".to_string();
        }
        Some(si)
    }      
    else if let Some(mut si) = test_re("romdot", this_line) {   // roman numerals dot   i., ii.
        if si.tag == "i." && prev_tag.starts_with('h') {
            si.tag_name = "aldot".to_string();
        }
        if si.tag == "v." && prev_tag.starts_with('u') {
            si.tag_name = "aldot".to_string();
        }
        if si.tag == "x." && prev_tag.starts_with('w') {
            si.tag_name = "aldot".to_string();
        }
        Some(si) 
    }  
    else if let Some(mut si) = test_re("romcapdot", this_line) {   // capital roman numerals dot   I., II.
        if si.tag == "I." && prev_tag.starts_with('H') {
            si.tag_name = "alcapdot".to_string();
        }
        if si.tag == "V." && prev_tag.starts_with('U') {
            si.tag_name = "alcapdot".to_string();
        }
        if si.tag == "X." && prev_tag.starts_with('W') {
            si.tag_name = "alcapdot".to_string();
        }
        Some(si) 
    }     
*/
    let raw_result = if let Some(si) = test_re("romrtpar", this_line) { Some(si) }    
    else if let Some(si) = test_re("romdot", this_line) { Some(si) }  // roman numerals dot   i., ii.
    else if let Some(si) = test_re("romcapdot", this_line) { Some(si) }   // capital roman numerals dot   I., II.

    else if let Some(s1) = test_re("aldottab", this_line) { Some(s1) }      // alpha-period followed by tab   a.\t, b.\t
    else if let Some(s2) = test_re("aldot", this_line) { Some(s2) }         // alpha period. a., b.
    else if let Some(s3) = test_re("alcapdot", this_line) { Some(s3) }      // alpha caps period. A., B.
    else if let Some(s5) = test_re("alrpar", this_line) { Some(s5) }        // alpha with right bracket. a), b)
    else if let Some(s6) = test_re("ospc", this_line) { Some(s6) }          // open 'o' bullet followed by space or tab, o , o

    else if let Some(s10) = test_re("e_num", this_line) { Some(s10) }       // exclusion as E or e numbers, optional space E 01, E 02
    else if let Some(s11) = test_re("i_num", this_line) { Some(s11) }        // inclusion as I or i numbers, optional space i1, i2 
    else {
        None
    };

    /*                    
    let mut tag_name = "none";
    let mut tag = "";
    let mut regex = "".to_string();

    for r in IE_RE_MAP.iter() {

        let re = r.1;
        if let Some(c) = re.captures(&this_line) {

            tag = c.get_match().as_str();
            tag_name = *r.0; 
            regex = re.to_string();

            break;
        }
    }
    */  
    
    match raw_result {
        None => None, 
        Some(rr) => {

                let tag = rr.tag.clone();

                // Dismbiguate where necessary
            
                if tag == "i." || tag == "i.\t" || tag == "I." || tag == "i)"  {
                    
                    // first or scond in list -> latin letters - next line should be ii)


                    // preceding at same indent level = 'h.', 'h.\tab', 'H.', 'h)'-> letter,
                    

                    // neither?


                }

                if tag == "e." {
                    // check for e.g.
                }

                if tag == "N." {
                    // check for N.B.
                }

                if tag == r"o\s" {
                    // check for 'o' bullet'
                }


                if tag == "v." || tag == "v.\t" || tag == "V." || tag == "v)"  {

                }

                Some(rr)  // for now
        }
    }
   
}


// may need to do some checking / corrections before coming out of the loop

pub fn test_against_other_res(this_line: &String) ->  Option<RegexResults>{

    let raw_result = 
    if let Some(s1) = test_re("alinpar", this_line) { Some(s1) }            // alpha in parentheses. (a), (b)
    else if let Some(s2) = test_re("numinpar", this_line) { Some(s2) }      // bracketed numbers (1), (2)
    else if let Some(s3) = test_re("numinbrs", this_line) { Some(s3) }      // numbers in square brackets   [1], [2]
    
    else if let Some(s4) = test_re("buls1", this_line) { Some(s4) }         // various bullets 1  
    else if let Some(s5) = test_re("buls2", this_line) { Some(s5) }         // various bullets 2  
    else if let Some(s6) = test_re("bultab", this_line) { Some(s6) }        // ? bullet and space or tab     
    else if let Some(s7) = test_re("dshtab", this_line) { Some(s7) }        // hyphen followed by space or tab, -\t, -\t 
    else if let Some(s8) = test_re("strtab", this_line) { Some(s8) }        // asterisk followed by space or tab  *\t, *\t      

    else if let Some(s9) = test_re("rominpar", this_line) { Some(s9) }      // roman numerals double bracket   (i), (ii)
    else if let Some(s10) = test_re("dshonly", this_line) { Some(s10) }     // dash only   -, -
    else if let Some(s11) = test_re("dblstr", this_line) { Some(s11) }      // two asterisks   **, **
    else if let Some(s4) = test_re("stronly", this_line) { Some(s4) }       // asterisk only   *, *
    else if let Some(s5) = test_re("semcolonly", this_line) { Some(s5) }    // semi-colon only   ;, ; 
    else if let Some(s6) = test_re("qmkonly", this_line) { Some(s6) }       // question mark only   ?, ?  
    else if let Some(s7) = test_re("invqm", this_line) { Some(s7) }         // inverted question mark only   ¿, ¿

    else {
        None
    };

    match raw_result {
        None => None, 
        Some(rr) => {

                let tag = rr.tag.clone();

                // Dismbiguate where necessary
            
                if tag == "i." || tag == "i.\t" || tag == "I." || tag == "i)"  {
                    
                    // first or scond in list -> latin letters - next line should be ii)


                    // preceding at same indent level = 'h.', 'h.\tab', 'H.', 'h)'-> letter,
                    

                    // neither?


                }

                if tag == "e." {
                    // check for e.g.
                }

                if tag == "N." {
                    // check for N.B.
                }

                if tag == r"o\s" {
                    // check for 'o' bullet'
                }


                if tag == "v." || tag == "v.\t" || tag == "V." || tag == "v)"  {

                }

                Some(rr)  // for now
        }
    }

   // if ldr_name == "none" {None} else {Some((leader.to_string(), ldr_name.to_string(), regex))}

}


