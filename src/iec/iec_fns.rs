
use super::iec_helper::*;
use regex::Regex;
use super::iec_res::*;
//use log::info;

pub fn original_process_iec(sd_sid: &String, in_string: &String, input_type: &str) -> (i32, Vec<IECLine>) {   // Should be a vector of IECLine

    // Oriinally in C# as GetNumberedCriteria
    // Parent process for the other functions

    let crits: Vec<IECLine> = Vec::new();
    let mut _final_cr_lines: Vec<IECLine> = Vec::new();
    let mut initial_cr_lines: Vec<IECLine> = Vec::new();
    let mut processed_cr_lines: Vec<IECLine> = Vec::new();


    let mut null_result = false;

    // input string fully cleaned before being passed in,
    // and checked as not being a place holder.
    // Some further minor cleaning 
    
    let mut in_st = in_string.to_string();

    in_st = in_st.replace("..", ".");
    in_st = in_st.replace(",.", ".");

    // Task now is to create a list of lines, as separated by any carriage returns in the text.
    // The proportion of I/E source strings that are split using carriage returns appears to 
    // vary with the source, but in the majority of cases the data is split this way.
    // There are, however, many cases of spurious CRs splitting lines that are really one statement, 
    // as well as many examples where the criteria list is provided as a single line, without CRs.

    // Establish a struct with suitable code and name attributes for this type of criteria
    // The sd_sid is included to enable possible later checking and error resolution.

    let tv = TypePars::new(sd_sid, input_type);   // sets up a struct for key values

    // Split the input string into separate lines by splitting on carriage returns.
    // Trim the resultant lines and eliminate any that are empty, or that just contain multiple underlines

    let raw_lines: Vec<&str> = in_st.split("\n")
                .map(|p| p.trim())
                .filter(| t| *t != "")
                .filter(| t| !(*t).contains("____") )
                .collect();

    // Join any odd lines with 1, 2, or 3 characters to the preceding or following line (depending on content)

    let repaired_lines = coalesce_very_short_lines(&raw_lines);

    // Then transfer data to vector of IECLine structs (each IECLine will be processed further below).
                
    if repaired_lines.len() == 0 {
        null_result = true;                      // return immediately with no data
    }

    else if repaired_lines.len() == 1 {
                    
        // A single line, i.e. with no carriage returns

        match trim_internal_iec_headers(&repaired_lines[0])
        {
            Some(sline) => {
                processed_cr_lines.push(IECLine{    // temp, should be final CR lines
                seq_num: 1,
                type_id: tv.no_sep,
                split_type: "none".to_string(),
                leader: "All".to_string(),
                indent_level: 0,
                indent_seq_num: 1,
                sequence_string: format!("{}0A", tv.sequence_start),
                text: sline,
                });
            },
            None =>  {
                null_result = true; // line has disappeared after trimming
            },
        }
    }

    else {

        // multiple lines

        for (i, s) in repaired_lines.iter().enumerate() {
            initial_cr_lines.push (
                IECLine::new((i + 1) as i32, tv.type_id, &"cr".to_string(), s));
        }

        // Initially try to find leader characters for each split line
        // then try to correct common errors in the list

        processed_cr_lines = identify_line_leaders(&mut initial_cr_lines, &tv);
        _final_cr_lines = try_to_repair_split_lines(&processed_cr_lines, &tv);
    }

    // We now have one or many 'final_cr_line's. Each now needs to be processed to see
    // it it corresponds to a single criterion or is itself a list of criteria.
    // If the former, the single line can be added to the 'expanded_lines', if the 
    // latter, the set of identified criteria is added.

    // for now, just return the lines semi-processed...
 
    if null_result {
        (0, crits)
    }
    else {
        (2, processed_cr_lines)
    }
 }



 fn identify_line_leaders(initial_lines:&Vec<IECLine>, tv: &TypePars) ->  Vec<IECLine> {

    let mut processed_lines: Vec<IECLine> = Vec::new();

    // Takes a mutable reference to the intiial set of IECLines (initial_lines))
    // and completes the values of each IECLine, in situ, before returning a reference 
    // to those same IECLines.

    let mut i = 0;
    let max_i = initial_lines.len() - 1;  
    let mut level = 0;

    let mut num_no_leader = 0;
    let mut previous_ldr_name = "none".to_string();
    let mut previous_regex_string = "none".to_string();
   
    let mut levels: Vec<Level> = vec![Level::new(&"none".to_string(), 0)];    // Initialise the level vector

    for ie_ln in initial_lines {

        i += 1;

        let this_line = ie_ln.text.clone();
        let mut ldr_name = "none".to_string(); // initial defaults - signify no leader found
        let mut leader = "".to_string();

        // What leader character(s), if any, are starting this line?
        // Can we match on the successful regex string (if any) from the previous line?
        
        if previous_regex_string != "none".to_string() {

            // But numdott REs can give false results, a n.m. is still 
            // picked up as n. if it directly follows n.
            
            if !previous_regex_string.starts_with("numdot") {

                let re = Regex::new(&previous_regex_string).unwrap();
                if let Some(c) = re.captures(&this_line) {
                    leader = c.get_match().as_str().to_string();
                    ldr_name = previous_ldr_name.clone();     
                    // previous_regex_string can stay the same
                }
            }
            else {
                ldr_name = "none".to_string();
            }
        }

        if ldr_name == "none".to_string() {
            
            // Use first character(s) of line to see which regex collection to use

            let first_char = &this_line.chars().next().unwrap();

            if first_char.is_digit(10) {
                
                // If the second or third characteris a period use the 
                // numdot family of REs, otherwise the normal numeric family

                let second_char = &this_line.chars().nth(1).unwrap();
                let third_char = &this_line.chars().nth(2).unwrap();

                if *second_char == '.' || second_char.is_digit(10) && *third_char == '.' {
                    
                    match test_against_numdot_res(&this_line) {
                        Some((s1, s2, s3)) => {
                            leader = s1;
                            ldr_name = s2;
                            previous_regex_string = s3;
                        },
                        None => {},

                        // may need to do some checking / corrections before coming out of the loop



                    }
                }
                else {
                    match test_against_numeric_res(&this_line) {
                        Some((s1, s2, s3)) => {
                            leader = s1;
                            ldr_name = s2;
                            previous_regex_string = s3;
                        },
                        None => {},
                    }
                }
            }
            else if first_char.is_alphabetic() {

                match test_against_alpha_res(&this_line) {
                    Some((s1, s2, s3)) => {
                        leader = s1;
                        ldr_name = s2;
                        previous_regex_string = s3;
                    },
                    None => {
                        ldr_name = "none".to_string();
                    },
                }
            }
            else {
                match test_against_other_res(&this_line) {

                    Some((s1, s2, s3)) => {
                        leader = s1;
                        ldr_name = s2;
                        previous_regex_string = s3;
                    },
                    None => {
                        ldr_name = "none".to_string();
                    },
                }
            }
        }


        /*  

                // some regex patterns have to have additional checks. In other cases 
                // simply break out of the loop with the matched pattern value.

                if ldr_name == "numdotspc"
                {
                    // Turn into numdot, without the space, to ensure that the header type
                    // remains the same even if there are variations in spacing in the source.

                    ldr_name = "numdot";
                    leader = leader.trim();
                    break;
                }

                if ldr_name == "numdotnumalcap" || ldr_name == "numalcap" {

                    // The alpha cap is normally from the text; the space
                    // between the leader and the text is missing.
                    // Take the Alpha character fromthe end of the leader.

                    leader = &leader[0..leader.len() - 1];

                }
            }
        }
        */

        // If a leader has been found ...see if the leader has changed from the
        // most previously used - implying the indentation level has changed.
        // If it has changed 
        // else - store the line as a sub-header, unless it is the last line of the set
        // when it will be classified as a supplementary statement.
        
        let seq_num: i32;
        if ldr_name != "none"
        {
            if ldr_name != previous_ldr_name
            {
                // If the leader style has changed use the get_level function to obtain the 
                // appropriate indent level for the new header type. This function will add 
                // the leader type to the levels vector, if it is not already present in the 
                // collection (the level returned being that of the new entry), otherwise it 
                // will simply return the associated level number.

                level = get_level(&ldr_name.to_string(), &mut levels);

                // if level = 1, (and is not the first) we have 'returned to a 'top level' leader.
                // The rest of the levels array needs to be cleared so that identification of
                // lower level leaders is kept 'local' to an individual top level element, and 
                // built up as necessary for each top level element.

                if level == 1 && levels.len() > 2    // Remove all but the first (level 1) entry from the levels vector
                {
                   for _lev in levels.drain(2..) {}
                }
            }
            
            // Create a nwe iec_line object

            if ie_ln.leader != "Spp".to_string()  // fields may have already been set above, e.g. with '*'
            {
                levels[level].current_seq_num += 1;
                seq_num = levels[level].current_seq_num;

                let new_text = this_line[leader.len()..].trim().to_string();
                let split_type = format!("{}-{}", &ie_ln.split_type, ldr_name);

                processed_lines.push(IECLine {
                    seq_num: ie_ln.seq_num,
                    type_id: ie_ln.type_id,
                    split_type: split_type,
                    leader: leader,
                    indent_level: level as i32,
                    indent_seq_num: seq_num,
                    sequence_string: format!("{}{:0>2}", tv.sequence_start, seq_num),
                    text: new_text,
                });
            }
        }

        else
        {
            num_no_leader += 1; // keep a tally as ALL the lines may be without a leader

            if i == max_i
            {
                // initially at least, make this final line without any 'leader' character
                // a supplement (at the same indent level as the previous criteria).

                levels[level].current_seq_num += 1;
                seq_num = levels[level].current_seq_num;

                processed_lines.push(IECLine {
                    seq_num: ie_ln.seq_num,
                    type_id: tv.post_crit,
                    split_type: ie_ln.split_type.clone(),
                    leader: "Spp".to_string(),
                    indent_level: level as i32,
                    indent_seq_num: seq_num,
                    sequence_string: format!("{}{:0>2}", tv.sequence_start, seq_num),
                    text: this_line,
                });
            }
            else
            {
                // Otherwise, by default, add a line without any 'header' character as a sub-header
                // in the list (at the same indent level as the previous criteria) 

                levels[level].current_seq_num += 1;
                seq_num = levels[level].current_seq_num;

                processed_lines.push(IECLine {
                    seq_num: ie_ln.seq_num,
                    type_id: tv.grp_hdr,
                    split_type: ie_ln.split_type.clone(),
                    leader: "Hdr".to_string(),
                    indent_level: level as i32,
                    indent_seq_num: seq_num,
                    sequence_string: format!("{}{:0>2}", tv.sequence_start, seq_num),
                    text: this_line,
                });

            }
        }

        previous_ldr_name = ldr_name;

    }
    

    // check the 'all without a leader' possibility - allowing a single exception

    if  (processed_lines.len() > 4 && num_no_leader >= processed_lines.len() - 1) ||
        (processed_lines.len() > 2 && num_no_leader == processed_lines.len())
        {

            // First check all lines have a length of at least 1, i.e. are not empty, before proceeding further
            // Empty lines may occur - very rarely - if a line is 'all leader'
            // (though most of these should have been eliminated at the beginning)
            // or if, for example, the original split in CTG left a leader before the 'Exclusion Criteria' statement
       
            let mut checked_lines: Vec<IECLine> = Vec::new();
            for ln in processed_lines {
                if ln.text.trim() != "" {
                    checked_lines.push(ln);
                }
            } 

            // None (or very few) of the lines have a leader character. If they (or most of them) had proper 
            // termination, or consistent line starting, then it is possible that they are
            // simply differentiated by the CRs alone...

            let mut assume_crs_only = check_if_all_lines_end_consistently(&checked_lines, 1)
                                   || check_if_all_lines_start_with_caps(&checked_lines, 1)
                                   || check_if_all_lines_start_with_lower_case(&checked_lines, 0);

            // otherwise check for a consistent bullet type character

            let mut use_as_header = "".to_string();

            if !assume_crs_only
            {
                // a chance that an unknown bullet character has been used to start each line
                // start with the second line (as the first may be different) and see if they are all the same
                // Don't test letters as some people use formulaic criteria all starting with the same word

                let test_char = &checked_lines[1].text.chars().next().unwrap();  // should always be at least one character in each line
                if !test_char.is_alphabetic()
                {
                    let mut valid_start_chars = 0;
                    
                    for k in 1..checked_lines.len()
                    {
                        // May be no termination applied but each line starts with a capital letter

                        let start_char = &checked_lines[k].text.chars().next().unwrap();
                        if start_char == test_char
                        {
                            valid_start_chars += 1;
                        }
                    }

                    if valid_start_chars == checked_lines.len() - 1
                    {
                        assume_crs_only = true;
                        use_as_header = test_char.to_string();
                    }
                }
            }

            // for these records (assumed split on crs only) the fields in the IEC objects need to be changed
            // which means constructing yet another vector over these criteria

            if assume_crs_only
            {

                let mut revised_lines: Vec<IECLine> = Vec::new();

                let mut line_num = 0;
                let leader_string = if use_as_header == "" { "@".to_string() } else { use_as_header.clone() };
                
                for ln in checked_lines {
                    
                    let mut rev_text  = ln.text.clone();  // as a default
                     if use_as_header != "".to_string() // single character only
                    {
                        if line_num == 0
                        {
                            if ln.text.chars().next().unwrap().to_string() == use_as_header
                            {
                                rev_text = ln.text[1..].to_string();
                            }
                        }
                        else
                        {
                            if ln.text.len() >= 2
                            {
                                rev_text = ln.text[1..].to_string();
                            }
                        }
                    }


                    // Identify what appear to be headers but only make initial hdr
                    // have indent 0, if it fits the normal pattern
                    
                    let type_id: i32;
                    let indent_level: i32;
                    let indent_seq_num: i32;
                    let leader:String;

                    if ln.text.ends_with(':') || ln.text == ln.text.to_uppercase()
                    {
                        leader = leader_string.clone() + "Hdr";
                        type_id = tv.grp_hdr;

                        if line_num == 0
                        {
                            indent_level = 0;
                            indent_seq_num = 1;
                        }
                        else
                        {
                            indent_level = 1;
                            indent_seq_num = line_num;
                        }
                    }
                    else
                    {
                        leader = leader_string.clone();
                        indent_level = 1;
                        indent_seq_num = line_num;
                        type_id = tv.type_id;
                    }

                    revised_lines.push(IECLine {
                        seq_num: ln.seq_num,
                        type_id: type_id,
                        split_type: "cr assumed".to_string(),
                        leader: leader,
                        indent_level: indent_level,
                        indent_seq_num: indent_seq_num,
                        sequence_string: format!("{}{:0>2}", tv.sequence_start, indent_seq_num),
                        text: rev_text,
                    });

                    line_num +=1;

                }

                revised_lines
            }

            else {
                checked_lines   // return the lines as modified before the 'assume cr' processing above
            }

        }
        else {
            processed_lines    // return the originally derived IEC lines
        }

}


fn try_to_repair_split_lines(__processed_lines:&Vec<IECLine>, _tv: &TypePars) ->  Vec<IECLine>{

    let final_lines: Vec<IECLine> = Vec::new();




    final_lines


 }
    
