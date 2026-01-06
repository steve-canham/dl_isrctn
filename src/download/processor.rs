use crate::data_models::xml_models;
use crate::data_models::json_models::*; 
use crate::helpers::string_extensions::*;
use crate::helpers::download_helpers::{count_option, split_identifier, classify_identifier};

use crate::err::AppError;
use chrono::Utc;
use std::sync::LazyLock;
use regex::Regex;
use log::info;

// processes study, returns json file model that model can be printed, 
// and / or it can be saved to the database...

#[allow(unused_variables)]
pub fn process_study(s: xml_models::FullTrial) -> Result<Study, AppError> {

    let study = s.trial;

    let sd_sid = format!("ISRCTN{}", study.isrctn.value);
    let downloaded = Utc::now().to_rfc3339()[0..19].to_string(); // current date time as ISO 8601

    // Registration data block.
    
    let registration = Registration {
         date_id_assigned: study.isrctn.date_assigned.as_date_opt(),
         last_updated: study.last_updated.as_datetime_opt(),
         version: study.version.as_text_opt(),
         doi: study.external_refs.doi.as_filtered_ident_opt()
    };

    // Set up titles.

    let d = study.trial_description;

    let mut titles: Vec<Title> = Vec::new();
    let mut pt: String = "".to_string();
    let mut st: String = "".to_string();

    // change to include import code
    // also include plauible title check (though perhaps not needed for ISRCTN)

    if let Some(title) = d.title.as_text_opt() {
        pt = title.clone();
        titles.push(Title::new(15, "Public_title".to_string(), title));
    }

    if let Some(title) = d.scientific_title.as_text_opt() && title != pt {
        st = title.clone();
        titles.push(Title::new(16, "Scientific title".to_string(), title));
    }

    if let Some(title) = d.acronym.as_text_opt() && title != pt && title != st {
        titles.push(Title::new(14, "Acronym".to_string(), title));
    }

    // Identifiers

    // Processing identifiers involves the array of secondary identifiers only.
    // These seem to always repeat the data in the specific fields (which is therefore redundant)
    // and in the later records at least contain more data - i.e. lists of multiple ids
    // are split properly and in many cases have their 'type' identified more accurately.

    // The system therefore loops through these secondary id recordsw. For each record,
    // if a valid id is found, the 'type', as described in the data, is used to determine further
    // action. Some id types can be pushed straight to the collecting vector. In other cases the 
    // id is sent to a helper function to see if its type can be identified. In the case of the 
    // 'Sponsor's protocol number' the id is also examined to see if it should be split before 
    // further processing. If the type can be idfentified the value is pushed to the vector with
    // that additional information. Otherwise it is added as a general grant or sponsor's protocol id.

    let er = study.external_refs;
    let sec_ids = er.secondary_number_list.secondary_numbers;
    let mut s_identifiers =  Vec::new();
    let mut iras_number = "0".to_string(); // for possible future check against repetition;
 
    static RE_CPMS_NUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{5}$").unwrap());
    static RE_NIHR_NUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{6}$").unwrap());

    if let Some(idents) = sec_ids {
        for ident in &idents {

            let id_value = &ident.value.as_filtered_ident_opt();
            if let Some(id) = id_value {

                let id_type = ident.number_type.as_text_opt();
                if let Some(id_type_string) = id_type {

                    match id_type_string.as_str() {
                        "iras" => {
                            iras_number = id.clone();
                            s_identifiers.push(Identifier::new(303, "IRAS ID".to_string(), id.to_string()));
                        }, 

                        "ctis" => {
                            if let Some(id_reg) = id_value.regularise_hyphens(){
                               if  id_reg.len() == 14 || id_reg.len() == 17 {
                                    if &id_reg[4..6] == "-5" {
                                        s_identifiers.push(Identifier::new(135, "EMA CTIS ID".to_string(), id_reg.to_string()));
                                    } else {
                                        s_identifiers.push(Identifier::new(123, "EMA Eudract ID".to_string(), id_reg.to_string()));
                                    }
                                }
                                else {
                                    s_identifiers.push(Identifier::new(179, "Malformed registry Id (CTIS claimed)".to_string(), id_reg.to_string()));
                                }
                            }
                        }, 

                        "nct" => {
                            s_identifiers.push(Identifier::new(120, "NCT ID".to_string(), id.to_string()));
                        }, 

                        "cpms" => {   // May need to have prefix removed
                            if RE_CPMS_NUM.is_match(&id) {   // already a digit string
                                s_identifiers.push(Identifier::new(304, "CPMS ID".to_string(), id.to_string()));
                            }
                            else {
                                let (type_id, type_string, id_post) = classify_identifier(id.to_string());
                                s_identifiers.push(Identifier::new(type_id, type_string, id_post));
                            }
                        }, 

                        "nihr" => {   
                            if RE_NIHR_NUM.is_match(&id) {   // already a digit string
                                s_identifiers.push(Identifier::new(416, "NIHR ID".to_string(), id.to_string()));
                            }
                            else {// May need to have prefix removed
                                let (type_id, type_string, id_post) = classify_identifier(id.to_string());
                                s_identifiers.push(Identifier::new(type_id, type_string, id_post));
                            }
                        }, 

                        "Grant Code" => {
                            if let Some(id_reg) = id_value.regularise_hyphens(){

                                // Try to classify. if unable assume it is an as yet undefined funder / grant code

                                let (mut type_id, mut type_string, id_post) = classify_identifier(id_reg);
                                if type_string == "???".to_string() {
                                    type_id = 400;
                                    type_string = "Grant / Contract ID (unclassified)".to_string();
                                }
                                s_identifiers.push(Identifier::new(type_id, type_string, id_post));

                            }
                        }, 

                        "Protocol serial number" => {
                            if let Some(id_reg) = id_value.regularise_hyphens(){

                                // Turn current value into a vector.
                                // If it includes a comma or a sem-colon it may be a vector of multiple strings
                                // (Until recently this category of id was used as a 'catch all' for additional ids)

                                let mut split_ids = vec![id_reg.clone()];
                                if id_reg.contains(",") || id_reg.contains(";"){
                                    split_ids = split_identifier(&id_reg);
                                }
                                
                                for each_id in split_ids {

                                    // For each try to classify. If unable assume it is a spionsor id.

                                    let (mut type_id, mut type_string, id_post) = classify_identifier(each_id);
                                    if type_string == "???".to_string() {
                                        if id_post.to_lowercase().starts_with("grant")
                                        || id_post.to_lowercase().starts_with("contract") {
                                            type_id = 400;
                                            type_string = "Grant / Contract ID (unclassified)".to_string();
                                        } 
                                        else {
                                            type_id = 502;
                                            type_string = "Sponsor's ID (presumed)".to_string();
                                        }
                                    }

                                    // In some cases this id seems to include the IRAS number - check before adding.

                                    let mut add_new = true;
                                    if type_id == 303 && id_post == iras_number {
                                        add_new = false;
                                    }
                                    if add_new {
                                        s_identifiers.push(Identifier::new(type_id, type_string, id_post));
                                    }
                                }
                            }
                        }, 

                        _ => {

                            // Some other string used for the id type - try to classify, if unable use 'as is'

                            if let Some(id_reg) = id_value.regularise_hyphens(){
                                let (mut type_id, mut type_string, id_post) = classify_identifier(id_reg);
                                if type_string == "???".to_string() {
                                    type_id = 990;
                                    type_string = id_type_string;
                                }
                                s_identifiers.push(Identifier::new(type_id, type_string, id_post));


                            }
                        },
          
                    }
                }
            }
        }
    }
    


    let identifiers = count_option(s_identifiers);
    
    // Summary block

    let mut plain_summ = d.plain_english_summary.as_text_opt();

    if let Some(mut summ) = plain_summ {
        let end_point =  summ.find("What are the possible benefits and risks");
        if let Some(ep) = end_point {
            summ = summ[..ep].to_string();
        }

        summ = summ.replace("Background and study aims", "Background and study aims\n");
        summ = summ.replace("Who can participate?", "\nWho can participate?\n");
        summ = summ.replace("What does the study involve?", "\nWhat does the study involve?\n");

        plain_summ = Some(summ).clean_multiline();
    }
    
    let summary = Summary {
        plain_english_summary: plain_summ,
        study_hypothesis: d.study_hypothesis.as_text_opt(),
        primary_outcome: d.primary_outcome.as_text_opt(),
        secondary_outcome: d.secondary_outcome.as_text_opt(),
        overall_end_date: study.trial_design.overall_end_date.as_date_opt(),
        trial_website: d.trial_website.as_text_opt(),
    };

    // 

    let mut s_primary_outcomes: Vec<OutcomeMeasure> = Vec::new();
    if d.primary_outcomes.outcome_measures.len() > 0 {
        for om in d.primary_outcomes.outcome_measures {
            s_primary_outcomes.push(OutcomeMeasure { 
                variable: om.variable, 
                method: om.method, 
                timepoints: om.timepoints,
            })
        }
    }
    let primary_outcomes =  count_option(s_primary_outcomes);

    let mut s_secondary_outcomes: Vec<OutcomeMeasure> = Vec::new();
    if d.secondary_outcomes.outcome_measures.len() > 0 {
        for om in d.secondary_outcomes.outcome_measures {
            s_secondary_outcomes.push(OutcomeMeasure { 
                variable: om.variable, 
                method: om.method, 
                timepoints: om.timepoints,
            })
        }
    } 
    let secondary_outcomes =  count_option(s_secondary_outcomes);

    // Ethics Committee data

    let ethics = Ethics {
        ethics_approval_required: d.ethics_approval_required.as_text_opt(),
        ethics_approval: d.ethics_approval.as_text_opt(),
    };

    let mut ethics_comms = Vec::new();
    if d.ethics_committee_list.ethics_committees.len() > 0 {
        for ec in d.ethics_committee_list.ethics_committees {
            
            let committee = EthicsCommittee {
                name: ec.committee_name.as_text_opt(),
                approval_status: ec.approval_status.as_text_opt(),
                status_date: ec.status_date.as_date_opt(),
                committee_reference: ec.committee_reference.as_text_opt(),
            };
            ethics_comms.push(committee);
        }
    }
    let ethics_committees = count_option(ethics_comms);

    // Design block

    let ds = study.trial_design;

    let design = Design {
        study_design: ds.study_design.as_text_opt(),
        primary_study_design: ds.primary_study_design.as_text_opt(),
        secondary_study_design: ds.secondary_study_design.as_text_opt(),
    };

    // Trial type list and trial settings list

    let mut t_types: Vec<String> = Vec::new();
    if ds.trial_type_list.trial_types.len() > 0 {
        for tt in ds.trial_type_list.trial_types {
            if let Some(s) = tt.trial_type.as_text_opt(){
                t_types.push(s);
            }
        }
    }
    let trial_types = count_option(t_types);

    
    let mut t_settings: Vec<String> = Vec::new();
    if ds.trial_setting_list.trial_settings.len() > 0 {
        for ts in ds.trial_setting_list.trial_settings {
            if let Some(s) = ts.trial_setting.as_text_opt(){
                t_settings.push(s);
            }
        }
    }
    let trial_settings = count_option(t_settings);
    

    // Conditions

    let mut conds: Vec<Condition> = Vec::new();
    if study.condition_list.conditions.len() > 0 {
        for c in study.condition_list.conditions {
            conds.push(Condition {
                        description: c.description.as_text_opt(),
                        disease_class1: c.disease_class1.as_text_opt(),
                        disease_class2: c.disease_class2.as_text_opt(),
            });
        }
    }

    if conds.len() > 1 {
        info!("{} conditions listed for {}", conds.len(), sd_sid);
    }

    let conditions = count_option(conds);


    // Interventions

    let mut intervents: Vec<Intervention> = Vec::new();
    if study.intervention_list.interventions.len() > 0 {
        for i in study.intervention_list.interventions {
            intervents.push(Intervention {
                        description: i.description.as_text_opt(),
                        int_type: i.intervention_type.as_text_opt(),
                        pharma_study_types: i.pharmaceutical_study_types.as_text_opt(),
                        phase: i.phase.as_text_opt(),
                        drug_names: i.drug_names.as_text_opt(),
            });
        }
    }

    if intervents.len() > 1 {
        info!("{} conditions listed for {}", intervents.len(), sd_sid);
    }

    let interventions = count_option(intervents);


    // Study Contacts
 
    let mut s_contacts:Vec<StudyContact> = Vec::new();
    if s.contacts.len() > 0 {
        for c in s.contacts {

            let mut s_contact_types:Vec<String> = Vec::new();
            if c.contact_type_list.contact_types.len() > 0 {
                for ct in c.contact_type_list.contact_types {
                    if let Some(s) = ct.contact_type.as_text_opt(){
                        s_contact_types.push(s);
                    }
                }
            }
            let contact_types = count_option(s_contact_types);

            s_contacts.push(StudyContact{
                title: c.title.as_text_opt(),
                forename: c.forename.as_text_opt(),
                surname: c.surname.as_text_opt(),
                orcid: c.orcid.as_text_opt(),
                contact_types: contact_types,
                address: c.contact_details.address.as_text_opt(),
                city: c.contact_details.city.as_text_opt(),
                country: c.contact_details.country.as_text_opt(),
                email: c.contact_details.email.as_text_opt(),
                privacy: c.privacy.as_text_opt(),
            });
        }
    }
    let contacts = count_option(s_contacts);

    // Study Sponsors

    let mut s_sponsors:Vec<StudySponsor> = Vec::new();
    if s.sponsors.len() > 0 {
        for sp in s.sponsors {
            s_sponsors.push (StudySponsor {
                organisation: sp.organisation.as_text_opt(),
                website: sp.website.as_text_opt(),
                sponsor_type: sp.sponsor_type.as_text_opt(),
                ror_id: sp.ror_id.as_text_opt(),
                address: sp.contact_details.address.as_text_opt(),
                city: sp.contact_details.city.as_text_opt(),
                country: sp.contact_details.country.as_text_opt(),
                email: sp.contact_details.email.as_text_opt(),
                privacy: sp.privacy.as_text_opt(),
                commercial_status: sp.commercial_status.as_text_opt(),
            });
        }
    }
    let sponsors = count_option(s_sponsors);

    // Study Funders

    let mut s_funders:Vec<StudyFunder> = Vec::new();
    if s.funders.len() > 0 {
        for f in s.funders {
            s_funders.push(StudyFunder {
                name: f.name.as_text_opt(),
                fund_ref: f.fund_ref.as_text_opt(),
            });
        }
    }
    let funders = count_option(s_funders);

    // Participanmt Types

    let p = study.participants;

    let mut part_types: Vec<String> = Vec::new();
    if p.participant_type_list.participant_types.len() > 0 {
        for pt in p.participant_type_list.participant_types {
            if let Some(s) = pt.participant_type.as_text_opt(){
                part_types.push(s);
            }
        }
    }
    let participant_types = count_option(part_types);

    // Participants

    let lal = p.lower_age_limit;
    let mut l_age_limit = None;
    let mut l_age_limit_num= None;
    let mut l_age_limit_units = None;
    if let Some(al) = lal {
        l_age_limit = al.value.as_text_opt();
        l_age_limit_units = al.unit.as_text_opt();
        l_age_limit_num = al.num_unit.as_f32_opt();
    }
    

    let ual = p.upper_age_limit;
    let mut u_age_limit = None;
    let mut u_age_limit_num = None;
    let mut u_age_limit_units = None;
    if let Some(al) = ual {
        u_age_limit = al.value.as_text_opt();
        u_age_limit_units= al.unit.as_text_opt();
        u_age_limit_num = al.num_unit.as_f32_opt(); 
    }
     
    let participants = Participants {
            age_range: p.age_range.as_text_opt(),
            l_age_limit: l_age_limit,
            l_age_limit_num: l_age_limit_num,
            l_age_limit_units: l_age_limit_units,
            u_age_limit: u_age_limit,
            u_age_limit_num: u_age_limit_num,
            u_age_limit_units: u_age_limit_units,
            gender: p.gender.as_text_opt(),
            inclusion: p.inclusion.as_text_opt(),
            exclusion: p.exclusion.as_text_opt(),
            patient_info_sheet: p.patient_info_sheet.as_text_opt(),
    };

    // Recruitment
   
    let recruitment = Recruitment {
            target_enrolment: p.target_enrolment.as_text_opt(),
            total_final_enrolment: p.total_final_enrolment.as_text_opt(),
            total_target: p.total_target.as_text_opt(),
            recruitment_start: p.recruitment_start.as_date_opt(),
            recruitment_end: p.recruitment_end.as_date_opt(),
            recruitment_start_status_override: p.recruitment_start_status_override.as_text_opt(),
            recruitment_status_override: p.recruitment_status_override.as_text_opt(),
    };

    let mut s_centres: Vec<StudyCentre> = Vec::new();
    if p.centre_list.centres.len() > 0 {
        for c in p.centre_list.centres {
            s_centres.push(StudyCentre {
                name: c.name.as_text_opt(),
                address: c.address.as_text_opt(),
                city: c.city.as_text_opt(),
                state: c.state.as_text_opt(),
                country: c.country.as_text_opt(),
            });
        }  
    }
    let centres = count_option(s_centres);


    let mut init_countries: Vec<String> = Vec::new();
    if p.country_list.countries.len() > 0 {
        for c in p.country_list.countries  {
            if let Some(s) = c.country.as_text_opt(){
                init_countries.push(s);
            }
        }
    }

    // Some country name tidying to be done here
    let mut s_countries: Vec<String> = Vec::new();
    for c_init in init_countries {

        // Regularise these common alternative spellings / allocations.

        let mut c = c_init.replace("Korea, South", "South Korea");
        c = c.replace("Congo, Democratic Republic", "Democratic Republic of the Congo");
        
        let c_lower = c.to_ascii_lowercase();
        if c_lower == "england" || c_lower == "scotland" ||
           c_lower == "wales" || c_lower == "northern ireland"
        {
                c = "United Kingdom".to_string();
        }

        if c_lower == "united states of america"
        {
                c = "United States".to_string();
        }

        // Check for duplicates before adding, especially after changes above.

        if s_countries.len() == 0
        {
            s_countries.push(c);
        }
        else {
            let mut add_country = true;
            for c_check in &s_countries {
                if c.as_str() == c_check {
                    add_country = false;
                    break;
                }
            }
            if add_country {
                s_countries.push(c);
            }
        }
    }

    let countries = count_option(s_countries);

    // Results

    let r = study.results;

    let mut s_data_policies: Vec<String> = Vec::new();
    if r.data_policy_list.data_policies.len() > 0 {
        for dp in r.data_policy_list.data_policies  {
            if let Some(s) = dp.data_policy.as_text_opt(){
                s_data_policies.push(s);
            }
        }
    }
    let data_policies = count_option(s_data_policies);


    let results = Results {
            publication_plan: r.publication_plan.as_text_opt(),
            intent_to_publish: r.intent_to_publish.as_date_opt(),
            publication_details: r.publication_details.as_text_opt(),
            publication_stage: r.publication_stage.as_text_opt(),
            biomed_related: r.biomed_related.as_bool_opt(),
            basic_report: r.basic_report.as_text_opt(),
            plain_english_report: r.plain_english_report.as_text_opt(),
    };
    
    // Outputs

    let mut s_outputs: Vec<StudyOutput> = Vec::new();
    let ops = study.output_list.outputs;
    if let Some(output_list) = ops {
        for op in output_list {

            // defaults 
            let mut external_link_url = None;
            let mut file_id = None;
            let mut original_filename = None;
            let mut download_filename = None;
            let mut version = None;
            let mut mime_type = None;

            let external_link = op.external_link;
            if let Some(el) = external_link {
                external_link_url = el.url.as_text_opt();
            }

            let local_file = op.local_file;
            if let Some(lf) = local_file {
                file_id = lf.file_id.as_text_opt();
                original_filename = lf.original_filename.as_text_opt(); 
                download_filename = lf.download_filename.as_text_opt();
                version = lf.version.as_text_opt();
                mime_type = lf.mime_type.as_text_opt();
            }

            s_outputs.push(StudyOutput {
                description: op.description.as_text_opt(),
                production_notes: op.production_notes.as_text_opt(),
                output_type: op.output_type.as_text_opt(),
                artefact_type: op.artefact_type.as_text_opt(),
                date_created: op.date_created.as_date_opt(),
                date_uploaded: op.date_uploaded.as_date_opt(),
                peer_reviewed: op.peer_reviewed.as_bool_opt(),
                patient_facing: op.patient_facing.as_bool_opt(),
                created_by: op.created_by.as_text_opt(),

                external_link_url: external_link_url,
                file_id: file_id,
                original_filename: original_filename, 
                download_filename: download_filename,
                version: version,
                mime_type: mime_type,
            });
        }
    }
    let outputs = count_option(s_outputs);

    // Attached Files
    
    let mut s_attached_files: Vec<AttachedFile> = Vec::new();
    let afs = study.attached_file_list.attached_files;
    if let Some(file_list) = afs {
        for af in file_list {
            s_attached_files.push( AttachedFile { 
                description: af.description.as_text_opt(),
                name: af.name.as_text_opt(),
                id: af.id.as_text_opt(),
                is_public: af.public.as_bool_opt(),
                mime_type: af.mime_type.as_text_opt(),
            });
        }
    }
    let attached_files = count_option(s_attached_files);

    let mut has_sharing_plan = false;
    if let Some(b) = study.miscellaneous.ipd_sharing_plan {
        if b.to_ascii_lowercase() == "yes" {
            has_sharing_plan = true;
        }

    }
    let ipd = IPD {
            ipd_sharing_plan: has_sharing_plan,
            ipd_sharing_statement: r.ipd_sharing_statement.as_text_opt(),
    };


    let json_study = Study { 
        sd_sid, 
        downloaded,
        registration, 
        titles, 
        identifiers,
        summary,
        primary_outcomes,
        secondary_outcomes,
        ethics,
        ethics_committees,
        design,
        trial_types,
        trial_settings,
        conditions,
        interventions, 
        contacts,
        sponsors,
        funders,
        participant_types,
        participants,
        recruitment,
        centres,
        countries,
        data_policies,
        results,
        outputs,
        attached_files,
        ipd,
    };

    Ok(json_study)

}

