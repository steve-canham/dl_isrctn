
use super::iec_helper::*;


pub struct Criterion {
    pub crit_type_id: i32,
    pub seq_num: i32,
    pub text: String,
}

/* 
// temp stuff
pub fn process_iec(input: &Option<String>, input_type: &str) -> (i32, Vec<Criterion>) {
 
    let mut crits: Vec<Criterion> = Vec::new();

    let mut crit_base_type = 0;
    if input_type == "e" {
        crit_base_type += 10;
    }

    match input {
             
       Some(s) => {

        let s_low = s.to_lowercase();

        if !s_low.contains("not provided")
           && !(s_low.contains("inclusion") 
                && (s_low.contains("not meet")  || s_low.contains("not match")  || s_low.contains("not compl")
                    || s_low.contains("not met")  || s_low.contains("not fulfill")  || s_low.contains("failure")))
           {
                if !s.contains("\n") {
                   
                    // Single line - Most are single statements expressing a single criterion

                    if s.starts_with ("1.") {      // First of a list that was never extended
                        crits.push (Criterion {
                            crit_type_id: 1 + crit_base_type,
                            seq_num: 1,
                            text: s[2..].trim().to_string(),
                        });
                    }
                    else {
                        crits.push (Criterion {
                            crit_type_id: 1 + crit_base_type,
                            seq_num: 1,
                            text: s.to_string(),
                        });
                     }
                }
                else {
                    // multiple lines, likely to be numbered criteria - but probably not always!

                    if s.contains("1.") && s.contains("\n2.")      // by far the commonest pattern for ISRCTN
                    || s.contains("1.") && s.contains("\n3.")      // there are a few!
                    {
                        if let Some(cs) = cr_numbered_strings(s) {
                            let mut i = 0;
                            for c in cs {
                                i += 1;
                                crits.push (Criterion {
                                    crit_type_id: 3 + crit_base_type,
                                    seq_num: i,
                                    text: c.to_string(),
                                });
                            }
                        }
                    }
                    else if s.contains("1.") && s.contains("\n1.1")      // found in a small number
                    {
                        if let Some(cs) = cr_1_1_numbered_strings(s) {
                            let mut i = 0;
                            for c in cs {
                                i += 1;
                                crits.push (Criterion {
                                    crit_type_id: 3 + crit_base_type,
                                    seq_num: i,
                                    text: c.to_string(),
                                });
                            }
                        }
                    }
                    else {

                        // has CRs but not numbered.
                        // Occasionally are criteria separated by CRs but
                        // more often not structured 

                        crits.push (Criterion {    
                            crit_type_id: 101 + crit_base_type,
                            seq_num: -1,
                            text: s.to_string(),
                        });
                    }
                }
            }
                                   
            (5, crits)
       },

       None => (0, crits),
    }
}


pub fn  cr_numbered_strings(input: &String) -> Option<Vec<&str>> {

    static RE_CRNUM_SPLITTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n\d{1,2}\.").unwrap());
    let res: Vec<&str> = RE_CRNUM_SPLITTER.split(input).collect();

    let mut result: Vec<&str> = Vec::new();
    if res.len() > 0 {
        for mut r in res {
            if r.starts_with("1.") { r = &r[2..];}
            result.push(r.trim());
        }
    }

    match result.len() {
        0 => None,
        _ => Some(result)
    }
}


pub fn  cr_1_1_numbered_strings(input: &String) -> Option<Vec<&str>> {

    static RE_CRNUM1_SPLITTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n\d{1,2}\.\d{1,2}\.").unwrap());
    let res: Vec<&str> = RE_CRNUM1_SPLITTER.split(input).collect();

    let mut result: Vec<&str> = Vec::new();
    if res.len() > 0 {
        for mut r in res {
            if r.starts_with("1.") { r = &r[2..];}
            result.push(r.trim());
        }
    }

    match result.len() {
        0 => None,
        _ => Some(result)
    }
}
*/


pub fn original_process_iec(sd_sid: &String, in_string: &String, input_type: &str) -> (i32, Vec<Criterion>) {   // Should be a vector of IECLine

    // Oriinally in C# as GetNumberedCriteria
    // Parent process for the other functions

    let crits: Vec<Criterion> = Vec::new();
    let mut _final_cr_lines: Vec<IECLine> = Vec::new();
    let mut initial_cr_lines: Vec<IECLine> = Vec::new();
    let mut _processed_cr_lines: Vec<IECLine> = Vec::new();


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
                .filter(| t| (*t).contains("____") )
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
                _final_cr_lines.push(IECLine{
                seq_num: 1,
                type_id: tv.no_sep,
                split_type: "none".to_string(),
                leader: Some("All".to_string()),
                indent_level: Some(0),
                indent_seq_num: Some(1),
                sequence_string: Some(format!("{}0A", tv.sequence_start)),
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
                IECLine::new(i+1, tv.type_id, &"cr".to_string(), s));
        }

        // Initially try to find leader characters for each split line
        // then try to correct common errors in the list

        _processed_cr_lines = identify_line_leaders(&mut initial_cr_lines, &tv);
        _final_cr_lines = try_to_repair_split_lines(&_processed_cr_lines, &tv);
    }

    // We now have one or many 'final_cr_line's. Each now needs to be processed to see
    // it it corresponds to a single criterion or is itself a list of criteria.
    // If the former, the single line can be added to the 'expanded_lines', if the 
    // latter, the set of identified criuteria is added.

    

    if null_result {
        (0, crits)
    }
    else {
        (0, crits)
    }
 }



 fn identify_line_leaders(initial_lines:&mut Vec<IECLine>, tv: &TypePars) ->  Vec<IECLine>{

    let processed_lines: Vec<IECLine> = Vec::new();

    let mut i = 0;
    let max_i = initial_lines.len() - 1;  

    let mut level = 0;
    let mut _num_no_leader = 0;
    let mut old_ldr_name = "none";

    let mut levels: Vec<Level> = Vec::new();    // Initialise the level vector

    for ie_ln in initial_lines {

        let this_line = ie_ln.text.clone();
        i += 1;
        let mut ldr_name = "none"; // initial defaults - signify no leader found
        let mut leader = "";

        // What leader character(s), if any, are starting this line?

        for r in REGEX_MAP.iter() {

            let re = r.1;
            
            if let Some(c) = re.captures(&this_line) {
                leader = c.get_match().as_str();
                ldr_name = *r.0; 

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

        // If a leader has been found ...see if the leader has changed from the
        // most previously used - implying the indentation level has changed.
        // If it has changed 
        // else - store the line as a sub-header, unless it is the last line of the set
        // when it will be classified as a supplementary statement.
        
        let seq_num: i32;
        if ldr_name != "none"
        {

            if ldr_name != old_ldr_name
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

                if level == 1 && levels.len() != 1    // Remove all but the first (level 1) entry from the levels vector
                {
                    //levels = levels[0..1].to_owned();
                }
            }
            
            // Change the properties of the iec_line object

            if ie_ln.leader != Some("Spp".to_string())  // may have already been set above, e.g. with '*'
            {
                levels[level].current_seq_num += 1;
                seq_num = levels[level].current_seq_num;

                ie_ln.leader = Some(leader.to_string());
                ie_ln.indent_level = Some(level);
                ie_ln.indent_seq_num = Some(seq_num); // increment before applying
                ie_ln.text = this_line[leader.len()..].trim().to_string();
            }
        }

        else
        {
            _num_no_leader += 1; // keep a tally as ALL the lines may be without a leader

            if i == max_i
            {
                // initially at least, make this final line without any 'leader' character
                // a supplement (at the same indent level as the previous criteria).

                levels[level].current_seq_num += 1;
                seq_num = levels[level].current_seq_num;

                ie_ln.leader = Some("Spp".to_string());
                ie_ln.indent_level = Some(level);
                ie_ln.indent_seq_num = Some(seq_num); // increment before applying
                ie_ln.type_id = tv.post_crit;
            }
            else
            {
                // Otherwise, by default, add a line without any 'header' character as a sub-header
                // in the list (at the same indent level as the previous criteria) 

                levels[level].current_seq_num += 1;
                seq_num = levels[level].current_seq_num;

                ie_ln.leader = Some("Hdr".to_string());
                ie_ln.indent_level = Some(level);
                ie_ln.indent_seq_num = Some(seq_num); // increment before applying
                ie_ln.type_id = tv.grp_hdr;
            }
        }

        old_ldr_name = ldr_name;
    }

    processed_lines

}


    






 fn try_to_repair_split_lines(__processed_lines:&Vec<IECLine>, _tv: &TypePars) ->  Vec<IECLine>{

    let final_lines: Vec<IECLine> = Vec::new();




    final_lines


 }
    
