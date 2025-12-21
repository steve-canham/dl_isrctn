/*
use regex::Regex;
use std::sync::LazyLock;
use log::error;
use chrono::NaiveDate;
use super::who_helper::{get_db_name, get_source_id, get_type, get_status, 
    get_conditions, split_and_dedup_countries,
    add_int_study_features, add_obs_study_features, add_eu_design_features,
    add_masking_features, add_phase_features, add_eu_phase_features, split_and_add_ids};
use super::gen_helper::{StringExtensions, DateExtensions};
use super::file_models::{WHOLine, WHORecord, WhoStudyFeature, SecondaryId, WHOSummary, MeddraCondition};


pub fn process_line(w: WHOLine,source_id: i32, sid: &String, study_type: i32, study_status: i32, 
                    remote_url:&Option<String>, study_idents: Option<Vec<SecondaryId>>, countries: Option<Vec<String>>) -> Option<WHORecord>  {

    let design_list = w.study_design.tidy();
    let design_orig = design_list.clone();
    let phase_statement = w.phase.tidy();
    let phase_orig = phase_statement.clone();

    let condition_list = w.conditions.replace_unicodes();
    let conditions_option: Option<Vec<String>>;
    let meddraconds_option: Option<Vec<MeddraCondition>>;
    if condition_list.is_some()
    {
        // Need a specific routine to deal with EUCTR and the MedDRA listings

        let conditions: Vec<String>;
        let meddraconds: Vec<MeddraCondition>;

        (conditions, meddraconds) = get_conditions(&condition_list.unwrap(), source_id);
        conditions_option = match conditions.len() {
            0 => None,
            _ => Some(conditions)
        };
        meddraconds_option = match meddraconds.len() {
            0 => None,
            _ => Some(meddraconds)
        };
    }
    else {
        conditions_option = None;
        meddraconds_option = None;
    }
   
    let mut features = Vec::<WhoStudyFeature>::new();

    if design_list.is_some()  {
        let des_list = &design_list.unwrap().to_lowercase();
        if study_type == 11
        {
            let mut fs = add_obs_study_features(des_list);
            features.append(&mut fs);
        }
        else
        {      
            if source_id == 100123 {
                let mut fs = add_eu_design_features(des_list);
                features.append(&mut fs);
            }
            else {         
                let mut fs = add_int_study_features(des_list);
                features.append(&mut fs);
                let mut fs = add_masking_features(des_list);
                features.append(&mut fs);
            }
        }
    }


    if phase_statement.is_some() {
        let phase_statement = &phase_statement.unwrap().to_lowercase();
        if source_id == 100123 {
            let mut fs = add_eu_phase_features(phase_statement);
            features.append(&mut fs);
        }
        else {
            let mut fs = add_phase_features(phase_statement);
            features.append(&mut fs);
        }
        
    }

    let study_features = match features.len() {
        0 => None,
        _ => Some(features)
    };



    let mut agemin = w.age_min.tidy();
    let mut agemin_units: Option<String> = None;
    let mut agemax = w.age_max.tidy();
    let mut agemax_units: Option<String> = None;

    if source_id != 100123 {

        if agemin.is_some()
        {
            let pat = r#"\d+"#;
            let re = Regex::new(pat).unwrap();
            let amin = agemin.unwrap();
            if re.is_match(&amin) {
                let caps = re.captures(&amin).unwrap();
                let min = &caps[0];
                agemin = Some(min.to_string());

                let min_units = amin.get_time_units();
                if min_units != "".to_string() {
                    agemin_units = Some(min_units);
                }
            }
            else {
                agemin = None;
            }
        }

        if agemax.is_some()
        {
            let pat = r#"\d+"#;
            let re = Regex::new(pat).unwrap();
            let amax = agemax.unwrap();
            if re.is_match(&amax) {
                let caps = re.captures(&amax).unwrap();
                let max = &caps[0];
                agemax = Some(max.to_string());

                let max_units = amax.get_time_units();
                if max_units != "".to_string() {
                    agemax_units = Some(max_units);
                }
            }
            else {
                agemax = None;
            }
        }
    }

 
    let mut gender = w.gender.tidy();
    if gender.is_some()
    {
        let gen_res = gender.unwrap().to_lowercase();
        if gen_res.contains("both")
        {
            gender = Some("Both".to_string());
        }
        else
        {
            let f = gen_res.contains("female") || gen_res.contains("women") || gen_res == "f";
            let mut gen2 = gen_res.clone();
            if f {
                gen2 = gen_res.replace("female", "").replace("women", "")
            }
            let m = gen2.contains("male") || gen_res.contains("men") || gen_res == "m";
            
            if m && f
            {
                gender = Some("Both".to_string());
            }
            else if m {
                gender = Some("Male".to_string());
            }
            else if f {
                gender = Some("Female".to_string()); 
            }
            else if gen_res == "-" {
                gender = None;
            }
            else // still no match...
            {
                gender = Some(format!("?? Unable to classify ({})", gen_res));
            }
        }
    }


    let mut inc_crit = w.inclusion_criteria.tidy();
    if inc_crit.is_some() {
        let mut crit = inc_crit.unwrap();
        if crit.to_lowercase().starts_with("inclusion criteria")
        {
            let complex_trim = |c| c == ' ' || c == ':' || c == ',';
            crit = crit[18..].trim_matches(complex_trim).to_string();

            if source_id == 100123  {

                // remove last part of the ic

                let f = crit.find("Are the trial subjects under 18?");
                if f.is_some() {
                    let pos = f.unwrap();
                    let crit2 = crit[..pos].to_string();
                    let age_data = crit[pos..].to_string();
                    let mut children: bool = false;
                    let mut adult: bool = false;
                    let mut aged: bool = false;

                    if age_data.contains("Are the trial subjects under 18? yes") {
                        children = true;

                    }
                    if age_data.contains("F.1.2 Adults (18-64 years) yes") {
                        adult = true;

                    }
                    if age_data.contains("F.1.3 Elderly (>=65 years) yes") {
                        aged = true;
                    }
                    
                    if children {
                        agemin = None;
                        agemin_units = None;

                        if !adult {
                            agemax = Some("17".to_string());
                            agemax_units = Some("Years".to_string());
                        }
                        else {     // adult 
                            if !aged {
                                agemax = Some("64".to_string());
                                agemax_units = Some("Years".to_string());
                            }
                            else {    // no upper limit
                                agemax = None;
                                agemax_units = None;
                            }
                        }
                    }
                    else {     // no children
                        if adult {
                            agemin = Some("18".to_string());
                            agemin_units = Some("Years".to_string());

                            if !aged {
                                agemax = Some("64".to_string());
                                agemax_units = Some("Years".to_string());
                            }
                            else {
                                agemax = None;
                                agemax_units = None;
                            }
                        }
                        else {       // aged only
                            agemin = Some("65".to_string());
                            agemin_units = Some("Years".to_string());

                            agemax = None;
                            agemax_units = None;
                        }
                    }
                    
                    crit = crit2;
                
                }
            }
        }
        inc_crit = crit.replace_tags_and_unicodes();
    }

    let mut exc_crit = w.exclusion_criteria.tidy();
    if exc_crit.is_some() {
        let mut crit = exc_crit.unwrap();
        if crit.to_lowercase().starts_with("exclusion criteria")
        {
            let complex_trim = |c| c == ' ' || c == ':' || c == ',';
            crit = crit[18..].trim_matches(complex_trim).to_string();
        }
        exc_crit = crit.replace_tags_and_unicodes();
    }

   
    let mut ipd_plan = w.results_ipd_plan.replace_tags_and_unicodes();
    if ipd_plan.is_some() {
         let plan = ipd_plan.clone().unwrap().to_lowercase();
         if plan.len() < 11 || plan == "not available" || plan == "not avavilable" 
            || plan == "not applicable" || plan.starts_with("justification or reason for")  
        {
            ipd_plan = None;
        }
    }

    let mut ipd_description = w.results_ipd_description.replace_tags_and_unicodes();
    if ipd_description.is_some() {
         let desc = ipd_description.clone().unwrap().to_lowercase();
         if desc.len() < 11 || desc == "not available" || desc == "not avavilable" 
            || desc == "not applicable" || desc.starts_with("justification or reason for")  
        {
            ipd_description = None;
        }
    }
   
    
    Some(WHORecord  {
        source_id: source_id, 
        record_date: w.last_updated.as_iso_date(),
        sd_sid: sid.clone(), 
        pub_title: w.pub_title.replace_unicodes(),
        scientific_title: w.scientific_title.replace_unicodes(),
        remote_url: remote_url.clone(),
        pub_contact_givenname: w.pub_contact_first_name.tidy(),
        pub_contact_familyname: w.pub_contact_last_name.tidy(),
        pub_contact_email: w.pub_contact_email.tidy(),
        pub_contact_affiliation: w.pub_contact_affiliation.tidy(),
        scientific_contact_givenname: w.sci_contact_first_name.tidy(),
        scientific_contact_familyname: w.sci_contact_last_name.tidy(),
        scientific_contact_email: w.sci_contact_email.tidy(),
        scientific_contact_affiliation: w.sci_contact_affiliation.tidy(),
        study_type_orig: w.study_type.tidy(),
        study_type: study_type,
        date_registration: w.date_registration.as_iso_date(),
        date_enrolment: w.date_enrollement.as_iso_date(),
        target_size: w.target_size.tidy(),
        study_status_orig: w.recruitment_status.tidy(),
        study_status: study_status,
        primary_sponsor: w.primary_sponsor.tidy(),
        secondary_sponsors: w.secondary_sponsors.tidy(),
        source_support: w.source_support.tidy(),
        interventions: w.interventions.replace_tags_and_unicodes(),
        agemin: agemin,
        agemin_units:agemin_units,
        agemax: agemax,
        agemax_units: agemax_units,
        gender: gender,
        inclusion_criteria: inc_crit,
        exclusion_criteria: exc_crit,
        primary_outcome: w.primary_outcome.replace_tags_and_unicodes(),
        secondary_outcomes: w.secondary_outcomes.replace_tags_and_unicodes(),
        bridging_flag: w.bridging_flag.tidy(),
        bridged_type: w.bridged_type.tidy(),
        childs: w.childs.tidy(),
        type_enrolment: w.type_enrolment.tidy(),
        retrospective_flag: w.retrospective_flag.tidy(),
        results_actual_enrollment: w.results_actual_enrollment.tidy(),
        results_url_link: w.results_url_link.tidy(),
        results_summary: w.results_summary.tidy(),
        results_date_posted: w.results_date_posted.as_iso_date(),
        results_date_first_pub: w.results_date_first_pub.as_iso_date(),
        results_url_protocol: w.results_url_protocol.tidy(),
        ipd_plan: ipd_plan,
        ipd_description:ipd_description,
        results_date_completed: w.results_date_completed.as_iso_date(),
        results_yes_no: w.results_yes_no.tidy(),
        design_string: design_orig,
        phase_string: phase_orig,
        country_list: countries,
        secondary_ids: study_idents,
        study_features: study_features,
        condition_list: conditions_option,
        meddra_condition_list: meddraconds_option,
    })
}


pub fn summarise_line(w: &WHOLine, i: i32) -> Option<WHOSummary>  {

    let sid = w.trial_id.replace("/", "-").replace("\\", "-").replace(".", "-");
    let mut sd_sid = sid.trim().to_string();
    
    if sd_sid == "" || sd_sid == "null" || sd_sid == "NULL" {        // Seems to happen, or has happened in the past, with one Dutch trial.
        error!("Well that's weird - no study id on line {}!", i);
        return None;
    }

    let source_id = get_source_id(&sd_sid);
    if source_id == 0
    {
        error!("Well that's weird - can't match the study id's {} source on line {}!", sd_sid, i);
        return None;
    }

    if source_id == 100123 {
        sd_sid = sd_sid[0..19].to_string(); // lose country specific suffix
    }

    let mut title = w.pub_title.replace_unicodes();
    if title.is_none() {
        title = w.scientific_title.replace_unicodes();
    }
    
    let study_type = w.study_type.tidy();
    let stype = get_type(&study_type);

    let status: i32;
    if w.results_yes_no.to_lowercase() == "yes" {
        status = 30;   // completed
    }
    else {
        let study_status = w.recruitment_status.tidy();
        status = get_status(&study_status);
    }
    
    let mut secondary_ids = Vec::<SecondaryId>::new();
    let sec_ids = w.sec_ids.tidy();

    if sec_ids.is_some()
    {
        let mut initial_ids = split_and_add_ids(&secondary_ids, &sd_sid, &sec_ids.unwrap(), "secondary ids");
        secondary_ids.append(&mut initial_ids);
    }
        
    let bridging_flag = w.bridging_flag.tidy();
    if bridging_flag.is_some() {
        let mut br_flag = bridging_flag.unwrap();

        if source_id == 100123 {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{4}-[0-9]{6}-[0-9]{2}-").unwrap());
            if RE.is_match(&br_flag) {
                br_flag = br_flag[0..19].to_string() // lose country specific suffix
            }
        }
        if br_flag != sd_sid
        {
            let mut bridge_ids = split_and_add_ids(&secondary_ids, &sd_sid, &br_flag, "bridging flag");
            secondary_ids.append(&mut bridge_ids);
        }
    }

    let childs = w.childs.tidy();
    if childs.is_some()
    {
        let mut child_ids = split_and_add_ids(&secondary_ids, &sd_sid, &childs.unwrap(), "bridged child recs");
        secondary_ids.append(&mut child_ids);
    }

    let secids = match secondary_ids.len() {
        0 => None,
        _ => Some(secondary_ids)
    };
   

    let res_posted = get_naive_date (&w.results_date_posted);
    let res_first_pub = get_naive_date (&w.results_date_first_pub);
    let res_completed = get_naive_date (&w.results_date_completed);
    let date_last_rev = get_naive_date (&w.last_updated);
    
    let date_reg = w.date_registration.as_iso_date();
    let reg_year: i32 = match date_reg {
        Some(d) => d[0..4].parse().unwrap_or(0),
        None => 0,
    };
   
    let mut table_name = get_db_name(source_id);
    let mut suffix: &str;

    if source_id == 100120 {
        if reg_year < 2010 {
            suffix = "_lt_2010";
        }
        else if reg_year < 2015 {
            suffix = "_2010_14";
        }
        else if reg_year < 2020 {
            suffix = "_2015_19";
        }
        else if reg_year < 2025 {
            suffix = "_2020_24";
        }
        else {
            suffix = "_2025_29";
        }
        table_name = table_name + suffix;
    }

    if source_id == 100118 || source_id == 100121 
        || source_id == 100127 {
        if reg_year < 2020 {
            suffix = "_lt_2020";
        }
        else {
            suffix = "_ge_2020";
        }
        table_name = table_name + suffix;
    }
    

    let country_list = w.countries.tidy();
    let countries: Option<Vec<String>>;
    if country_list.is_some()
    {
        countries = split_and_dedup_countries(source_id, &country_list.unwrap());
    }
    else {
        countries = None;
    }
  
    Some(WHOSummary {
        source_id: source_id, 
        sd_sid: sd_sid, 
        title: title,
        remote_url: w.url.tidy(),
        study_type: stype,
        study_status: status,
        secondary_ids: secids,
        date_registration: w.date_registration.as_iso_date(),
        date_enrolment: w.date_enrollement.as_iso_date(),
        results_yes_no: w.results_yes_no.tidy(),
        results_url_link: w.results_url_link.tidy(),
        results_url_protocol: w.results_url_protocol.tidy(),
        results_date_posted: res_posted,
        results_date_first_pub: res_first_pub,
        results_date_completed: res_completed,
        table_name: table_name,
        country_list: countries,
        date_last_rev: date_last_rev,    // assumed to be always present
    })
}


fn get_naive_date (dt: &String) -> Option<NaiveDate> {

   match dt.as_iso_date()
   {
        Some(s) => {
            let base_date = NaiveDate::parse_from_str("1900-01-01", "%Y-%m-%d").unwrap();
            let d = match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => base_date,
            };
            
            if d != base_date {
                Some(d)
            }
            else {
                None
            }
        },
        None => None,
   }
}
*/