

#[allow(dead_code)]
pub struct Study2 {

    pub isrctn_id: String,
    pub sd_sid: String,
}
#[allow(dead_code)]
impl Study2{
    pub fn new() -> Self {
        Study2 {  
        isrctn_id: "00000000".to_string(),
        sd_sid: "ISRCTN00000000".to_string(),
        }
   }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Study
{
    pub sd_sid: String, 
    pub registration: Registration,

    pub titles: Vec<Title>,
    pub identifiers: Vec<Identifier>,
    pub summary: Summary,

    pub ethics: Ethics,
    pub ethics_committees: Vec<EthicsCommittee>,

    pub trial_types: Vec<String>,
    pub trial_settings: Vec<String>,

   /*
    pub design: Design,
    pub conditions: Vec<Condition>,
    pub interventions: Vec<Intervention>,

    pub contacts: Vec<StudyContact>,
    pub sponsors: Vec<StudySponsor>,
    pub funders: Vec<StudyFunder>,

    pub participant_types: Vec<String>,
    pub participants: Participants,

    pub recruitment: Recruitment,
    pub centres: Vec<StudyCentre>,
    pub countries: Vec<String>,
    pub trial_settings: Vec<String>,

    pub data_policies: Vec<String>,
    pub results: Results,
    pub outputs: Vec<StudyOutput>,
    pub attached_files: Vec<AttachedFile>,
    pub ipd: IPD,
    */
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Registration
{
    pub date_id_assigned: String,
    pub last_updated: String,
    pub version: String,
    pub doi : String,
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Summary
{
    pub plain_english_summary: String,
    pub study_hypothesis: String,
    pub primary_outcome: String,
    pub secondary_outcome: String,
    pub overall_end_date: String,
    pub trial_website: String,
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Design
{
    pub study_design: String,
    pub primary_study_design: String,
    pub secondary_study_design: String,
    pub phase: String,
}

#[allow(dead_code)]
impl Design {
    pub fn new(study_design: String,  primary_study_design: String,
    secondary_study_design: String,  phase: String) -> Self {
        Design {
           study_design,
           primary_study_design,
           secondary_study_design,
           phase,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Ethics
{
    pub ethics_approval_required: String,
    pub ethics_approval: String,
}

#[allow(dead_code)]
impl Ethics {
    pub fn new(ethics_approval_required: String, 
        ethics_approval: String) -> Self {
        Ethics {
           ethics_approval_required,
           ethics_approval,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct EthicsCommittee
{
    pub name: String,
    pub approval_status: String,
    pub status_date: String,
    pub committee_reference: String,
}

#[allow(dead_code)]
impl EthicsCommittee {
    pub fn new(name: String, approval_status: String,  
        status_date: String, committee_reference: String,) -> Self {
        EthicsCommittee {
           name,
           approval_status,
           status_date,
           committee_reference,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Participants
{
    pub age_range: String,
    pub l_age_limit: String,
    pub l_age_limit_num: f32,
    pub l_age_limit_units: String,
    pub u_age_limit: String,
    pub u_age_limit_num: f32,
    pub u_age_limit_units: String,
    pub gender: String,
    pub inclusion: String,
    pub exclusion: String,
    pub patient_info_sheet: String,
}

#[allow(dead_code)]
impl Participants {
    pub fn new(age_range: String, l_age_limit: String, l_age_limit_num: f32,
            l_age_limit_units: String, u_age_limit: String, u_age_limit_num: f32,
            u_age_limit_units: String, gender: String, 
            inclusion: String, exclusion: String, 
            patient_info_sheet: String) -> Self {
        Participants {
            age_range,
            l_age_limit,
            l_age_limit_num,
            l_age_limit_units,
            u_age_limit,
            u_age_limit_num,
            u_age_limit_units,
            gender,
            inclusion,
            exclusion,
            patient_info_sheet,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Recruitment
{
    pub target_enrolment: String,
    pub total_final_enrolment: String,
    pub total_target: String,
    pub recruitment_start: String,
    pub recruitment_end: String,
    pub recruitment_start_status_override: String,
    pub recruitment_status_override: String,
}

#[allow(dead_code)]
impl Recruitment {
    pub fn new(target_enrolment: String, total_final_enrolment: String,
                total_target: String, recruitment_start: String,
                recruitment_end: String, 
                recruitment_start_status_override: String,
                recruitment_status_override: String) -> Self {
        Recruitment {
            target_enrolment,
            total_final_enrolment,
            total_target,
            recruitment_start,
            recruitment_end,
            recruitment_start_status_override,
            recruitment_status_override,
           
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Results
{
    pub publication_plan: String,
    pub intent_to_publish: String,
    pub publication_details: String,
    pub publication_stage: String,
    pub biomed_related: bool,
    pub basic_report: String,
    pub plain_english_report: String,
}

// ?? additional fields to be opened up

#[allow(dead_code)]
impl Results {
    pub fn new(publication_plan: String, intent_to_publish: String,
            publication_details: String, publication_stage: String,
            biomed_related: bool, basic_report: String,
            plain_english_report: String) -> Self {
        Results {
            publication_plan,
            intent_to_publish,
            publication_details,
            publication_stage,
            biomed_related,
            basic_report,
            plain_english_report,
        }
    }
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct IPD
{
    pub ipd_sharing_plan: String,    // Yes or No
    pub ipd_sharing_statement: String,

}

// ?? additional fields to be opened up

#[allow(dead_code)]
impl IPD {
    pub fn new(ipd_sharing_plan: String, ipd_sharing_statement: String) -> Self {
        IPD {
            ipd_sharing_plan,
            ipd_sharing_statement,
        }
   }
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Condition
{
    pub description: String,
    pub disease_class1: String,
    pub disease_class2: String,
}

#[allow(dead_code)]
impl Condition {
    pub fn new(description: String, disease_class1: String, disease_class2: String) -> Self {
        Condition {
            description,
            disease_class1,
            disease_class2,
        }
   }
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Intervention 
{
    pub description: String,
    pub int_type: String,
    pub pharmaceutical_study_types: String,
    pub drug_names: String,
}


#[allow(dead_code)]
impl Intervention {
    pub fn new(description: String, int_type: String, 
        pharmaceutical_study_types: String,drug_names: String) -> Self {
        Intervention {
        description,
        int_type,
        pharmaceutical_study_types,
        drug_names
        }
   }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Title
{
    pub title_type_id: i32,
    pub title_type: String,
    pub title_value: String,
}

#[allow(dead_code)]
impl Title {
    pub fn new(title_type_id: i32, title_type: String, title_value: String) -> Self {
        Title {
        title_type_id,
        title_type,
        title_value,
        }
   }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Identifier
{
    pub identifier_type_id: i32,
    pub identifier_type: String,
    pub identifier_value: String,
}

#[allow(dead_code)]
impl Identifier {
    pub fn new(identifier_type_id: i32, identifier_type: String, identifier_value: String) -> Self {
        Identifier {
        identifier_type_id,
        identifier_type,
        identifier_value,
        }
   }
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct StudyCentre
{
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
}

#[allow(dead_code)]
impl StudyCentre{
    pub fn new(name: String, address: String, city: String,
               state: String, country: String) -> Self {
        StudyCentre {
        name,
        address,
        city,
        state,
        country,
        }
    }
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct StudyOutput
{
    pub output_type: String,
    pub artefact_type: String,
    pub date_created: String,
    pub date_uploaded: String,
    pub peer_reviewed: bool,
    pub patient_facing: bool,
    pub created_by: String,
    pub description: String,
    pub production_notes: String,

    pub external_link_url: String,

    pub file_id: String,
    pub original_filename: String,
    pub download_filename: String,
    pub version: String,
    pub mime_type: String,

}  

#[allow(dead_code)]
impl StudyOutput{
    
    pub fn new(description: String,  production_notes: String, 
                        output_type: String,  artefact_type: String,  date_created: String,
                        date_uploaded: String, peer_reviewed: bool, patient_facing: bool, 
                        created_by: String,  external_link_url: String, file_id: String,
                        original_filename: String, download_filename: String, 
                        version: String,  mime_type: String) -> Self {
        StudyOutput {  
            description,
            production_notes,
            output_type,
            artefact_type,
            date_created,
            date_uploaded,
            peer_reviewed,
            patient_facing,
            created_by,
            external_link_url,
            file_id,
            original_filename,
            download_filename,
            version,
            mime_type,
            }
        }
}


#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct AttachedFile
{
    pub description: String,
    pub name: String,
    pub id: String,
    pub is_public: bool,
    pub mime_type: String,
}

#[allow(dead_code)]
impl AttachedFile{
    pub fn new(description: String, name: String, id: String, 
            is_public: bool, mime_type: String,) -> Self {
        AttachedFile  {
        description,
        name,
        id,
        is_public,
        mime_type,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct StudyContact
{
    pub title: String,
    pub forename: String,
    pub surname: String,
    pub orcid: String,
    pub contact_type: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub email: String,
    pub privacy: String,
}

#[allow(dead_code)]
impl StudyContact{
    pub fn new(title: String, forename: String,  surname: String,  orcid: String,
           contact_type: String, address: String, city: String,  country: String,  
           email: String, privacy: String) -> Self {
        StudyContact {
            title,
            forename,
            surname,
            orcid,
            contact_type,
            address,
            city,
            country,
            email,
            privacy,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct StudySponsor
{
    pub organisation: String,
    pub website: String,
    pub sponsor_type: String,
    pub ror_id: String,    
    pub address: String,
    pub city: String,
    pub country: String,
    pub email: String,
    pub privacy: String,
    pub commercial_status: String,
}

#[allow(dead_code)]
impl StudySponsor{
        pub fn new(organisation: String,  website: String, 
            sponsor_type: String, ror_id: String, 
            address: String, city: String, country: String, email: String, 
            privacy: String, commercial_status: String) -> Self {
        StudySponsor {
            organisation,
            website,
            sponsor_type,
            ror_id,
            address,
            city,
            country,
            email,
            privacy,
            commercial_status,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct StudyFunder
{
    pub name: String,
    pub fund_ref: String,
}

#[allow(dead_code)]
impl StudyFunder{
    pub fn new(name: String, fund_ref: String) -> Self {
        StudyFunder {  
            name,
            fund_ref,
        }
    }
}

