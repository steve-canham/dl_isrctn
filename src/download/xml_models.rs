//use chrono::NaiveDate;

#[allow(dead_code)]

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename = "allTrials")]
pub struct AllTrials
{
    #[serde(rename = "@totalCount")]
    pub total_count: i32,

    #[serde(rename = "fullTrial")]
    pub full_trials: Vec<FullTrial>,
}

/* 
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialList
{
    #[serde(rename = "fullTrial", default)]
    pub full_trial: Vec<FullTrial>,
}
*/

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct FullTrial
{
    pub trial: Trial,

    #[serde(rename = "contact", default)]
    pub contacts: Vec<Contact>,
    #[serde(rename = "sponsor", default)]
    pub sponsors: Vec<Sponsor>,
    #[serde(rename = "funder", default)]
    pub funders: Vec<Funder>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialAgents          // stripped down version of FullTrial for testing purposes
{
    #[serde(rename = "contact", default)]
    pub contacts: Vec<Contact>,
    #[serde(rename = "sponsor", default)]
    pub sponsors: Vec<Sponsor>,
    #[serde(rename = "funder", default)]
    pub funders: Vec<Funder>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Trial
{
    #[serde(rename = "@lastUpdated")]
    pub last_updated:  Option<String>,
    #[serde(rename = "@version")]
    pub version:  Option<String>,

    pub isrctn: Isrctn,
    #[serde(rename = "trialDescription")]
    pub trial_description: Description,
    #[serde(rename = "externalRefs")]
    pub external_refs: ExternalRefs,
    #[serde(rename = "trialDesign")]
    pub trial_design:Design,
    pub participants: Participants,
    #[serde(rename = "conditions")]
    pub condition_list: ConditionList,
    #[serde(rename = "interventions")]
    pub intervention_list: InterventionList,
    pub results: Results,
    #[serde(rename = "outputs")]
    pub output_list: OutputList,
    pub parties: Parties,
    #[serde(rename = "attachedFiles")]
    pub attached_file_list: AttachedFileList,
    pub miscellaneous: Miscellaneous,

}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Isrctn
{
    #[serde(rename = "@dateAssigned")]
    pub date_assigned: Option<String>,
    #[serde(rename = "$value")]
    pub value: i32,
}

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
    #[serde(rename = "ethicsCommittees")]
    pub ethics_committee_list: EthicsCommitteeList,
    #[serde(rename = "ethicsApproval")]
    pub ethics_approval: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct EthicsCommitteeList
{
    #[serde(rename = "ethicsCommittee", default)]
    pub ethics_committees: Vec<EthicsCommittee>,
}

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
    pub secondary_number_list: SecondaryNumberList,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SecondaryNumberList
{
    #[serde(rename = "secondaryNumber", default)]
    pub secondary_numbers: Option<Vec<SecondaryNumber>>,
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

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Design
{
    #[serde(rename = "studyDesign")]
    pub study_design: Option<String>,
    #[serde(rename = "primaryStudyDesign")]
    pub primary_study_design: Option<String>,
    #[serde(rename = "secondaryStudyDesign")]
    pub secondary_study_design: Option<String>,
    #[serde(rename = "trialSettings")]
    pub trial_setting_list: TrialSettingList,
    #[serde(rename = "trialTypes")]
    pub trial_type_list: TrialTypeList,
    #[serde(rename = "overallEndDate")]
    pub overall_end_date: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialSettingList
{
    #[serde(rename = "trialSetting", default)]
    pub trial_settings: Vec<TrialSetting>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialSetting
{
    #[serde(rename = "$value")]
    pub trial_setting: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialTypeList
{
    #[serde(rename = "trialType", default)]
    pub trial_types: Vec<TrialType>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct TrialType
{
    #[serde(rename = "$value")]
    pub trial_type: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Participants
{
    #[serde(rename = "recruitmentCountries")]
    pub country_list: CountryList,

    #[serde(rename = "trialCentres")]
    pub centre_list: CentreList,
   
    #[serde(rename = "participantTypes")]
    pub participant_type_list: ParticipantTypeList,

    pub inclusion: Option<String>,
    #[serde(rename = "ageRange")]
    pub age_range: Option<String>,

    #[serde(rename = "lowerAgeLimit")]
    pub lower_age_limit: Option<AgeLimit>,
    #[serde(rename = "upperAgeLimit")]
    pub upper_age_limit: Option<AgeLimit>,

    pub gender: Option<String>,
    #[serde(rename = "targetEnrolment")]
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

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct CountryList
{
    #[serde(rename = "country", default)]
    pub countries: Vec<Country>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Country
{
    #[serde(rename = "$value")]
    pub country: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct CentreList
{
    #[serde(rename = "trialCentre")]
    pub centres: Vec<Centre>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Centre
{
    #[serde(rename = "@id")]
    pub id: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,

}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ParticipantTypeList
{
    #[serde(rename = "participantType", default)]
    pub participant_types: Vec<ParticipantType>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ParticipantType
{
    #[serde(rename = "$value")]
    pub participant_type: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct AgeLimit
{
    #[serde(rename = "@unit")]
    pub unit: Option<String>,
    #[serde(rename = "@value")]
    pub num_unit: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ConditionList
{
    #[serde(rename = "condition", default)]
    pub conditions: Vec<Condition>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Condition
{
    pub description: Option<String>,

    #[serde(rename = "diseaseClass1")]
    pub disease_class1: Option<String>,

    #[serde(rename = "diseaseClass2")]
    pub disease_class2: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct InterventionList
{
    #[serde(rename = "intervention", default)]
    pub interventions: Vec<Intervention>,
}

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

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Results
{
    #[serde(rename = "publicationPlan")]
    pub publication_plan: Option<String>,
    #[serde(rename = "ipdSharingStatement")]
    pub ipd_sharing_statement: Option<String>,
    #[serde(rename = "intentToPublish")]
    pub intent_to_publish: Option<String>,
    #[serde(rename = "dataPolicies")]
    pub data_policy_list: DataPolicyList,
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

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DataPolicyList
{
    #[serde(rename = "dataPolicy", default)]
    pub data_policies: Vec<DataPolicy>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct DataPolicy
{
    #[serde(rename = "$value")]
    pub data_policy: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct OutputList
{
    #[serde(rename = "output", default)]
    pub outputs: Option<Vec<Output>>,
}

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

    #[serde(rename = "externalLink")]
    pub external_link: Option<ExternalLink>,
    #[serde(rename = "localFile")]
    pub local_file: Option<LocalFile>,

    pub description: Option<String>,
    #[serde(rename = "productionNotes")]
    pub production_notes: Option<String>,

}

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
    #[serde(rename = "@length")]
    pub length: Option<String>,
    #[serde(rename = "@md5sum")]
    pub md5sum: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ExternalLink
{
    #[serde(rename = "@url")]
    pub url: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Parties
{
    #[serde(rename = "funderId", default)]
    pub funder_ids: Option<Vec<String>>,

    #[serde(rename = "contactId", default)]
    pub contact_ids: Option<Vec<String>>,

    #[serde(rename = "sponsorId", default)]
    pub sponsor_ids: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Miscellaneous
{
    #[serde(rename = "ipdSharingPlan", default)]
    pub ipd_sharing_plan: Option<String>, // Yes or No
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct AttachedFileList
{
    #[serde(rename = "attachedFile", default)]
    pub attached_files: Option<Vec<AttachedFile>>
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct AttachedFile
{
    #[serde(rename = "@downloadUrl")]
    pub download_url: Option<String>,

    pub description: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub public: Option<String>,  // actually boolean
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub length: Option<String>,
    pub md5sum: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Contact
{
    #[serde(rename = "@id")]
    pub id: String,

    pub title:  Option<String>,
    pub forename:  Option<String>,
    pub surname:  Option<String>,
    pub orcid:  Option<String>,
  
    #[serde(rename = "contactTypes")]
    pub contact_type_list:ContactTypeList,

    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    pub privacy:  Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ContactTypeList
{
    #[serde(rename = "contactType", default)]
    pub contact_types:  Vec<ContactType>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ContactType
{
    #[serde(rename = "$value")]
    pub contact_type:  Option<String>,
}

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
            secondary_numbers: Some(vec![secnum1, secnum2, secnum3]),
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
            secondary_numbers: Some(vec![secnum1, secnum2, secnum3]),
        };

        let exrefs = ExternalRefs {
            doi: Some("10.1186/ISRCTN10601218".to_string()),
            eudra_ct_number: Some("Nil known".to_string()),
            iras_number: Some("".to_string()),
            ctg_number: Some("Nil known".to_string()),
            protocol_serial_number: Some("CPMS2023".to_string()),
            secondary_number_list: sec_num_list,
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
            secondary_numbers: None,
        };

        let exrefs = ExternalRefs {
            doi: Some("10.1186/ISRCTN10601218".to_string()),
            eudra_ct_number: Some("Nil known".to_string()),
            iras_number: Some("".to_string()),
            ctg_number: Some("Nil known".to_string()),
            protocol_serial_number: Some("CPMS2023".to_string()),
            secondary_number_list: sec_num_list,
        };
        
        let der_struct: ExternalRefs = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(exrefs, der_struct);

    }

    #[test]
    fn check_can_parse_trial_description() {

        let xml_string = r#"<trialDescription thirdPartyFilesAcknowledgement="true">
            <acknowledgment>true</acknowledgment>
            <title>Gene expression in bladder cancer and normal tissues</title>
            <scientificTitle>Bladder cancer and normal tissues</scientificTitle>
            <acronym/>
            <studyHypothesis>Expression of RBM15 in bladder cancer and normal tissue</studyHypothesis>
            <plainEnglishSummary>Background and study aims In this study, patients with bladder cancer were examined</plainEnglishSummary>
            <primaryOutcomes/>
            <primaryOutcome>Survival measured using data collected at an annual telephone call for 6 years</primaryOutcome>
            <secondaryOutcomes/>
            <secondaryOutcome>Transfer or not measured using data collected during a telephone call every 6 months</secondaryOutcome>
            <trialWebsite/>
            <ethicsApprovalRequired>Ethics approval required</ethicsApprovalRequired>
            <ethicsCommittees>
                <ethicsCommittee id="5247b77a-f096-40ed-9b87-e836ee9d7a68" approvalStatus="approved" statusDate="2022-03-10T00:00:00.000Z">
                <committeeName>Haikou Municipal People's Hospital Biomedical Ethics Committee</committeeName>
                <contactDetails>
                    <address>No.43 Renmin Avenue</address>
                    <city>Haikou</city>
                    <state/>
                    <country>China</country>
                    <zip>570208</zip>
                    <telephone/>
                    <email/>
                </contactDetails>
                <committeeReference>2023-055</committeeReference>
                </ethicsCommittee>
            </ethicsCommittees>
        </trialDescription>"#;

        let cdets = ContactDetails {
            address: Some("No.43 Renmin Avenue".to_string()),
            city: Some("Haikou".to_string()),
            state: Some("".to_string()),
            country: Some("China".to_string()),
            zip: Some("570208".to_string()),
            telephone: Some("".to_string()),
            email: Some("".to_string()),
        };

        let ecomm = EthicsCommittee {
            id: Some("5247b77a-f096-40ed-9b87-e836ee9d7a68".to_string()),
            approval_status: Some("approved".to_string()),
            status_date: Some("2022-03-10T00:00:00.000Z".to_string()),
            committee_name: Some("Haikou Municipal People's Hospital Biomedical Ethics Committee".to_string()),
            contact_details: cdets,
            committee_reference: Some("2023-055".to_string()),
        };

        let eclist = EthicsCommitteeList { 
            ethics_committees: vec![ecomm],
        };
        
        let td = Description {
            third_party_ack: Some("true".to_string()),
            acknowledgment: Some("true".to_string()),
            title: Some("Gene expression in bladder cancer and normal tissues".to_string()),
            scientific_title: Some("Bladder cancer and normal tissues".to_string()),
            acronym: Some("".to_string()),
            study_hypothesis: Some("Expression of RBM15 in bladder cancer and normal tissue".to_string()),
            plain_english_summary: Some("Background and study aims In this study, patients with bladder cancer were examined".to_string()),
            primary_outcomes: Some("".to_string()),
            primary_outcome: Some("Survival measured using data collected at an annual telephone call for 6 years".to_string()),
            secondary_outcomes: Some("".to_string()),
            secondary_outcome: Some("Transfer or not measured using data collected during a telephone call every 6 months".to_string()),
            trial_website: Some("".to_string()),
            ethics_approval_required: Some("Ethics approval required".to_string()),
            ethics_committee_list: eclist,
            ethics_approval: None,
        };

        let der_struct: Description = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(td, der_struct);

    }  

    #[test]
    fn check_can_parse_trial_design() {

        let xml_string = r#"<trialDesign>
            <studyDesign>Observational cohort study</studyDesign>
            <primaryStudyDesign>Observational</primaryStudyDesign>
            <secondaryStudyDesign>Cohort study</secondaryStudyDesign>
            <trialSettings/>
            <trialTypes>
                <trialType>Treatment</trialType>
                <trialType>Quality of life</trialType>
            </trialTypes>
            <overallEndDate>2024-06-12T00:00:00.000Z</overallEndDate>
        </trialDesign>"#;

        let tt1 = TrialType {
            trial_type: Some("Treatment".to_string()),
        };

        let tt2 = TrialType {
            trial_type: Some("Quality of life".to_string()),
        };

        let tt_list = TrialTypeList {
            trial_types: vec![tt1, tt2],
        };

        let ts_list = TrialSettingList {
            trial_settings: vec![],
        };

        let des = Design {
            study_design: Some("Observational cohort study".to_string()),
            primary_study_design: Some("Observational".to_string()),
            secondary_study_design: Some("Cohort study".to_string()),
            trial_type_list: tt_list,
            trial_setting_list: ts_list,
            overall_end_date: Some("2024-06-12T00:00:00.000Z".to_string()),
        };
        
        let der_struct: Design = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(des, der_struct);

    }

    #[test]
    fn check_can_parse_participants() {

        let xml_string = r#"<participants>
            <recruitmentCountries>
                <country>United Kingdom</country>
                <country>Saudi Arabia</country>
            </recruitmentCountries>
            <trialCentres>
                <trialCentre id="d1b6a41f-06b2-47d3-a920-9758f4dadbcc">
                    <name>Complife Italia srl</name>
                    <address>Corso San Maurizio, 25A </address>
                    <city>Biella (BI)</city>
                    <state/>
                    <country>Italy</country>
                    <zip>13900</zip>
                </trialCentre>
                <trialCentre id="d93398aa-14ab-447f-8d83-532dc4648fb9">
                    <name>Complife Italia srl</name>
                    <address>Via Fratelli Signorelli, 159</address>
                    <city>Garbagnate Milanese (MI)</city>
                    <state/>
                    <country>Italy</country>
                    <zip>20024</zip>
                    </trialCentre>
                </trialCentres>
            <participantTypes>
                <participantType>Healthy volunteer</participantType>
                <participantType>Patient</participantType>
            </participantTypes>
            <inclusion>1. Medically stable adults aged 18–80 years 2. Experienced a first-ever ischemic or hemorrhagic stroke</inclusion>
            <ageRange>Senior</ageRange>
            <lowerAgeLimit unit="years" value="18.0">18 Years</lowerAgeLimit>
            <upperAgeLimit unit="years" value="80.0">80 Years</upperAgeLimit>
            <gender>All</gender>
            <targetEnrolment>62</targetEnrolment>
            <totalFinalEnrolment>62</totalFinalEnrolment>
            <totalTarget/>
            <exclusion>1. Presence of severe spasticity (Modified Ashworth Scale score >3) 2. Currently participating in another rehabilitation program</exclusion>
            <patientInfoSheet/>
            <recruitmentStart>2024-01-01T00:00:00.000Z</recruitmentStart>
            <recruitmentEnd>2024-05-01T00:00:00.000Z</recruitmentEnd>
            <recruitmentStartStatusOverride/>
            <recruitmentStatusOverride/>
        </participants>"#;

        let cy1 = Country{
            country: Some("United Kingdom".to_string()),
        };

        let cy2 = Country{
            country: Some("Saudi Arabia".to_string()),
        };

        let cy_list = CountryList {
            countries: vec![cy1, cy2],
        };

        let cn1 = Centre {
            id: Some("d1b6a41f-06b2-47d3-a920-9758f4dadbcc".to_string()),
            name: Some("Complife Italia srl".to_string()),
            address: Some("Corso San Maurizio, 25A ".to_string()),
            city: Some("Biella (BI)".to_string()),
            state: Some("".to_string()),
            country: Some("Italy".to_string()),
            zip: Some("13900".to_string()),
        };

        let cn2 = Centre {
            id: Some("d93398aa-14ab-447f-8d83-532dc4648fb9".to_string()),
            name: Some("Complife Italia srl".to_string()),
            address: Some("Via Fratelli Signorelli, 159".to_string()),
            city: Some("Garbagnate Milanese (MI)".to_string()),
            state: Some("".to_string()),
            country: Some("Italy".to_string()),
            zip: Some("20024".to_string()),
        };

        let cn_list = CentreList {
            centres: vec![cn1, cn2],
        };

        let pt1 = ParticipantType {
            participant_type: Some("Healthy volunteer".to_string()),
        };

         let pt2 = ParticipantType {
            participant_type: Some("Patient".to_string()),
        };

        let pt_list = ParticipantTypeList {
            participant_types: vec![pt1, pt2],
        };

        let lal = AgeLimit {
            unit: Some("years".to_string()),
            num_unit: Some("18.0".to_string()),
            value: Some("18 Years".to_string()),
        };

        let ual = AgeLimit {
            unit: Some("years".to_string()),
            num_unit: Some("80.0".to_string()),
            value: Some("80 Years".to_string()),
        };
   
        let partics = Participants {
            country_list: cy_list,
            centre_list: cn_list,
            participant_type_list: pt_list,
            inclusion: Some("1. Medically stable adults aged 18–80 years 2. Experienced a first-ever ischemic or hemorrhagic stroke".to_string()),
            age_range: Some("Senior".to_string()),
            lower_age_limit: Some(lal),
            upper_age_limit: Some(ual),
            gender: Some("All".to_string()),
            target_enrolment: Some("62".to_string()),
            total_final_enrolment: Some("62".to_string()),

            total_target: Some("".to_string()),

            exclusion: Some("1. Presence of severe spasticity (Modified Ashworth Scale score >3) 2. Currently participating in another rehabilitation program".to_string()),
            patient_info_sheet: Some("".to_string()),

            recruitment_start: Some("2024-01-01T00:00:00.000Z".to_string()),
            recruitment_end: Some("2024-05-01T00:00:00.000Z".to_string()),
            recruitment_start_status_override: Some("".to_string()),
            recruitment_status_override: Some("".to_string()),
        };
        
        let der_struct: Participants = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(partics, der_struct);

    }

    #[test]
    fn check_can_parse_conditions() {

        let xml_string = r#"<conditions>
            <condition>
                <description>Bladder cancer</description>
                <diseaseClass1>Cancer</diseaseClass1>
                <diseaseClass2/>
            </condition>
        </conditions>"#;

        let c1 = Condition
        {
            description: Some("Bladder cancer".to_string()),
            disease_class1: Some("Cancer".to_string()),
            disease_class2: Some("".to_string()),
        };

        let c_list = ConditionList {
            conditions: vec![c1],
        };
        
        let der_struct: ConditionList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(c_list, der_struct);

    }

    #[test]
    fn check_can_parse_conditions2() {

        let xml_string = r#"<conditions>
            <condition>
                <description>Bladder cancer</description>
                <diseaseClass1>Cancer</diseaseClass1>
                <diseaseClass2/>
            </condition>
            <condition>
                <description>Therapeutic exercise for lateral epicondylitis</description>
                <diseaseClass1>Musculoskeletal Diseases</diseaseClass1>
                <diseaseClass2/>
            </condition>
        </conditions>"#;

        let c1 = Condition
        {
            description: Some("Bladder cancer".to_string()),
            disease_class1: Some("Cancer".to_string()),
            disease_class2: Some("".to_string()),
        };

        let c2 = Condition
        {
            description: Some("Therapeutic exercise for lateral epicondylitis".to_string()),
            disease_class1: Some("Musculoskeletal Diseases".to_string()),
            disease_class2: Some("".to_string()),
        };

        let c_list  = ConditionList {
            conditions: vec![c1, c2],
        };
        
        let der_struct: ConditionList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(c_list, der_struct);

    }

    #[test]
    fn check_can_parse_interventions() {

        let xml_string = r#"<interventions>
            <intervention>
                <description>This study enrols patients with bladder cancer, collects tissue samples at the time of surgery, and follows them for 6 years.</description>
                <interventionType>Other</interventionType>
                <pharmaceuticalStudyTypes/>
                <phase/>
                <drugNames/>
            </intervention>
        </interventions>"#;

        let i1 = Intervention {
            description: Some("This study enrols patients with bladder cancer, collects tissue samples at the time of surgery, and follows them for 6 years.".to_string()),
            intervention_type: Some("Other".to_string()),
            pharmaceutical_study_types: Some("".to_string()),
            phase: Some("".to_string()),
            drug_names: Some("".to_string()),
        };

        let i_list = InterventionList {
            interventions: vec![i1],
        };

        let der_struct: InterventionList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(i_list, der_struct);

    }
    
    #[test]
    fn check_can_parse_interventions2() {

        let xml_string = r#"<interventions>
            <intervention>
                <description>This study enrols patients with bladder cancer, collects tissue samples at the time of surgery, and follows them for 6 years.</description>
                <interventionType>Other</interventionType>
                <pharmaceuticalStudyTypes/>
                <phase/>
                <drugNames/>
            </intervention>
            <intervention>
                <description>Patient recruitment will take place in antenatal or labour ward of UMMC. All women admitted for a planned vaginal birth will be assessed for eligibility.</description>
                <interventionType>Drug</interventionType>
                <pharmaceuticalStudyTypes/>
                <phase>Not Applicable</phase>
                <drugNames>Oxytocin for injection (10 IU), syntometrine for injection (fixed dose oxytocin 5 IU and ergometrine 0.5 mg), oral misoprostol tablet, placebo tablet</drugNames>
            </intervention>
        </interventions>"#;

        let i1 = Intervention {
            description: Some("This study enrols patients with bladder cancer, collects tissue samples at the time of surgery, and follows them for 6 years.".to_string()),
            intervention_type: Some("Other".to_string()),
            pharmaceutical_study_types: Some("".to_string()),
            phase: Some("".to_string()),
            drug_names: Some("".to_string()),
        };

        let i2 = Intervention {
            description: Some("Patient recruitment will take place in antenatal or labour ward of UMMC. All women admitted for a planned vaginal birth will be assessed for eligibility.".to_string()),
            intervention_type: Some("Drug".to_string()),
            pharmaceutical_study_types: Some("".to_string()),
            phase: Some("Not Applicable".to_string()),
            drug_names: Some("Oxytocin for injection (10 IU), syntometrine for injection (fixed dose oxytocin 5 IU and ergometrine 0.5 mg), oral misoprostol tablet, placebo tablet".to_string()),
        };

        let i_list = InterventionList {
            interventions: vec![i1, i2],
        };
        
        let der_struct: InterventionList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(i_list, der_struct);

    }

    #[test]
    fn check_can_parse_results() {

        let xml_string = r#"<results>
            <publicationPlan/>
            <ipdSharingStatement>Raw data will be stored on Complife servers. A backup copy of the raw data will be also in a cloud-based backup server. Tables containing the raw data (output of the measurements) will also be included in the study report...</ipdSharingStatement>
            <intentToPublish>2025-06-30T00:00:00.000Z</intentToPublish>
            <dataPolicies>
                <dataPolicy>Stored in non-publicly available repository</dataPolicy>
                <dataPolicy>Some other data policy to be announced</dataPolicy>
            </dataPolicies>
            <publicationDetails/>
            <publicationStage/>
            <biomedRelated>false</biomedRelated>
            <basicReport>Basic results in https://clinicaltrials.servier.com/wp-content/uploads/CL2-95005-003-anonymised-synopsis.pdf</basicReport>
            <plainEnglishReport>https://clinicaltrials.servier.com/wp-content/uploads/CL2-95005-003-laysummary-2019.06.27-1.pdf</plainEnglishReport>
        </results>"#;

        let dp1 = DataPolicy {
            data_policy: Some("Stored in non-publicly available repository".to_string()),
        };

        let dp2 = DataPolicy {
            data_policy: Some("Some other data policy to be announced".to_string()),
        };

        let dp_list = DataPolicyList {
            data_policies: vec![dp1, dp2]
        };

        let res = Results {
            publication_plan: Some("".to_string()),
            ipd_sharing_statement: Some("Raw data will be stored on Complife servers. A backup copy of the raw data will be also in a cloud-based backup server. Tables containing the raw data (output of the measurements) will also be included in the study report...".to_string()),
            intent_to_publish: Some("2025-06-30T00:00:00.000Z".to_string()),
            data_policy_list: dp_list,
            publication_details: Some("".to_string()),
            publication_stage: Some("".to_string()),
            biomed_related: Some("false".to_string()),
            basic_report: Some("Basic results in https://clinicaltrials.servier.com/wp-content/uploads/CL2-95005-003-anonymised-synopsis.pdf".to_string()),
            plain_english_report: Some("https://clinicaltrials.servier.com/wp-content/uploads/CL2-95005-003-laysummary-2019.06.27-1.pdf".to_string()),
        };
        
        let der_struct: Results = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(res, der_struct);

    }

    #[test]
    fn check_can_parse_outputs1() {

        let xml_string = r#"<outputs>
            <output id="10c26a6d-033e-4698-ac3c-c033d035670d" outputType="pis" artefactType="LocalFile" dateCreated="2024-08-01T00:00:00.000Z" dateUploaded="2025-01-06T00:00:00.000Z" peerReviewed="false" patientFacing="true" createdBy="">
                <localFile fileId="e49116a0-7fc1-4827-910b-a8a51eb27504" originalFilename="46631_PIS_V1_01Aug24.pdf" downloadFilename="46631_PIS_V1_01Aug24.pdf" version="1" mimeType="application/pdf" length="171960" md5sum="07bd6ba0765c826cb24fadbcdb8a99a1"/>
                <description/>
                <productionNotes/>
            </output>
            <output id="990dba78-caa1-4a5f-9c11-bd4e18d54358" outputType="pis" artefactType="ExternalLink" dateCreated="2025-11-11T00:00:00.000Z" dateUploaded="2025-11-11T00:00:00.000Z" peerReviewed="false" patientFacing="true" createdBy="Migration">
                <externalLink url="See study outputs table"/>
                <description>Participant information sheet</description>
                <productionNotes>Migrated from patient info sheet field</productionNotes>
            </output>
            <output id="9896bfc4-8ddd-4b16-a8fe-5ab36a850ea3" outputType="protocolfile" artefactType="LocalFile" dateCreated="2024-08-01T00:00:00.000Z" dateUploaded="2025-01-06T00:00:00.000Z" peerReviewed="false" patientFacing="false" createdBy="">
                <localFile fileId="71516b58-3143-4702-87db-c6316a95c11e" originalFilename="46631_PROTOCOL_V1_01Aug24.pdf" downloadFilename="46631_PROTOCOL_V1_01Aug24.pdf" version="1" mimeType="application/pdf" length="389925" md5sum="5742d8a791fdd4b465224eca1e3dad67"/>
                <description/>
                <productionNotes/>
            </output>
        </outputs>"#;

        let lf1 = LocalFile {
            file_id: Some("e49116a0-7fc1-4827-910b-a8a51eb27504".to_string()),
            original_filename: Some("46631_PIS_V1_01Aug24.pdf".to_string()),
            download_filename: Some("46631_PIS_V1_01Aug24.pdf".to_string()),
            version: Some("1".to_string()),
            mime_type: Some("application/pdf".to_string()),
            length: Some("171960".to_string()),
            md5sum: Some("07bd6ba0765c826cb24fadbcdb8a99a1".to_string()),
        };

        let out1 = Output {
            id: Some("10c26a6d-033e-4698-ac3c-c033d035670d".to_string()),
            output_type: Some("pis".to_string()),
            artefact_type: Some("LocalFile".to_string()),
            date_created: Some("2024-08-01T00:00:00.000Z".to_string()),
            date_uploaded: Some("2025-01-06T00:00:00.000Z".to_string()),
            peer_reviewed: Some("false".to_string()),
            patient_facing: Some("true".to_string()),
            created_by: Some("".to_string()),
            external_link: None,
            local_file: Some(lf1),
            description: Some("".to_string()),
            production_notes: Some("".to_string()),
        };

        let el = ExternalLink { 
            url: Some("See study outputs table".to_string()),
        };

        let out2 = Output {
            id: Some("990dba78-caa1-4a5f-9c11-bd4e18d54358".to_string()),
            output_type: Some("pis".to_string()),
            artefact_type: Some("ExternalLink".to_string()),
            date_created: Some("2025-11-11T00:00:00.000Z".to_string()),
            date_uploaded: Some("2025-11-11T00:00:00.000Z".to_string()),
            peer_reviewed: Some("false".to_string()),
            patient_facing: Some("true".to_string()),
            created_by: Some("Migration".to_string()),
            external_link: Some(el),
            local_file: None,
            description: Some("Participant information sheet".to_string()),
            production_notes: Some("Migrated from patient info sheet field".to_string()),
        };

        let lf3 = LocalFile {
            file_id: Some("71516b58-3143-4702-87db-c6316a95c11e".to_string()),
            original_filename: Some("46631_PROTOCOL_V1_01Aug24.pdf".to_string()),
            download_filename: Some("46631_PROTOCOL_V1_01Aug24.pdf".to_string()),
            version: Some("1".to_string()),
            mime_type: Some("application/pdf".to_string()),
            length: Some("389925".to_string()),
            md5sum: Some("5742d8a791fdd4b465224eca1e3dad67".to_string()),
        };

        let out3 = Output {
            id: Some("9896bfc4-8ddd-4b16-a8fe-5ab36a850ea3".to_string()),
            output_type: Some("protocolfile".to_string()),
            artefact_type: Some("LocalFile".to_string()),
            date_created: Some("2024-08-01T00:00:00.000Z".to_string()),
            date_uploaded: Some("2025-01-06T00:00:00.000Z".to_string()),
            peer_reviewed: Some("false".to_string()),
            patient_facing: Some("false".to_string()),
            created_by: Some("".to_string()),
            external_link: None,
            local_file: Some(lf3),
            description: Some("".to_string()),
            production_notes: Some("".to_string()),
        };
        
        let outs = OutputList {
            outputs: Some(vec![out1, out2, out3])
        };
        
        let der_struct: OutputList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(outs, der_struct);

    }
    
    #[test]
    fn check_can_parse_outputs2() {

        let xml_string = r#"<outputs> </outputs>"#;

        let outs = OutputList {
            outputs: None
        };
        
        let der_struct: OutputList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(outs, der_struct);
    }

    #[test]
    fn check_can_parse_parties1() {

        let xml_string = r#"<parties>
            <funderId>70122d5b-2c2a-4bdf-a960-e4fc34a0d4f2</funderId>
            <contactId>b284a888-e980-458f-99eb-a5fb6b24d138</contactId>
            <contactId>dc48af05-8b07-453a-9481-58d4aafcde2b</contactId>
            <contactId>534b530d-dcb7-4c0b-91cf-503b5ed822e9</contactId>
            <sponsorId>92c18708-6d54-4c14-be73-fabf85ca4ff8</sponsorId>
        </parties>"#;

        let pars = Parties {
            funder_ids: Some(vec!["70122d5b-2c2a-4bdf-a960-e4fc34a0d4f2".to_string()]),
            contact_ids: Some(vec!["b284a888-e980-458f-99eb-a5fb6b24d138".to_string(), "dc48af05-8b07-453a-9481-58d4aafcde2b".to_string(), "534b530d-dcb7-4c0b-91cf-503b5ed822e9".to_string()]),
            sponsor_ids: Some(vec!["92c18708-6d54-4c14-be73-fabf85ca4ff8".to_string()]),
        };
        
        let der_struct: Parties = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(pars, der_struct);
    }

     #[test]
    fn check_can_parse_parties2() {

        let xml_string = r#"<parties>
            <sponsorId>92c18708-6d54-4c14-be73-fabf85ca4ff8</sponsorId>
        </parties>"#;

        let pars = Parties {
            funder_ids: None,
            contact_ids: None,
            sponsor_ids: Some(vec!["92c18708-6d54-4c14-be73-fabf85ca4ff8".to_string()]),
        };
        
        let der_struct: Parties = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(pars, der_struct);
    }

    #[test]
    fn check_can_parse_miscellaneous() {

        let xml_string = r#"<miscellaneous>
            <ipdSharingPlan>No</ipdSharingPlan>
        </miscellaneous>
"#;

        let mis: Miscellaneous = Miscellaneous {
            ipd_sharing_plan: Some("No".to_string()),
        };
        
        let der_struct: Miscellaneous = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(mis, der_struct);
    }

    #[test]
    fn check_can_parse_attached_files1() {

        let xml_string = r#"<attachedFiles/>"#;

        let atts = AttachedFileList {
            attached_files: None,
        };
        
        let der_struct: AttachedFileList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(atts, der_struct);
    }

    
    #[test]
    fn check_can_parse_attached_files2() {

        let xml_string = r#"<attachedFiles>
        <attachedFile downloadUrl="https://www.isrctn.com/editorial/retrieveFile/e49116a0-7fc1-4827-910b-a8a51eb27504/46631">
            <description>Participant information sheet</description>
            <name>46631_PIS_V1_01Aug24.pdf</name>
            <id>e49116a0-7fc1-4827-910b-a8a51eb27504</id>
            <public>true</public>
            <mimeType>application/pdf</mimeType>
            <length>171960</length>
            <md5sum>07bd6ba0765c826cb24fadbcdb8a99a1</md5sum>
        </attachedFile>
        <attachedFile downloadUrl="https://www.isrctn.com/editorial/retrieveFile/71516b58-3143-4702-87db-c6316a95c11e/46631">
            <description>Protocol file</description>
            <name>46631_PROTOCOL_V1_01Aug24.pdf</name>
            <id>71516b58-3143-4702-87db-c6316a95c11e</id>
            <public>true</public>
            <mimeType>application/pdf</mimeType>
            <length>389925</length>
            <md5sum>5742d8a791fdd4b465224eca1e3dad67</md5sum>
        </attachedFile>
    </attachedFiles>"#;

        let af1 = AttachedFile {
            download_url: Some("https://www.isrctn.com/editorial/retrieveFile/e49116a0-7fc1-4827-910b-a8a51eb27504/46631".to_string()),
            description: Some("Participant information sheet".to_string()),
            name: Some("46631_PIS_V1_01Aug24.pdf".to_string()),
            id: Some("e49116a0-7fc1-4827-910b-a8a51eb27504".to_string()),
            public: Some("true".to_string()),
            mime_type: Some("application/pdf".to_string()),
            length: Some("171960".to_string()),
            md5sum: Some("07bd6ba0765c826cb24fadbcdb8a99a1".to_string()),
        };

        let af2 = AttachedFile {
            download_url: Some("https://www.isrctn.com/editorial/retrieveFile/71516b58-3143-4702-87db-c6316a95c11e/46631".to_string()),
            description: Some("Protocol file".to_string()),
            name: Some("46631_PROTOCOL_V1_01Aug24.pdf".to_string()),
            id: Some("71516b58-3143-4702-87db-c6316a95c11e".to_string()),
            public: Some("true".to_string()),
            mime_type: Some("application/pdf".to_string()),
            length: Some("389925".to_string()),
            md5sum: Some("5742d8a791fdd4b465224eca1e3dad67".to_string()),
        };
        
        let atts = AttachedFileList {
            attached_files: Some(vec![af1, af2]),
        };
        
        let der_struct: AttachedFileList = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(atts, der_struct);

    }
     
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
                <contactType>Public</contactType>
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

        let ct1 = ContactType {
            contact_type: Some("Scientific".to_string()),
        };
        let ct2 = ContactType {
            contact_type: Some("Public".to_string()),
        };

        let ct_list = ContactTypeList {
            contact_types: vec![ct1, ct2],
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
            contact_type_list: ct_list,
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

        let ct_list1 = ContactTypeList {
            contact_types: vec![ct1],
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
            contact_type_list: ct_list1,
            contact_details: cdets1,
            privacy: Some("Protected".to_string()),
        };

        let ct2 = ContactType {
            contact_type: Some("Scientific".to_string()),
        };

        let ct_list2 = ContactTypeList {
            contact_types: vec![ct2],
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
            contact_type_list: ct_list2,
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
            contacts: vec![c1, c2],
            sponsors: vec![sp],
            funders: vec![f1, f2],
        };
        let der_struct: TrialAgents = quick_xml::de::from_str(xml_string).unwrap();
        assert_eq!(ta, der_struct);
    }  


    /*



    */
}