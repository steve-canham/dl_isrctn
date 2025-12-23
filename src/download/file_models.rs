//use chrono::NaiveDate;

#[allow(dead_code)]

#[derive(serde::Deserialize)]
#[serde(rename = "allTrials")]
pub struct AllTrials
{
    #[serde(rename = "@totalCount")]
    pub total_count: i32,

    #[serde(rename = "@fullTrial")]
    pub full_trial: Vec<FullTrial>,
}


#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct FullTrial
{
    pub trial: Trial,

    #[serde(default)]
    pub contact: Vec<Contact>,

    #[serde(default)]
    pub sponsor: Vec<Sponsor>,

    #[serde(default)]
    pub funder: Vec<Funder>,

}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialAgents          // stripped down version of FullTrial for testing purposes
{
    #[serde(default)]
    pub contact: Vec<Contact>,

    #[serde(default)]
    pub sponsor: Vec<Sponsor>,

    #[serde(default)]
    pub funder: Vec<Funder>,
}


#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Trial
{
    #[serde(rename = "@lastUpdated")]
    last_updated:  Option<String>,
    #[serde(rename = "@version")]
    version:  Option<String>,

    pub isrctn: Isrctn,
    #[serde(rename = "trialDescription")]
    pub trial_description: Description,
    #[serde(rename = "ExternalRefs")]
    pub external_refs: Option<ExternalRefs>,
    #[serde(rename = "trialDesign")]
    pub trial_design:Option<Design>,
    pub participants: Participants,
    #[serde(rename = "Conditions")]
    pub interventions: Interventions,
    pub results: Results,
    #[serde(default)]
    pub outputs: Vec<Output>,
    pub parties: Parties,
    #[serde(rename = "attachedFiles", default)]
    pub attached_files: Vec<AttachedFile>,
    pub miscellaneous: Miscellaneous,

}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Isrctn
{
    #[serde(rename = "@dateAssigned")]
    pub date_assigned: Option<String>,
    #[serde(rename = "$value")]
    pub value: i32,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Description
{
    #[serde(rename = "@thirdPartyFilesAcknowledgement")]
    pub third_party_ack: Option<String>,  // actually a bool
    pub acknowledgment: Option<String>,  // actually a bool
    pub title: Option<String>,
    #[serde(rename = "scientificTitle")]
    pub scientific_title: Option<String>,
    pub acronym: Option<String>,
    #[serde(rename = "studyHypothesis")]
    pub study_hypothesis: Option<String>,
    #[serde(rename = "plainEnglishSummary")]
    pub plain_english_summary: Option<String>,

    #[serde(rename = "primaryOutcomes", default)]
    pub primary_outcomes: Option<String>,
    #[serde(rename = "primaryOutcome", default)]
    pub primary_outcome: Option<String>,

    #[serde(rename = "secondaryOutcomes", default)]
    pub secondary_outcomes: Option<String>,
    #[serde(rename = "secondaryOutcome")]
    pub secondary_outcome: Option<String>,

    #[serde(rename = "trialWebsite")]
    pub trial_website: Option<String>,
    #[serde(rename = "ethicsApprovalRequired")]
    pub ethics_approval_required: Option<String>,
    #[serde(rename = "ethicsCommittees", default)]
    pub ethics_committees: Vec<EthicsCommittee>,
    #[serde(rename = "ethicsApproval")]
    pub ethics_approval: Option<String>,
}


#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct EthicsCommittee
{
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@approvalStatus")]
    pub approval_status: Option<String>,
    #[serde(rename = "@statusDate")]
    pub status_date: Option<String>,
    #[serde(rename = "committeeName")]
    pub committee_name: Option<String>,
    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    #[serde(rename = "committeeReference")]
    pub committee_reference: Option<String>,
}


/* 
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PrimaryOutcome
{
    #[serde(rename = "PrimaryOutcome")]
    pub primary_outcome: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SecondaryOutcome
{
    #[serde(rename = "SecondaryOutcome", default)]
    pub secondary_outcome: Option<String>,
}
*/

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ExternalRefs
{
    pub doi: Option<String>,
    #[serde(rename = "eudraCTNumber")]
    pub eudra_ct_number: Option<String>,
    #[serde(rename = "irasNumber")]    
    pub iras_number: Option<String>,
    #[serde(rename = "clinicalTrialsGovNumber")]   
    pub ctg_number: Option<String>,
    #[serde(rename = "protocolSerialNumber")]    
    pub protocol_serial_number: Option<String>,
    #[serde(rename = "secondaryNumbers")]
    pub secondary_numbers: SecondaryNumberList,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SecondaryNumberList
{
    #[serde(rename = "secondaryNumber", default)]
    pub secondary_number: Vec<SecondaryNumber>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SecondaryNumber
{
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@numberType")]
    pub number_type: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Design
{
    #[serde(rename = "studyDesign")]
    pub study_design: Option<String>,
    #[serde(rename = "primaryStudyDesign")]
    pub primary_study_design: Option<String>,
    #[serde(rename = "secondaryStudyDesign")]
    pub secondary_study_design: Option<String>,
    #[serde(rename = "trialSettings", default)]
    pub trial_settings: Vec<TrialSetting>,
    #[serde(rename = "trialTypes", default)]
    pub trial_types: Vec<TrialType>,
    #[serde(rename = "overallEndDate")]
    pub overall_end_date: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialSetting
{
    #[serde(rename = "trialSetting")]
    pub trial_setting: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialType
{
    #[serde(rename = "TrialType", default)]
    pub trial_type: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Participants
{
    #[serde(rename = "recruitmentCountries", default)]
    pub recruitment_countries: Vec<String>,

    #[serde(rename = "trialCentres", default)]
    pub trial_centres: Vec<Centre>,
   
    #[serde(rename = "participantTypes", default)]
    pub participant_type: Option<Vec<ParticipantType>>,
    pub inclusion: Option<String>,
    #[serde(rename = "ageRange")]
    pub age_range: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "target_enrolment")]
    pub target_enrolment: Option<String>,
    #[serde(rename = "totalFinalEnrolment")]
    pub total_final_enrolment: Option<String>,
    #[serde(rename = "totalTarget")]
    pub total_target: Option<String>,
    pub exclusion: Option<String>,
    #[serde(rename = "patientInfoSheet")]
    pub patient_info_sheet: Option<String>,
    #[serde(rename = "recruitmentStart")]
    pub recruitment_start: Option<String>,
    #[serde(rename = "recruitmentEnd")]
    pub recruitment_end: Option<String>,
    #[serde(rename = "recruitmentStartStatusOverride")]
    pub recruitment_start_status_override: Option<String>,
    #[serde(rename = "recruitmentStatusOverride")]
    pub recruitment_status_override: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ParticipantType
{
    #[serde(rename = "ParticipantType")]
    pub participant_type: Option<String>,
}


#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Centre
{
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
    pub id: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Conditions
{
    #[serde(rename = "Condition", default)]
    pub condition: Vec<Condition>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Condition
{
    pub description: Option<String>,

    #[serde(rename = "diseaseClass1")]
    pub disease_class1: Option<String>,

    #[serde(rename = "diseaseClass2")]
    pub disease_class2: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Interventions
{
    #[serde(rename = "Intervention", default)]
    pub intervention: Vec<Intervention>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Intervention
{
    pub description: Option<String>,
    #[serde(rename = "interventionType")]
    pub intervention_type: Option<String>,
    #[serde(rename = "pharmaceuticalStudyTypes")]
    pub pharmaceutical_study_types: Option<String>,
    pub phase: Option<String>,
    #[serde(rename = "drugNames")]
    pub drug_names: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Results
{
    #[serde(rename = "publicationPlan")]
    pub publication_plan: Option<String>,
    #[serde(rename = "ipdSharingStatement")]
    pub ipd_sharing_statement: Option<String>,
    #[serde(rename = "intentToPublish")]
    pub intent_to_publish: Option<String>,
    #[serde(rename = "dataPolicies", default)]
    pub data_policies: Vec<DataPolicy>,
    #[serde(rename = "publicationDetails")]
    pub publication_details: Option<String>,
    #[serde(rename = "publicationStage")]
    pub publication_stage: Option<String>,
    #[serde(rename = "biomedRelated")]
    pub biomed_related: Option<String>,  // actually a bool
    #[serde(rename = "basicReport")]
    pub basic_report: Option<String>,
    #[serde(rename = "plainEnglishReport")]
    pub plain_english_report: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DataPolicy
{
    #[serde(rename = "dataPolicy", default)]
    pub data_policy: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Output
{
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@outputType")]
    pub output_type: Option<String>,
    #[serde(rename = "@artefactType")]
    pub artefact_type: Option<String>,
    #[serde(rename = "@dateCreated")]
    pub date_created: Option<String>,
    #[serde(rename = "@dateUploaded")]
    pub date_uploaded: Option<String>,
    #[serde(rename = "@peerReviewed")]
    pub peer_reviewed: Option<String>,  // actually a bool
    #[serde(rename = "@patientFacing")]
    pub patient_facing: Option<String>,  // actually a bool
    #[serde(rename = "@createdBy")]
    pub created_by: Option<String>,

    #[serde(rename = "ExternalLink")]
    pub external_link: Option<ExternalLink>,
    pub description: Option<String>,
    #[serde(rename = "productionNotes")]
    pub production_notes: Option<String>,
    #[serde(rename = "LocalFile")]
    pub local_file: Option<LocalFile>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct LocalFile
{
    #[serde(rename = "@fileId")]
    pub file_id: Option<String>,
    #[serde(rename = "@originalFilename")]
    pub original_filename: Option<String>,
    #[serde(rename = "@downloadFilename")]
    pub download_filename: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@mimeType")]
    pub mime_type: Option<String>,
    pub md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ExternalLink
{
    #[serde(rename = "@url")]
    pub url: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Parties
{
    #[serde(rename = "funderId", default)]
    pub funder_id: Vec<String>,

    #[serde(rename = "contactId", default)]
    pub contact_id: Vec<String>,

    #[serde(rename = "sponsorId", default)]
    pub sponsor_id: Vec<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Miscellaneous
{
    #[serde(rename = "ipdSharingPlan", default)]
    ipd_sharing_plan: Option<String>, // Yes or No
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct AttachedFile
{
    #[serde(rename = "@downloadUrl")]
    pub download_url: String,

    pub description: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub public: Option<String>,  // actually boolean

    #[serde(rename = "@mimeType")]
    pub mime_type: Option<String>,
    pub length: Option<String>,
    pub md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Contact
{
    #[serde(rename = "@id")]
    pub id: String,

    pub title:  Option<String>,
    pub forename:  Option<String>,
    pub surname:  Option<String>,
    pub orcid:  Option<String>,
  
    #[serde(rename = "contactTypes", default)]
    pub contact_types: Vec<ContactType>,

    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    pub privacy:  Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ContactType
{
    #[serde(rename = "contactType")]
    pub contact_type:  Option<String>,
}


#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Sponsor
{
    #[serde(rename = "@id")]
    pub id: String,

    pub organisation:  Option<String>,
    pub website:  Option<String>,

    #[serde(rename = "sponsorType")]
    pub sponsor_type:  Option<String>,

    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    pub privacy:  Option<String>,

    #[serde(rename = "rorId")]
    pub ror_id:  Option<String>,

    #[serde(rename = "commercialStatus")]
    pub commercial_status:  Option<String>,
}


#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ContactDetails
{
    pub address:  Option<String>,
    pub city:  Option<String>,
    pub state:  Option<String>,
    pub country:  Option<String>,
    pub zip:  Option<String>,
    pub telephone:  Option<String>,
    pub email:  Option<String>,
}


#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Funder
{
    #[serde(rename = "@id")]
    pub id: String,
    pub name:  Option<String>,
    #[serde(rename = "fundRef")]
    pub fund_ref: Option<String>,
}



#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn check_can_parse_fund_ref1() {

        let xml_string = r#"<funder id="6bb69969-98bf-47fa-aa68-1f4e5123e3a9">
            <name>Queen Victoria Hospital NHS Trust (UK)</name>
        </funder>"#;
        let add = Funder {
            id: "6bb69969-98bf-47fa-aa68-1f4e5123e3a9".to_string(),
            name: Some("Queen Victoria Hospital NHS Trust (UK)".to_string()),
            fund_ref: None,
        };
        let der_struct: Funder = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(add, der_struct);
    }  

    #[test]
    fn check_can_parse_fund_ref2() {

        let xml_string = r#"<funder id="fff95a08-8d06-4d83-8fb3-3485ae0d5824">
            <name>Federal Centre for Health Education (BZgA) (Germany)</name>
            <fundRef>http://dx.doi.org/10.13039/501100003108</fundRef>
        </funder>"#;
        let add = Funder {
            id: "fff95a08-8d06-4d83-8fb3-3485ae0d5824".to_string(),
            name: Some("Federal Centre for Health Education (BZgA) (Germany)".to_string()),
            fund_ref: Some("http://dx.doi.org/10.13039/501100003108".to_string()),
        };
        let der_struct: Funder = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(add, der_struct);
    }  

    #[test]
    fn check_can_parse_contact_details1() {

        let xml_string = r#"<contactDetails>
            <address>Academic Department O&amp;G, 3rd Floor Birmingham Women's Hospital Mindelsohn Way</address>
            <city>Edgbaston</city>
            <state/>
            <country>United Kingdom</country>
            <zip>B15 2TG</zip>
            <telephone/>
            <email/>
        </contactDetails>"#;

        let cdets = ContactDetails {
            address: Some("Academic Department O&G, 3rd Floor Birmingham Women's Hospital Mindelsohn Way".to_string()),
            city: Some("Edgbaston".to_string()),
            state: Some("".to_string()),
            country: Some("United Kingdom".to_string()),
            zip: Some("B15 2TG".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);
    }  

    #[test]
    fn check_can_parse_contact_details2() {

        let xml_string = r#"<contactDetails>
            <address>Holtye Road West Sussex</address>
            <city>East Grinstead</city>
            <state/>
            <country>United Kingdom</country>
            <zip>RH19 3DZ</zip>
            <telephone>+44 (0)1342 414000</telephone>
            <email>hf@cct.com</email>
        </contactDetails>"#;
        let cdets = ContactDetails {
            address: Some("Holtye Road West Sussex".to_string()),
            city: Some("East Grinstead".to_string()),
            state: Some("".to_string()),
            country: Some("United Kingdom".to_string()),
            zip: Some("RH19 3DZ".to_string()),
            telephone: Some("+44 (0)1342 414000".to_string()),
            email: Some("hf@cct.com".to_string()),
        };
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);
    }  

    #[test]
    fn check_can_parse_sponsor() {

        let xml_string = r#"<sponsor id="ed9a1d4b-28fe-4bfa-afda-b7f79196de22">
            <organisation>
            Federal Centre for Health Education (BZgA) (Germany)
            </organisation>
            <website/>
            <sponsorType>Government</sponsorType>
            <contactDetails>
                <address/>
                <city/>
                <state/>
                <country/>
                <zip/>
                <telephone/>
                <email/>
            </contactDetails>
            <privacy/>
            <rorId>https://ror.org/054c9y537</rorId>
            <commercialStatus>Non-commercial</commercialStatus>
        </sponsor>"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        let sp = Sponsor {
            id: "ed9a1d4b-28fe-4bfa-afda-b7f79196de22".to_string(),
            organisation: Some("\n            Federal Centre for Health Education (BZgA) (Germany)\n            ".to_string()),
            website: Some("".to_string()),
            sponsor_type: Some("Government".to_string()),
            contact_details: cdets,
            privacy: Some("".to_string()),
            ror_id: Some("https://ror.org/054c9y537".to_string()),
            commercial_status: Some("Non-commercial".to_string())
        };
        let der_struct: Sponsor = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(sp, der_struct);
    }  

    #[test]
    fn check_can_parse_contact() {

        let xml_string = r#"<contact id="8fad61c9-9167-4fc3-a286-826b5eeec568">
            <title>Mr</title>
            <forename>Benjamin</forename>
            <surname>Jonas</surname>
            <orcid/>
            <contactTypes>
                <contactType>Scientific</contactType>
            </contactTypes>
            <contactDetails>
                <address>Delphi GmbH Kaiserdamm 8</address>
                <city>Berlin</city>
                <state/>
                <country>Germany</country>
                <zip>14057</zip>
                <telephone>-</telephone>
                <email>jonas@delphi-gesellschaft.de</email>
            </contactDetails>
            <privacy>Public</privacy>
        </contact>"#;

        let ct = ContactType {
            contact_type: Some("Scientific".to_string()),
        };
        let cdets = ContactDetails {
            address: Some("Delphi GmbH Kaiserdamm 8".to_string()),
            city: Some("Berlin".to_string()),
            state: Some("".to_string()),
            country: Some("Germany".to_string()),
            zip: Some("14057".to_string()),
            telephone: Some("-".to_string()),
            email: Some("jonas@delphi-gesellschaft.de".to_string()),
        };
        let cn  = Contact {
            id: "8fad61c9-9167-4fc3-a286-826b5eeec568".to_string(),
            title: Some("Mr".to_string()),
            forename: Some("Benjamin".to_string()),
            surname: Some("Jonas".to_string()),
            orcid: Some("".to_string()),
            contact_types: vec![ct],
            contact_details: cdets,
            privacy: Some("Public".to_string()),
        };
        let der_struct: Contact = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cn, der_struct);
    }  

    #[test]
    fn check_can_parse_trial_agents() {

        let xml_string = r#"<trialAgents>
            <contact id="a407a287-d1a5-4f7f-921b-c9a4cf8a8efc">
                <title>Mr</title>
                <forename>Christopher</forename>
                <surname>Hutchison</surname>
                <orcid/>
                <contactTypes>
                    <contactType>Public</contactType>
                </contactTypes>
                <contactDetails>
                    <address>Ul. Plachkovitsa 1, Entr. A, Floor 5, Apt.18 </address>
                    <city>Sofia</city>
                    <state/>
                    <country>Bulgaria</country>
                    <zip>1164</zip>
                    <telephone/>
                    <email/>
                </contactDetails>
                <privacy>Protected</privacy>
            </contact>

            <contact id="0a74c913-56b2-4368-8977-b3f0eaeb481b">
                <title>Dr</title>
                <forename>Fergus</forename>
                <surname>Jepson</surname>
                <orcid/>
                <contactTypes>
                    <contactType>Scientific</contactType>
                </contactTypes>
                <contactDetails>
                    <address>
                    Preston Specialist Mobility Rehabilitation Centre Preston Business Centre Watling Street Road
                    </address>
                    <city> Fulwood, Preston</city>
                    <state/>
                    <country>United Kingdom</country>
                    <zip>PR2 8DY</zip>
                    <telephone/>
                    <email/>
                </contactDetails>
                <privacy>Protected</privacy>
            </contact>

            <sponsor id="cedbe54d-5e00-48c8-b8ee-7ad07431c796">
                <organisation>ProsFit Technologies UK Ltd.</organisation>
                <website/>
                <sponsorType>Industry</sponsorType>
                <contactDetails>
                    <address/>
                    <city/>
                    <state/>
                    <country/>
                    <zip/>
                    <telephone/>
                    <email/>
                </contactDetails>
                <privacy/>
                <commercialStatus>Commercial</commercialStatus>
            </sponsor>

            <funder id="531d4e35-108e-4dbc-b20d-2de7c3f5945c">
                <name>The Richard and Jack Wiseman Trust</name>
            </funder>

            <funder id="d73d9e47-ac72-48f5-9462-829d004abd77">
                <name>British Maternal and Fetal Medicine Society</name>
            </funder>
        </trialAgents>"#;

        let ct1 = ContactType {
            contact_type: Some("Public".to_string()),
        };
        let cdets1 = ContactDetails {
            address: Some("Ul. Plachkovitsa 1, Entr. A, Floor 5, Apt.18 ".to_string()),
            city: Some("Sofia".to_string()),
            state: Some("".to_string()),
            country: Some("Bulgaria".to_string()),
            zip: Some("1164".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        let c1  = Contact {
            id: "a407a287-d1a5-4f7f-921b-c9a4cf8a8efc".to_string(),
            title: Some("Mr".to_string()),
            forename: Some("Christopher".to_string()),
            surname: Some("Hutchison".to_string()),
            orcid: Some("".to_string()),
            contact_types: vec![ct1],
            contact_details: cdets1,
            privacy: Some("Protected".to_string()),
        };
        let ct2 = ContactType {
            contact_type: Some("Scientific".to_string()),
        };
        let cdets2 = ContactDetails {
            address: Some("\n                    Preston Specialist Mobility Rehabilitation Centre Preston Business Centre Watling Street Road\n                    ".to_string()),
            city: Some(" Fulwood, Preston".to_string()),
            state: Some("".to_string()),
            country: Some("United Kingdom".to_string()),
            zip: Some("PR2 8DY".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        let c2  = Contact {
            id: "0a74c913-56b2-4368-8977-b3f0eaeb481b".to_string(),
            title: Some("Dr".to_string()),
            forename: Some("Fergus".to_string()),
            surname: Some("Jepson".to_string()),
            orcid: Some("".to_string()),
            contact_types: vec![ct2],
            contact_details: cdets2,
            privacy: Some("Protected".to_string()),
        };
        let cdets3 = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        let sp = Sponsor {
            id: "cedbe54d-5e00-48c8-b8ee-7ad07431c796".to_string(),
            organisation: Some("ProsFit Technologies UK Ltd.".to_string()),
            website: Some("".to_string()),
            sponsor_type: Some("Industry".to_string()),
            contact_details: cdets3,
            privacy: Some("".to_string()),
            ror_id: None,
            commercial_status: Some("Commercial".to_string())
        };
        let f1 = Funder {
            id: "531d4e35-108e-4dbc-b20d-2de7c3f5945c".to_string(),
            name: Some("The Richard and Jack Wiseman Trust".to_string()),
            fund_ref: None,
        };
        let f2 = Funder {
            id: "d73d9e47-ac72-48f5-9462-829d004abd77".to_string(),
            name: Some("British Maternal and Fetal Medicine Society".to_string()),
            fund_ref: None,
        };
        let ta  = TrialAgents {
            contact: vec![c1, c2],
            sponsor: vec![sp],
            funder: vec![f1, f2],
        };
        let der_struct: TrialAgents = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(ta, der_struct);
    }  

    #[test]
    fn check_can_parse_isrctn() {

        let xml_string = r#"<isrctn dateAssigned="2025-02-26T07:23:16.665489Z">10601218</isrctn>"#;
      
        let isrctn = Isrctn {
            date_assigned: Some("2025-02-26T07:23:16.665489Z".to_string()),
            value: 10601218,
        };
        let der_struct: Isrctn = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(isrctn, der_struct);
    }  

     #[test]
    fn check_can_parse_secondary_numbers() {

        let xml_string = r#"<secondaryNumbers>
                <secondaryNumber id="a100d9b2-ad1c-4f87-af8e-667c9ea507d0" numberType="ctis">Nil known</secondaryNumber>
                <secondaryNumber id="532f4409-ad5e-4f99-9556-c1e8ff69f086" numberType="nct">Nil known</secondaryNumber>
                <secondaryNumber id="6dc730b6-55a9-4b9d-8cc5-d148677fb537" numberType="Protocol serial number">CPMS2023</secondaryNumber>
            </secondaryNumbers>"#;

        let secnum1 = SecondaryNumber {
            id: Some("a100d9b2-ad1c-4f87-af8e-667c9ea507d0".to_string()),
            number_type: Some("ctis".to_string()),
            value:Some("Nil known".to_string()), 
        };
        let secnum2 = SecondaryNumber {
            id: Some("532f4409-ad5e-4f99-9556-c1e8ff69f086".to_string()),
            number_type: Some("nct".to_string()),
            value:Some("Nil known".to_string()), 
        };
        let secnum3 = SecondaryNumber {
            id: Some("6dc730b6-55a9-4b9d-8cc5-d148677fb537".to_string()),
            number_type: Some("Protocol serial number".to_string()),
            value:Some("CPMS2023".to_string()), 
        };
        let sec_num_list = SecondaryNumberList {
            secondary_number: vec![secnum1, secnum2, secnum3],
        };
        
        let der_struct: SecondaryNumberList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(sec_num_list, der_struct);

    }  

    #[test]
    fn check_can_parse_external_refs() {

        let xml_string = r#"
        <externalRefs>
            <doi>10.1186/ISRCTN10601218</doi>
            <eudraCTNumber>Nil known</eudraCTNumber>
            <irasNumber/>
            <clinicalTrialsGovNumber>Nil known</clinicalTrialsGovNumber>
            <protocolSerialNumber>CPMS2023</protocolSerialNumber>
            <secondaryNumbers>
                <secondaryNumber id="a100d9b2-ad1c-4f87-af8e-667c9ea507d0" numberType="ctis">Nil known</secondaryNumber>
                <secondaryNumber id="532f4409-ad5e-4f99-9556-c1e8ff69f086" numberType="nct">Nil known</secondaryNumber>
                <secondaryNumber id="6dc730b6-55a9-4b9d-8cc5-d148677fb537" numberType="Protocol serial number">CPMS2023</secondaryNumber>
            </secondaryNumbers>
        </externalRefs>"#;

        let secnum1 = SecondaryNumber {
            id: Some("a100d9b2-ad1c-4f87-af8e-667c9ea507d0".to_string()),
            number_type: Some("ctis".to_string()),
            value:Some("Nil known".to_string()), 
        };
        let secnum2 = SecondaryNumber {
            id: Some("532f4409-ad5e-4f99-9556-c1e8ff69f086".to_string()),
            number_type: Some("nct".to_string()),
            value:Some("Nil known".to_string()), 
        };
        let secnum3 = SecondaryNumber {
            id: Some("6dc730b6-55a9-4b9d-8cc5-d148677fb537".to_string()),
            number_type: Some("Protocol serial number".to_string()),
            value:Some("CPMS2023".to_string()), 
        };
        let sec_num_list = SecondaryNumberList {
            secondary_number: vec![secnum1, secnum2, secnum3],
        };

        let exrefs = ExternalRefs {
            doi: Some("10.1186/ISRCTN10601218".to_string()),
            eudra_ct_number: Some("Nil known".to_string()),
            iras_number: Some("".to_string()),
            ctg_number: Some("Nil known".to_string()),
            protocol_serial_number: Some("CPMS2023".to_string()),
            secondary_numbers: sec_num_list,
        };
        
        let der_struct: ExternalRefs = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(exrefs, der_struct);

    }  

    #[test]
    fn check_can_parse_external_refs2() {

        let xml_string = r#"
        <externalRefs>
            <doi>10.1186/ISRCTN10601218</doi>
            <eudraCTNumber>Nil known</eudraCTNumber>
            <irasNumber/>
            <clinicalTrialsGovNumber>Nil known</clinicalTrialsGovNumber>
            <protocolSerialNumber>CPMS2023</protocolSerialNumber>
            <secondaryNumbers/>
        </externalRefs>"#;

        let sec_num_list = SecondaryNumberList {
            secondary_number: vec![],
        };

        let exrefs = ExternalRefs {
            doi: Some("10.1186/ISRCTN10601218".to_string()),
            eudra_ct_number: Some("Nil known".to_string()),
            iras_number: Some("".to_string()),
            ctg_number: Some("Nil known".to_string()),
            protocol_serial_number: Some("CPMS2023".to_string()),
            secondary_numbers: sec_num_list,
        };
        
        let der_struct: ExternalRefs = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(exrefs, der_struct);

    }

    #[test]
    fn check_can_parse_trial_description() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }  


    #[test]
    fn check_can_parse_trial_design() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }

    #[test]
    fn check_can_parse_participants() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }

    #[test]
    fn check_can_parse_conditions() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }
    
    #[test]
    fn check_can_parse_interventions() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }

    #[test]
    fn check_can_parse_results() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }

#[test]
    fn check_can_parse_outputs() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }


    #[test]
    fn check_can_parse_parties() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }

    #[test]
    fn check_can_parse_miscellaneous() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }

    #[test]
    fn check_can_parse_attached_files() {

        let xml_string = r#"
        <contactDetails>
            <address></address>
            <city></city>
            <state/>
            <country></country>
            <zip></zip>
            <telephone></telephone>
            <email></email>
        </contactDetails>
"#;

        let cdets = ContactDetails {
            address: Some("".to_string()),
            city: Some("".to_string()),
            state: Some("".to_string()),
            country: Some("".to_string()),
            zip: Some("".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };
        
        let der_struct: ContactDetails = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(cdets, der_struct);

    }
    /*



    */
}