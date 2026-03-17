

#[derive(Debug, Clone)]
pub struct CLine
{
    pub seq_num: i32,
    pub tag_type: String,
    pub tag: String,
    pub indent_level: i32,
    pub text: String,
}


#[derive(Debug, Clone)]
pub struct IECLine
{
    pub seq_num: i32,
    pub type_id: i32,
    pub tag_type: String,
    pub tag: String,
    pub indent_level: i32,
    pub indent_seq_num: i32,
    pub sequence_string: String,
    pub text: String,
}


impl IECLine {

    pub fn from_cline(cline: &CLine, type_id: i32, indent_seq_num: i32, sequence_string: String) -> Self {

        IECLine { 
            seq_num: cline.seq_num,
            type_id: type_id,
            tag_type: cline.tag_type.clone(),
            tag: cline.tag.clone(),
            indent_level: cline.indent_level,
            indent_seq_num: indent_seq_num,
            sequence_string: sequence_string,
            text: cline.text.clone(),
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
            1 => "n".to_string(),
            2 => "e".to_string(),
            3 => "g".to_string(),
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
// a set of criteria. The level of the line is given by the position of 
// the name in the vector. 
// The L0 tag ("All") is at pos(0), the L1 tag ("Hdr") is always at posotion 1. 
// The L2 level holds the tag of the current top level criteria, further
// entries in the vector hold the sub-criteria tags, if any.
// The current_seq_num field gives the current sequence number within the level of
// the line.

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

/*
pub fn get_level(tag_style: &String, levels: &mut Vec<Level>) -> i32 {

   // if levels.len() == 2  // as on initial use - there are initial entries for levels 0 and 1
   // {
   //     levels.push(Level::new(tag_style, 0));
   //     return 1;      // differentiates the very first item in any list
   // }

    // See if the level header has been used - if so
    // return level, if not add and return new level
    
    let mut found_level = 0;
    for i in 2..levels.len() {
        
        if tag_style == &levels[i].level_name
        {
            found_level = i;
            break;
        }
    }

    if found_level == 0 {
        levels.push(Level::new(tag_style, 0));
        found_level = levels.len() - 1;
    }

    found_level as i32
}
     */