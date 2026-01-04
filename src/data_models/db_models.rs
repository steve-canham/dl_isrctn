use chrono::{NaiveDate, NaiveDateTime};


pub struct DBStudy {

    pub sd_sid: String,

    pub summary: DBSummary,
    pub dates: DBStudyDates,
    pub participants: DBStudyPartics,

    pub titles: Option<Vec<DBTitle>>, 
    pub identifiers: Option<Vec<DBIdentifier>>,

    /*
    pub orgs: Option<Vec<DBOrganisation>>, 
    pub people: Option<Vec<DBPeople>>, 

    pub ie_crit: Option<Vec<DBIECriterion>>, 

    pub locations: Option<Vec<DBLocation>>, 
    pub countries: Option<Vec<DBCountry>>, 

    pub topics: Option<Vec<DBTopic>>, 
    pub conditions: Option<Vec<DBCondition>>, 
    pub features: Option<Vec<DBFeature>>, 

    pub relationships: Option<Vec<DBRelationship>>, 
    pub references: Option<Vec<DBReference>>, 
    pub available_material: Option<Vec<DBIPDAvail>>, 

    pub data_objects: Option<Vec<DBDataObject>>, 
*/
}
#[allow(dead_code)]
pub struct DBDataObject {

    pub sd_sid: String,
    pub sd_oid: String,

    pub summary: DBDataObjectSummary,
    pub dataset: Option<DBObjDataSet>,

    pub obj_titles: Option<Vec<DBObjTitle>>,
    pub obj_instances: Option<Vec<DBObjInstance>>,
    pub obj_dates: Option<Vec<DBObjDate>>,

}

pub struct DBSummary {
    pub display_title: String,
    pub brief_description: String,

    pub type_id: i32,
	pub status_id: i32,

    pub status_override: Option<String>,
    pub start_status_override: Option<String>,

    pub iec_flag: i32,
    pub ipd_sharing: bool,
	pub ipd_sharing_plan: Option<String>,
    pub date_last_revised: Option<NaiveDate>,
	pub dt_of_data_fetch: NaiveDateTime,
}

pub struct DBStudyDates {
    pub reg_year: Option<i32>,  
	pub reg_month: Option<i32>,
    pub reg_date_type: Option<String>,         
	pub start_year: Option<i32>, 
	pub start_month: Option<i32>,   
    pub start_date_type: Option<String>,       
	pub comp_year: Option<i32>,
	pub comp_month: Option<i32>,  
	pub comp_date_type: Option<String>,      
	pub res_year: Option<i32>,  
	pub res_month: Option<i32>,   
    pub res_date_type: Option<String>,    
}

pub struct DBStudyPartics {

    pub enrolment_target: Option<String>, 
    pub enrolment_final: Option<String>, 
    pub enrolment_total: Option<String>, 

    pub enrolment: Option<String>, 
	pub enrolment_type: Option<String>,

	pub gender_flag: Option<String>,
    pub min_age_as_string: Option<String>,
	pub min_age: Option<f32>,  
	pub min_age_units_id: Option<i32>,
    pub max_age_as_string: Option<String>,
	pub max_age: Option<f32>,  
	pub max_age_units_id: Option<i32>, 
	pub age_group_flag: Option<i32>, 
}


pub struct DBTitle {
    pub title_type_id: i32,
    pub title_text: String,
    pub is_default: bool,
    pub comment: Option<String>,
}

pub struct DBIdentifier {
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
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

#[allow(dead_code)]
pub struct DBIECriterion {
    pub ie_type_id:  i32,
    pub ie_num: i32,
    pub criterion: i32,
}

#[allow(dead_code)]
pub struct DBLocation {
    pub facility: String,
    pub address: Option<String>,
    pub city_name: Option<String>,
    pub disamb_name: Option<String>,
    pub country_name: Option<String>,
}

#[allow(dead_code)]
pub struct DBCountry {
    pub country_name: String,
}

#[allow(dead_code)]
pub struct DBTopic {
    pub original_value: String,
    pub topic_type_id:  i32,
    pub ct_type_id: Option<i32>,
    pub ct_code: Option<String>,
}

#[allow(dead_code)]
pub struct DBCondition {
    pub original_value: String,
    pub original_class1: Option<String>,
    pub original_class2: Option<String>,
    pub ct_type_id: Option<i32>,
    pub ct_code: Option<String>,
}

#[allow(dead_code)]
pub struct DBFeature {
    pub original_value: String,
    pub feature_type_id: i32,
    pub feature_value_id: i32,
}

#[allow(dead_code)]
pub struct DBRelationship {
    pub relationship_type_id: i32,
    pub target_sd_sid: String,
}

#[allow(dead_code)]
pub struct DBReference {
    pub pmid: Option<String>,
    pub citation: Option<String>,
    pub doi: Option<String>,
    pub type_id: Option<i32>,
    pub comments: Option<String>,
}

#[allow(dead_code)]
pub struct DBIPDAvail {
    pub ipd_name: String,
    pub ipd_type:Option<String>,
    pub ipd_url: Option<String>,
    pub ipd_comment: Option<String>,
}

#[allow(dead_code)]
pub struct DBDataObjectSummary {
    pub title: String,
    pub version: Option<i32>,
    pub display_title: String,

    pub doi: Option<String>,
    pub publication_year:Option<String>,
    pub object_class_id: Option<i32>,
    pub object_type_id: Option<i32>,

    pub managing_org: Option<String>,
    pub lang_code: Option<String>,
    pub access_type_id: Option<String>,
    pub access_details: Option<String>,
    pub access_details_url: Option<String>,

    pub eosc_category: Option<i32>,
    pub dt_of_data: NaiveDateTime,
}

#[allow(dead_code)]
pub struct DBObjDataSet {
    pub record_keys_type_id: Option<i32>,
    pub record_keys_details: Option<String>,
    pub deident_type_id: Option<i32>,
    pub deident_details: Option<String>,
    pub consent_type_id: Option<i32>,
    pub consent_details: Option<String>,
}

#[allow(dead_code)]
pub struct DBObjTitle {
    pub title_type_id: String,
    pub title_text: String,
    pub is_default: bool,
    pub comments: Option<String>,
}

#[allow(dead_code)]

pub struct DBObjInstance {
    pub system: String,
    pub url: Option<String>,
    pub url_accessible: bool,
    pub resource_type_id: Option<i32>,
    pub resource_size: Option<i32>,
    pub resource_size_units: Option<String>,
    pub resource_comments: Option<String>,
}

#[allow(dead_code)]
pub struct DBObjDate {
    pub date_type_id: i32,
    pub date_is_range: bool,
    pub date_as_string: String,
    pub start_year: i32,
    pub start_month: i32,
    pub start_day: i32,
    pub end_year: Option<i32>,
    pub end_month: Option<i32>,
    pub end_day: Option<i32>, 
    pub details: Option<String>,
}
