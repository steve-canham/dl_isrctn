
use crate::download::xml_models;
use crate::download::json_models; 
use crate::err::AppError;
use super::json_models::*;
use chrono::Utc;
use log::info;

use super::isrctn_helper::{count_option, split_identifier, classify_identifier, 
                            StringExtensions, OptionStringExtensions};


// processes study, returns json file model that model can be printed, 
// and / or it can be saved to the database...

#[allow(unused_variables)]
pub fn process_study(s: xml_models::FullTrial) -> Result<json_models::Study, AppError> {

    let study = s.trial;

    let sd_sid = format!("ISRCTN{:?}", study.isrctn.value);
    let downloaded = Utc::now().to_rfc3339()[0..19].to_string(); // current date time as ISO 8601

    // Registration data block.
    
    let registration = Registration {
         date_id_assigned: study.isrctn.date_assigned.as_date_opt(),
         last_updated: study.last_updated.as_datetime_opt(),
         version: study.version.as_text_opt(),
         doi: study.external_refs.doi.as_filtered_text_opt()
    };

    // Set up titles.

    let d = study.trial_description;

    let mut titles: Vec<Title> = Vec::new();
    let mut pt: String = "".to_string();
    let mut st: String = "".to_string();

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

    // Processing identifiers involves two loops. The first simply gets the identifiers as written 
    // from their source fields, checking for duplicates amongst the 'secondary id' list (in most,
    // perhaps all, cases this list seems to duplicate the data present in the specific fields).

    // The second loop goes through and tries to identify common identifier types amongst those
    // simply listed as 'sponsor protocol id', because many of these are IRASS, CPMS or NIHR numbers.
    // It also checks - amongst the 'sponsor protocol id' values - for commas, which often signify
    // multiple identifiers written on the same line. If commas are found that signify listed
    // identifiers the line is split and each constituent part is added as a separate identifier.

    let er = study.external_refs;
    let mut idents: Vec<Identifier> = Vec::new();

    let ema = er.eudra_ct_number.as_filtered_text_opt();
    if let Some(id) = ema {
        if &id[4..6] == "-5" {
            idents.push(Identifier::new(135, "EMA CTIS ID".to_string(), id));
        } else {
            idents.push(Identifier::new(123, "EMA Eudract ID".to_string(), id));
        }
    }

    let iras = er.iras_number.as_filtered_text_opt();
    let mut iras_value = "".to_string();  // for possible comparison later - IRAS values often seem to be duplicated
    if let Some(id) = iras {
        iras_value = id.clone();
        idents.push(Identifier::new(303, "IRAS ID".to_string(), id));
    }

    let ctg = er.ctg_number.as_filtered_text_opt();
    if let Some(id) = ctg {
        idents.push(Identifier::new(120, "NCT ID".to_string().to_string(), id));
    }

    let prot = er.protocol_serial_number.as_filtered_text_opt();
    if let Some(id) = prot {
        idents.push(Identifier::new(502, "Sponsor's ID (presumed)".to_string(), id));
    }
    
    let ids = er.secondary_number_list.secondary_numbers;
    if let Some(nums) = ids {
        for num in &nums {
            let num_string = num.value.as_filtered_text_opt();
            if let Some(id) = num_string {

                // Has number already been supplied? - in almost all cases they seem to be

                let mut add_id = true;
                for ident in &idents {
                    if id == ident.identifier_value {
                        add_id = false;
                        break;
                    }
                }
                if add_id {
                    
                    info!("New identifier listed in secondary numbers: {}, for {}", sd_sid, id);
                    idents.push(Identifier::new(990, "Other Id (provenance not supplied)".to_string(), id));
                   
                }
            }
        }
    }

    // If the 'protocol serial number' or any additional identifier contain commas, see if they can be split
    // also see if they can be reclassified to something more specific. Other ids can be used unchanged.

    let mut processed_idents: Vec<Identifier> = Vec::new();
    for ident in idents {
        if ident.identifier_type_id == 502 || ident.identifier_type_id == 990 {  // See if has a comma and could be split
            if ident.identifier_value.contains(",") {
                let split_ids = split_identifier(&ident.identifier_value);
                if split_ids.len() > 1 {
                    for split_id in split_ids {    // Process each ident and add to processed_idents
                        let old_ident = Identifier::new(ident.identifier_type_id , ident.identifier_type.clone(), split_id);
                        let new_ident = classify_identifier(old_ident);
                        let mut add_new = true;
                        if new_ident.identifier_type_id == 303 && new_ident.identifier_value == iras_value {
                            add_new = false;
                        }
                        if add_new {
                            processed_idents.push(new_ident);
                        }
                    }
                }
            }
            else {   // no splitting - just process the ident and add to processed_idents vector

                let new_ident = classify_identifier(ident);
                let mut add_new = true;
                if new_ident.identifier_type_id == 303 && new_ident.identifier_value == iras_value {
                    add_new = false;
                }
                if add_new {
                    processed_idents.push(new_ident);
                }
            }
        }
    else {
            processed_idents.push(ident);
        }

    }

    let identifiers = count_option(processed_idents);
    
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

        plain_summ = summ.compress_spaces();
    }
    
    let summary = Summary {
        plain_english_summary: plain_summ,
        study_hypothesis: d.study_hypothesis.as_text_opt(),
        primary_outcome: d.primary_outcome.as_text_opt(),
        secondary_outcome: d.secondary_outcome.as_text_opt(),
        overall_end_date: study.trial_design.overall_end_date.as_date_opt(),
        trial_website: d.trial_website.as_text_opt(),
    };

    let primary_outcomes = d.primary_outcomes.as_text_opt();
    if let Some(s) = primary_outcomes {
        info!("Primary outcomes (plural field) found for {}.\n {}", sd_sid, s);
    }
    let secondary_outcomes = d.secondary_outcomes.as_text_opt();
    if let Some(s) = secondary_outcomes {
        info!("Secondary outcomes (plural field) found for {}.\n {}", sd_sid, s);
    }

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
        l_age_limit_num = al.num_unit.as_float_opt();
    }
    

    let ual = p.upper_age_limit;
    let mut u_age_limit = None;
    let mut u_age_limit_num = None;
    let mut u_age_limit_units = None;
    if let Some(al) = ual {
        u_age_limit = al.value.as_text_opt();
        u_age_limit_units= al.unit.as_text_opt();
        u_age_limit_num = al.num_unit.as_float_opt(); 
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


    let ipd = IPD {
            ipd_sharing_plan: study.miscellaneous.ipd_sharing_plan.as_bool_opt(),
            ipd_sharing_statement: r.ipd_sharing_statement.as_text_opt(),
    };


    let json_study = json_models::Study { 
        sd_sid, 
        downloaded,
        registration, 
        titles, 
        identifiers,
        summary,
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

