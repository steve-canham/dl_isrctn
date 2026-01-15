use super::iec_structs::*;


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


pub fn remove_iec_header_text(s: &String) -> Option<String> {

    if s.len() < 3 {
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

/* 
pub fn is_spurious_line(input_line: &String) -> bool {

    if input_line.is_empty()
    {
        true;
    }

    else {

        string line = input_line.Trim().ToLower();
        if (line is "inclusion:" or "included:" or "exclusion:" or "excluded:")
        {
            return true;;
        }

        input_line = input_line.Replace("key inclusion criteria", "", true, CultureInfo.CurrentCulture);
        input_line = input_line.Replace("inclusion criteria include", "", true, CultureInfo.CurrentCulture);
        input_line = input_line.Replace("key exclusion criteria", "", true, CultureInfo.CurrentCulture);
        ;
        input_line = input_line.Replace("exclusion criteria include", "", true, CultureInfo.CurrentCulture);
        input_line = input_line.Replace("key criteria", "", true, CultureInfo.CurrentCulture);
        input_line = input_line.Replace("inclusion criteria", "", true, CultureInfo.CurrentCulture);
        input_line = input_line.Replace("exclusion criteria", "", true, CultureInfo.CurrentCulture);
        
        if (string.IsNullOrEmpty(input_line) || input_line.Length < 4)
        {
            return true;;
        }
        
        return false;  // the default if the line passes the tests below
    }
}
*/

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