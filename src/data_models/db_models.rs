use chrono::{DateTime, Utc};


#[allow(dead_code)]
pub struct DBStudy {

    pub sd_sid: String,

    pub summary: DBSummary,
    pub dates: DBStudyDates,
    pub participants: DBStudyPartics,

    pub identifiers: Option<Vec<DBIdentifier>>,
    pub titles: Option<Vec<DBTitle>>, 

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

}
#[allow(dead_code)]
pub struct DBDataObject {

    pub sd_sid: String,

    pub summary: DBDataObjectSummary,
    pub dataset: Option<DBObjDataSet>,

    pub obj_titles: Option<Vec<DBObjTitle>>,
    pub obj_instances: Option<Vec<DBObjInstance>>,
    pub obj_dates: Option<Vec<DBObjDate>>,

}

#[allow(dead_code)]
pub struct DBSummary {
    pub sd_sid: String,
    pub display_title: Option<String>,
    pub brief_description: Option<String>,

    pub type_id: Option<i32>,
	pub status_id: Option<i32>,
    pub iec_flag: Option<i32>,
    pub ipd_sharing: Option<bool>,
	pub ipd_sharing_plan: Option<String>,
	pub dt_of_data: DateTime<Utc>,
}

#[allow(dead_code)]
pub struct DBStudyDates {
    pub sd_sid: String,
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

#[allow(dead_code)]
pub struct DBStudyPartics {
    pub sd_sid: String,
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


#[allow(dead_code)]
pub struct DBTitle {
    pub sd_sid: String,
    pub title_type_id: i32,
    pub title_text: String,
    pub is_default: bool,
    pub comment: Option<String>,
}

pub struct DBIdentifier {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBOrganisation {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBPeople {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBIECriterion {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBLocation {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBCountry {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBTopic {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBCondition {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBFeature {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBRelationship {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBReference {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBIPDAvail {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBDataObjectSummary {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}
#[allow(dead_code)]
pub struct DBObjDataSet {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBObjTitle {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]

pub struct DBObjInstance {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

#[allow(dead_code)]
pub struct DBObjDate {
    pub sd_sid: String,
    pub id_value: String,
    pub id_type_id: i32,
    pub id_type: String,
}

/*


fn get_orgs_sql<'a>() -> &'a str {
    
    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_organisations;
    CREATE TABLE sd.study_organisations(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , contrib_type_id        INT             NULL
    , organisation_id        INT             NULL
    , organisation_name      VARCHAR         NULL
    , organisation_ror_id    VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL
    );
    CREATE INDEX study_organisations_sid ON sd.study_organisations(sd_sid);"#
}

/*



#[derive(serde::Serialize)]
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

#[derive(serde::Serialize)]
pub struct StudyFunder
{
    pub name: Option<String>,
    pub fund_ref: Option<String>,
}

#[derive(serde::Serialize)]
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
*/

fn get_people_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_people;
    CREATE TABLE sd.study_people(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , contrib_type_id        INT             NULL
    , person_given_name      VARCHAR         NULL
    , person_family_name     VARCHAR         NULL
    , person_full_name       VARCHAR         NULL
    , orcid_id               VARCHAR         NULL
    , person_affiliation     VARCHAR         NULL
    , organisation_id        INT             NULL
    , organisation_name      VARCHAR         NULL
    , organisation_ror_id    VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL
    );
    CREATE INDEX study_people_sid ON sd.study_people(sd_sid);"#
}

fn get_ie_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_titles;
    CREATE TABLE sd.study_titles(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , ie_type_id             INT                
    , unsplit_text           VARCHAR
    , ie_num                 int          
    , criterion              VARCHAR
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_titles_sid ON sd.study_titles(sd_sid);"#
}

fn get_locations_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_locations;
    CREATE TABLE sd.study_locations(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , facility_org_id        INT             NULL
    , facility               VARCHAR         NULL
    , facility_ror_id        VARCHAR         NULL
    , city_id                INT             NULL
    , city_name              VARCHAR         NULL
    , disamb_id              INT             NULL
    , disamb_name            VARCHAR         NULL
    , country_id             INT             NULL
    , country_name           VARCHAR         NULL
    , status_id              INT             NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL          
    );
    CREATE INDEX study_locations_sid ON sd.study_locations(sd_sid);"#
}

fn get_countries_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_countries;
    CREATE TABLE sd.study_countries(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , country_id             INT             NULL
    , country_name           VARCHAR         NULL
    , status_id              INT             NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL  default now()       -- already coded when added                                   
    );
    CREATE INDEX study_countries_sid ON sd.study_countries(sd_sid);"#
}

fn get_topics_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_topics;
    CREATE TABLE sd.study_topics(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , topic_type_id          INT             NULL
    , original_value         VARCHAR         NULL       
    , original_ct_type_id    INT             NULL
    , original_ct_code       VARCHAR         NULL 
    , mesh_code              VARCHAR         NULL
    , mesh_value             VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL
    );
    CREATE INDEX study_topics_sid ON sd.study_topics(sd_sid);"#
}

fn get_conditions_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_conditions;
    CREATE TABLE sd.study_conditions(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , original_value         VARCHAR         NULL
    , original_ct_type_id    INT             NULL
    , original_ct_code       VARCHAR         NULL                 
    , icd_code               VARCHAR         NULL
    , icd_name               VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL
    );
    CREATE INDEX study_conditions_sid ON sd.study_conditions(sd_sid);"#
}

/*

#[derive(serde::Serialize)]
pub struct Condition
{
    pub description: Option<String>,
    pub disease_class1: Option<String>,
    pub disease_class2: Option<String>,
}

*/

fn get_features_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_features;
    CREATE TABLE sd.study_features(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , feature_type_id        INT             NULL
    , feature_value_id       INT             NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_features_sid ON sd.study_features(sd_sid);"#
}

fn get_rels_sql<'a>() -> &'a str {
    
    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_relationships;
    CREATE TABLE sd.study_relationships(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , relationship_type_id   INT             NULL
    , target_sd_sid          VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_relationships_sid ON sd.study_relationships(sd_sid);
    CREATE INDEX study_relationships_target_sid ON sd.study_relationships(target_sd_sid);"#
}

fn get_refs_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_references;
    CREATE TABLE sd.study_references(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , pmid                   VARCHAR         NULL
    , citation               VARCHAR         NULL
    , doi                    VARCHAR         NULL	
    , type_id                INT             NULL
    , comments               VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_references_sid ON sd.study_references(sd_sid);"#
}

fn get_ipd_available_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_ipd_available;
    CREATE TABLE sd.study_ipd_available(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , ipd_id                 VARCHAR         NULL
    , ipd_type               VARCHAR         NULL
    , ipd_url                VARCHAR         NULL
    , ipd_comment            VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_ipd_available_sid ON sd.study_ipd_available(sd_sid);"#
}

fn get_data_objects_sql<'a>() -> &'a str {

    r#"DROP TABLE IF EXISTS sd.data_objects;
    CREATE TABLE sd.data_objects(
      id                     INT             GENERATED ALWAYS AS IDENTITY PRIMARY KEY
    , sd_oid                 VARCHAR         NOT NULL
    , sd_sid                 VARCHAR         NULL
    , title                  VARCHAR         NULL
    , version                VARCHAR         NULL
    , display_title          VARCHAR         NULL
    , doi                    VARCHAR         NULL 
    , doi_status_id          INT             NULL
    , publication_year       INT             NULL
    , object_class_id        INT             NULL
    , object_type_id         INT             NULL
    , managing_org_id        INT             NULL
    , managing_org           VARCHAR         NULL
    , managing_org_ror_id    VARCHAR         NULL
    , lang_code              VARCHAR         NULL
    , access_type_id         INT             NULL
    , access_details         VARCHAR         NULL
    , access_details_url     VARCHAR         NULL
    , url_last_checked       DATE            NULL
    , eosc_category          INT             NULL
    , add_study_contribs     BOOLEAN         NULL
    , add_study_topics       BOOLEAN         NULL
    , datetime_of_data_fetch TIMESTAMPTZ     NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL   
    );    
    CREATE INDEX data_objects_oid ON sd.data_objects(sd_oid);
    CREATE INDEX data_objects_sid ON sd.data_objects(sd_sid);"#
}

fn get_datasets_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.object_datasets;
    CREATE TABLE sd.object_datasets(
      id                     INT             GENERATED ALWAYS AS IDENTITY PRIMARY KEY
    , sd_oid                 VARCHAR         NULL
    , record_keys_type_id    INT             NULL 
    , record_keys_details    VARCHAR         NULL    
    , deident_type_id        INT             NULL  
    , deident_direct         BOOLEAN         NULL   
    , deident_hipaa          BOOLEAN         NULL   
    , deident_dates          BOOLEAN         NULL   
    , deident_nonarr         BOOLEAN         NULL   
    , deident_kanon          BOOLEAN         NULL   
    , deident_details        VARCHAR         NULL    
    , consent_type_id        INT             NULL  
    , consent_noncommercial  BOOLEAN         NULL
    , consent_geog_restrict  BOOLEAN         NULL
    , consent_research_type  BOOLEAN         NULL
    , consent_genetic_only   BOOLEAN         NULL
    , consent_no_methods     BOOLEAN         NULL
    , consent_details        VARCHAR         NULL 
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );

    CREATE INDEX object_datasets_oid ON sd.object_datasets(sd_oid)"#
}

fn get_obj_dates_sql<'a>() -> &'a str { 

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.object_dates;
    CREATE TABLE sd.object_dates(
      id                     INT             GENERATED ALWAYS AS IDENTITY PRIMARY KEY
    , sd_oid                 VARCHAR         NULL
    , date_type_id           INT             NULL
    , date_is_range          BOOLEAN         NULL default false
    , date_as_string         VARCHAR         NULL
    , start_year             INT             NULL
    , start_month            INT             NULL
    , start_day              INT             NULL
    , end_year               INT             NULL
    , end_month              INT             NULL
    , end_day                INT             NULL
    , details                VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX object_dates_oid ON sd.object_dates(sd_oid);"#
}

fn get_obj_instances_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.object_instances;
    CREATE TABLE sd.object_instances(
      id                     INT             GENERATED ALWAYS AS IDENTITY PRIMARY KEY
    , sd_oid                 VARCHAR         NULL
    , system_id              INT             NULL
    , system                 VARCHAR         NULL
    , url                    VARCHAR         NULL
    , url_accessible         BOOLEAN         NULL
    , url_last_checked       DATE            NULL
    , resource_type_id       INT             NULL
    , resource_size          VARCHAR         NULL
    , resource_size_units    VARCHAR         NULL
    , resource_comments      VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL   
    );
    CREATE INDEX object_instances_oid ON sd.object_instances(sd_oid);"#

}

fn get_obj_titles_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.object_titles;
    CREATE TABLE sd.object_titles(
      id                     INT             GENERATED ALWAYS AS IDENTITY PRIMARY KEY
    , sd_oid                 VARCHAR         NULL
    , title_type_id          INT             NULL
    , title_text             VARCHAR         NULL
    , lang_code              VARCHAR         NOT NULL
    , lang_usage_id          INT             NOT NULL default 11
    , is_default             BOOLEAN         NULL
    , comments               VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX object_titles_oid ON sd.object_titles(sd_oid);"#
}


*/