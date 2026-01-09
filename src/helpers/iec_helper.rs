use std::sync::LazyLock;
use regex::Regex;


pub struct Criterion {
    pub crit_type_id: i32,
    pub seq_num: i32,
    pub text: String,
}


pub fn process_iec(input: &Option<String>, input_type: &str) -> (i32, Vec<Criterion>) {
 
    let mut crits: Vec<Criterion> = Vec::new();

    let mut crit_base_type = 0;
    if input_type == "e" {
        crit_base_type += 10;
    }

    match input {
       
       Some(s) => {
            if !s.contains("\n") {

                // single line
                crits.push (Criterion {
                    crit_type_id: 2,
                    seq_num: 0,
                    text: s.to_string(),

                });
            }
            else {
                // multiple lines, likely to be numbered criteria - but probably not always!

                if s.contains("1.") && s.contains("\n2.")
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
                else {
                    crits.push (Criterion {    // has CRs but not numbered
                        crit_type_id: 99,
                        seq_num: -1,
                        text: s.to_string(),
                    });
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


