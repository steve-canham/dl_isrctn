
use super::iec_helper::*;
use super::iec_res::*;
use super::iec_structs::*;

//use log::info;

pub fn original_process_iec(sd_sid: &String, in_string: &String, input_type: &str) -> (i32, Vec<IECLine>) {   // Should be a vector of IECLine

    // Oriinally in C# as GetNumberedCriteria
    // Parent process for the other functions

    let crits: Vec<IECLine> = Vec::new();
    let mut final_cr_lines: Vec<IECLine> = Vec::new();
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

        if !is_redundant_header(&repaired_lines[0])
        {
            processed_cr_lines.push(IECLine {    // temp, should be final CR lines
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

        processed_cr_lines = process_initial_lines( &repaired_lines, &tv);

        final_cr_lines = repair_split_lines(&mut processed_cr_lines, &tv);
        
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
        (2, final_cr_lines)
    }

    // to be continued...

}


fn process_initial_lines(initial_lines:&Vec<String>, tv: &TypePars) ->  Vec<IECLine> {

    let mut tagged_lines: Vec<IECLine> = Vec::new();

    // Consumes the intiial lines, as split by CR, to create
    // an IECLine struct for each, returning a vector of those IECLines.

    let max_i = initial_lines.len() - 1;  
    let mut num_no_tag = 0;
    let mut previous_tag_type = "none".to_string();
    let mut previous_level = 0;
    let mut levels: Vec<String> = vec!["none".to_string(), "Hdr".to_string()];    // Initialise the level vector
    //let mut hdr_seq_num = 0;
    
    // Start of loop
    
    for (i, ln) in initial_lines.into_iter().enumerate() {

        let mut iec: Option<IECLine> = None;    // re-initialise as None
               
        // What tag character(s), if any, are starting this line?

        // First - can we match on the successful regex string (if any) 
        // from the previous line?
        // BUT - numdot REs can give false results, e.g. 'n.m.' is still 
        // picked up as 'n.' if it directly follows 'n.' Therefore do 
        // not use this shortcut if the last tag was a numdot.
        
        if previous_tag_type != "none".to_string() 
                && !previous_tag_type.starts_with("numdot"){

            if let Some(rr) = test_re(&previous_tag_type,&ln) {
                iec = Some(IECLine {
                    seq_num: (i + 1) as i32,
                    tag: rr.tag,
                    type_id: tv.type_id,
                    tag_type: rr.tag_type,
                    indent_level: previous_level,
                    text: rr.text,
                    indent_seq_num: 0,
                    sequence_string:"".to_string(),
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
                    iec = test_against_numdot_res(tv.type_id, i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
                }
                else {
                    iec = test_against_numeric_res(tv.type_id, i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
                }
            }
            else if first_char.is_alphabetic() {
                iec = test_against_alpha_res(tv.type_id, i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
            }
            else {
                iec = test_against_other_res(tv.type_id, i, ln, &previous_tag_type, previous_level, &tagged_lines, &mut levels);
            }

        }


        let iec_line= match iec {

            Some(l) => l,   // found a tag
            None => {                // no tag - construct a 'header' IECLine
                    num_no_tag += 1;
                    let mut tag = "Hdr".to_string();
                    let mut type_id = tv.grp_hdr;

                    if i == max_i       // initially, make this final line without a tag a 'supplement'
                    {
                        tag = "Spp".to_string();
                        type_id = tv.post_crit;
                    }

                    IECLine {
                        seq_num: (i + 1) as i32,
                        type_id: type_id,
                        tag_type: "none".to_string(),
                        tag: tag,
                        indent_level: 1,
                        indent_seq_num: 0,
                        sequence_string: "".to_string(),
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
    
    /*  

        // Construct the IECLine fields from the tag (if any)
        // found, the derived indent and sequence numbers, and the line text. 

        //  let in_hdr_seq: bool;
        //  let num_in_seq: i32;
       
        // if a tag
        //    in_hdr_seq = false;

        }
      else
        {
            // If no tag, store the line as a sub-header, unless it is the last 
            // line of the set when it will be classified as a supplementary statement.

            //     in_hdr_seq = true;
          

        // Sequence numbers and strings constructed differently
        // for header lines (though these will be over written later).
        // This code may therefore probably go.

        if in_hdr_seq {
            hdr_seq_num += 1;
            num_in_seq = hdr_seq_num;
        }
        else {
            levels[level].current_seq_num += 1;
            num_in_seq = levels[level].current_seq_num;
        }

        let seq_string = match tag.as_str() {
            "Hdr" => format!("{}.H{:0>2}", tv.sequence_start, num_in_seq),
            "Spp" => format!("{}.S{:0>2}", tv.sequence_start, num_in_seq),
            _ => format!("{}.{:0>2}", tv.sequence_start, num_in_seq),
        };
*/

    // At moment possibility of a preformed IEC for some tags, e.g. *
    // Need to investigate new best way of handling this
    
    // Finally, again check all lines have a length of at least 1, i.e. are not empty, 
    // before proceeding further. Empty lines may occur - very rarely - if a line is 'just tag'
    // (though most of these should have been eliminated at the beginning)
    // or if, for example, the original split in CTG left a tag before the 'Exclusion Criteria' statement
    
    let processed_lines: Vec<IECLine> = tagged_lines
                                       .into_iter()
                                       .filter(|t| t.text != "")
                                       .collect();

    // Check the 'all without a tag' possibility - allowing a single exception

    if  (processed_lines.len() > 4 && num_no_tag >= processed_lines.len() - 1) ||
        (processed_lines.len() > 2 && num_no_tag == processed_lines.len())
        {
            // None (or very few) of the lines have a tag character. If they (or most of them) had proper 
            // termination, or consistent line starting, then it is possible that they are
            // simply differentiated by the CRs alone...

            let mut assume_crs_only = check_if_all_lines_end_consistently(&processed_lines, 1)
                                   || check_if_all_lines_start_with_caps(&processed_lines, 1)
                                   || check_if_all_lines_start_with_lower_case(&processed_lines, 0);

            // otherwise check for a consistent bullet type character

            let mut possible_tag = "".to_string();

            if !assume_crs_only
            {
                // a chance that an unknown bullet character has been used to start each line
                // start with the second line (as the first may be different) and see if they are all the same
                // Don't test letters as some people use formulaic criteria all starting with the same word

                let test_char = &processed_lines[1].text.first_char();  // should always be at least one character in each line
                if !test_char.is_alphabetic()
                {
                    let mut valid_start_chars = 0;
                    
                    for k in 1..processed_lines.len()
                    {
                        // May be no termination applied but each line starts with a capital letter

                        let start_char = &processed_lines[k].text.chars().next().unwrap();
                        if start_char == test_char
                        {
                            valid_start_chars += 1;
                        }
                    }

                    if valid_start_chars == processed_lines.len() - 1
                    {
                        assume_crs_only = true;
                        possible_tag = test_char.to_string();
                    }
                }
            }

            if assume_crs_only
            {
                // for these records (assumed split on crs only) the fields in the IEC objects need to be changed
                // which means constructing yet another vector over these criteria

                let mut revised_lines: Vec<IECLine> = Vec::new();

                let mut line_num = 0;
                let tag_string = if possible_tag == "" { "@".to_string() } else { possible_tag.clone() };

                let mut crit_seq_num = 0;
                //hdr_seq_num = 0;
                let mut hdr_seq_num = 0;
                for ln in processed_lines {
                    
                    let mut rev_text  = ln.text.clone();  // as a default
                    if possible_tag != "".to_string() // single character only
                    {
                        if line_num == 0
                        {
                            if ln.text.chars().next().unwrap().to_string() == possible_tag
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

                    // Identify what appear to be headers 
                    // Otherwise treat all lines as being 'normal' criteria
                    
                    let type_id: i32;
                    let indent_level: i32;
                    let indent_seq_num: i32;
                    let tag:String;


                    if ln.text.ends_with(':') || ln.text == ln.text.to_uppercase()
                    {
                        tag = tag_string.clone() + "Hdr";
                        type_id = tv.grp_hdr;
                        indent_level = 1;
                        hdr_seq_num += 1;
                        indent_seq_num = hdr_seq_num
                    }
                    else
                    {
                        tag = tag_string.clone();
                        indent_level = 2;
                        type_id = tv.type_id;
                        crit_seq_num += 1;
                        indent_seq_num = crit_seq_num
                    }

                    let seq_string = if tag.contains("Hdr") {
                       format!("{}.H{:0>2}", tv.sequence_start, indent_seq_num)
                    }
                    else {
                        format!("{}.{:0>2}", tv.sequence_start, indent_seq_num)
                    };
                    
                    revised_lines.push(IECLine {
                        seq_num: ln.seq_num,
                        type_id: type_id,
                        tag_type: "cr assumed".to_string(),
                        tag: tag,
                        indent_level: indent_level,
                        indent_seq_num: indent_seq_num,
                        sequence_string: seq_string,
                        text: rev_text,
                    });

                    line_num +=1;

                }

                revised_lines
            }

            else {
                processed_lines   // return the lines as modified before the 'assume cr' processing above
            }

        }
        else {
            processed_lines    // return the originally derived IEC lines
        }

}




fn repair_split_lines(plines:&mut Vec<IECLine>, tv: &TypePars) ->  Vec<IECLine>{

    // Repair some of the more obvious mis-interpretations
    
    // First, working backwards, re-aggregate lines split with spurious \n.
    // Add the first line (no previous line for it to merge into).
    // Then loop through the rest.

    let mut reversed_lines: Vec<IECLine> = Vec::new();
    let max_i = plines.len();

    //plines[0].clone()

    for i in (0..max_i).rev() {
        
        let mut transfer_line = true;  
        if i > 0 {

            // by default

            let this_text = plines[i].text.clone();
            let prev_text = plines[i - 1].text.clone();
            let init_char = &this_text.chars().next().unwrap_or('?'); 

            // Remove (i.e. don't transfer) simple header lines or headings with no information

            if is_redundant_header(&this_text) {
                transfer_line = false;
            }
            else {

                // Try and identify spurious 'headers', i.e. lines without leaders, caused by spurious CRs.
                // Following recent revisions spurious CRs no longer seem to exist within CGT IEC data. 
                // Lines without headers (usually 1., 2., *, *) are therefore normally genuine header
                // statements for this source. The next two routines therefore do not apply for CTG data.

                if !tv.sd_sid.starts_with("NCT")
                {
                    // Consider lines originally specified as headers (leave out last line)  

                    if plines[i].type_id == tv.grp_hdr && i < plines.len() - 1
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
                                        plines[i - 1].type_id = tv.grp_hdr;
                                    }
                                    transfer_line = false;
                                }

                            }
                        }
                    }

                    // Consider lines originally specified as 'supplementary' final lines
                    
                    if plines[i].type_id == tv.post_crit {

                        let low_line = this_text.to_lowercase();
                        if !this_text.ends_with(':') && !this_text.starts_with('*')
                            && !low_line.starts_with("note") && !low_line.starts_with("other ")
                            && !low_line.starts_with("for further ") && !low_line.starts_with("further") 
                            && !low_line.starts_with("for more ") && !low_line.starts_with("more ") {

                            // Almost always is a spurious supplement, better considerd as a normal criterion.
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
                                // Reset the indent level and seq number to be follow those 
                                // of the preceding criterion line.

                                plines[i].indent_level = plines[i - 1].indent_level;
                                plines[i].indent_seq_num = plines[i - 1].indent_seq_num + 1;
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

    // Put things back in correct order    

    // Add in sequence numbers and strings to try to 
    // ensure numbering is continuous and reflects indent levels

    // Need to establish yet another collection as the 
    // iteration process consumes the original items.
    // There does not seem to be any easy way around this!

    // First do a reversal in place of the existing reversed records
    // so that they are in the correct order.
    // Then create yet another blank vector.

    reversed_lines.reverse();

    let mut sequenced_lines: Vec<IECLine> = Vec::new();

    // Clarify situation with one or two criteria only.

    if reversed_lines.len() == 1
    { 
        let single_iec = IECLine {
            seq_num: 1,
            tag_type:"none".to_string(),
            type_id: tv.no_sep,
            tag: "All".to_string(),
            indent_level: 0,
            indent_seq_num:1,
            sequence_string:tv.sequence_start.clone() + ".A00",
            text: reversed_lines[0].text.clone()
        };
        sequenced_lines.push(single_iec);
    }

    if reversed_lines.len() == 2 && reversed_lines[0].type_id == tv.grp_hdr {

        let top_text = reversed_lines[0].text.clone();
        let bottom_text = reversed_lines[1].text.clone();
        if top_text.ends_with(":") && top_text.to_lowercase().contains("criteria")
        {
            // Probably a genuine header (unusual). Make the second line a criterion

            reversed_lines[1].type_id = tv.type_id;
            reversed_lines[1].tag = "-1-".to_string();
            reversed_lines[1].indent_level = 2;
            reversed_lines[1].indent_seq_num = 1;

            reversed_lines[0].sequence_string = format!("{}.H01", tv.sequence_start);
            reversed_lines[1].sequence_string = format!("{}.01", tv.sequence_start);

            sequenced_lines.push(reversed_lines[0].clone());
            sequenced_lines.push(reversed_lines[1].clone());

          //  info!("Pushing pair0 {} ::: {}", reversed_lines[0].text, reversed_lines[1].text);
        }
        else
        {
            if check_if_all_lines_end_consistently(&reversed_lines, 0)
                || check_if_all_lines_start_with_caps(&reversed_lines, 0)
            {
                // More likely that these are a pair of criteria statements 
                // (or multiple criteria statements)

                reversed_lines[0].seq_num = 1;
                reversed_lines[1].seq_num = 2;
                reversed_lines[0].tag_type = "cr pair".to_string();
                reversed_lines[1].tag_type = "cr pair".to_string();
                
                reversed_lines[0].type_id = tv.type_id;
                reversed_lines[0].tag = "-1-".to_string();
                reversed_lines[0].indent_level = 2;
                reversed_lines[0].indent_seq_num = 1;
                
                reversed_lines[1].type_id = tv.type_id;
                reversed_lines[1].tag = "-2-".to_string();
                reversed_lines[1].indent_level = 2;
                reversed_lines[1].indent_seq_num = 2;

                reversed_lines[0].sequence_string = format!("{}.01", tv.sequence_start);
                reversed_lines[1].sequence_string = format!("{}.02", tv.sequence_start);
                
                // In case they include them strip lines of headers.
                // Are not removed beforehand as first and last lines are not processed
                
                reversed_lines[0].text = top_text;  
                reversed_lines[1].text = bottom_text; 

                sequenced_lines.push(reversed_lines[0].clone());
                sequenced_lines.push(reversed_lines[1].clone());
                
                //info!("Pushing pair1 {} ::: {}", reversed_lines[0].text, reversed_lines[1].text);
            }

            else if !(top_text.ends_with('.') || top_text.ends_with(';') || top_text.ends_with(':'))
                        && bottom_text.chars().next().unwrap().is_lowercase()
            {
                // More likely they are a single statement split for some reason

                reversed_lines[0].text = format!("{} {}", reversed_lines[0].text, reversed_lines[1].text)
                                            .replace("  ", " ");
                reversed_lines[0].seq_num = 1;
                reversed_lines[0].tag_type = "none".to_string();
                reversed_lines[0].type_id = tv.no_sep;
                reversed_lines[0].tag = "All".to_string();
                reversed_lines[0].indent_level = 0;
                reversed_lines[0].indent_seq_num = 1;
                reversed_lines[0].sequence_string = tv.sequence_start.clone() + ".A00";

                sequenced_lines.push(reversed_lines[0].clone());
               
            }
            else
            {
                // leave as a hdr / spp pair...

                reversed_lines[0].sequence_string = format!("{}.{}01", tv.sequence_start, &reversed_lines[0].tag.chars().next().unwrap());
                reversed_lines[1].sequence_string = format!("{}.{}02", tv.sequence_start, &reversed_lines[1].tag.chars().next().unwrap() );

                sequenced_lines.push(reversed_lines[0].clone());
                sequenced_lines.push(reversed_lines[1].clone());

                //info!("Pushing pair2 {} ::: {}", reversed_lines[0].text, reversed_lines[1].text);
            }

        }
    }


    if reversed_lines.len() > 2
    {
        // Add in sequence numbers and strings to try to 
        // ensure numbering is continuous and reflects indent levels

        // Need to establish yet another collection as the 
        // iteration process consumes the original items.
        // There does not seem to be any easy way around this!

        // Assumed (for now) lines are in the correct order.
        // Otherwise would need Ord, Cmp traits to be defined
        // Try to get away without this sort of thing:
        // reversed_lines = reversed_lines.OrderBy(c => c.seq_num).ThenBy(c => c.indent_seq_num).ToList();

        let seq_start = format!("{}.", &tv.sequence_start);    //starts with e. or i. (or g.)
        let mut seq_base = seq_start.clone();  

        let mut old_level = -1;
        let mut level_pos_store = vec![0; 8];  // up to 8 levels possible
        let mut current_level_pos = 0;

        for i in 0..reversed_lines.len(){    // reversed lines now in normal order

            let seq_string: String;
            let mut new_iecline = reversed_lines[i].clone();
            let level = reversed_lines[i].indent_level; //  assumed always non-null

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
                    seq_string = reversed_lines[i].sequence_string.clone();
                }

                else if level == 1  {   // Hdr or Spp

                    level_pos_store[1] += 1;
                    current_level_pos = level_pos_store[1];

                    let tag2 = reversed_lines[i].tag.clone();
                    let mut level1_prefix = tag2.chars().next().unwrap();

                    if level1_prefix == '@' {
                        level1_prefix = tag2.chars().nth(1).unwrap();
                    }
                    seq_base = seq_start.clone();
                    seq_string = format!("{}{}{:02}", seq_base, level1_prefix, level_pos_store[1]);
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

                    let tag2 = new_iecline.tag.clone();
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

            new_iecline.indent_seq_num = current_level_pos; //seq_string.to_string();
            new_iecline.sequence_string = seq_string; //seq_string.to_string();
            sequenced_lines.push(new_iecline);

        }

        

    }


    sequenced_lines


}

    /*
           
                    if (level != old_level)
                    {
                        // a change of level so reset parameters to construct the sequence string

                        if (old_level != -1)
                        {
                            level_pos[old_level] = current_level_pos; // store the most recently used value
                        }

                        if (level == 1)
                        {
                            sequence_base = sequence_start;
                            current_level_pos = level_pos[1];
                        }
                        else
                        {
                            if (level > old_level)
                            {
                                sequence_base = seq_string + "."; // current string plus dot separator
                                current_level_pos = 0;
                            }
                            else
                            {
                                // level less than old level
                                // use current set of values to construct the base
                                sequence_base = sequence_start;
                                for (int b = 1; b < level; b++)
                                {
                                    sequence_base += level_pos[b].ToString("0#") + ".";
                                }

                                current_level_pos = level_pos[level]; // restore the previous value
                            }
                        }

                        old_level = level;
                    }

                    seq_string = sequence_base + (++current_level_pos).ToString("0#");
                }

                t.sequence_string = seq_string;
            }
        }

        return revised_lines;
    }
     */




    
