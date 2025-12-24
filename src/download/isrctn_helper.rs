/* 
use super::file_models::{MeddraCondition, SecIdBase, 
    SecondaryId, WhoStudyFeature};
use std::sync::LazyLock;
use regex::Regex;
use std::collections::HashSet;


pub fn get_type(study_type: &Option<String>) -> i32 {
    
    if study_type.is_some() 
    {
        let ts: &str;
        let t = study_type.clone().unwrap().to_lowercase();
        if t.starts_with("intervention")
            || t == "BA/BE"
        {
            ts = "Interventional";
        }
        else if t.starts_with("observation")
              || t.starts_with("epidem")
              || t == "PMS"
              || t == "Relative factors research"
              || t == "Cause"
              || t == "Cause/Relative factors study"
              || t == "Health Services Research"
              || t == "Health services reaserch"
        {
            ts = "Observational";
        }
        else if t == "Expanded Access"
        {
            ts = "Expanded Access";
        }
        else if t == "Diagnostic test"
        {
            ts = "Diagnostic test";
        }
        else if t == "Not Specified" || t == "N/A" {
            ts = "Not provided";
        }
        else if t  ==  "Other" 
             || t  == "Others,meta-analysis etc" 
             || t.to_lowercase()  == "basic science"
             || t  == "Prevention"
             || t  == "Screening"
             || t == "Treatment study"
        {
            ts = "Other";
        }
        else  
        {
            //stype == "Not Specified" || stype == "N/A" 
            ts = "Not provided";
        }
        
        // Other changed from 16 to 99
        // Diagnostic test added

        match ts {
            "Interventional" => 11, 
            "Observational" => 12,
            "Observational patient registry" => 13,
            "Expanded Access" => 14,
            "Funded programme" => 15,
            "Diagnostic test" => 16,
            "Other" => 99,
            "Not provided" => 0,
            _ => 99
        }
    }
    else {
        0
    }
}

pub fn get_status(status: &Option<String>) -> i32 {

    if status.is_some() {
        let ss: &str;
        let s = status.clone().unwrap().to_lowercase();
        if s.len() > 5  {
            if s == "complete" || s == "completed" 
                || s == "complete: follow-up complete" || s == "complete: follow up complete" 
                || s == "data analysis completed" || s == "main results already published"
                || s == "approved for marketing"
            {
                ss = "Completed";
            }
            else if s == "complete: follow-up continuing" 
                || s == "complete: follow up continuing" || s == "active, not recruiting" 
                || s == "closed to recruitment of participants" || s == "no longer recruiting" 
                || s == "not recruiting" || s == "recruitment completed"
                || s == "enrollment closed"
                || s == "recruiting stopped after recruiting started"
            {
                ss = "No longer recruiting";
            }
            else if s == "recruiting" || s =="open public recruiting" 
            || s == "open to recruitment" || s =="in enrollment"
            {
                ss = "Recruiting";
            }
            else if s.contains("pending")
                || s == "not yet recruiting"
                || s == "without startig enrollment"
                || s == "preinitiation"
            {
                ss = "Not yet recruiting";
            }
            else if s.contains("suspended")
                || s.contains("temporarily closed")
                || s == "temporary halt"
                || s == "temporarily not available"
            {
                ss = "Suspended";
            }
            else if s.contains("terminated")
                || s.contains("stopped early")
                || s == "stopped"
            {
                ss = "Terminated";
            }
            else if s.contains("withdrawn")
            {
                ss = "Withdrawn";
            }
            else if s.contains("enrolling by invitation")
            {
                ss = "Enrolling by invitation";
            }
            else if s == "ongoing" 
                    || s == "authorised-recruitment may be ongoing or finished" 
                    || s == "available"
            {
                ss = "Ongoing, recruitment status unclear";
            }
            else if s == "not applicable" {
                ss = "Recorded as not applicable";
            }
            else
            {
                // = 'withheld', or = 'unknown', or = 'no longer available'
                // or = 'deleted from source registry', or = 'unknown status'
                // or 'temporarily not available'
                ss = "Not provided";
            }
        }
        else {
            ss = "Not provided";
        }

        match ss {
            "Not yet recruiting"=> 10,
            "Withdrawn"=> 12,
            "Recruiting"=> 14,
            "Enrolling by invitation"=> 16,
            "No longer recruiting" => 18,
            "Ongoing, recruitment status unclear"=> 20,            
            "Suspended"=> 25,
            "Completed" => 30,
            "Terminated"=> 32,
            "Recorded as not applicable"=> 99,
            "Not provided"=> 0,
            _ => 0
           }
        }
    else {
        0
    }
}


pub fn get_conditions(condition_list: &String, source_id: i32) -> (Vec<String>, Vec<MeddraCondition>) {

    // replace line breaks and hashes with semi-colons, then split

    let mut clist = condition_list.replace("<br>", ";").replace("<br/>", ";");
    clist = clist.replace("#", ";");
    let sep_conds: Vec<&str> = clist.split(";").collect();
    let mut conds = Vec::<String>::new();
    let mut medra_conds = Vec::<MeddraCondition>::new();

    for s in sep_conds
    {
        let complex_trim = |c| c == ' ' || c == '('|| c == '.' || c == ';' || c == '-';
        let s1 = s.trim_matches(complex_trim);
        if s1 != "" && s1.len() >= 3
        {
            if source_id == 100123  && s1.to_lowercase().starts_with("meddra") {
                
                // Of type (but without line breaks): 
                // MedDRA version: 20.0  // Level: PT  // Classification code 10005003  // Term: Bladder cancer
                // System Organ Class: 10029104 - Neoplasms benign, malignant and unspecified (incl cysts and polyps)",
                // MedDRA version: 21.1  //Level: LLT  //Classification code 10022877  //Term: Invasive bladder cancer
                // System Organ Class: 10029104 - Neoplasms benign, malignant and unspecified (incl cysts and polyps)",

                let re = Regex::new(r"MedDRA version: (?<v>.+)Level").unwrap();
                let version = match re.captures(&s1)
                {
                    Some(c) => c["v"].to_string(),
                    None => "".to_string(),
                };
 
                let re = Regex::new(r"Level: (?<level>.+)Classific").unwrap();
                let level = match re.captures(&s1)
                {
                    Some(c) => c["level"].to_string(),
                    None => "".to_string(),
                };

                let re = Regex::new(r"Classification code (?<code>[0-9]+)").unwrap();
                let code = match re.captures(&s1)
                {
                    Some(c) => c["code"].to_string(),
                    None => "".to_string(),
                };

                let re = Regex::new(r"Term: (?<term>.+)System").unwrap();
                let term = match re.captures(&s1)
                {
                    Some(c) => c["term"].to_string(),
                    None => "".to_string(),
                };

                let re = Regex::new(r"System Organ Class: (?<soccode>[0-9]+)").unwrap();
                let soc_code = match re.captures(&s1)
                {
                    Some(c) => c["soccode"].to_string(),
                    None => "".to_string(),
                };

                let re = Regex::new(r"System Organ Class: (.+) - (?<socterm>.+)$").unwrap();
                let soc_term = match re.captures(&s1)
                {
                    Some(c) => c["socterm"].to_string(),
                    None => "".to_string(),
                };
                
                let mc = MeddraCondition::new(version, level, code, term, soc_code, soc_term);
                medra_conds.push(mc);
            }
            else {
                conds.push(s1.to_string());
            }

            // Most processing code for condition data now all moved to Harvester
            // module, as it is easier to correct and extend there (changes
            // do not require global WHO re-download!).
            // Conditions exported from here a a simple string array.
        }
    }

    // In some cases conditions are duplicated in the WHO list
    // Duplication canm also occur of SOCs if mulitple MedDRA entries provided
    // Therefore need to de-duplicate

    let mut uniques = HashSet::new();
    conds.retain(|e| uniques.insert(e.clone()));

    (conds, medra_conds)    

}


pub fn split_and_dedup_countries(source_id: i32, country_list: &String) -> Option<Vec<String>> {

    // country list known to be non-null and already 'tidied'.

    let in_strings: Vec<&str> = country_list.split(';').collect();
    let mut out_strings = Vec::<String>::new();
   
    for c in in_strings
    {
        // Sri Lankan registry (in particular) uses commas to list countries
        // but commas appear legitimately in many versions of country names
        
        let mut this_c = c.trim().to_lowercase().replace(".", "");
        let mut this_c_consumed = false;

        if source_id == 100127 {// Some odd 'regional' countries used by the Japanese registries
            this_c = this_c.replace("asia except japan", "asia");
            this_c = this_c.replace("asia exept japan", "asia");
            this_c = this_c.replace("japan,asia(except japan)", "asia");
            this_c = this_c.replace("asia(except japan)", "asia");
            this_c = this_c.replace("none (japan only)", "japan");
            this_c = this_c.replace("none other than japan", "japan");
        }

        if this_c.contains(',') {

            // Sri Lankan registry (in particular) uses commas to list countries
            // but commas appear legitimately in many versions of country names

            let complex_trim = |c| c == ',';    // remove ending commas and double spaces
            this_c = this_c.trim_matches(complex_trim).replace("  ", " ");

            this_c = this_c.replace("aiwan, prov", "aiwan - prov");
            this_c = this_c.replace("aiwan, tai", "aiwan - tai");
            this_c = this_c.replace("congo,", "congo - ");
            this_c = this_c.replace("iran,", "iran - ");
            this_c = this_c.replace("sar,", "sar - ");
            this_c = this_c.replace("kong, chi", "kong - chi");
            this_c = this_c.replace("korea, dem", "korea - dem");
            this_c = this_c.replace("korea, nor", "korea - nor");
            this_c = this_c.replace("korea, sou", "korea - sou");
            this_c = this_c.replace("korea, rep", "korea - rep");
            this_c = this_c.replace("macedonia,", "macedonia - ");
            this_c = this_c.replace("moldova,", "moldova - ");
            this_c = this_c.replace("palastine,", "palastine - ");
            this_c = this_c.replace("palastinian,", "palastinian - ");
            this_c = this_c.replace("tanzania,", "tanzania - ");
            this_c = this_c.replace("islands,", "islands - ");

            if this_c.contains(',') {
                let added_strings: Vec<&str> = this_c.split(',').collect();
                for ac in added_strings {
                    let act = ac.trim();
                    if add_country_name(act, &out_strings) {
                        out_strings.push(act.to_string());
                    }
                }
                this_c_consumed = true;   // countries have been added from this string
            }
        }

        if !this_c_consumed && add_country_name(&this_c, &out_strings) {
            out_strings.push(this_c.to_string());
        }
    }

    return Some(out_strings);
}


fn add_country_name(new_name:&str, out_strings: &Vec::<String>) -> bool {

    if out_strings.len() == 0
    {
        true
    }
    else {
        let mut add_string = true;
        for s in out_strings
        {
            if s == new_name
            {
                add_string = false;
                break;
            }
        }
        add_string 
    }
}



pub fn add_int_study_features(design_list: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    let design = design_list.replace(" :", ":"); // to make comparisons easier

    if design.contains("purpose: treatment")
    {
        fs.push(WhoStudyFeature::new(21, "Primary purpose", 400, "Treatment"));
    }
    if design.contains("purpose: diagnosis") || design.contains("diagnostic")
    {
        fs.push(WhoStudyFeature::new(21, "Primary purpose", 410, "Diagnostic"));
    }    
    if design.contains("supportive care") || design.contains("purpose: supportive")
    {
        fs.push(WhoStudyFeature::new(21, "Primary purpose", 415, "Supportive care"));
    }

    if design.contains("non-randomized")   
     || design.contains("nonrandomized")
     || design.contains("non-randomised")
     || design.contains("nonrandomised")
     || design.contains("non-rct")
    {
        fs.push(WhoStudyFeature::new(22, "Allocation type", 210, "Nonrandomised"));
    }
    else if design.contains("randomized")
         || design.contains("randomised")
         || design.contains(" rct")
    {
        fs.push(WhoStudyFeature::new(22, "Allocation type", 205, "Randomised"));
    }

    if design.contains("parallel")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 305, "Parallel assignment"));
    }

    if design.contains("crossover")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 310, "Crossover assignment"));
    }

    if design.contains("factorial")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 315, "Factorial assignment"));
    }

    fs
}


pub fn add_obs_study_features(design: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    
    if design.contains("observational study model")
    {
        if design.contains("cohort")
        {
            fs.push(WhoStudyFeature::new(30, "Observational model", 600, "Cohort"));
        }
        if design.contains("case-control") || design.contains("case control")
        {
            fs.push(WhoStudyFeature::new(30, "Observational model", 605, "Case-control"));
        }
        if design.contains("case-crossover") || design.contains("case crossover")
        {
            fs.push(WhoStudyFeature::new(30, "Observational model", 615, "Case-crossover"));
        }

    }
    if design.contains("time perspective")
    {
        if design.contains("retrospective")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 700, "Retrospective"));
        }
        if design.contains("prospective")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 705, "Prospective"));
        }
        if design.contains("cross-sectional") || design.contains("crosssectional")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 710, "Cross-sectional"));
        }
        if design.contains("longitudinal")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 730, "longitudinal"));
        }
    }


    if design.contains("biospecimen retention")
    {
        if design.contains("not collect nor archive")
        {
            fs.push(WhoStudyFeature::new(32, "Biospecimens retained", 800, "None retained"));
        }
        if design.contains("collect & archive- sample with dns")
        {
            fs.push(WhoStudyFeature::new(32, "Biospecimens retained", 805, "Samples with DNA"));
        }
    }

    fs
}


pub fn add_masking_features(design_list: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    let design = design_list.replace(" :", ":"); // to make comparisons easier

    if design.contains("open label")
       || design.contains("open-label")
       || design.contains("no mask")
       || design.contains("masking not used")
       || design.contains("not blinded")
       || design.contains("non-blinded")
       || design.contains("no blinding")
       || design.contains("no masking")
       || design.contains("masking: none")
       || design.contains("masking: open")
       || design.contains("blinding: open")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 500, "None (Open Label)"));
    }
    else if design.contains("single blind")
     || design.contains("single-blind")
     || design.contains("single - blind")
     || design.contains("masking: single")
     || design.contains("outcome assessor blinded")
     || design.contains("participant blinded")
     || design.contains("investigator blinded")
     || design.contains("blinded (patient/subject)")
     || design.contains("blinded (investigator/therapist)")
     || design.contains("blinded (assessor)")
     || design.contains("blinded (data analyst)")
     || design.contains("uni-blind")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 505, "Single"));
    }
    else if design.contains("double blind")
     || design.contains("double-blind")
     || design.contains("doble-blind")
     || design.contains("double - blind")
     || design.contains("double-masked")
     || design.contains("masking: double")
     || design.contains("blinded (assessor, data analyst)")
     || design.contains("blinded (patient/subject, investigator/therapist")
     || design.contains("masking:participant, investigator, outcome assessor")
     || design.contains("participant and investigator blinded")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 510, "Double"));
    }
    else if design.contains("triple blind")
     || design.contains("triple-blind")
     || design.contains("blinded (patient/subject, caregiver, investigator/therapist, assessor")
     || design.contains("masking:participant, investigator, outcome assessor")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 515, "Triple"));
    }
    else if design.contains("quadruple blind") || design.contains("quadruple-blind")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 520, "Quadruple"));
    }
    else if design.contains("masking used") || design.contains("blinding used")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 502, "Blinded (no details)"));
    }
    else if design.contains("masking:not applicable")
     || design.contains("blinding:not applicable")
     || design.contains("masking not applicable")
     || design.contains("blinding not applicable")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 599, "Not applicable"));
    }
    else if design.contains("masking: unknown")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 525, "Not provided"));
    }

    fs
}


pub fn add_phase_features(phase: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    
    if phase != "not selected" && phase != "not applicable"
        && phase != "na" && phase != "n/a"
    {
        if phase == "phase 0" || phase == "phase-0" || phase == "phase0" 
        || phase ==  "0" || phase ==  "0 (exploratory trials)" 
        || phase == "phase 0 (exploratory trials)" || phase ==  "0 (exploratory trials)"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 105, "Early phase 1"));
        }
        else if phase == "1" || phase ==  "i" || phase ==  "i (phase i study)" 
                 || phase == "phase-1" || phase ==  "phase 1" || phase ==  "phase i" || phase ==  "phase1"
        {
            fs.push(WhoStudyFeature::new(20, "phase", 110, "Phase 1"));
        }
        else if phase == "1-2" || phase ==  "1 to 2" || phase ==  "i-ii" 
        || phase ==  "i+ii (phase i+phase ii)" || phase ==  "phase 1-2" 
        || phase ==  "phase 1 / phase 2" || phase ==  "phase 1/ phase 2" 
        || phase == "phase 1/phase 2" || phase ==  "phase i,ii" || phase == "phase1/phase2"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 115, "Phase 1/Phase 2"));
        }
        else if phase == "2" || phase ==  "2a" || phase ==  "2b" 
        || phase ==  "ii" || phase ==  "ii (phase ii study)" || phase ==  "iia" 
        || phase ==  "iib" || phase ==  "phase-2" || phase ==  "phase 2" || phase ==  "phase ii" || phase ==  "phase2"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 120, "Phase 2"));
        }
        else if phase == "2-3" || phase == "ii-iii" || phase ==  "phase 2-3" 
        || phase == "phase 2 / phase 3" || phase == "phase 2/ phase 3" 
        || phase ==  "phase 2/phase 3" || phase == "phase2/phase3" || phase == "phase ii,iii"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 125, "Phase 2/Phase 3"));
        }
        else if phase == "3" || phase ==  "iii" || phase ==  "iii (phase iii study)" 
        || phase ==  "iiia" || phase ==  "iiib" || phase ==  "3-4" || phase ==  "phase-3" 
        || phase ==  "phase 3" || phase ==  "phase 3 / phase 4" 
        || phase ==  "phase 3/ phase 4" || phase ==  "phase3" || phase ==  "phase iii"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 130, "Phase 3"));
        }
        else if phase == "4" || phase ==  "iv" || phase ==  "iv (phase iv study)" 
                 || phase == "phase-4" || phase ==  "phase 4" || phase ==  "post-market" 
                 || phase ==  "post marketing surveillance" || phase ==  "phase4" || phase ==  "phase iv"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 135, "Phase 4"));
        }
        else
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 1500, phase));
        }
    }

    fs
}


pub fn  split_and_add_ids(existing_ids: &Vec::<SecondaryId>, sd_sid: &String, in_string: &String, source_field: &str) -> Vec<SecondaryId>
    {
        // in_string already known to be non-null, non-empty.

        let ids: Vec<&str> = in_string.split(";").collect();
        let mut id_list = Vec::<SecondaryId>::new();

        for s in ids
        {
            let complex_trim = |c| c==' '|| c =='\''|| c ==';'|| c=='‘'|| c=='’';
            let secid = s.trim_matches(complex_trim);

            if secid.len() >= 3 && secid != sd_sid
            {
                let secid_low = secid.to_lowercase();
                if secid_low.chars().any(|c| c.is_digit(10))    // has to include at least 1 number
                    && !secid_low.starts_with("none")
                    && !secid_low.starts_with("nil")
                    && !secid_low.starts_with("not ")
                    && !secid_low.starts_with("date")
                    && !secid_low.starts_with("version")
                    && !secid_low.starts_with("??")
                {
                    let sec_id_base = get_sec_id_details(secid);
                   
                    // Is the id the same as the sid? (With EUCTR may be, 
                    // because it is simply anoher country code variation)
                    // Has this id been added before?
                  
                    let mut add_id = true;

                    if sec_id_base.processed_id == sd_sid.to_string() {
                        add_id = false;
                    }
                    else if existing_ids.len() > 0
                    {
                        for s in existing_ids
                        {
                            if sec_id_base.processed_id == s.processed_id
                            {
                                add_id = false;
                                break;
                            }
                        }
                    }

                    if add_id
                    {
                        id_list.push(SecondaryId::new_from_base(source_field.to_string(), 
                                                    secid.to_string(), sec_id_base));
                    }
                }
            }
        }

        id_list  // may overlap ids with the ids from the other source fields
    }
    

pub fn get_sec_id_details(sec_id: &str) -> SecIdBase
    {

        let mut sid = contains_nct(sec_id);
        if sid.is_none() {
            sid = contains_euctr(sec_id);
            if sid.is_none() {
                sid = contains_isrctn(sec_id);
                if sid.is_none() {
                    sid = contains_actrn(sec_id);
                    if sid.is_none() {
                        sid = contains_drks(sec_id);
                        if sid.is_none() {
                            sid = contains_ctri(sec_id);
                            if sid.is_none() {
                                sid = contains_who(sec_id);
                            }
                        }
                    }
                }
            }
        }

        if sid.is_none() {
            sid = contains_umin(sec_id);
            if sid.is_none() {
                sid = contains_jcrt(sec_id);  
                if sid.is_none() {
                    sid = contains_jprn(sec_id);  
                    if sid.is_none() {
                        sid = contains_nl(sec_id);
                        if sid.is_none() {
                            sid = contains_ntr(sec_id);
                            if sid.is_none() {
                                sid = contains_rpuec(sec_id);
                            }
                        }
                    }
                }
            }
        }

        /*  

        // transfer to later processing

        if sid.is_none() && sd_sid.starts_with("RBR")
        {
            sid = contains_anvisa(sec_id);
            if sid.is_none() {
                sid = contains_brethics(sec_id);
            }
        }
        */
               
        if sid.is_none() {    // still...
            
            let upid = sec_id.to_uppercase();
            let sec_id_source = match upid {
            _ if upid.starts_with("CHICTR") => 100118,
            _ if upid.starts_with("IRCT") => 100125,
            _ if upid.starts_with("KCT") =>  100119,
            _ if upid.starts_with("CTIS") => 110428,
            _ if upid.starts_with("RBR") => 100117, 
            _ if upid.starts_with("RPCEC") => 100122,
            _ if upid.starts_with("PACTR") => 100128,
            _ if upid.starts_with("SLCTR") => 100130,
            _ if upid.starts_with("TCTR") => 100131,
            _ if upid.starts_with("LBCTR") => 101989,
            _ if upid.starts_with("ITMCTR") => 109108,
            _ if upid.starts_with("CHIMCTR") => 104545,
            _ => 0
            };

            if sec_id_source > 0 {
                sid = Some(SecIdBase{
                    processed_id: sec_id.to_string(),
                    sec_id_source: sec_id_source, 
                    sec_id_type_id: 11,
                    sec_id_type: "Trial Registry ID".to_string(),
                })
            }
        }
              
        if sid.is_none() {
            
            // Return the original secondary id without any source.
            sid = Some(SecIdBase{
                processed_id: sec_id.to_string(),
                sec_id_source: 0, 
                sec_id_type_id: 0,
                sec_id_type: "?".to_string(),
            })
        }
        
        sid.unwrap()   // always has a value

    }

    
    // Collection of regex check functions, where the regex is stored
    // in a static lazy lock variable - i.e. it needs instantiating only once.

    fn contains_nct(hay: &str) -> Option<SecIdBase> {
        if hay.contains("NCT") {
            let hay2 = hay.replace("NCT ", "NCT").replace("NCTNumber", "");
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NCT[0-9]{8}").unwrap());
            match RE.captures(&hay2) {
                Some(s) => {
                let id = &s[0];
                if id == "NCT11111111" || id == "NCT99999999" 
                || id == "NCT12345678" || id == "NCT87654321" {
                    None
                }
                else {
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 100120, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                }
            },
                None => None,
            }  
        }
        else {
           None 
        }
    }

    fn contains_euctr(hay: &str) -> Option<SecIdBase> {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"20[0-9]{2}-[0-9]{6}-[0-9]{2}").unwrap());
        static RE_CTIS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"20[2-9][0-9]-5[0-9]{5}-[0-9]{2}").unwrap());
        match RE.captures(hay) {
            Some(s) => {
                match RE_CTIS.captures(hay) {
                    Some (s1) => {
                        let id = &s1[0];
                        let processed_id = format!("CTIS{}", &id[0..14]);
                        Some(SecIdBase{
                            processed_id: processed_id.to_string(),
                            sec_id_source: 110428, 
                            sec_id_type_id: 11,
                            sec_id_type: "Trial Registry ID".to_string(),
                        })
                    },
                    None => {   // assumed EUCTR
                        let id = &s[0];
                        let processed_id = format!("EUCTR{}", &id[0..14]);
                        Some(SecIdBase{
                            processed_id: processed_id.to_string(),
                            sec_id_source: 100123, 
                            sec_id_type_id: 11,
                            sec_id_type: "Trial Registry ID".to_string(),
                        })
                    },
                }
             },
            None => None,  // no eu match at all
        }
    }

    fn contains_isrctn(hay: &str) -> Option<SecIdBase> {
        if hay.contains("ISRCTN") {
            let mut hay2 = hay.replace("ISRCTN ", "ISRCTN").replace("(ISRCTN)", "");
            hay2 = hay2.replace("ISRCTN(International", "ISRCTN");
            hay2 = hay2.replace("ISRCTN: ", "ISRCTN");
            hay2 = hay2.replace("ISRCTNISRCTN", "ISRCTN");
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"ISRCTN[0-9]{8}").unwrap());
            match RE.captures(&hay2) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 100126, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }


    fn contains_actrn(hay: &str) -> Option<SecIdBase> {
        if hay.contains("ACTRN") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"ACTRN[0-9]{14}").unwrap());
            match RE.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 100116, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }


    fn contains_drks(hay: &str) -> Option<SecIdBase> {
        if hay.contains("DRKS") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"DRKS[0-9]{8}").unwrap());
            match RE.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 100124, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }


    fn contains_ctri(hay: &str) -> Option<SecIdBase> {
        if hay.contains("CTRI") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CTRI/[0-9]{4}/[0-9]{2,3}/[0-9]{6}").unwrap());
            match RE.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    let processed_id = id.replace("/", "-");  // internal representation for CTRI
                    Some(SecIdBase{
                        processed_id: processed_id,
                        sec_id_source: 100120, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }


    fn contains_who(hay: &str) -> Option<SecIdBase> {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"1111-[0-9]{4}-[0-9]{4}").unwrap());
        match RE.captures(hay) {
            Some(s) => {
                let id = &s[0];
                let processed_id = format!("U{}", id);  // internal representation for CTRI
                Some(SecIdBase{
                    processed_id: processed_id,
                    sec_id_source: 100115, 
                    sec_id_type_id: 11,
                    sec_id_type: "Trial Registry ID".to_string(),
                })
            },
            None => None,
        }
    }

    fn contains_umin(hay: &str) -> Option<SecIdBase> {
        if hay.contains("UMIN") {
            static RE1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"UMIN[0-9]{9}").unwrap());
            static RE2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"UMIN-CTR[0-9]{9}").unwrap());
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{9}").unwrap());
            if RE1.is_match(hay) || RE2.is_match(hay) {
                match RE.captures(hay) {
                    Some(s) => {
                        let id = &s[0];
                        let processed_id = format!("JPRN-UMIN{}", id);  
                        Some(SecIdBase{
                            processed_id: processed_id,
                            sec_id_source: 100127, 
                            sec_id_type_id: 11,
                            sec_id_type: "Trial Registry ID".to_string(),
                        })
                    },
                    None => None,
                }
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
    

    fn contains_jcrt(hay: &str) -> Option<SecIdBase> {
        if hay.contains("jRCT") {
            static RE1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"jRCTs[0-9]{9}").unwrap());
            static RE2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"jRCT[0-9]{10}").unwrap());
            match RE1.captures(hay) {
                Some(s1) => {
                    let id = Some(&s1[0]);
                    let processed_id = format!("JPRN-{:?}", id);  // internal representation for CTRI
                    Some(SecIdBase{
                        processed_id: processed_id,
                        sec_id_source: 100127, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                },

                None => {
                    match RE2.captures(hay) {
                        Some(s2) => {
                            let id = Some(&s2[0]);
                            let processed_id = format!("JPRN-{:?}", id);  // internal representation for CTRI
                            Some(SecIdBase{
                                processed_id: processed_id,
                                sec_id_source: 100127, 
                                sec_id_type_id: 11,
                                sec_id_type: "Trial Registry ID".to_string(),
                            })
                        },
                        None => None,
                    }
                }
            }
        }
        else {
            None
        }
    }


    fn contains_jprn(hay: &str) -> Option<SecIdBase> {
        if hay.starts_with("JPRN") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{8}").unwrap());
            if RE.is_match(hay) {
                match RE.captures(hay) {
                    Some(s) => {
                        let id = &s[0];
                        let processed_id = format!("JPRN-UMIN{}", id);  // internal representation for CTRI
                        Some(SecIdBase{
                            processed_id: processed_id,
                            sec_id_source: 100127, 
                            sec_id_type_id: 11,
                            sec_id_type: "Trial Registry ID".to_string(),
                        })
                    },
                    None => None,
                }
            }
            else {
                Some(SecIdBase{
                    processed_id: hay.to_string(),
                    sec_id_source: 100127, 
                    sec_id_type_id: 11,
                    sec_id_type: "Trial Registry ID".to_string(),
                })
            }
        }
        else {
            None
        }
    }
        

    fn contains_nl(hay: &str) -> Option<SecIdBase> {
        if hay.starts_with("NL") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^NL\d{1,4}$").unwrap());
            match RE.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 100132, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }


    fn contains_ntr(hay: &str) -> Option<SecIdBase> {
        if hay.starts_with("NTR") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^NTR\d{1,4}$").unwrap());
            match RE.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 100132, 
                        sec_id_type_id: 45,
                        sec_id_type: "Obsolete NTR number".to_string(),
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }


    fn contains_rpuec(hay: &str) -> Option<SecIdBase> {
        if hay.starts_with("PER") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^PER-[0-9]{3}-").unwrap());
            match RE.captures(hay) {
                Some(_s) => { 
                    let id = hay;
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 100129, 
                        sec_id_type_id: 11,
                        sec_id_type: "Trial Registry ID".to_string(),
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }
*/
    /* 

    // transfer these to later processing - priority here is other registry ids

    fn contains_anvisa(hay: &str) -> Option<SecIdBase> {
        if hay.starts_with("RBR") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{8}.[0-9].[0-9]{4}.[0-9]{4}").unwrap());
            match RE.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 102000,  // Brazilian regulatory authority, ANVISA
                        sec_id_type_id: 41,
                        sec_id_type: "Regulatory Body ID".to_string(),
                    })},
                None => None,
            } 
        }
        else {
            None
        }
    }

    fn contains_brethics(hay: &str) -> Option<SecIdBase> {
        if hay.starts_with("RBR") {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9].[0-9]{3}.[0-9]{3}").unwrap());
            match RE.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_source: 102001,  // Brazilian ethics committee approval number
                        sec_id_type_id: 12,
                        sec_id_type: "Ethics Review ID".to_string(),
                    })},
                None => None,
            }
        }
        else {
            None
        }
    }
*/




