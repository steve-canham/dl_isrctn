use crate::data_models::json_models::*;
use crate::data_models::db_models::*;
use crate::helpers::string_extensions::*;
use crate::helpers::name_extensions::*;
use crate::iec::iec_fns::*;
use crate::iec::iec_structs::IECLine;

use super::support_fns::*;
use chrono::{NaiveDate, NaiveDateTime, Utc, Datelike};
use std::sync::LazyLock;
use regex::Regex;
use log::info;

// The processor needs to creat a full DB version of each study's data,
// working on the whole of the data to present it in aa 'database ready' form.
// This form has singleton fields for the 1:1 study comonents (study summary,
// dates and participants respectively), and vector fields for each of the
// 1:n data types, that will be stored as separate tables.

pub fn process_study_data(s: &Study) -> DBStudy {

    let sd_sid =  s.sd_sid.clone();

    // process titles

    let pub_title = s.public_title.clone().clean();
    let sci_title = s.scientific_title.clone().clean();
    let acronym = s.acronym.clone().clean();

    let (db_titles, display_title) = derive_titles(&pub_title, &sci_title, &acronym);

    // Summary

    // Brief description.
    // By default taken from the 'plain english summmary', as truncated and cleaned
    // during the data download process. But if this is missing, or states it was
    // not provided at the time the record was created, a description is constructed from the
    // study hypotheses and primary outcome fields.

    let description = match s.summary.plain_english_summary.clone() {
        Some (s) if !s.to_lowercase().starts_with("not provided") => Some(s),
        _ => {
            let hypothesis = s.summary.study_hypothesis.clone().clean_multiline();
            let poutcome = s.summary.primary_outcome.clone().clean_multiline();
            derive_description(&hypothesis, &poutcome)
        },
    };

    let description_text = match description {  // Then extract the text or use a default value
        Some(d) => d,
        None => "No description text provided".to_string(),
    };

    // Study status - Sometimes from override study / study start status but more commonly from dates.
    // Means periodic full import or a separate mechanism to update statuses against dates.
    // It appears that all relevant dates are always available.
    // Need to check later for results being published, in which cases ensure status is completed

    let status_override = s.recruitment.recruitment_status_override.clone();
    let start_status_override = s.recruitment.recruitment_start_status_override.clone();

    let status_string = match &status_override {
        Some(sov) if sov == "Stopped" => "Terminated",
        Some(sov) => sov,  // usually 'Suspended'

        _ => match &start_status_override {
            Some(ssov) if ssov == "Anticipated" => "Not yet recruiting",
            Some(ssov) => ssov,    // Not clear if this exists

            _ => {     // No status overrides - use dates

                let se_date = date_from_iso_string(s.summary.overall_end_date.clone());
                let rs_date = date_from_iso_string(s.recruitment.recruitment_start.clone());
                let re_date = date_from_iso_string(s.recruitment.recruitment_end.clone());
                let today = Utc::now().date_naive();

                match se_date {
                    Some(sed) if sed <= today => "Completed",    // end date already passed
                    _ => match rs_date {                         // no end date or end date in future
                        Some(rsd) if rsd > today => "Not yet recruiting",        // resruitment start in the future
                        Some(_) => match re_date {                               // recruitment started....
                                Some(red) if red <= today =>  "Active, not recruiting",  // recruitment started and ended
                                Some(_) => "Recruiting",                                  // recruitment started, not yet ended
                                _ => "Ongoing, recruitment status unclear",               // recruitment started, no end date given
                            },
                        _ => "Unknown"                          // No recruitment start date and study not completed
                    }
                }
            },
        },
    };
    let status_opt = if status_string == "" {None} else {Some(status_string.to_string())};

    let type_id = get_study_type(&s.design.primary_study_design);

    let date_last_revised = match &s.registration.last_updated {
        Some(ds) if ds.len() > 10 =>
            match NaiveDate::parse_from_str(&ds[..10], "%Y-%m-%d") {
            Ok(d) => Some(d),
            Err(_) => None,
        },
        _ => None,
    };

    let dt_of_data_fetch = NaiveDateTime::parse_from_str(&s.downloaded, "%Y-%m-%dT%H:%M:%S").unwrap();

    let summary = DBSummary {
        display_title: display_title,
        brief_description: description_text,
        type_id: type_id,
        status_id: get_study_status(&status_opt),
        status_override: status_override,
        start_status_override: start_status_override,
        is_ipd_sharing: s.ipd.ipd_sharing_plan,
        ipd_sharing_plan: s.ipd.ipd_sharing_statement.clone(),
        date_last_revised: date_last_revised,
        dt_of_data_fetch: dt_of_data_fetch,
    };

    // dates

    let (reg_year, reg_month, reg_date_type) = split_date_string(s.registration.date_id_assigned.clone());
    let (start_year, start_month, start_date_type) = split_date_string(s.recruitment.recruitment_start.clone());
    let (comp_year, comp_month, comp_date_type) = split_date_string(s.summary.overall_end_date.clone());
    let (res_year, res_month, res_date_type) = split_date_string(s.results.intent_to_publish.clone());

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

    let (enrolment, enrolment_type) = match s.recruitment.total_final_enrolment.clone() {
        Some(f) if f != "0" =>  (Some(f), Some("a".to_string())),
        _ => {
            match s.recruitment.target_enrolment.clone() {
                Some(t) if t != "0" => (Some(t), Some("e".to_string())),
                _ => (None, None),
            }
        },
    };

    let gender_flag = match s.participants.gender.clone() {
        Some(gf) if gf.to_lowercase().starts_with("not") => None,
        Some(gf) => Some(gf.to_lowercase().chars().next().unwrap().to_string()),  // "a", "m" or "f"
        _ => None,
    };

    let min_age_as_string = s.participants.l_age_limit.clone();
    let (min_age, min_age_units, min_num_days) = split_age_string(&min_age_as_string);

    let max_age_as_string = s.participants.u_age_limit.clone();
    let (max_age, max_age_units, max_num_days) = split_age_string(&max_age_as_string);

    let age_group_flag = derive_age_group_flag(min_num_days, max_num_days);

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
    // Then the secondary ids already identified in the data download

    db_ids.push(DBIdentifier {
        id_value: sd_sid.clone(),
        id_type_id: 126,
        id_type: "ISRCTN ID".to_string(),
    });

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

    let db_orgs = derive_orgs(&sd_sid, &s.sponsors, &s.funders);

    // Contacts

    let db_peop = derive_contacts(&s.contacts);

    // Countries

    let db_countries = s.countries.clone();  // both Option<Vec<String>>

    // Conditions

    let db_conds = derive_conditions(&s.conditions);

    // Features

    let secondary_design = &s.design.secondary_study_design.clone().unwrap_or("".to_string());
    let study_design = &s.design.study_design.clone().unwrap_or("".to_string());
    let design = format!("{} {}", secondary_design, study_design).trim().to_lowercase();

    let db_feats = derive_features(type_id, &s.interventions, &design, &s.trial_types);

    // Topics

    let db_tops = derive_topics(&s.interventions);

    // IE Criteria

    let incs = &s.participants.inclusion.clean_multiline();
    let excs = &s.participants.exclusion.clean_multiline();
    let (db_iec, iec_flag) = derive_iec(&sd_sid, incs, excs);
    participants.iec_flag =  iec_flag;

    // Outputs and objects

    // First add the ISCRTN web page as an object

    let mut db_objects: Vec<DBObject> = Vec::new();

    let date_created_string = s.registration.date_id_assigned.clone();
    let reg_year = match &date_created_string {
        Some(s) if s.len() > 3 =>  {
            match s[0..4].parse::<i32>()
            {
                Ok(n) => Some(n),
                Err(_e) => None,
            }
        },
        _ => None,
    };

    let sid = sd_sid.replace("ISRCTN", "isrctn-");  // use as the sd_sid in object Ids

    db_objects.push(DBObject {
        object_type: "Trial Registry Entry".to_string(),
        object_id: Some(format!("{}/trial_reg_entry", &sid)),
        object_id_type: Some("Constructed, sid/object type".to_string()),
        display_name: Some("ISRCTN Registry Entry".to_string()),
        date_created: date_created_string.as_date_opt(),
        date_published: date_created_string.as_date_opt(),
        date_updated: s.registration.last_updated.clone().as_date_opt(),
        publication_year: reg_year,
        object_notes: Some("".to_string()),
        access_url: Some(format!("https://www.isrctn.com/{}", &sd_sid)),
        access_type: Some("public".to_string()),
        url_target_type: Some("web page".to_string()),
        instance_notes: Some("Data also available as XML via ISRCTN API".to_string()),
    });

    // Add any objects from Files

    db_objects.append(&mut derive_files(&s.files, &sid));

    let (mut link_objects, link_pubs, link_pub_instances) = process_links(&s.links, &sid);

    // Add any objects from Links
    
    db_objects.append(&mut link_objects);

    DBStudy {

        sd_sid: sd_sid,
        summary: summary,
        dates: dates,
        participants: participants,
        titles: db_titles,
        identifiers: db_ids,
        orgs: db_orgs,
        people: db_peop,
        countries: db_countries,
        conditions: db_conds,
        features: db_feats,
        topics: db_tops,
        ie_crit: db_iec,
        objects: option_from_count(db_objects),
        publications: option_from_count(link_pubs),
        pub_instances: option_from_count(link_pub_instances)
    }
}


fn derive_titles(pub_title: &Option<String>,sci_title: &Option<String>,acronym: &Option<String>,)
                                                -> (Option<Vec<DBTitle>>, String) {

    // Obtain the strings, lower case strings for comparisons,
    // and an integer indicator of presence for each title type.

    let (pt, pt_lc, mut pt_val) = match pub_title {
        Some(p) => (p.clone(), p.to_lowercase(), 2),
        None => ("".to_string(), "".to_string(), 0),
    };
    let (st, st_lc, mut st_val) = match sci_title {
        Some(s) => (s.clone(), s.to_lowercase(), 4),
        None => ("".to_string(), "".to_string(), 0),
    };
    let (at, at_lc, mut at_val) = match acronym {
        Some(a) => (a.clone(), a.to_lowercase(), 8),
        None => ("".to_string(), "".to_string(), 0),
    };

    // Amend the values associated with titles to reflect
    // the same title being provided in different roles.

    let mut db_ts: Vec<DBTitle> = Vec::new();

    if pt_val > 0 && st_val > 0 && pt_lc == st_lc {   // the commonest situation
        pt_val += 4;
        st_val = 0;
    }
    if pt_val > 0 && at_val > 0 && pt_lc == at_lc {
        pt_val += 8;
        at_val = 0;
    }
    if st_val > 0 && at_val > 0 && st_lc == at_lc {
        st_val += 8;
        at_val = 0;
    }

    // Finally generate the structs corresponding to the three possible titles

    let mut display_title = "".to_string();
    if pt_val > 0 {
        let mut dbt = DBTitle {
            title_text: pt.clone(),
            is_public: true,
            is_scientific: false,
            is_acronym: false,
            is_display: true,
            comment: Some("From ISRCTN".to_string()),
        };
        if pt_val > 2 {
            if pt_val == 6 {
                dbt.is_scientific = true;
            }
            if pt_val >= 10 {
                dbt.is_acronym = true;
            }
        }
        display_title = pt;
        db_ts.push(dbt);
    }

    if st_val > 0 {
        let mut dbt = DBTitle {
            title_text: st.clone(),
            is_public: false,
            is_scientific: true,
            is_acronym: false,
            is_display: false,
            comment: Some("From ISRCTN".to_string()),
        };
        if st_val > 4 {
            dbt.is_acronym = true;
        }
        if pt_val == 0 {
            dbt.is_display = true;
            display_title = st;
        }
        db_ts.push(dbt);
    }

    if at_val > 0 {           // acronym must be different from other titles supplied
        let mut dbt = DBTitle {
            title_text: at.clone(),
            is_public: false,
            is_scientific: false,
            is_acronym: true,
            is_display: false,
            comment: Some("From ISRCTN".to_string()),
        };
        if pt_val == 0 && st_val == 0 {    // would be very unusual
            dbt.is_display = true;
            display_title = at;
        }
        db_ts.push(dbt);
    }

    (option_from_count(db_ts),display_title)
}


fn derive_description(hypothesis: &Option<String>, poutcome: &Option<String>) -> Option<String> {

    let hyp = match hypothesis {
        Some(h) if !h.to_lowercase().starts_with("not provided") => {
            if !h.to_lowercase().starts_with("hypothes") && !h.to_lowercase().starts_with("study hyp")
            {
                Some(format!("Study hypothesis: {}", h))
            }
            else {
                Some(h.to_string())
            }
        },
        _ => None,
    };

    let pout = match poutcome {
        Some(p) if !p.to_lowercase().starts_with("not provided") => {
            if !p.to_lowercase().starts_with("primary") && !p.to_lowercase().starts_with("outcome")
            {
                Some(format!("Primary outcome: {}", p))
            }
            else {
                Some(p.to_string())
            }
        },
        _ => None,
    };

    // Combine the two, if they both exist, or just use one

    match hyp {
        Some(hy) => {
            match pout {
                    Some (po) => Some(format!("{}\n{}", hy, po)),
                    None => Some(hy),
            }
        },
        None => pout,
    }
}


fn derive_features(type_id: i32, interventions: &Option<Vec<Intervention>>, design: &String,
                                trial_types: &Option<Vec<String>>) -> Option<Vec<DBFeature>> {

    let mut db_feats: Vec<DBFeature>= Vec::new();

    if type_id == 11 {
        if let Some(ints) = interventions {
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

        if *design != "".to_string()
        {
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
    }

    if  type_id == 12 && *design != "".to_string() {    // observational study

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

    if let Some(tts) = trial_types {
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

     option_from_count(db_feats)
}


fn derive_age_group_flag(min_num_days: f64, max_num_days: f64) -> i32 {
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
    age_group_flag
}


fn derive_contacts(contacts: &Option<Vec<StudyContact>>) -> Option<Vec<DBPerson>> {

    let mut db_peop: Vec<DBPerson> = Vec::new();

    if let Some(contacts) = contacts {
        for c in contacts {
            if c.surname.appears_plausible_person_name() {
                if let Some(cts) = &c.contact_types {
                    let mut roles: Vec<String> = Vec::new();
                    for ct in cts {
                        let role_to_add = match ct.to_lowercase().as_str() {
                            "principal investigator" => "Principal Investigator",
                            "scientific" => "Scientific contact",
                            _ => ""
                        };
                        if role_to_add != "" {
                            roles.push(role_to_add.to_string());
                        }
                    }
                    if roles.len() > 0 {
                        db_peop.push(DBPerson {
                            full_name: get_full_name(c.forename.clone(), c.surname.clone()),
                            listed_as: Some(roles.join(", ")),
                            orcid_id: c.orcid.tidy_orcid(),
                            affiliation: c.address.clone(),
                            email_domain: c.email.extract_domain(),
                        });
                    }
                }
            }
        }
    }
    option_from_count(db_peop)
}


fn derive_orgs (sd_sid: &String, sponsors: &Option<Vec<StudySponsor>>,
                funders: &Option<Vec<StudyFunder>>) -> Option<Vec<DBOrganisation>> {

    let mut db_orgs: Vec<DBOrganisation> = Vec::new();

    // sponsors  // ? TidyOrgName(sid);

    if let Some(sponsors) = sponsors {
        for s in sponsors {
            if s.organisation.appears_plausible_org_name() {
                let sname = s.organisation.clean().tidy_org_name(sd_sid);
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
            else {
               info!("odd org sponsor name{:?}, for {}", s.organisation.clone(), &sd_sid)
            }
        }
    }

    // funders

    if let Some(funders) = funders {
        for f in funders {
            if f.name.appears_plausible_org_name() {
                let fname = f.name.clean().tidy_org_name(sd_sid);
                let mut duplicated = false;   // See if that name has been used before as a sponsor.
                for dbo in &mut db_orgs {
                    if let Some(true) = dbo.is_sponsor {
                        if dbo.org_name == fname {       // Add contribution type
                            dbo.is_funder = Some(true);
                            dbo.org_cref_id = f.fund_ref.clone();
                            duplicated = true;
                            break;
                        }
                    }
                }

                if !duplicated   // Add as a separate funder.
                {
                    db_orgs.push(DBOrganisation {
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
               info!("odd org funder name{:?}, for {}", f.name.clone(), &sd_sid)
            }
        }
    }

    option_from_count(db_orgs)
}


fn derive_conditions(conds: &Option<Vec<Condition>>) -> Option<Vec<DBCondition>>{

    let mut db_conds: Vec<DBCondition>= Vec::new();
    if let Some(cons) = conds {
        for c in cons {
            let (c2, desc) = match c.description.clone() {
                Some(ds) if ds.starts_with("Topic:") => {
                    let mut c2_section: Option<String> = None;
                    let mut desc_section: Option<String> = None;

                    let ds_3parts: Vec<&str> = ds.split(';').collect();
                    if ds_3parts.len() == 3 {
                        let mut c2_sec = ds_3parts[1].trim().to_string();
                        let mut desc_sec = ds_3parts[2].trim().to_string();
                        if c2_sec.starts_with("Subtopic:") {
                            c2_sec = c2_sec[9..].trim().to_string();
                        }
                        if desc_sec.starts_with("Disease:") {
                            desc_sec = desc_sec[8..].trim().to_string();
                        }
                        c2_section = Some(c2_sec);
                        desc_section = Some(desc_sec);
                    }

                    if ds_3parts.len() == 1 {
                        let ds_2parts: Vec<&str> = ds.split('/').collect();
                        if ds_2parts.len() == 2 {
                            c2_section = Some(ds_2parts[1].trim().to_string());  // why None for desc - needs checking
                        }
                    }

                    (c2_section, desc_section)
                },
                Some(ds) => {      // A description present but 'normal', not 'Topic:...
                    let c2 = c.disease_class2.clone();
                    if let Some(c) = &c2    // If desc is the same as c2, make it None
                        && c.trim().to_lowercase() == ds.trim().to_lowercase() {
                        (c2, None)
                    }
                    else {
                        (c2, Some(ds))
                    }
                },
                None => (c.disease_class2.clone(), None),   // No description present, just leave c2 'as is'
            };

            db_conds.push ( DBCondition {
                class1: c.disease_class1.clone(),
                class2: c2,
                specific: desc,
            });
        }
    }
    option_from_count(db_conds)
}


fn derive_topics(interventions: &Option<Vec<Intervention>>, ) -> Option<Vec<DBTopic>> {

    let mut db_tops: Vec<DBTopic>= Vec::new();

    if let Some(intervs) = interventions {
        for interv in intervs {

            let topic_type = match interv.int_type.clone() {
                Some(t) if t == "Device".to_string() => t,
                _ => "Chemical / agent".to_string(),  // the default
            };

            if let Some(mut dn) = interv.drug_names.clone().clean() {
                let dn_lc = dn.to_lowercase();
                if !dn_lc.starts_with("the sponsor has confirmed")
                && !dn_lc.starts_with("the health research authority")
                && !dn_lc.starts_with("not provided")
                {
                    let source = dn.clone();  // keep copy of the original data
                    dn = dn.replace("\u{00AE}", ""); //  lose (r) Registration mark
                    dn = dn.replace("\u{2122}", ""); //  lose (tm) Trademark mark
                    dn = dn.replace("[", "(").replace("]", ")"); //  regularise brackets
                    dn = dn.replace(" and ", ", "); // in most cases indicates end of list

                    if dn.len() < 250 {    // very long entries in this field often 'mini-essays' and cannot be split

                        if dn.contains("1.") && dn.contains("\n2.")
                        {
                            let dns = get_cr_numbered_strings(&dn);
                                for dn in dns {
                                    db_tops.push(DBTopic {
                                    source: source.clone(),
                                    topic_type: topic_type.clone(),
                                    topic_value: dn.to_string(),
                                    });
                                }
                        }
                        else if dn.contains("1. ") && dn.contains("2. ")
                        {
                            let dns = get_numbered_strings(&dn);
                                for dn in dns {
                                    db_tops.push(DBTopic {
                                    source: source.clone(),
                                    topic_type: topic_type.clone(),
                                    topic_value: dn.to_string(),
                                    });
                                }
                        }
                        else if dn.contains(',') {

                            // if there are commas split on the commas (does not work for devices).
                            // though there is an issue of commas in brackets

                            if topic_type != "Device".to_string() {
                                let sns = get_comma_delim_strings(&dn, 4);
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
                            db_tops.push(DBTopic {    // no numbers or commas
                                source: source,
                                topic_type: topic_type,
                                topic_value: dn,
                            });
                        }
                    }
                    else {      // long topic entries
                        db_tops.push(DBTopic {
                            source: source,
                            topic_type: topic_type,
                            topic_value: dn,
                        });
                    }
                }
            }
        }
    }
    option_from_count(db_tops)
}


fn derive_iec(sd_sid: &String, incs: &Option<String>, excs: &Option<String>) -> (Option<Vec<IECLine>>, i32) {

    let mut db_iec: Vec<IECLine> = Vec::new();
    let mut inc_result = 0;
    let mut exc_result = 0;

    if incs.is_not_a_place_holder() {
        if let Some(inc_para) = incs {
            let (inc_result_code, mut inc_criteria) = original_process_iec(sd_sid, &inc_para, "inclusion");
            inc_result = inc_result_code;

            if inc_criteria.len() > 0 {
                db_iec.append(&mut inc_criteria);
            }
        }
    }

    if excs.is_not_a_place_holder() {
        if let Some(exc_para) = excs {
            let (exc_result_code, mut exc_criteria) = original_process_iec(sd_sid , &exc_para, "exclusion");
            exc_result = exc_result_code;

            if exc_criteria.len() > 0 {
                db_iec.append(&mut exc_criteria);
            }
        }
    }

    let iec_flag =  inc_result + exc_result;

    (option_from_count(db_iec),iec_flag)
}


fn derive_files(study_files: &Option<Vec<StudyFile>>, sid: &String) -> Vec<DBObject> {

    let mut db_objects: Vec<DBObject> = Vec::new();
    if let Some(files) = study_files{
        for f in files {

            let file_type_string = f.file_type.clone().unwrap_or_default().to_lowercase();
            let (object_type, object_type_in_id) = match file_type_string.as_str() {
                "abstract" => ("Abstract","abstract"),
                "hrasummary" => ("HRA Summary", "hra_summary"),
                "trialwebsite" => ("Trial Website", "trial_website"),
                "protocolarticle" => ("Protocol Article", "protocol_article"),
                "protocolpreprint" => ("Protocol Preprint", "protocol_preprint"),
                "thesis" => ("Thesis", "thesis"),
                "protocolother" => ("Protocol (other format)", "protocol_other"),
                "sap" => ("SAP", "sap"),
                "otherfiles" => ("Other file", "other_file"),
                "interimresults" => ("Interim Results", "interim_esults"),
                "otherunpublished" => ("Other Unpublished", "other_unpublished"),
                "protocolfile" => ("Protocol File", "protocol_file"),
                "otherpublications" => ("Other Publication", "other_publication"),
                "poster" => ("Poster", "poster"),
                "dataset" => ("Dataset", "dataset"),
                "basicresults" => ("Basic Results", "basic_results"),
                "resultsarticle" => ("Results Article", "results_article"),
                "preprint" => ("Preprint", "preprint"),
                "funderreport" => ("Funder Report", "funder_report"),
                "plainenglishresults" => ("Plain English Results", "plain_english_results"),
                "pis" => ("PIS", "pis"),
                _ => ("?", "?"),
            };

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

            let publication_year = match upload_dt {      // try upload date first
                Some(ud) => Some(ud.year()),
                None => match creation_dt {     // creation date if no upload date
                    Some (cd) => Some(cd.year()),
                    None => None,
                }
            };

            let mut object_notes: Option<String> = None;
            if let Some(mut d) = f.description.clone() {
                d = d.trim().trim_matches(':').to_string();
                if d.to_lowercase() != object_type.to_lowercase() {
                    object_notes = Some(capitalize_first(&d));
                }
            }

            // derive name and instance notes

            let file_size = match f.length {
                Some(ln) if ln > 0 => {
                    let kb_num: f64 = ln as f64 / 1024.0;
                    if kb_num >= 1024.0 {
                        let mb_num = kb_num/ 1024.0;
                        format!("{:.2}Mb", mb_num)
                    }
                    else {
                        format!("{:.0}Kb", kb_num)
                    }
                },
                _ => "".to_string(),
            };

            let (file_type, file_ending) = match f.mime_type.clone() {
                Some(mut m_type) if m_type.trim() != "" => {
                    m_type = m_type.replace("application/", "");
                    match m_type.trim() {
                        "pdf" => ("PDF", ".pdf"),
                        "x-zip-compressed" => ("ZIP", ".zip"),
                        "msword" => ("DOC", ".doc"),
                        "vnd.openxmlformats-officedocument.wordprocessingml.document" => ("DOCX", ".docx"),
                        "vnd.openxmlformats-officedocument.spreadsheetml.sheet" => ("XSLX", "xslx"),
                        "csv" => ("CSV", ".csv"),
                        "x-spss-sav" => ("SPSS SAV", ".sav"),
                        _ => ("???", ".???"),
                    }
                },
                _ => ("", "")
            };

            let instance_notes = if file_type != "" {
                if file_size != "" {
                    Some(format!("{} file, {}", file_type, file_size))
                }
                else {
                    Some(format!("{} file", file_type))
                }
            }
            else {
                None
            };

            let ver = match f.version.clone() {
                Some(v) if v.to_lowercase().contains('v') => v.trim().to_string(),
                Some(v) if v.trim() != "" => format!("v{}", v.trim()),
                _ => "".to_string()
            };

            let (display_name, object_id, object_id_type) = match f.download_filename.clone() {
                Some(dlfn) if dlfn != "".to_string() => {
                    let mut dfn = dlfn.clone();
                    if file_ending != "" {
                        dfn = dfn.replace(file_ending, "");
                    }
                    if !dfn.to_lowercase().contains(&ver) {
                    dfn = format!("{}.{}", dlfn, ver);
                    }
                    (Some(dlfn), Some(format!("{}/{}", sid, dfn)), Some("Constructed from name".to_string()))
                },
                _ => (None, Some(format!("{}/{}", sid, object_type_in_id)), Some("Constructed from type".to_string())),
            };

            db_objects.push(DBObject {
                object_type: object_type.to_string(),
                object_id: object_id,
                object_id_type: object_id_type,
                display_name: display_name,
                date_created: creation_dt,
                date_published: upload_dt,
                date_updated: None,
                publication_year: publication_year,
                object_notes: object_notes,
                access_url: f.download_url.clone(),
                access_type: Some("public".to_string()),
                url_target_type: Some("file download".to_string()),
                instance_notes: instance_notes,
            });
        }
    }
    db_objects
}


fn process_links(study_links: &Option<Vec<StudyLink>>, sid: &String) -> (Vec<DBObject>, Vec<DBPublication>, Vec<DBPublicationInstance>) {

    let mut db_objects: Vec<DBObject> = Vec::new();
    let mut db_pubs: Vec<DBPublication> = Vec::new();
    let mut db_pub_instances: Vec<DBPublicationInstance> = Vec::new();

    if let Some(links) = study_links {
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

            let publication_year = match upload_dt {      // try upload date first
                Some(ud) => Some(ud.year()),
                None => match creation_dt {     // creation date if no upload date
                    Some (cd) => Some(cd.year()),
                    None => None,
                }
            };

            let mut external_url = lk.link_url.clone().unwrap_or_else(|| "".to_string());
            external_url = external_url.replace("http://", "https://");    // regularise url scheme
            external_url = external_url.replace("doi.org10.", "doi.org/10.");  // repair needed rarely

            let link_type_string = lk.link_type.clone().unwrap_or_default().to_lowercase();
            let (object_type, object_type_in_id) = match link_type_string.as_str() {
                "abstract" => ("Abstract","abstract"),
                "hrasummary" => ("HRA Summary", "hra_summary"),
                "trialwebsite" => ("Trial Website", "trial_website"),
                "protocolarticle" => ("Protocol Article", "protocol_article"),
                "protocolpreprint" => ("Protocol Preprint", "protocol_preprint"),
                "thesis" => ("Thesis", "thesis"),
                "protocolother" => ("Protocol (other format)", "protocol_other"),
                "sap" => ("SAP", "sap"),
                "otherfiles" => ("Other file", "other_file"),
                "interimresults" => ("Interim Results", "interim_esults"),
                "otherunpublished" => ("Other Unpublished", "other_unpublished"),
                "protocolfile" => ("Protocol File", "protocol_file"),
                "otherpublications" => ("Other Publication", "other_publication"),
                "poster" => ("Poster", "poster"),
                "dataset" => ("Dataset", "dataset"),
                "basicresults" => ("Basic Results", "basic_results"),
                "resultsarticle" => ("Results Article", "results_article"),
                "preprint" => ("Preprint", "preprint"),
                "funderreport" => ("Funder Report", "funder_report"),
                "plainenglishresults" => ("Plain English Results", "plain_english_results"),
                "pis" => ("PIS", "pis"),
                _ => ("?", "?"),
            };
            
            let mut object_notes: Option<String> = None;
            if let Some(mut d) = lk.description.clone() {
                d = d.trim().trim_matches(':').to_string();
                if d.to_lowercase() != object_type.to_lowercase() {
                    object_notes = Some(capitalize_first(&d));
                }
            }

            let mut is_pub = false;
            if object_type_in_id.contains("article")
                || object_type_in_id.contains("preprint")
                || object_type_in_id.contains("abstract")
                || object_type_in_id == "interim_results"
                || object_type_in_id == "other_publications"
            && !(external_url.ends_with("pdf")
                || external_url.ends_with("zip")
                || external_url.ends_with("csv")
                || external_url.ends_with("xlsx")
                || external_url.ends_with("docx")) {
                is_pub = true;
            }

            if is_pub {

                let low_url =  external_url.to_lowercase();
                let mut categorised = false;
                
                let mut pub_id = None;
                let mut pub_id_type = None;
                let mut instance_type = None;
                let mut instance_id = None;
                let mut instance_notes = None;
                let mut access_url =  None;
                let mut access_type =  None;
                let mut url_target_type =  None;
                
                static RE_PM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{6,8}").unwrap());
                static RE_PMC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"PMC[0-9]{6,7}").unwrap());

                if low_url.contains("pubmed") { // extract pmid and redo the url to ensure in current form
                    match RE_PM.captures(&external_url.clone()) {
                        Some(s) => {
                            pub_id = Some(s[0].to_string().clone());
                            pub_id_type = Some("PMID".to_string());
                            
                            instance_type = Some("PubMed entry".to_string());
                            instance_id = Some(s[0].to_string().clone());
                            instance_notes = None;
                            
                            access_url =  Some(format!("https://pubmed.ncbi.nlm.nih.gov/{}/", s[0].to_string()));
                            access_type =  Some("public".to_string());
                            url_target_type =  Some("web page".to_string());
                            categorised = true;
                        },
                        None => {},
                    }
                }
                
                if low_url.contains("doi") && !categorised {   // store doi

                    // get substring from the beginning of the 10.

                    let mut doi = String::new();
                    if low_url.starts_with("https://dx.doi.org/10.") {
                        doi = (&low_url[19..]).to_string();
                    }
                    if low_url.starts_with("https://doi.org/10.") {
                        doi = (&low_url[16..]).to_string();
                    }
                    if low_url.starts_with("https://www.doi.org/10.") {
                        doi = (&low_url[20..]).to_string();
                    }
                    if !doi.is_empty() {
                        pub_id = Some(doi.clone());
                        pub_id_type = Some("doi".to_string());
                        
                        instance_type = Some("Publisher's abstract or full text".to_string());
                        instance_id = Some("doi".to_string());
                        instance_notes = None;
                        
                        access_url =  Some(format!("https://doi.org/{}", doi));
                        access_type =  Some("public".to_string());
                        url_target_type =  Some("web page".to_string());
                        categorised = true;
                    }
                }

                if low_url.contains("pmc") && !categorised {

                    // extract pmc id and redo the url to ensure in current form

                    match RE_PMC.captures(&external_url.clone()) {
                        Some(s) => {
                            pub_id = Some(s[0].to_string().clone());
                            pub_id_type = Some("PMC ID".to_string());
                            
                            instance_type = Some("PubMed Central full text".to_string());
                            instance_id = Some(s[0].to_string().clone());
                            instance_notes = None;
                            access_url =  Some(format!("https://pmc.ncbi.nlm.nih.gov/articles/{}/", s[0].to_string()));
                            access_type =  Some("public".to_string());
                            url_target_type =  Some("web page with download".to_string());
                            categorised = true;
                        },
                        None => {},
                    }
                }

                if !categorised {  // probably a publisher's web site URL
        
                    pub_id = Some(external_url.clone());
                    pub_id_type = Some("url".to_string());
                    
                    instance_type = Some("Publisher's abstract or full text".to_string());
                    instance_id = Some(external_url.clone());
                    instance_notes = None;
                    
                    access_url =  Some(external_url.clone());
                    access_type =  Some("public".to_string());
                    url_target_type =  Some("web page".to_string());
                }

                db_pubs.push(DBPublication {
                    pub_type: Some(object_type.to_string()),
                    pub_id: pub_id.clone(),
                    pub_id_type: pub_id_type,
                    pub_notes: object_notes,
                    date_created: creation_dt,
                    date_published: upload_dt,
                    date_updated: None,
                    publication_year: publication_year, 
                });

                db_pub_instances.push(DBPublicationInstance {
                    pub_id: pub_id,
                    instance_type: instance_type,
                    instance_id: instance_id,
                    instance_lang: Some("en".to_string()),
                    instance_notes: instance_notes,
                    access_url: access_url,
                    access_type: access_type,
                    url_target_type: url_target_type,
                });
            }
            else {

                // A non publication link,
                // may be to a web page or a file.

                let instance_type: Option<String>;
                if external_url.ends_with("pdf")
                    || external_url.ends_with("zip")
                    || external_url.ends_with("csv")
                    || external_url.ends_with("xlsx")
                    || external_url.ends_with("docx")
                    || external_url.ends_with("xls")
                    || external_url.ends_with("doc") {    // url target is a file

                    instance_type = Some("File download".to_string());
                    if external_url.contains("articles/PMC") {  // Some correction required after web site updated
                        external_url = external_url.replace("articles/PMC", "articles/instance/");
                    }
                }
                else {
                    instance_type = Some("Web page".to_string());
                }

                db_objects.push(DBObject {
                    object_type: object_type.to_string(),
                    object_id: Some(format!("{}/{}", sid, object_type_in_id)),
                    object_id_type: Some("Constructed from type".to_string()),
                    display_name: None,
                    date_created: creation_dt,
                    date_published: upload_dt,
                    date_updated: None,
                    publication_year: publication_year,
                    object_notes: object_notes,
                    access_url: if external_url != "".to_string() {Some(external_url)} else {None},
                    access_type: Some("public".to_string()),
                    url_target_type: instance_type,
                    instance_notes: None,
                });
            }
        }
    }
    (db_objects, db_pubs, db_pub_instances)

}
