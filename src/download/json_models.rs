
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

    pub design: Design,
    pub trial_types: Vec<String>,
    pub trial_settings: Vec<String>,

    pub conditions: Vec<Condition>,
    pub interventions: Vec<Intervention>,

    pub contacts: Vec<StudyContact>,
    pub sponsors: Vec<StudySponsor>,
    pub funders: Vec<StudyFunder>,

    pub participant_types: Vec<ParticipantType>,
    pub participants: Participants,

    pub recruitment: Recruitment,
    pub centres: Vec<StudyCentre>,
    pub countries: Vec<String>,

    pub data_policies: Vec<String>,
    pub results: Results,
    pub outputs: Vec<StudyOutput>,
    pub attached_files: Vec<AttachedFile>,
    pub ipd: IPD,

}

#[derive(serde::Serialize)]
pub struct Registration
{
    pub date_id_assigned: Option<String>,
    pub last_updated: Option<String>,
    pub version: Option<String>,
    pub doi : Option<String>,
}


#[derive(serde::Serialize)]
pub struct Title
{
    pub title_type_id: i32,
    pub title_type: String,
    pub title_value: String,
}

impl Title {
    pub fn new(title_type_id: i32, title_type: String, title_value: String) -> Self {
        Title {
        title_type_id,
        title_type,
        title_value,
        }
   }
}

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

#[derive(serde::Serialize)]
pub struct Summary
{
    pub plain_english_summary: Option<String>,
    pub study_hypothesis: Option<String>,
    pub primary_outcome: Option<String>,
    pub secondary_outcome: Option<String>,
    pub overall_end_date: Option<String>,
    pub trial_website: Option<String>,
}


#[derive(serde::Serialize)]
pub struct Ethics
{
    pub ethics_approval_required: Option<String>,
    pub ethics_approval: Option<String>,
}

#[derive(serde::Serialize)]
pub struct EthicsCommittee
{
    pub name: Option<String>,
    pub approval_status: Option<String>,
    pub status_date: Option<String>,
    pub committee_reference: Option<String>,
}


#[derive(serde::Serialize)]
pub struct Design
{
    pub study_design: Option<String>,
    pub primary_study_design: Option<String>,
    pub secondary_study_design: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Condition
{
    pub description: String,
    pub disease_class1: String,
    pub disease_class2: String,
}


#[derive(serde::Serialize)]
pub struct Intervention 
{
    pub description: String,
    pub int_type: String,
    pub pharma_study_types: String,
    pub phase: String,
    pub drug_names: String,
}





#[derive(serde::Serialize)]
pub struct ParticipantType
{
    pub participant_type: String,
}

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

#[derive(serde::Serialize)]
pub struct Results
{
    pub publication_plan: String,
    pub intent_to_publish: String,
    pub publication_details: String,
    pub publication_stage: String,
    pub biomed_related: String,    // actually a bool
    pub basic_report: String,
    pub plain_english_report: String,
}

#[derive(serde::Serialize)]
pub struct IPD
{
    pub ipd_sharing_plan: String,    // Yes or No
    pub ipd_sharing_statement: String,

}


#[derive(serde::Serialize)]
pub struct StudyCentre
{
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
}


#[derive(serde::Serialize)]
pub struct StudyOutput
{
    pub output_type: String,
    pub artefact_type: String,
    pub date_created: String,
    pub date_uploaded: String,
    pub peer_reviewed: String,   // is bool really
    pub patient_facing: String,   // is bool really
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


#[derive(serde::Serialize)]
pub struct AttachedFile
{
    pub description: String,
    pub name: String,
    pub id: String,
    pub is_public: String,   // is bool really
    pub mime_type: String,
}


#[derive(serde::Serialize)]
pub struct StudyContact
{
    pub title: String,
    pub forename: String,
    pub surname: String,
    pub orcid: String,
    pub contact_types: Vec<ContactType>,
    pub address: String,
    pub city: String,
    pub country: String,
    pub email: String,
    pub privacy: String,
}

#[derive(serde::Serialize)]
pub struct ContactType
{
    pub contact_type: String,
}

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

#[derive(serde::Serialize)]
pub struct StudyFunder
{
    pub name: String,
    pub fund_ref: String,
}

