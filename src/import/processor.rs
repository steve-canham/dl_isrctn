
use crate::data_models::json_models::Study;
use crate::data_models::db_models::*;

use crate::helpers::string_extensions::*;
use crate::helpers::name_extensions::*;
use crate::helpers::iec_fns::*;
use crate::helpers::iec_helper::IECLine;

use super::support_fns::*;

use chrono::{NaiveDate, NaiveDateTime, Local}; 
use log::info;

 
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

    for t in &s.titles {

        if t.title_type_id == 15 { 
            let pub_title = Some(t.title_value.clone()).clean();  
            if let Some(p) = pub_title {
                pub_title_string = p.clone().to_lowercase();

                pub_title_db = Some(DBTitle {
                    title_text: p,
                    is_default: true,
                    is_public: true,
                    is_scientific: false,
                    is_acronym: false,
                    comment: Some("From ISRCTN".to_string()),
                });
            }
        }

        if t.title_type_id == 16 { 
            let sci_title = Some(t.title_value.clone()).clean(); 
            if let Some(s) = sci_title {
                sci_title_string = s.clone().to_lowercase();

                sci_title_db = Some(DBTitle {
                    title_text: s,
                    is_default: false,
                    is_public: false,
                    is_scientific: true,
                    is_acronym: false,
                    comment: Some("From ISRCTN".to_string()),
                });
            }
        }

        if t.title_type_id == 14 { 
            let acronym = Some(t.title_value.clone()).clean(); 
            if let Some(a) = acronym {
                acronym_string = a.clone().to_lowercase();

                acronym_db = Some(DBTitle {
                    title_text: a,
                    is_default: false,
                    is_public: false,
                    is_scientific: false,
                    is_acronym: true,
                    comment: Some("From ISRCTN".to_string()),
                });
            }
        }

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

            sdb.is_default = true;
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
                
                adb.is_default = true;
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
    // More commonly compare dates with today to get current status.
    // Means periodic full import or a separate mechanism to update statuses against dates.
    // It appears that all 4 dates are always available.

    let mut status_string = "Unkown";
   
    if let Some(st) = &s.recruitment.recruitment_status_override {
        if st == "Stopped" {
            status_string = "Terminated";
        }
        else {
           status_string = st;  // Usually 'Suspended'
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

    let mut _iec_flag = 0;   // for now

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
        iec_flag: _iec_flag,
        ipd_sharing: s.ipd.ipd_sharing_plan,
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

    let gender_flag = Some("a".to_string());
    let age_group_flag = Some(4);

    let mut enrolment = None;
    let mut enrolment_type = None;
    if let Some(f) = s.recruitment.total_final_enrolment.clone() {
        enrolment = Some(f);
        enrolment_type = Some("a".to_string());
    }
    else {
        if let Some(t) = s.recruitment.target_enrolment.clone() {
            enrolment = Some(t);
            enrolment_type = Some("e".to_string());
        } 
    }

    let participants = DBStudyPartics {

        enrolment_target: s.recruitment.target_enrolment.clone(), 
        enrolment_final: s.recruitment.total_final_enrolment.clone(),
        enrolment_total: s.recruitment.total_target.clone(),
        enrolment: enrolment, 
        enrolment_type: enrolment_type,

        gender_flag: gender_flag,
        min_age_as_string: s.participants.l_age_limit.clone(),
        min_age: s.participants.l_age_limit_num,  
        min_age_units_id: get_age_units(&s.participants.l_age_limit_units),
        max_age_as_string: s.participants.u_age_limit.clone(),
        max_age: s.participants.u_age_limit_num,  
        max_age_units_id: get_age_units(&s.participants.u_age_limit_units),
        age_group_flag: age_group_flag, 
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
                    contrib_type: "sponsor".to_string(), 
                    org_name: sname, 
                    country: s.country.clone(),
                    org_ror_id: s.ror_id.clone(), 
                    org_cref_id: None, 
                    sponsor_type: s.sponsor_type.clone(),
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

                    if dbo.contrib_type == "sponsor".to_string() {
                        if dbo.org_name == fname {  // Change contribution type and try to combine information

                            dbo.contrib_type = "sponsor & funder".to_string();
                            dbo.org_cref_id = f.fund_ref.clone();
                            duplicated = true;
                            break;
                        }
                    }
                }

                if !duplicated   // Add as a separate funder.
                {
                    db_funds.push(DBOrganisation { 
                    contrib_type: "funder".to_string(), 
                    org_name: fname,
                    country: None,
                    org_ror_id: None,
                    org_cref_id: f.fund_ref.clone(),
                    sponsor_type: None, 
                    });
                }
            }
            else {
                info!("odd org name{:?}, for {}", f.name.clone(), &sd_sid)
            }
        }
    }

    db_orgs.append(&mut db_funds);

    
    // contacts

    let mut db_peop: Vec<DBPerson> = Vec::new();

    if let Some(contacts) = &s.contacts {
        for c in contacts {
            if c.surname.appears_plausible_person_name() {
                if let Some(v) = &c.contact_types {
                    db_peop.push(DBPerson {
                        contrib_type: v.join(","),
                        given_name: c.forename.clone(),
                        family_name: c.surname.clone(),
                        orcid_id: c.orcid.tidy_orcid(),
                        affiliation: c.address.clone(),
                        email_domain: c.email.extract_domain(),
                    });
                }
            }
            else {
                info!("odd person name{:?}, for {}", c.surname.clone(), &sd_sid)
            }
        }
    }


    // locations

    let mut db_locs: Vec<DBLocation> = Vec::new();

    if let Some(locs) = &s.centres {
        for loc in locs {
            db_locs.push(DBLocation { 
                facility: loc.name.clone(), 
                address: loc.address.clone(), 
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
          
            let mut specific = c.description.clone();
            
            // Move specific value into class 2 if class 2 empty

            let c2 = match c.disease_class2.clone() {
                Some(c) => Some(c),
                None => specific.clone(),  
            };

            // Drop specific if the same as class 2, whether from above or from the start

            if let Some(c) = &c2 && let Some(s) = &specific
               && c.trim().to_lowercase() == s.trim().to_lowercase() {
                specific = None;
            }

            db_conds.push ( DBCondition {
                class1: c.disease_class1.clone(),
                class2: c2,
                specific: specific,

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
            // text in brackets should beb appended to the text before
            
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

                        // very long entries in this field often ;mini-essays' and cannot be split
                    
                        if drug_names.contains("1.") && drug_names.contains("\n2.")
                        {
                            if let Some(dns) = get_cr_numbered_strings(&drug_names) {
                                for dn in dns {
                                    db_tops.push(DBTopic {
                                    source: source.clone(),
                                    topic_type: topic_type.clone(),
                                    value: dn.to_string(),
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
                                    value: dn.to_string(),
                                    });
                                }
                            }
                        }
                        else if drug_names.contains(',') {
                            
                            // if there are commas split on the commas (does not work for devices).

                            if int_type== "Drug" || int_type == "Supplement" {
                                
                                let sns = get_comma_delim_strings(&drug_names, 4); 
                                for sn in sns {
                                    db_tops.push(DBTopic {
                                    source: source.clone(),
                                    topic_type: topic_type.clone(),
                                    value: sn.to_string(),
                                    });
                                }
                            }
                        }
                        else
                        {
                            db_tops.push(DBTopic {
                                source: source,
                                topic_type: topic_type,
                                value: drug_names,
                            });
                        }
                    }
                    else {         // long entries
                        db_tops.push(DBTopic {
                                source: source,
                                topic_type: topic_type,
                                value: drug_names,
                            });
                    }
                }
            }
        }
    }

    // IE Criteria

    let mut db_iec: Vec<IECLine>= Vec::new();

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

    // process result codes to get overall iec status

    _iec_flag = inc_result + exc_result;  // to revisit!
   
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
    }

}

