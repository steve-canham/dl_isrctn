use crate::data_models::json_models::Study;
use crate::data_models::db_models::*;
use crate::helpers::import_helpers::*;
use crate::helpers::string_extensions::*;
use chrono::{NaiveDate, NaiveDateTime, Local}; 

//use crate::AppError;
//use log::info;

 
// The processor needs to creat a full DB version of each study's data,
// working on the whole of the data to present it in aa 'database ready' form.
// This form has singleton fields for the 1:1 study comonents (study summary, 
// dates and participants respectively), and vector fields for each of the 
// 1:n data types, that will be stored as separate tables.

#[allow(dead_code)]
pub fn process_study_data(s: &Study) -> DBStudy {
    
    let sd_sid =  s.sd_sid.clone();

    // process titles
    
    // Repair the cockup made in the doewwnload process;
    // These titles should stay as single option<String> fields.
    // or be processed as below within the download process.

    let mut pub_title: Option<String> = None;
    let mut sci_title: Option<String> = None;
    let mut acronym: Option<String> = None;

    for t in &s.titles {
        if t.title_type_id == 15 { pub_title = Some(t.title_value); }
        if t.title_type_id == 16 { sci_title = Some(t.title_value); }
        if t.title_type_id == 14 { acronym = Some(t.title_value); }
    }

    let mut db_ts: Vec<DBTitle> = Vec::new();
    let mut display_title = "No title provided".to_string();

    // should do full string clean

    if let Some(t) = pub_title{
        processed_pub_title = pub_title.clean_string();
        //match t.replace_apostrophes() {}
        display_title = t.replace_apostrophes(); // = public title, default
        db_ts.push(DBTitle {
                title_type_id: 15,
                title_text: t.to_string(),
                is_default: true,
                comment: Some("From ISRCTN".to_string()),
             });
    }

    if let Some(t) = sci_title{
        let scientific_title = t.to_string().replace_apostrophes(); 
        if display_title_opt.is_none() {
           display_title_opt = scientific_title.clone();
        }
        db_ts.push(DBTitle {
                title_type_id: 16,
                title_text: t.to_string(),
                is_default: display_title_opt == scientific_title,
                comment: Some("From ISRCTN".to_string()),
        });
    }


    if let Some(t) = acronym{
        let acronym = t.to_string().replace_apostrophes(); 
        if display_title_opt.is_none() {
           display_title_opt = acronym.clone();
        }
        db_ts.push(DBTitle {
                title_type_id: 14,
                title_text: t.to_string(),
                is_default: display_title_opt == acronym,
                comment: Some("From ISRCTN".to_string()),
        });
    }
    
    // Summary 
    
    // brief description
    // By default taken from the 'plain english summmary', as truncated during the
    // data download process. But if this is missing, or states it was not provided
    // at the time the record was created, a description is constructed from the 
    // study hypotheses and primary outcome fields.

    let description = match s.summary.plain_english_summary.clone() {
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
        let mut hypothesis = s.summary.study_hypothesis.clone();
        let mut poutcome = s.summary.primary_outcome.clone();

        // add string_clean ******************************************************************************************8
        // string hypothesis = r.studyHypothesis.StringClean() ?? "";
        // string poutcome = r.primaryOutcome.StringClean() ?? "";

        if let Some(h) = hypothesis {
           if !h.to_lowercase().starts_with("not provided") {
               if !h.to_lowercase().starts_with("hypothes") && !h.to_lowercase().starts_with("study hyp")
               {
                    hypothesis =  Some(format!("Study hypothesis: {}", h));
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
        None => "No description text provided".to_string,
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
    
    let iec_flag = 0;   // for now

    let date_last_revised = match &s.registration.last_updated {
        Some(ds) => match NaiveDate::parse_from_str(&ds.clone(), "%Y-%m-%d"){
            Ok(d) =>Some(d),
            Err(_) => None,
        },
        None => None
    };
    let dt_of_data_fetch = NaiveDateTime::parse_from_str(&s.downloaded, "%Y-%m-%dT%H:%M:%S").unwrap();
    
    /*
        s.brief_description = r.plainEnglishSummary;
        if (string.IsNullOrEmpty(s.brief_description) 
            || s.brief_description.ToLower().StartsWith("not provided"))
        {
            string hypothesis = r.studyHypothesis.StringClean() ?? "";
            string poutcome = r.primaryOutcome.StringClean() ?? "";
            if (hypothesis != "" && !hypothesis.ToLower().StartsWith("not provided"))
            {
                if (!hypothesis.ToLower().StartsWith("hypothes") && !hypothesis.ToLower().StartsWith("study hyp"))
                {
                    hypothesis = "Study hypothesis: " + hypothesis;
                }
                s.brief_description = hypothesis;
            }
            if (poutcome != "" && !poutcome.ToLower().StartsWith("not provided"))
            {
                if (!poutcome.ToLower().StartsWith("primary") && !poutcome.ToLower().StartsWith("outcome"))
                {
                    poutcome = "Primary outcome: " + poutcome;
                }
                s.brief_description += s.brief_description == "" ? poutcome : "\n" + poutcome;
            }
        }
     */

    


    let summary = DBSummary {
        display_title: display_title,
        brief_description: description_text,
        type_id: get_study_type(&s.design.primary_study_design),
        status_id: get_study_status(&status_opt),
        status_override: s.recruitment.recruitment_status_override.clone(),
        start_status_override: s.recruitment.recruitment_start_status_override.clone(),
        iec_flag: iec_flag,
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
    if let Some(ids) = &s.identifiers {
        for id in ids {
            db_ids.push(DBIdentifier { 
                id_value: id.identifier_value.clone(), 
                id_type_id: id.identifier_type_id, 
                id_type: id.identifier_type.clone(),
            })
        }
    }

    DBStudy {

        sd_sid: sd_sid,
        summary: summary,
        dates: dates,
        participants: participants,
        titles: option_from_count(db_ts),
        identifiers: option_from_count(db_ids),

    }

}

