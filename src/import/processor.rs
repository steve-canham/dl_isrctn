use crate::data_models::json_models::Study;
use crate::data_models::db_models::*;
use crate::helpers::import_helpers::*;
use crate::helpers::string_extensions::*;
use crate::helpers::name_extensions::*;
use chrono::{NaiveDate, NaiveDateTime, Local}; 

//use crate::AppError;
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
    
    let iec_flag = 0;   // for now

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
        
    if let Some(sponsors) = &s.sponsors {
        for s in sponsors {
            if s.organisation.appears_plausible_org_name() {
                db_orgs.push(DBOrganisation { 
                    contrib_type: "sponsor".to_string(), 
                    org_name: s.organisation.clone(), 
                    org_ror_id: s.ror_id.clone(), 
                    org_cref_id: None 
                });
            }
        }
    }

    // funders

    if let Some(funders) = &s.funders {
        for f in funders {
            if f.name.appears_plausible_org_name() {

                // See if that name has been used before as a sponsor.

                for dbo in &mut db_orgs {
                    if dbo.contrib_type == "sponsor".to_string() && dbo.org_name == f.name {
                        // Change contribution type and  try to combinbe information
                        dbo.contrib_type = "sponsor & funder".to_string();
                        dbo.org_cref_id = f.fund_ref.clone();
                        break;
                    }
                    else {

                        // Add as a separate funder.

                        db_funds.push(DBOrganisation { 
                        contrib_type: "funder".to_string(), 
                        org_name: f.name.clone(),
                        org_ror_id: None,
                        org_cref_id: f.fund_ref.clone(),
                        });
                    }
                }
            }
        }
    }

    db_orgs.append(&mut db_funds);
    // contacts

    //if let Some(contacts) = &s.contacts {
        //for c in contacts {

        //}
   // }

/*


        // Study sponsor(s) and funders.

        var sponsors = r.sponsors;
        string? sponsor_name = null;    // For later use
        if (sponsors?.Any() is true)
        {
            foreach (var stSponsor in sponsors)
            {
                string? org = stSponsor.organisation;
                if (org.AppearsGenuineOrgName())
                {
                    string? orgname = org.TidyOrgName(sid);
                    organisations.Add(new StudyOrganisation(sid, 54, "Trial Sponsor", null, orgname));
                }
            }
            if (organisations.Any())
            {
                sponsor_name = organisations[0].organisation_name;
            }
        }

        var funders = r.funders;
        if (funders?.Any() is true)
        {
            foreach (var funder in funders)
            {
                string? funder_name = funder.name;
                if (!string.IsNullOrEmpty(funder_name) && funder_name.AppearsGenuineOrgName())
                {
                    // check a funder is not simply the sponsor...(or repeated).

                    bool add_funder = true;
                    funder_name = funder_name.TidyOrgName(sid);
                    if (organisations.Count > 0)
                    {
                        foreach (var c in organisations)
                        {
                            if (funder_name == c.organisation_name)
                            {
                                add_funder = false;
                                break;
                            }
                        }
                    }

                    if (add_funder)
                    {
                        organisations.Add(new StudyOrganisation(sid, 58, "Study Funder", null, funder_name));
                    }
                }
            }
        }



#[derive(Serialize, Deserialize)]
pub struct StudyContact
{
    pub title: Option<String>,
    pub forename: Option<String>,
    pub surname: Option<String>,
    pub orcid: Option<String>,
    pub contact_types: Option<Vec<String>>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub privacy: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StudySponsor
{
    pub organisation: Option<String>,
    pub website: Option<String>,
    pub sponsor_type: Option<String>,
    pub ror_id: Option<String>,  
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub privacy: Option<String>,
    pub commercial_status: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StudyFunder
{
    pub name: Option<String>,
    pub fund_ref: Option<String>,
}
#[allow(dead_code)]
pub struct DBOrganisation {
    pub contrib_type_id: i32,
    pub org_name: String,
    pub org_ror_id: Option<String>,
    pub org_cref_id: Option<String>,
}

#[allow(dead_code)]
pub struct DBPerson {
    pub contrib_type_id: i32,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub full_name: Option<String>,
    pub orcid_id: Option<String>,
    pub affiliation: Option<String>,
    pub email_domain: Option<String>,
}

*/

    // People






    DBStudy {

        sd_sid: sd_sid,
        summary: summary,
        dates: dates,
        participants: participants,
        titles: option_from_count(db_ts),
        identifiers: option_from_count(db_ids),
        orgs: option_from_count(db_orgs),

    }

}

