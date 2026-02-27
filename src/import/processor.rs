use crate::data_models::json_models::Study;
use crate::data_models::db_models::*;
use crate::helpers::string_extensions::*;
use crate::helpers::name_extensions::*;
use crate::iec::iec_fns::*;
use crate::iec::iec_structs::IECLine;

use super::support_fns::*;
use chrono::{NaiveDate, NaiveDateTime, Local}; 
use std::sync::LazyLock;
use regex::Regex;
//use log::info;

 
// The processor needs to creat a full DB version of each study's data,
// working on the whole of the data to present it in aa 'database ready' form.
// This form has singleton fields for the 1:1 study comonents (study summary, 
// dates and participants respectively), and vector fields for each of the 
// 1:n data types, that will be stored as separate tables.

pub fn process_study_data(s: &Study) -> DBStudy {
    
    let sd_sid =  s.sd_sid.clone();

    // process titles
    
    // Repair the cockup made in the doewwnload process;
    // These titles should stay as single option<String> fields.
    // or be processed as below within the download process.

    let mut pub_title_db: Option<DBTitle> = None;
    let mut sci_title_db: Option<DBTitle> = None;
    let mut acronym_db: Option<DBTitle> = None;

    let mut pub_title_string = "".to_string();
    let mut sci_title_string = "".to_string();
    let mut acronym_string = "".to_string();

    let mut db_ts: Vec<DBTitle> = Vec::new();
    let display_title: String;

    // Set up the possible DBTitle instances and
    // the associated strings

    let pub_title = s.public_title.clone().clean();  
    if let Some(p) = pub_title {
        pub_title_string = p.clone().to_lowercase();

        pub_title_db = Some(DBTitle {
            title_text: p,
            is_public: true,
            is_scientific: false,
            is_acronym: false,
            is_display: true,
            comment: Some("From ISRCTN".to_string()),
        });
    }

    let sci_title = s.scientific_title.clone().clean(); 
    if let Some(s) = sci_title {
        sci_title_string = s.clone().to_lowercase();

        sci_title_db = Some(DBTitle {
            title_text: s,
            is_public: false,
            is_scientific: true,
            is_acronym: false,
            is_display: false,
            comment: Some("From ISRCTN".to_string()),
        });
    }

    let acronym = s.acronym.clone().clean(); 
    if let Some(a) = acronym {
        acronym_string = a.clone().to_lowercase();

        acronym_db = Some(DBTitle {
            title_text: a,
            is_public: false,
            is_scientific: false,
            is_acronym: true,
            is_display: false,
            comment: Some("From ISRCTN".to_string()),
        });
    }

    // Check for presence and duplication, and allocate 
    // default status and display string

    if let Some (mut pdb) = pub_title_db {

        // is_default and _is public already set
        // display text is the default;

        display_title = pdb.title_text.clone();

        // is the scientific title the same, if so
        // adjust the DBTitle objects

        if pub_title_string == sci_title_string  {
            pdb.is_scientific = true;
            sci_title_db = None;
        }

        // is the acronym title the same, if so
        // adjust the DBTitle objects

        if pub_title_string  == acronym_string  {
            pdb.is_acronym = true;
            acronym_db = None;
        }

        // is the acronym title the same as the scientific title, if so
        // adjust the DBTitle objects

        if sci_title_db.is_some() 
                && (sci_title_string == acronym_string) {
            let mut sdb = sci_title_db.unwrap();
            sdb.is_acronym = true;       // mark sci_title as 'is acronym'
            sci_title_db = Some(sdb);    // recreate the DBTitle object
            acronym_db = None;
        }

        // push whatever DBTitle objects are left to the models's vector 

        db_ts.push(pdb);
        if sci_title_db.is_some() {db_ts.push (sci_title_db.unwrap());}
        if acronym_db.is_some() {db_ts.push (acronym_db.unwrap());}


    } 
    else {
        
        // No public title  - a bit odd but...
        // First check if a scientific title exists

        if let Some (mut sdb) = sci_title_db {

            // Make the scientific title the default and set 
            // the display text.

            sdb.is_display = true;
            display_title = sdb.title_text.clone();

            // is the acronym title the same as the scientific title, if so
            // adjust the DBTitle objects

            if sci_title_string == acronym_string {

                sdb.is_acronym =  true;
                acronym_db = None;

            }

            db_ts.push(sdb);
            if acronym_db.is_some() {db_ts.push (acronym_db.unwrap());}
        }
        else {
            
            // Now check if at least an acronym exists 

            if let Some (mut adb) = acronym_db {

                // Very odd but just in case, set 
                // is default and display title accordingly
                
                adb.is_display = true;
                display_title = adb.title_text.clone();
                db_ts.push(adb);
            }
            else {

                // If not no title data was supplied at all (!)
                display_title = "No title data provided".to_string();
            }
        }
    }

        
    // Summary 
    
    // brief description
    // By default taken from the 'plain english summmary', as truncated during the
    // data download process. But if this is missing, or states it was not provided
    // at the time the record was created, a description is constructed from the 
    // study hypotheses and primary outcome fields.

    let mut description = match s.summary.plain_english_summary.clone() {
        Some (s) => {
                if s.to_lowercase().starts_with("not provided") {
                    None
                }
                else {
                    Some(s)
                }
        }
        None => None,
    };
    
    // No valid decsriotion in plain english summary...

    if description == None {
        let mut hypothesis = s.summary.study_hypothesis.clone().clean_multiline();
        let mut poutcome = s.summary.primary_outcome.clone().clean_multiline();

        if let Some(h) = hypothesis {
           if !h.to_lowercase().starts_with("not provided") {
               if !h.to_lowercase().starts_with("hypothes") && !h.to_lowercase().starts_with("study hyp")
               {
                    hypothesis = Some(format!("Study hypothesis: {}", h));
               }
               else {
                   hypothesis = Some(h);
               }
           }
           else {
               hypothesis = None;
           }
        }

        if let Some(p)  = poutcome {
            if !p.to_lowercase().starts_with("not provided") {
                if !p.to_lowercase().starts_with("primary") && !p.to_lowercase().starts_with("outcome")
                {
                    poutcome = Some(format!("Primary outcome: {}", p));
                }
                else {
                    poutcome = Some(p);
                }
            }
            else {
               poutcome = None;
           }
        }
        
        // Combine the two, if they both exist, or just use one

        description = match hypothesis {
            Some( h) => {
                match poutcome {
                        Some (p) => Some(format!("{}\n{}", h, p)),
                        None => Some(h), 
                } 
            },
            None => poutcome,
        };
         
    }

    // Finally extract the text or use a default value

    let description_text = match description {
        Some(d) => d,
        None => "No description text provided".to_string(),
    };

    // study status

    // Study status from overall study status or more commonly from dates.
    // 'StatusOverride' field will only have a value if status is 'Suspended' or 'Stopped'.
    // 'StartStatusOverride' will only have a value if status is 'Anticipated' or ???
    // More commonly compare dates with today to get current status.
    // Means periodic full import or a separate mechanism to update statuses against dates.
    // It appears that all 4 dates are always available.

    let mut status_string = "Unknown";
   
    if let Some(st) = &s.recruitment.recruitment_status_override {
        if st == "Stopped" {
            status_string = "Terminated";
        }
        else {
           status_string = st;  // Usually 'Suspended'
        }
    }
    else if let Some(st) = &s.recruitment.recruitment_start_status_override {
        if st == "Anticipated" {
            status_string = "Not yet recruiting";
        }
    }
    else {
        if let Some(se_date) = date_from_iso_string(s.summary.overall_end_date.clone()) {
            
            let today = Local::now().date_naive();
            if se_date <= today {
                status_string = "Completed";
            }
            else {   // Study is still ongoing - recruitment dates required for exact status.
               
                if let Some(rs_date) = date_from_iso_string(s.recruitment.recruitment_start.clone()) {
                    if rs_date > today
                    {
                        status_string = "Not yet recruiting";
                    }
                    else
                    {
                        status_string = "Recruiting";
                    }
                }

                // But check if recruiting has now finished.

                if status_string == "Recruiting" {
                    if let Some(re_date) = date_from_iso_string(s.recruitment.recruitment_end.clone()) {
                        if re_date <= today
                        {
                            status_string = "Active, not recruiting";
                        }
                    }
                    else {
                            status_string = "Ongoing, recruitment status unclear";
                    }
                }
            }
        }
    }

    // Need to check later for results being published, in which cases ensure status is completed
    
    let status_opt = if status_string == "" {None} else {Some(status_string.to_string())};
    let type_id = get_study_type(&s.design.primary_study_design);

    let date_last_revised = match &s.registration.last_updated {
        Some(ds) => {
            if ds.len() > 10 {
                match NaiveDate::parse_from_str(&ds[..10], "%Y-%m-%d") {
                    Ok(d) =>Some(d),
                    Err(_) => None,
                }
            }
            else {
                None
            }
        },
        None => None
    };
    let dt_of_data_fetch = NaiveDateTime::parse_from_str(&s.downloaded, "%Y-%m-%dT%H:%M:%S").unwrap();
         
    let summary = DBSummary {
        display_title: display_title,
        brief_description: description_text,
        type_id: type_id,
        status_id: get_study_status(&status_opt),
        status_override: s.recruitment.recruitment_status_override.clone(),
        start_status_override: s.recruitment.recruitment_start_status_override.clone(),
        is_ipd_sharing: s.ipd.ipd_sharing_plan,
        ipd_sharing_plan: s.ipd.ipd_sharing_statement.clone(), 
        date_last_revised: date_last_revised,
        dt_of_data_fetch: dt_of_data_fetch, 
    };

    // dates
    
    let (reg_year, reg_month, reg_date_type) = split_date_string( s.registration.date_id_assigned.clone());
    let (start_year, start_month, start_date_type) = split_date_string( s.recruitment.recruitment_start.clone());
    let (comp_year, comp_month, comp_date_type) = split_date_string( s.summary.overall_end_date.clone());
    let (res_year, res_month, res_date_type) = split_date_string( s.results.intent_to_publish.clone());

    let dates = DBStudyDates {
        reg_year: reg_year,  
        reg_month: reg_month,
        reg_date_type: reg_date_type,         
        start_year: start_year, 
        start_month: start_month,   
        start_date_type: start_date_type,       
        comp_year: comp_year,
        comp_month: comp_month,  
        comp_date_type: comp_date_type,      
        res_year: res_year,  
        res_month: res_month,   
        res_date_type: res_date_type,    
    };

    // participants

    let mut enrolment = None;
    let mut enrolment_type: Option<String> = None;
    if let Some(f) = s.recruitment.total_final_enrolment.clone() && f != "0"{
        enrolment = Some(f);
        enrolment_type = Some("a".to_string());
    }
    else {
        if let Some(t) = s.recruitment.target_enrolment.clone() {
            if t == "0" {
                enrolment = None;
                enrolment_type = None;
            }
            else {
                enrolment = Some(t);
                enrolment_type = Some("e".to_string());
            }
        } 
    }

    let mut gender_flag = None;
    if let Some(g) = s.participants.gender.clone()  {
        if g.to_lowercase().starts_with("not") {
            gender_flag = None;
        }
        else {    // "a", "m" or "f"
            gender_flag = Some(g.to_lowercase().chars().next().unwrap().to_string());
        }
    }
 
    let min_age_as_string = s.participants.l_age_limit.clone();
    let (min_age, min_age_units, min_num_days) = split_age_string(&min_age_as_string);

    let max_age_as_string = s.participants.u_age_limit.clone();
    let (max_age, max_age_units, max_num_days) = split_age_string(&max_age_as_string);

    let mut age_group_flag = 0;
    let adult_start = 18.0 * 365.25;
    let adult_end = 65.0 * 365.25;
    
    if !(min_num_days == 0.0 && max_num_days == 0.0) {   // i.e. if at least one age limit is defined
        if min_num_days < adult_start {    // starts in child (or just max age defined)
            age_group_flag = 1;
            if max_num_days == 0.0 || max_num_days > adult_end {
                age_group_flag = 7;
            }
            if max_num_days > adult_start && max_num_days <= adult_end {
                 age_group_flag = 3;
            }
        }
        if min_num_days >= adult_start {
            if max_num_days == 0.0 {
                age_group_flag = 6;
            }
            else {
                if max_num_days <= adult_end {
                    age_group_flag = 2;
                }
                else {
                    age_group_flag = 6;
                }
            }
        }
        if min_num_days >= adult_end  {
            age_group_flag = 4;
        }
    }

    let mut participants = DBStudyPartics {

        enrolment_target: s.recruitment.target_enrolment.clone(), 
        enrolment_final: s.recruitment.total_final_enrolment.clone(),
        enrolment_total: s.recruitment.total_target.clone(),
        enrolment: enrolment, 
        enrolment_type: enrolment_type,
        gender_string: s.participants.gender.clone(),
        gender_flag: gender_flag,
        min_age_string: min_age_as_string,
        min_age: min_age,  
        min_age_units_id: min_age_units,
        max_age_string: max_age_as_string,
        max_age: max_age,  
        max_age_units_id: max_age_units,
        age_group_flag: age_group_flag, 
        iec_flag: 0,   // for now
    };

    // Identifiers

    let mut db_ids: Vec<DBIdentifier> = Vec::new();

    // Include the ISRCTN id as the first identifier

    db_ids.push(DBIdentifier { 
        id_value: sd_sid.clone(), 
        id_type_id: 126,
        id_type: "ISRCTN ID".to_string(),
    });

    // Then the secondary ids already identified in the data download

    if let Some(ids) = &s.identifiers {
        for id in ids {
            db_ids.push(DBIdentifier { 
                id_value: id.identifier_value.clone(), 
                id_type_id: id.identifier_type_id, 
                id_type: id.identifier_type.clone(),
            })
        }
    }


    // Organisations

    let mut db_orgs: Vec<DBOrganisation> = Vec::new();
    let mut db_funds: Vec<DBOrganisation> = Vec::new();

    // sponsors

    // ? TidyOrgName(sid);
        
    if let Some(sponsors) = &s.sponsors {
        for s in sponsors {
            if s.organisation.appears_plausible_org_name() {
                let sname = s.organisation.clean().tidy_org_name(&sd_sid);
                db_orgs.push(DBOrganisation { 
                    org_name: sname, 
                    org_country: s.country.clone(),
                    org_ror_id: s.ror_id.clone(), 
                    org_cref_id: None, 
                    is_sponsor: Some(true),   
                    is_funder: None,  
                    is_collaborator: None,  
                });
            }
        }
    }

    // funders
 
    if let Some(funders) = &s.funders {
        for f in funders {
            if f.name.appears_plausible_org_name() {
                let fname = f.name.clean().tidy_org_name(&sd_sid);

                // See if that name has been used before as a sponsor.

                let mut duplicated = false;
                for dbo in &mut db_orgs {

                    if let Some(true) = dbo.is_sponsor {
                        if dbo.org_name == fname {  // Change contribution type and try to combine information
                            dbo.is_funder = Some(true);
                            dbo.org_cref_id = f.fund_ref.clone();
                            duplicated = true;
                            break;
                        }
                    }
                }

                if !duplicated   // Add as a separate funder.
                {
                    db_funds.push(DBOrganisation { 
                    org_name: fname,
                    org_country: None,
                    org_ror_id: None,
                    org_cref_id: f.fund_ref.clone(),
                    is_sponsor: None,   
                    is_funder: Some(true),   
                    is_collaborator: None,  
                    });
                }
            }
            else {
               // info!("odd org name{:?}, for {}", f.name.clone(), &sd_sid)
            }
        }
    }

    db_orgs.append(&mut db_funds);

    
    // contacts

    let mut db_peop: Vec<DBPerson> = Vec::new();

    if let Some(contacts) = &s.contacts {
        for c in contacts {
            if c.surname.appears_plausible_person_name() {
                if let Some(cts) = &c.contact_types {
                    let mut role_list = "".to_string();
                    for ct in cts {
                        if ct.to_lowercase() == "scientific" {
                            role_list = "Scientific contact".to_string();
                        }
                        if ct.to_lowercase()  == "principal investigator" {
                            if role_list == "" {
                                role_list = "Principal Investigator".to_string();
                            }
                            else {
                                role_list = "Scientific contact, Principal Investigator".to_string()
                            }
                        }
                    }
                    if role_list != "" {
                        db_peop.push(DBPerson {
                            full_name: get_full_name(c.forename.clone(), c.surname.clone()),
                            listed_as: Some(role_list.to_string()), 
                            orcid_id: c.orcid.tidy_orcid(), 
                            affiliation: c.address.clone(),
                            email_domain: c.email.extract_domain(),
                        });
                    }
                }
            }
        }
    }


    // locations

    let mut db_locs: Vec<DBLocation> = Vec::new();

    if let Some(locs) = &s.centres {
        for loc in locs {
            db_locs.push(DBLocation { 
                fac_name: loc.name.clone(), 
                fac_address: loc.address.clone(), 
                city_name: loc.city.clone(), 
                disamb_name: loc.state.clone(), 
                country_name: loc.country.clone(),
            });
        }
    }

    // countries

    let mut db_countries: Vec<DBCountry> = Vec::new();

    if let Some(cies) = &s.countries {
        for c in cies {
            db_countries.push ( DBCountry {
                country_name: c.clone(),
            });
        }
    }


    // Conditions

    let mut db_conds: Vec<DBCondition>= Vec::new();

    if let Some(conds) = &s.conditions {
        for c in conds {
          
            let mut desc: Option<String> = None;
            let mut c2: Option<String> = None;
           
            // re-arrange these odd entries where all three field are put into the description

            if let Some(ds) = c.description.clone() {

                if ds.starts_with("Topic:") {
                    let ds_parts: Vec<&str> = ds.split(';').collect();
                    if ds_parts.len() == 3 {
                        let c2_section = ds_parts[1].trim();
                        let desc_section = ds_parts[2].trim();
                        if c2_section.starts_with("Subtopic:") {
                            c2 = Some(c2_section[9..].trim().to_string());
                        }
                        if desc_section.starts_with("Disease:") {
                            desc = Some(desc_section[8..].trim().to_string());
                        }
                    }
                    if ds_parts.len() == 1 {
                        let ds_parts2: Vec<&str> = ds.split('/').collect();
                        if ds_parts2.len() == 2 {
                            c2 = Some(ds_parts2[1].trim().to_string());
                            desc = None;
                        }
                    }
                }
                else {   // A description present but 'normal', not 'Topic:...

                    desc = Some(ds.clone());

                    // May be the same as c2, if so make it None

                     if let Some(c) = &c2 
                        && c.trim().to_lowercase() == ds.trim().to_lowercase() {
                        desc = None;
                     }
                }
            }
            else  {    // No description present, just leave c2 'as is'
                desc = None;
                c2 = c.disease_class2.clone();
            } 
            
            db_conds.push ( DBCondition {
                class1: c.disease_class1.clone(),
                class2: c2,
                specific: desc,

            });
        }
    }


    // Features

    let mut db_feats: Vec<DBFeature>= Vec::new();

    if type_id == 11 {
        if let Some(ints) = &s.interventions {
            for int in ints {
                if let Some(p) = &int.phase {
                    let phase = match p.as_str() {
                        "Not Applicable" => "Not applicable",
                        "Phase I" => "Phase 1",
                        "Phase I/II" => "Phase 1/Phase 2",
                        "Phase II" => "Phase 2",
                        "Phase II/III" => "Phase 2/Phase 3",
                        "Phase III" => "Phase 3",
                        "Phase III/IV" => "Phase 3",
                        "Phase IV" => "Phase 4",
                        "Not Specified" => "Not provided",
                        _ => "Not provided",
                    };
                    if phase != "Not provided" {
                        db_feats.push(DBFeature {
                            source: p.clone(),
                            feature_type: "Phase".to_string(),
                            feature_value: phase.to_string(),
                        });
                    }
                }
            }
        }
    }

    let secondary_design = &s.design.secondary_study_design.clone().unwrap_or("".to_string());
    let study_design = &s.design.study_design.clone().unwrap_or("".to_string());
    let design = format!("{} {}", secondary_design, study_design).trim().to_lowercase();

    if design != "".to_string()
    {
        if type_id == 11 {

            // Try to make terminology more consistent

            let mut ds = design.replace("randomized", "randomised")
                                .replace("non randomised", "non-randomised");
            ds = ds.replace("cross over", "cross-over").replace("crossover", "cross-over");
            ds = ds.replace("open label", "open-label").replace(" blind", "-blind");

            let allocation_type = match ds
            {
                _ if ds.contains("non-randomised") => "Nonrandomised",
                _ if ds.contains("randomised") => "Randomised",
                _ => "Not provided",
            };
            if allocation_type != "Not provided" {
                db_feats.push(DBFeature {
                    source: ds.clone(),
                    feature_type: "Allocation type".to_string(),
                    feature_value: allocation_type.to_string(),
                });
            }

            let intervention_model = match ds
            {
                _ if ds.contains("parallel") => "Parallel assignment",
                _ if ds.contains("cross-over") => "Crossover assignment",
                _ => "Not provided",
            };
            if intervention_model != "Not provided" {
                db_feats.push(DBFeature {
                    source: ds.clone(),
                    feature_type: "Intervention model".to_string(),
                    feature_value: intervention_model.to_string(),
                });
            }

            let masking = match ds
            {
                _ if ds.contains("open-label") => "None (Open Label)",
                _ if ds.contains("single-blind") => "Single",
                _ if ds.contains("double-blind") => "Double",
                _ if ds.contains("triple-blind") => "Triple",
                _ if ds.contains("quadruple-blind") => "Quadruple",
                _ => "Not provided",
            };
            if masking != "Not provided" {
                db_feats.push(DBFeature {
                    source: ds.clone(),
                    feature_type: "Masking".to_string(),
                    feature_value: masking.to_string(),
                });
            }
        }

        if type_id == 12 {

            let mut ds = design.replace("case ", "case-");
            ds = ds.replace("cross section", "cross-section");

            let observational_model = match ds
            {
                _ if ds.contains("cohort") => "Cohort",
                _ if ds.contains("case-control") => "Case-Control",
                _ if ds.contains("case-series") => "Case-only",
                _ if ds.contains("case-crossover") => "Case-crossover",
                _ if ds.contains("ecological") => "Ecologic or community study",
                _ => "Not provided",
            };
            if observational_model != "Not provided" {
                db_feats.push(DBFeature {
                    source: ds.clone(),
                    feature_type: "Observational model".to_string(),
                    feature_value: observational_model.to_string(),
                });
            }

            let time_perspective = match ds
            {
                _ if ds.contains("retrospective") => "Retrospective",
                _ if ds.contains("prospective") => "Prospective",
                _ if ds.contains("cross section") => "Cross-sectional",
                _ if ds.contains("longitudinal") => "Longitudinal",
                _ => "Not provided",
            };
            if time_perspective != "Not provided" {
                db_feats.push(DBFeature {
                    source: ds.clone(),
                    feature_type: "Time perspective".to_string(),
                    feature_value: time_perspective.to_string(),
                });
            }

        }

    }
 
    if let Some(tts) = &s.trial_types {
        for tt in tts {
            if tt != "Not Specified" {
                db_feats.push(DBFeature {
                    source: tt.clone(),
                    feature_type: "Primary focus".to_string(),
                    feature_value: tt.clone(),
                });
            }
        }
    }

    // Topics

    let mut db_tops: Vec<DBTopic>= Vec::new();

    if let Some(ints) = &s.interventions {
        for int in ints {
            
            let mut int_type = "";
            let mut topic_type ="Chemical / agent".to_string();  // effectively the default
            if let Some(it) = &int.int_type
            {
                int_type = it;
                if int_type == "Device" {
                    topic_type  = "Device".to_string();
                }
            }
            
            // problem remains of commas in brackets
            // square brackets should be parentheses
            // text in brackets should be appended to the text before
            
            if let Some(dn) = &int.drug_names.clean() {
                if !dn.to_lowercase().starts_with("the sponsor has confirmed")
                && !dn.to_lowercase().starts_with("the health research authority")
                && !dn.to_lowercase().starts_with("not provided")
                {
                    let mut drug_names = dn.to_string();
                    let source = drug_names.clone();
                    drug_names = drug_names.replace("\u{00AE}", ""); //  lose (r) Registration mark
                    drug_names = drug_names.replace("\u{2122}", ""); //  lose (tm) Trademark mark
                    drug_names = drug_names.replace("[", "(").replace("]", ")"); //  regularise brackets
                    drug_names = drug_names.replace(" and ", ", "); // in most cases indicates end of list

                    if drug_names.len() < 250 {

                        // very long entries in this field often 'mini-essays' and cannot be split
                    
                        if drug_names.contains("1.") && drug_names.contains("\n2.")
                        {
                            if let Some(dns) = get_cr_numbered_strings(&drug_names) {
                                for dn in dns {
                                    db_tops.push(DBTopic {
                                    source: source.clone(),
                                    topic_type: topic_type.clone(),
                                    topic_value: dn.to_string(),
                                    });
                                }
                            }
                        }
                        else if drug_names.contains("1. ") && drug_names.contains("2. ")
                        {
                            if let Some(dns) = get_numbered_strings(&drug_names) {
                                for dn in dns {
                                    db_tops.push(DBTopic {
                                    source: source.clone(),
                                    topic_type: topic_type.clone(),
                                    topic_value: dn.to_string(),
                                    });
                                }
                            }
                        }
                        else if drug_names.contains(',') {
                            
                            // if there are commas split on the commas (does not work for devices).

                            if int_type == "Drug" || int_type == "Supplement" {
                                
                                let sns = get_comma_delim_strings(&drug_names, 4); 
                                for sn in sns {
                                    db_tops.push(DBTopic {
                                    source: source.clone(),
                                    topic_type: topic_type.clone(),
                                    topic_value: sn.to_string(),
                                    });
                                }
                            }
                        }
                        else
                        {
                            db_tops.push(DBTopic {
                                source: source,
                                topic_type: topic_type,
                                topic_value: drug_names,
                            });
                        }
                    }
                    else {         // long entries
                        db_tops.push(DBTopic {
                                source: source,
                                topic_type: topic_type,
                                topic_value: drug_names,
                            });
                    }
                }
            }
        }
    }

    // IE Criteria

    let mut db_iec: Vec<IECLine> = Vec::new();

    let mut inc_result = 0;
    let mut exc_result = 0;

    let incs = &s.participants.inclusion.clean_multiline();
    if incs.is_not_a_place_holder() {
        if let Some(inc_para) = incs {
            let (inc_result_code, mut inc_criteria) = original_process_iec(&sd_sid, &inc_para, "inclusion");
            inc_result = inc_result_code;

            if inc_criteria.len() > 0 {
                db_iec.append(&mut inc_criteria);
            }
        }
    }
           
    let excs = &s.participants.exclusion.clean_multiline();
    if excs.is_not_a_place_holder() {
        if let Some(exc_para) = excs {
            let (exc_result_code, mut exc_criteria) = original_process_iec(&sd_sid , &exc_para, "exclusion");
            exc_result = exc_result_code;

            if exc_criteria.len() > 0 {
                db_iec.append(&mut exc_criteria);
            }
        }
    }

    participants.iec_flag =  inc_result + exc_result;



    // Outputs

    let mut db_pubs: Vec<DBPublication> = Vec::new();
    let mut db_objects: Vec<DBObject> = Vec::new();

    if let Some(links) = &s.links{
        for lk in links {

            let creation_dt = match lk.date_created.clone() {
                Some(ds) => match NaiveDate::parse_from_str(&ds, "%Y-%m-%d") {
                    Ok(d) => Some(d),
                    Err(_) => None,
                }
                None => None,
            };

            let upload_dt = match lk.date_uploaded.clone() {
                Some(ds) => match NaiveDate::parse_from_str(&ds, "%Y-%m-%d") {
                    Ok(d) => Some(d),
                    Err(_) => None,
                }
                None => None,
            };
            
            let link_type_string = lk.link_type.clone().unwrap_or_default().to_lowercase();
            let link_type = match link_type_string.as_str() {
                "abstract" => "Abstract",
                "hrasummary" => "HRA Summary",
                "trialwebsite" => "Trial Website",
                "protocolarticle" => "Protocol Article",
                "protocolpreprint" => "Protocol Preprint",
                "thesis" => "Thesis",
                "protocolother" => "Protocol (other format)",
                "sap" => "SAP",
                "otherfiles" => "Other files",
                "interimresults" => "Interim Results",
                "otherunpublished" => "Other Unpublished",
                "protocolfile" => "Protocol File",
                "otherpublications" => "Other Publications",
                "poster" => "Poster",
                "dataset" => "Dataset",
                "basicresults" => "Basic Results",
                "resultsarticle" => "Results Article",
                "preprint" => "Preprint",
                "funderreport" => "Funder Report",
                "plainenglishresults" => "Plain English Results",
                "pis" => "PIS",
                _ => "?",
            };

            let mut is_pub = false;
            let mut external_url = lk.link_url.clone().unwrap_or_default();
            external_url = external_url.replace("http://", "https://");    // regularise url scheme
            external_url = external_url.replace("doi.org10.", "doi.org/10.");  // repair needed rarely

            if link_type_string.contains("article") 
                || link_type_string.contains("preprint")
                || link_type_string.contains("abstract")
                || link_type_string == "interimresults" 
                || link_type_string == "otherpublications" 
            && !(external_url.ends_with("pdf")
                || external_url.ends_with("zip")  
                || external_url.ends_with("csv")  
                || external_url.ends_with("xlsx")  
                || external_url.ends_with("docx")) {
                
                is_pub = true;
            }
            
            if is_pub {

                // Simplify output type and description

                let link_type = match link_type {
                    "Abstract" => "Abstract",
                    "Protocol Article" => "Protocol", 
                    "Protocol Preprint" => "Protocol", 
                    "Protocol (other format)" => "Protocol", 
                    "Interim Results" => "Interim Results", 
                    "Protocol File" => "Protocol", 
                    "Other Publications" => "Other", 
                    "Results Article" => "Results", 
                    "Preprint" => "Preprint", 
                    _ => "?",
                };

                let mut notes: Option<String> = None;
                if let Some(mut d) = lk.description.clone() {
                    d = d.trim().trim_matches(':').to_string();
                    if d.to_lowercase() != link_type.to_lowercase() {
                        notes = Some(capitalize_first(&d));
                    }
                }

                let low_url =  external_url.to_lowercase();

                let mut pub_id = "".to_string();
                let mut pub_id_type = "".to_string();

                let mut doi_as_str= "".to_string();
                let mut pmid_as_str= "".to_string();
                let mut pmcid_as_str= "".to_string();
                let mut pubsite_url_as_str= "".to_string();
                let mut categorised = false;

                static RE_PM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{6,8}").unwrap());
                static RE_PMC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"PMC[0-9]{6,7}").unwrap());

                if low_url.contains("pubmed") {

                    // extract pmid and redo the url to ensure in current form
                    // test for an 8 digit string first, then a 7 digit

                    match RE_PM.captures(&external_url.clone()) {
                        Some(s) => {
                            pmid_as_str = s[0].to_string();
                            external_url = format!("https://pubmed.ncbi.nlm.nih.gov/{}/", pmid_as_str);
                            pub_id = pmid_as_str.clone();
                            pub_id_type = "PMID".to_string();
                            categorised =true;
                        },
                        None => {}, 
                    }
                }

                if low_url.contains("doi") {   // store doi
                    
                    // get substring from the beginning of the 10.

                    let mut doi = String::new();
                    if low_url.starts_with("https://dx.doi.org/10.") {
                        doi = (&low_url[19..]).to_string();
                    }
                    if low_url.starts_with("https://doi.org/10.") {
                        doi= (&low_url[16..]).to_string();
                    }
                    if low_url.starts_with("https://www.doi.org/10.") {
                        doi= (&low_url[20..]).to_string();
                    }
                    if ! doi.is_empty() {
                        external_url = format!("https://doi.org/{}", doi);  
                        doi_as_str = external_url.clone();  
                        pub_id = doi_as_str.clone(); 
                        pub_id_type = "doi".to_string();
                        categorised =true;  
                     }
                }
                
                if low_url.contains("pmc") && !categorised {

                    // extract pmc id and redo the url to ensure in current form

                    match RE_PMC.captures(&external_url.clone()) {
                        Some(s) => {
                            pmcid_as_str = s[0].to_string();
                            external_url = format!("https://pubmed.ncbi.nlm.nih.gov/{}/", pmcid_as_str);
                            pub_id = pmcid_as_str.clone(); 
                            pub_id_type = "PMC ID".to_string();
                            categorised =true;
                        },
                        None => {},  
                    }
                }

                if !categorised {  // probably a publisher's web site URL
                    pubsite_url_as_str = external_url.clone();
                    pub_id = pubsite_url_as_str.clone(); 
                    pub_id_type = "pub site URL".to_string();
                }
               
                db_pubs.push(DBPublication { 
                    pub_type: link_type.to_string(), 
                    pub_id: pub_id,
                    pub_id_type: pub_id_type,
                    pub_notes: notes, 
                    external_url: Some(external_url), 
                    doi: if doi_as_str == "".to_string() {None} else {Some(doi_as_str)},
                    pmid: if pmid_as_str == "".to_string() {None} else {Some(pmid_as_str)},
                    pmcid: if pmcid_as_str == "".to_string() {None} else {Some(pmcid_as_str)},
                    pubsite_url: if pubsite_url_as_str == "".to_string() {None} else {Some(pubsite_url_as_str)},
                    date_created: creation_dt, 
                    date_uploaded: upload_dt, 
                });
            }
            else {

                // a non publication link

                let mut details: Option<String> = None;
                if let Some(mut d) = lk.description.clone() {
                    d = d.trim().trim_matches(':').to_string();
                    if d.to_lowercase() != link_type.to_lowercase() {
                        details = Some(capitalize_first(&d));
                    }
                }

                // may be a web page or a file if has a file ending in url

                let mut access_url= lk.link_url.clone();
                let instance_type: Option<String>;
               
                if external_url.ends_with("pdf")
                || external_url.ends_with("zip")  
                || external_url.ends_with("csv")  
                || external_url.ends_with("xlsx")  
                || external_url.ends_with("docx") 
                || external_url.ends_with("xls")  
                || external_url.ends_with("doc") {

                    instance_type = Some("File download".to_string()); 

                    let mut access_url_string = access_url.clone().unwrap_or_default();
                    if access_url_string.contains("articles/PMC") {
                        access_url_string = access_url_string.replace("articles/PMC", "articles/instance/");
                        access_url = Some(access_url_string);
                    }
                }
                else {
                    instance_type = Some("Web page".to_string()); 
                }



                db_objects.push(DBObject { 
                    object_type: link_type.to_string(), 
                    object_id: format!("{}/{}", sd_sid.replace("ISRCTN", "isrctn-"), link_type_string),
                    object_id_type: "Constructed from type".to_string(), 
                    object_notes: details.clone(), 
                    display_name: None,
                    access_url: access_url, 
                    access_type: Some("Public".to_string()),
                    instance_type: instance_type,
                    instance_notes: None, 
                    date_created: creation_dt, 
                    date_uploaded: upload_dt, 
                });
            }
        }
    }
    

    if let Some(files) = &s.files{
        for f in files {

            let creation_dt = match f.date_created.clone() {
                Some(ds) => match NaiveDate::parse_from_str(&ds, "%Y-%m-%d") {
                    Ok(d) => Some(d),
                    Err(_) => None,
                }
                None => None,
            };

            let upload_dt = match f.date_uploaded.clone() {
                Some(ds) => match NaiveDate::parse_from_str(&ds, "%Y-%m-%d") {
                    Ok(d) => Some(d),
                    Err(_) => None,
                }
                None => None,
            };
            
            let file_type_string = f.file_type.clone().unwrap_or_default().to_lowercase();
            let file_type = match file_type_string.as_str() {
                "abstract" => "Abstract",
                "hrasummary" => "HRA Summary",
                "trialwebsite" => "Trial Website",
                "protocolarticle" => "Protocol Article",
                "protocolpreprint" => "Protocol Preprint",
                "thesis" => "Thesis",
                "protocolother" => "Protocol (other format)",
                "sap" => "SAP",
                "otherfiles" => "Other files",
                "interimresults" => "Interim Results",
                "otherunpublished" => "Other Unpublished",
                "protocolfile" => "Protocol File",
                "otherpublications" => "Other Publications",
                "poster" => "Poster",
                "dataset" => "Dataset",
                "basicresults" => "Basic Results",
                "resultsarticle" => "Results Article",
                "preprint" => "Preprint",
                "funderreport" => "Funder Report",
                "plainenglishresults" => "Plain English Results",
                "pis" => "PIS",
                _ => "?",
            };

            let mut details: Option<String> = None;
            if let Some(mut d) = f.description.clone() {
                d = d.trim().trim_matches(':').to_string();
                if d.to_lowercase() != file_type.to_lowercase() {
                    details = Some(capitalize_first(&d));
                }
            }

            // derive name and file notes

            let lg = match f.length {
                Some(len) => {
                    if len > 0 {
                        let kb_num: f64 = len as f64 / 1024.0;
                        if kb_num >= 1024.0 {
                            let mb_num = kb_num/ 1024.0;
                            format!("{:.2}Mb", mb_num)
                        }
                        else {
                            format!("{:.0}Kb", kb_num)
                        }
                    }
                    else {
                        String::new()
                    }
                },
                None => String::new(),
            };

            let mt = match f.mime_type.clone() {
                Some(m) => {
                    if !m.is_empty() {
                        let m_type = m.replace("application/", "");
                        let file_type = match m_type.as_str() {
                            "pdf" => "PDF",
                            "x-zip-compressed" => "ZIP",
                            "msword" => "DOC",
                            "vnd.openxmlformats-officedocument.wordprocessingml.document" => "DOCX",
                            "vnd.openxmlformats-officedocument.spreadsheetml.sheet" => "XSLX",
                            "x-spss-sav" => "SPSS SAV",
                            _ => "",
                        };
                        if file_type != ""{
                            file_type.to_string()
                        }
                        else {
                            String::new()
                        }
                    }
                    else {
                        String::new()
                    }
                },
                None => String::new(),
            };

            let mut i_notes = String::new();
            if !mt.is_empty() {
                if !lg.is_empty() {
                    i_notes = format!("{} file, {}", mt, lg);
                }
                else {
                    i_notes = format!("{} file", mt);
                }
            }

            let instance_notes = 
                if i_notes == String::new() 
                {None} else {Some(i_notes)};

            let ve = match f.version.clone() {
                Some(v) => if !v.is_empty() {
                    if v.to_lowercase().contains('v') {
                        v
                    }
                    else {
                        format!("v{}", v)
                    }
                }
                else {
                    String::new()
                },
                None => String::new(),
            };

            
            let df = match f.download_filename.clone() {
                 Some(dlfn) => if !dlfn.is_empty(){
                    let ending = match mt.as_str() {
                        "PDF" => ".pdf",
                        "ZIP" => ".zip",
                        "DOC" => ".doc",
                        "DOCX" => ".docx",
                        "XSLX" => ".xlsx",
                        "CSV" => ".csv",
                        "SPSS SAV" => ".sav",
                        _ => "",
                    };
                    if ending != "" {
                        let t_dlfn = dlfn.replace(ending, "");
                        if t_dlfn.to_lowercase().contains(ve.as_str()) {
                            t_dlfn
                        }
                        else {
                            format!("{}.{}", t_dlfn, ve)
                        }
                    }
                    else {
                        if dlfn.to_lowercase().contains(ve.as_str()) {
                            dlfn
                        }
                        else {
                            format!("{}.{}", dlfn, ve)
                        }
                    }
                }
                 else {
                    String::new()
                },
                 None => String::new(),
                                     
            };

            let display_name: Option<String>;
            let object_id: String;     
            let object_id_type: String;  

            if df == String::new() {
                display_name = None;
                object_id = format!("{}/{}", sd_sid.replace("ISRCTN", "isrctn-"), file_type);
                object_id_type =  "Constructed from type".to_string(); 
            }
            else {
                display_name = Some(df.clone());
                object_id = format!("{}/{}", sd_sid.replace("ISRCTN", "isrctn-"), df);
                object_id_type =  "Constructed from name".to_string(); 
            }
           
            db_objects.push(DBObject { 
                object_type: file_type.to_string(), 
                object_id: object_id,
                object_id_type: object_id_type, 
                object_notes: details.clone(), 
                display_name: display_name,
                access_url: f.download_url.clone(), 
                access_type: Some("Public".to_string()),
                instance_type: Some("File download".to_string()),
                instance_notes: instance_notes, 
                date_created: creation_dt, 
                date_uploaded: upload_dt, 
           });

        }
    }

   
    DBStudy {

        sd_sid: sd_sid,
        summary: summary,
        dates: dates,
        participants: participants,
        titles: option_from_count(db_ts),
        identifiers: option_from_count(db_ids),
        orgs: option_from_count(db_orgs),
        people: option_from_count(db_peop),
        locations: option_from_count(db_locs),
        countries: option_from_count(db_countries),
        conditions: option_from_count(db_conds),
        features: option_from_count(db_feats),
        topics: option_from_count(db_tops),
        ie_crit: option_from_count(db_iec),
        publications: option_from_count(db_pubs),
        objects: option_from_count(db_objects),
    }

}

