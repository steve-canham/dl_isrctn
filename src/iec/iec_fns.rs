
use super::iec_helper::*;
use super::iec_res::*;
use super::iec_structs::*;

// use log::info;

pub fn original_process_iec(sd_sid: &String, in_string: &String, input_type: &str) -> (i32, Vec<IECLine>) {   // Should be a vector of IECLine

    // Oriinally in C# as GetNumberedCriteria
    // Parent process for the other functions

    let crits: Vec<IECLine> = Vec::new();
    let mut final_cr_lines: Vec<IECLine> = Vec::new();

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

        if !is_redundant_header(&repaired_lines[0])
        {
            final_cr_lines.push(IECLine {    // temp, should be final CR lines
            seq_num: 1,
            type_id: tv.no_sep,
            tag_type: "none".to_string(),
            tag: "All".to_string(),
            indent_level: 0,
            indent_seq_num: 1,
            sequence_string: format!("{}.A00", tv.sequence_start),
            text: repaired_lines[0].clone(),
            });
        }
        else {
            null_result = true; // line has disappeared after trimming
        }
    }

    else {

        // multiple lines

        let mut tagged_lines = tag_lines(&repaired_lines);

        let mut joined_lines = repair_split_lines(&mut tagged_lines, &tv);
              
        if has_none_or_very_few_tags(&joined_lines)
        {
            joined_lines = process_no_tags_situations(&mut joined_lines)
        }
        
        final_cr_lines = sequence_lines(&joined_lines, &tv);
        
        //final_cr_lines = processed_cr_lines;
    }

    // Now have one or many 'final_cr_line's. Each now needs to be processed to see
    // it it corresponds to a single criterion or is itself a list of criteria.
    // If the former, the single line can be added to the 'expanded_lines', if the 
    // latter, the set of identified criteria is added.

    // for now, just return the lines semi-processed...
 
    if null_result {
        (0, crits)
    }
    else {
        let return_code;
        if final_cr_lines.len() == 1 {
            return_code = if input_type == "inclusion" {4} else {16};
        }
        else {
            return_code = if input_type == "inclusion" {8} else {32};
        }
        (return_code, final_cr_lines)
    }

    // to be continued...

}


fn tag_lines(initial_lines:&Vec<String>) ->  Vec<CLine> {

    let mut tagged_lines: Vec<CLine> = Vec::new();

    // Consumes the intiial lines, as split by CR, to create
    // an IECLine struct for each, returning a vector of those IECLines.

    let max_i = initial_lines.len() - 1;  
    let mut previous_tag_type = "none".to_string();
    let mut previous_level = 0;
    let mut levels: Vec<String> = vec!["none".to_string(), "Hdr".to_string()];    // Initialise the level vector
    //let mut hdr_seq_num = 0;
    
    // Start of loop
    
    for (i, ln) in initial_lines.into_iter().enumerate() {

        let mut iec: Option<CLine> = None;    // re-initialise as None
               
        // What tag character(s), if any, are starting this line?

        // First - can we match on the successful regex string (if any) 
        // from the previous line?
        // BUT - numdot REs can give false results, e.g. 'n.m.' is still 
        // picked up as 'n.' if it directly follows 'n.' Therefore do 
        // not use this shortcut if the last tag was a numdot.
        
        if previous_tag_type != "none".to_string() 
                && !previous_tag_type.starts_with("numdot"){

            if let Some(rr) = test_re(&previous_tag_type,&ln) {
                iec = Some(CLine {
                    seq_num: (i + 1) as i32,
                    tag: rr.tag,
                    tag_type: rr.tag_type,
                    indent_level: previous_level,
                    text: rr.text,
                });
            }
        }

        if iec.is_none() {    // still

            // Previous tag type does not work, (or it was a 'numdot').
            // So need to search through the REs - using first 
            // character(s) of line to see which regex cluster to use

            let first_char = &ln.first_char();
            if first_char.is_digit(10) {
                
                // If the second or third characteris a period use the 
                // numdot family of REs, otherwise the normal numeric family

                let second_char = &ln.nth_char(1);
                let third_char = &ln.nth_char(2);

                if *second_char == '.' || (second_char.is_digit(10) && *third_char == '.') {
                    iec = test_against_numdot_res(i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
                }
                else {
                    iec = test_against_numeric_res(i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
                }
            }
            else if first_char.is_alphabetic() {
                iec = test_against_alpha_res(i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
            }
            else {
                iec = test_against_other_res(i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
            }

        }


        let iec_line= match iec {

            Some(l) => l,   // found a tag
            None => {                // no tag - construct a 'header' IECLine
                    let mut tag = "Hdr".to_string();

                    if i == max_i       // initially, make this final line without a tag a 'supplement'
                    {
                        tag = "Spp".to_string();
                    }

                    CLine {
                        seq_num: (i + 1) as i32,
                        tag_type: "none".to_string(),
                        tag: tag,
                        indent_level: 1,
                        text: ln.to_string(),
                    }
             },
        };

        // Reset 'previous' variables and 
        // Push the new struct to the growing target vector

        previous_tag_type = iec_line.tag_type.clone();
        previous_level = iec_line.indent_level;
        tagged_lines.push(iec_line);

    }
  
    // Finally, again check all lines are not empty before proceeding further. 
    // Empty lines may occur - very rarely - if a line is 'just tag'
    // (though most of these should have been eliminated at the beginning)
    // or if, for example, the original split in CTG left a tag before the 'Exclusion Criteria' statement
    
    let processed_lines: Vec<CLine> = tagged_lines
                                       .into_iter()
                                       .filter(|t| t.text != "")
                                       .collect();
    processed_lines
}


fn repair_split_lines(plines:&mut Vec<CLine>, tv: &TypePars) ->  Vec<CLine>{

    // Repair some of the more obvious mis-interpretations
    
    // First, working backwards, re-aggregate lines split with spurious \n.
    // Add the first line (no previous line for it to merge into).
    // Then loop through the rest.

    let mut reversed_lines: Vec<CLine> = Vec::new();
    let max_i = plines.len() - 1;

    for i in (0..=max_i).rev() {
        
        let mut transfer_line = true;   // therefore first line added automatically
        if i > 0 {                            // but others need to be examined

            let this_text = plines[i].text.clone();

            // Remove (i.e. don't transfer) simple header lines or headings with no information

            if is_redundant_header(&this_text) {
                transfer_line = false;
            }
            else {

                let prev_text = plines[i - 1].text.clone();
                let init_char = &this_text.chars().next().unwrap_or('?'); 

                // Try and identify spurious 'headers', i.e. lines without leaders, caused by spurious CRs.
                // Following recent revisions spurious CRs no longer seem to exist within CGT IEC data. 
                // Lines without headers (usually 1., 2., *, *) are therefore normally genuine header
                // statements for this source. The routines below therefore do not apply for CTG data.

                if !tv.sd_sid.starts_with("NCT")
                {
                    // Consider lines originally specified as headers (leave out last line)  

                    if plines[i].indent_level == 1 && i < max_i   // a header line
                    {
                        // Ignore line starting with 'Note'- very likely to be a true 'header'. 
                        // Also ignore lines where preceding line ends with ':' (unless both lines end with :)
                        // as a final ':' usually denotes a true (sub-) header.
                        // (N.B. Very last line not checked).

                        if !this_text.to_lowercase().starts_with("note") && !prev_text.ends_with(':')
                        {
                            if !this_text.ends_with(':')  // Consider first lines without a final ':'.
                            {
                                // Check if the starts with a lower case letter or digit, and
                                // previous line does not end in a full stop or (semi) colon. If the case,
                                // add the line to the preceding line.

                                if !(prev_text.ends_with('.') || prev_text.ends_with(';') || prev_text.ends_with(':'))
                                            && (init_char.is_lowercase() || init_char.is_digit(10)) {  
                                        
                                    plines[i - 1].text = format!("{} {}", prev_text, this_text)
                                                        .replace("  ", " ");
                                    transfer_line = false;

                                    // Difficulty is that some spurious \n are mid-word...and some
                                    // are between words - no easy way to distinguish - hence the replace
                                }
                            }
    
                            // Then consider lines with a final ':'. Only those that do not
                            // start with a capital letter. This indicates a likely 'split' 
                            // header that should be merged with the line above.

                            if this_text.ends_with(':')
                                && (init_char.is_lowercase() || init_char.is_digit(10))
                            {

                                let prev_lchar = &plines[i - 1].text.last_char();
                                if *prev_lchar != '.' &&  *prev_lchar != ';'
                                {
                                    if *prev_lchar == ':'{   // both lines end in ':'

                                        // Merge, changing the first colon to a period.

                                        plines[i - 1].text = format!("{}. {}", prev_text[..(prev_text.len() - 1)].to_string(), this_text)
                                    }
                                    else {
                                        plines[i - 1].text = format!("{} {}", prev_text, this_text)
                                                        .replace("  ", " ");
                                    }
                                    transfer_line = false;
                                }

                            }

                            // Othewrwise lines that end with a colon are seen as 'true' headers.

                        }
                    }

                    // Consider lines originally specified as 'supplementary' final lines
                    
                    if plines[i].indent_level == 1 && i == max_i {    // Final line with no tag = 'supplementary' line

                        let low_line = this_text.to_lowercase();
                        if !this_text.ends_with(':') && !this_text.starts_with('*')
                            && !low_line.starts_with("note") && !low_line.starts_with("other ")
                            && !low_line.starts_with("for further ") && !low_line.starts_with("further") 
                            && !low_line.starts_with("for more ") && !low_line.starts_with("more ")
                            && !low_line.contains("exclusion") && !low_line.contains("excluded") {

                            // Almost always is a spurious supplement, better considered as a normal criterion.
                            // Whether should be joined depends on whether there is an initial
                            // lower case or upper case letter... 

                            if init_char.is_lowercase() 
                            {
                                plines[i - 1].text = format!("{} {}", prev_text, this_text)
                                                    .replace("  ", " ");
                                transfer_line = false;
                            }
                            else
                            {
                                // Reset the indent level and tag  - 
                                // Normally indent should follow those 
                                // of the preceding criterion line; as no tag was found 
                                // type should be 'cr_assumed' but tag remains as 'Spp'

                                // but always?

                                plines[i].indent_level = plines[i - 1].indent_level;
                               // if plines[i].indent_level == 1 {
                               //     plines[i].indent_level = 2;  // Ensure no longer a 'Spp'
                               // }
                                plines[i].tag_type = "cr assumed".to_string();
                                //plines[i].tag = "@Spp".to_string();
                            }
                        }
                    }
                }      
            }
        }

        if transfer_line
        {
            reversed_lines.push(plines[i].clone());
        }
    }

    // Put things back in correct order and return the vector   
   
    reversed_lines.reverse();
    reversed_lines              
 
}


fn process_no_tags_situations(processed_lines: &mut Vec<CLine>) -> Vec<CLine> {

    // Processed lines include at least 3 lines.
    // First check for an unidentified consistent bullet type character

    if let Some(c) = check_if_lines_all_start_with_same_bullet(&processed_lines) {
               
        for i in 0..processed_lines.len() {   // new bullet chharacter found!

            let orig_text = processed_lines[i].text.clone();
            let mut rev_text  = orig_text.clone();  // as a default

            if i == 0 || i == processed_lines.len() - 1
            {
                if processed_lines[i].text.first_char()== c
                {
                    rev_text = orig_text[1..].to_string();  // remove bullet if it appears on first or last lines
                }
            }
            else 
            {
                if orig_text.len() >= 2
                {
                    rev_text = orig_text[1..].to_string();
                }
            }

            if rev_text.ends_with(':') || rev_text == rev_text.to_uppercase()
            {
                processed_lines[i].tag = format!("{}Hdr", c);
                processed_lines[i].indent_level = 1;
            }
            else
            {
                processed_lines[i].tag = c.to_string();
                processed_lines[i].indent_level = 2;
            }

            processed_lines[i].text = rev_text;
            processed_lines[i].tag_type = "found bullet".to_string();

        }

    }

    // May be lines are similar enough to suggest that they are criteria split on CRs only

    
    if check_if_all_lines_end_consistently(&processed_lines, 1)
                  || check_if_all_lines_start_with_caps(&processed_lines, 1)
                  || check_if_all_lines_start_with_lower_case(&processed_lines, 0)
    {
        // May be lines are similar enough to suggest that they are criteria split on CRs only

        for i in 0..processed_lines.len() {
            
            let ptext = processed_lines[i].text.clone();

            // Identify what appear to be headers 
            // Otherwise treat all lines as being 'normal' criteria

            processed_lines[i].tag_type = "cr assumed".to_string();
            if ptext.ends_with(':') || ptext == ptext.to_uppercase()
            {
                processed_lines[i].tag = "@Hdr".to_string();
                processed_lines[i].indent_level = 1;
            }
            else
            {
                processed_lines[i].tag = "@".to_string();
                processed_lines[i].indent_level = 2;
            }
        }

    }

    processed_lines.to_owned()

}


fn sequence_lines(joined_lines: &Vec<CLine>, tv: &TypePars) -> Vec<IECLine> {

    let mut sequenced_lines: Vec<IECLine> = Vec::new();

    // Clarify situation with one or two criteria only.

    if joined_lines.len() == 1 { 
        let single_iec = IECLine {
            seq_num: 1,
            type_id: tv.no_sep,
            tag_type:"none".to_string(),
            tag: "All".to_string(),
            indent_level: 0,
            text: joined_lines[0].text.clone(),
            indent_seq_num: 1,
            sequence_string: format!("{}.A00", tv.sequence_start),
        };
        sequenced_lines.push(single_iec);
    }

    if joined_lines.len() == 2 {

        let top_text = joined_lines[0].text.clone();
        let bottom_text = joined_lines[1].text.clone();

        if top_text.ends_with(":") && top_text.to_lowercase().contains("criteria")
        {
            // Probably a genuine header (unusual). Make the second line a criterion

            let iec0 = IECLine {
                seq_num: 1,
                type_id: tv.grp_hdr,
                tag_type: joined_lines[0].tag_type.clone(),
                tag: joined_lines[0].tag.clone(),
                indent_level: 1,
                text: top_text,
                indent_seq_num: 1,
                sequence_string: format!("{}.H01", tv.sequence_start),
            };

            let iec1 = IECLine {
                seq_num: 2,
                type_id: tv.type_id,
                tag_type:joined_lines[1].tag_type.clone(),
                tag: "-1-".to_string(),
                indent_level: 2,
                text: bottom_text,
                indent_seq_num: 1,
                sequence_string: format!("{}.01", tv.sequence_start),
            };

            sequenced_lines.push(iec0);
            sequenced_lines.push(iec1);

        }
        else
        {
            if check_if_all_lines_end_consistently(&joined_lines, 0)
                || check_if_all_lines_start_with_caps(&joined_lines, 0)
            {
                // More likely that these are a pair of criteria statements 
                // (or multi-criteria statements)

                let iec0 = IECLine {
                    seq_num: 1,
                    type_id: tv.type_id,
                    tag_type: "cr pair".to_string(),
                    tag: "-1-".to_string(),
                    indent_level: 2,
                    text: top_text,
                    indent_seq_num: 1,
                    sequence_string: format!("{}.01", tv.sequence_start),
                };

                let iec1 = IECLine {
                    seq_num: 2,
                    type_id: tv.type_id,
                    tag_type:"cr pair".to_string(),
                    tag: "-2-".to_string(),
                    indent_level: 2,
                    text: bottom_text,
                    indent_seq_num: 1,
                    sequence_string: format!("{}.02", tv.sequence_start),
                };

                sequenced_lines.push(iec0);
                sequenced_lines.push(iec1);
            
            }

            else if !(top_text.ends_with('.') || top_text.ends_with(';') || top_text.ends_with(':'))
                        && bottom_text.chars().next().unwrap().is_lowercase()
            {
                // More likely they are a single statement split for some reason

                let combined_text =  format!("{} {}", top_text, bottom_text)
                                            .replace("  ", " ");

                let single_iec = IECLine {
                    seq_num: 1,
                    type_id: tv.no_sep,
                    tag_type:"none".to_string(),
                    tag: "All".to_string(),
                    indent_level: 0,
                    text: combined_text,
                    indent_seq_num: 1,
                    sequence_string: format!("{}.A00", tv.sequence_start),
                };
                sequenced_lines.push(single_iec);
            }
            else
            {
                // leave as a hdr / spp pair...

                sequenced_lines.push(IECLine::from_cline(&joined_lines[0],
                                     tv.grp_hdr, 1, format!("{}.H01", tv.sequence_start)));
                sequenced_lines.push(IECLine::from_cline(&joined_lines[1], 
                                     tv.post_crit,2,format!("{}.S01", tv.sequence_start)));

            }
        }
    }

    if joined_lines.len() > 2
    {
        // Add in sequence numbers and strings to try to 
        // ensure numbering is continuous and reflects indent levels
        // Assumed lines are in the correct order. 
     
        let seq_start = format!("{}.", &tv.sequence_start);    //starts with e. or i. (or g.)
        let mut seq_base = seq_start.clone();    // to begin with

        let mut old_level = -1;
        let mut level_pos_store = vec![0; 8];  // up to 8 levels possible
        let mut current_level_pos = 0;
        let max_i = joined_lines.len() - 1;

        for i in 0..=max_i {    

            let seq_string: String;
            let level = joined_lines[i].indent_level;  //  assumed always non-null
            let mut type_id = tv.type_id;    // by default

            if level != old_level  
            {   
                // A change of level so reset the sequence string   

                let lu = level as usize;
                let olu = old_level as usize;

                if old_level != -1  // store the most recently used value
                {
                    level_pos_store[olu] = current_level_pos; 
                }

                if level == 0 {
                    seq_string = "A0.00".to_string();
                    type_id = tv.no_sep;
                }

                else if level == 1  {   // Hdr or Spp

                    level_pos_store[1] += 1;
                    current_level_pos = level_pos_store[1];

                    let tag2 = joined_lines[i].tag.clone();
                    let mut level1_prefix = tag2.chars().next().unwrap();

                    if level1_prefix == '@' {
                        level1_prefix = tag2.chars().nth(1).unwrap();   // H or S
                    }
                    seq_base = seq_start.clone();
                    seq_string = format!("{}{}{:02}", seq_base, level1_prefix, level_pos_store[1]);

                    type_id = if tag2 == "Spp".to_string() {tv.post_crit} else {tv.grp_hdr}
                }

                else if level == 2  {

                    level_pos_store[2] += 1;
                    current_level_pos = level_pos_store[2];

                    seq_base = seq_start.clone();
                    seq_string = format!("{}{:02}", seq_base, current_level_pos);
                }

                else {
                    if level > old_level  // level has increased
                    {
                        level_pos_store[lu] = 1;
                        current_level_pos = 1;

                        let mut seq_nums = "".to_string();
                        for lev in 2..level {
                            seq_nums = format!("{}{:02}.", seq_nums, level_pos_store[lev as usize]);
                        }
                        seq_base = format!("{}{}", seq_start.clone(), seq_nums);
                        seq_string = format!("{}01", seq_base);
                    }
                    else
                    {   
                        // level less than old level - use current set of values to construct the base
 
                        level_pos_store[lu] += 1;
                        current_level_pos = level_pos_store[lu];  // restore the previous value + 1

                        let mut seq_nums = "".to_string();
                        for lev in 2..(level-1) {
                            seq_nums = format!("{}{:02}.", seq_nums, level_pos_store[lev as usize]);
                        }
                        seq_base = format!("{}{}", seq_start.clone(), seq_nums);
                        seq_string = format!("{}{:02}", seq_base, current_level_pos);
                    }
                }

                old_level = level;

            }
            else  {

                if level == 1  {   // Repeated Hdr or Spp line
                   
                    level_pos_store[1] += 1;
                    current_level_pos = level_pos_store[1];

                    let tag2 = joined_lines[i].tag.clone();
                    let mut level1_prefix = tag2.chars().next().unwrap();

                    if level1_prefix == '@' {
                        level1_prefix = tag2.chars().nth(1).unwrap();
                    }
                    seq_base = seq_start.clone();
                    seq_string = format!("{}{}{:02}", seq_base, level1_prefix, level_pos_store[1]);

                }

                else {

                current_level_pos += 1;
                seq_string = format!("{}{:02}", seq_base, current_level_pos);

                }
            }

            let new_iecline = IECLine {
                seq_num: joined_lines[i].seq_num,
                type_id: type_id,
                tag_type: joined_lines[i].tag_type.clone(),
                tag: joined_lines[i].tag.clone(),
                indent_level: joined_lines[i].indent_level,
                text: joined_lines[i].text.clone(),
                indent_seq_num: current_level_pos,
                sequence_string: seq_string,
            };

            sequenced_lines.push(new_iecline);

        }
    }


    sequenced_lines


}