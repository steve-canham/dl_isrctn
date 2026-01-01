use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use crate::err::AppError;
use log::info;

pub async fn build_sd_tables (pool: &Pool<Postgres>) -> Result<(), AppError> {  
    
    build_studies_table (pool).await?;
    build_titles_table (pool).await?;
    build_idents_table (pool).await?;
    build_orgs_table (pool).await?;
    build_people_table (pool).await?;
    build_locations_table (pool).await?;
    build_countries_table (pool).await?;
    build_ie_table (pool).await?;
    build_topics_table (pool).await?;
    build_conditions_table (pool).await?;
    build_features_table (pool).await?;
    build_rels_table (pool).await?;
    build_refs_table (pool).await?;
    build_links_table (pool).await?;
    build_ipd_available_table (pool).await?;

    build_data_objects_table (pool).await?;
    build_datasets_table (pool).await?;
    build_obj_titles_table (pool).await?;
    build_obj_instances_table (pool).await?;
    build_obj_dates_table (pool).await?;

    Ok(())
    
}



pub async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, AppError> {
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
}

pub async fn build_studies_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    create schema if not exists ad;
	DROP TABLE IF EXISTS ad.studies;
	CREATE TABLE ad.studies(
	  id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY  (start with 1000001 increment by 1)
	, sd_sid                 VARCHAR         NOT NULL
	, display_title          VARCHAR         NULL
	, title_lang_code        VARCHAR         NOT NULL default 'en'
	, brief_description      VARCHAR         NULL
	, reg_year           	 INT             NULL
	, reg_month        	     INT             NULL
    , reg_date_type          CHAR(1)         NULL
	, start_year      	     INT             NULL
	, start_month      	     INT             NULL
    , start_date_type        CHAR(1)         NULL
	, comp_year      		 INT             NULL
	, comp_month      	     INT             NULL
	, comp_date_type         CHAR(1)         NULL
	, res_year      		 INT             NULL
	, res_month      		 INT             NULL
    , res_date_type          CHAR(1)         NULL
	, type_id                INT             NOT NULL default 0
	, status_id        	     INT             NOT NULL default 0
	, enrolment              VARCHAR         NULL
	, enrolment_type         CHAR(1)         NULL
	, gender_flag            CHAR(1)         NULL
	, min_age                INT             NULL
	, min_age_units_id       INT             NULL
	, max_age                INT             NULL
	, max_age_units_id       INT             NULL
	, age_group_flag         INT             NOT NULL default 0
	, iec_flag               INT             NOT NULL default 0 
	, ipd_sharing			 VARCHAR         NULL
	, dt_of_data    	     TIMESTAMPTZ     NULL
	, added_on               TIMESTAMPTZ     NOT NULL default now()
	);
	CREATE INDEX studies_sid ON ad.studies(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("studies table (re)created");
    
    Ok(())

}

/*

#[derive(serde::Serialize)]
pub struct Participants
{
    pub age_range: Option<String>,
    pub l_age_limit: Option<String>,
    pub l_age_limit_num: Option<f32>,
    pub l_age_limit_units: Option<String>,
    pub u_age_limit: Option<String>,
    pub u_age_limit_num: Option<f32>,
    pub u_age_limit_units: Option<String>,
    pub gender: Option<String>,
    pub inclusion: Option<String>,
    pub exclusion: Option<String>,
    pub patient_info_sheet: Option<String>,
}
*/

pub async fn build_titles_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_titles;
    CREATE TABLE ad.study_titles(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , title_type_id          INT
    , title_text             VARCHAR
    , lang_code              VARCHAR         NOT NULL default 'en'
    , is_default             BOOL
    , comments               VARCHAR
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_titles_sid ON ad.study_titles(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study titles table (re)created");
    Ok(())

}

/*

#[derive(serde::Serialize)]
pub struct Title
{
    pub title_type_id: i32,
    pub title_type: String,
    pub title_value: String,
}

#[derive(serde::Serialize)]
pub struct Identifier
{
    pub identifier_type_id: i32,
    pub identifier_type: String,
    pub identifier_value: String,
}
*/

pub async fn build_idents_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

        let sql = r#"SET client_min_messages TO WARNING; 
        DROP TABLE IF EXISTS ad.study_identifiers;
        CREATE TABLE ad.study_identifiers(
          id                     INT             NOT NULL GENERATED BY DEFAULT AS IDENTITY
        , sd_sid                 VARCHAR         NOT NULL
        , id_value               VARCHAR         NULL
        , id_type_id             INT             NULL
        , id_type                VARCHAR         NULL
        , added_on               TIMESTAMPTZ     NOT NULL default now()
        , coded_on               TIMESTAMPTZ     NULL                                     
        );
        CREATE INDEX study_identifiers_sid ON ad.study_identifiers(sd_sid);"#;

       execute_sql(sql, pool).await?;
       info!("study identifiers table (re)created");


    Ok(())

}



pub async fn build_orgs_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_organisations;
    CREATE TABLE ad.study_organisations(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , contrib_type_id        INT             NULL
    , organisation_id        INT             NULL
    , organisation_name      VARCHAR         NULL
    , organisation_ror_id    VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL
    );
    CREATE INDEX study_organisations_sid ON ad.study_organisations(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study orgs table (re)created");
    
    Ok(())

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

pub async fn build_people_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_people;
    CREATE TABLE ad.study_people(
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
    CREATE INDEX study_people_sid ON ad.study_people(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study people table (re)created");
    
    Ok(())

}



pub async fn build_ie_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_titles;
    CREATE TABLE ad.study_titles(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , ie_type_id             INT                
    , unsplit_text           VARCHAR
    , ie_num                 int          
    , criterion              VARCHAR
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_titles_sid ON ad.study_titles(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study titles table (re)created");
    Ok(())

}

pub async fn build_locations_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_locations;
    CREATE TABLE ad.study_locations(
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
    CREATE INDEX study_locations_sid ON ad.study_locations(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study locations table (re)created");
    
    Ok(())

}


pub async fn build_countries_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_countries;
    CREATE TABLE ad.study_countries(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , country_id             INT             NULL
    , country_name           VARCHAR         NULL
    , status_id              INT             NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    , coded_on               TIMESTAMPTZ     NULL  default now()       -- already coded when added                                   
    );
    CREATE INDEX study_countries_sid ON ad.study_countries(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study countries table (re)created");
    
    Ok(())

}


pub async fn build_topics_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_topics;
    CREATE TABLE ad.study_topics(
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
    CREATE INDEX study_topics_sid ON ad.study_topics(sd_sid);"#;

    execute_sql(sql, pool).await?;
    info!("study topics table (re)created");
    
    Ok(())

}

pub async fn build_conditions_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_conditions;
    CREATE TABLE ad.study_conditions(
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
    CREATE INDEX study_conditions_sid ON ad.study_conditions(sd_sid);"#;

    execute_sql(sql, pool).await?;
    info!("study conditions table (re)created");
    
    Ok(())

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


pub async fn build_features_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_features;
    CREATE TABLE ad.study_features(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , feature_type_id        INT             NULL
    , feature_value_id       INT             NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()

    );
    CREATE INDEX study_features_sid ON ad.study_features(sd_sid);"#;

    execute_sql(sql, pool).await?;
    info!("study features table (re)created");
    
    Ok(())
}



pub async fn build_rels_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_relationships;
    CREATE TABLE ad.study_relationships(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , relationship_type_id   INT             NULL
    , target_sd_sid          VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_relationships_sid ON ad.study_relationships(sd_sid);
    CREATE INDEX study_relationships_target_sid ON ad.study_relationships(target_sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study relationships table (re)created");
    
    Ok(())

}



pub async fn build_refs_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_references;
    CREATE TABLE ad.study_references(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , pmid                   VARCHAR         NULL
    , citation               VARCHAR         NULL
    , doi                    VARCHAR         NULL	
    , type_id                INT             NULL
    , comments               VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_references_sid ON ad.study_references(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study refs table (re)created");
    
    Ok(())

}


pub async fn build_links_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_links;
    CREATE TABLE ad.study_links(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , link_label             VARCHAR         NULL
    , link_url               VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_links_sid ON ad.study_links(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study links table (re)created");
    
    Ok(())

}


pub async fn build_ipd_available_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.study_ipd_available;
    CREATE TABLE ad.study_ipd_available(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , ipd_id                 VARCHAR         NULL
    , ipd_type               VARCHAR         NULL
    , ipd_url                VARCHAR         NULL
    , ipd_comment            VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_ipd_available_sid ON ad.study_ipd_available(sd_sid);"#;

	execute_sql(sql, pool).await?;
    info!("study ipd available table (re)created");
    
    Ok(())

}



pub async fn build_data_objects_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"DROP TABLE IF EXISTS ad.data_objects;
    CREATE TABLE ad.data_objects(
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
    CREATE INDEX data_objects_oid ON ad.data_objects(sd_oid);
    CREATE INDEX data_objects_sid ON ad.data_objects(sd_sid);
    "#;

    execute_sql(sql, pool).await?;
    info!("data objects table (re)created");
    
    Ok(())

}

pub async fn build_datasets_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.object_datasets;
    CREATE TABLE ad.object_datasets(
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

    CREATE INDEX object_datasets_oid ON ad.object_datasets(sd_oid)
    "#;

    execute_sql(sql, pool).await?;
    info!("object datasets table (re)created");
    
    Ok(())

}


pub async fn build_obj_dates_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.object_dates;
    CREATE TABLE ad.object_dates(
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
    CREATE INDEX object_dates_oid ON ad.object_dates(sd_oid);"#;

    execute_sql(sql, pool).await?;
    info!("object dates table (re)created");
    
    Ok(())

}

pub async fn build_obj_instances_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.object_instances;
    CREATE TABLE ad.object_instances(
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
    CREATE INDEX object_instances_oid ON ad.object_instances(sd_oid);"#;

    execute_sql(sql, pool).await?;
    info!("object instances table (re)created");
    
    Ok(())

}


pub async fn build_obj_titles_table (pool: &Pool<Postgres>) -> Result<(), AppError> {  

    let sql = r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS ad.object_titles;
    CREATE TABLE ad.object_titles(
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
    CREATE INDEX object_titles_oid ON ad.object_titles(sd_oid);"#;

    execute_sql(sql, pool).await?;
    info!("object titles table (re)created");
    
    Ok(())

}


/*


#[derive(serde::Serialize)]
pub struct Study
{
    pub sd_sid: String, 
    pub downloaded: String,
    pub registration: Registration,

    pub titles: Vec<Title>,
    pub identifiers: Option<Vec<Identifier>>,
    
    pub summary: Summary,
    pub primary_outcomes: Option<Vec<OutcomeMeasure>>,
    pub secondary_outcomes: Option<Vec<OutcomeMeasure>>,

    pub ethics: Ethics,
    pub ethics_committees: Option<Vec<EthicsCommittee>>,

    pub design: Design,
    pub trial_types: Option<Vec<String>>,
    pub trial_settings: Option<Vec<String>>,

    pub conditions: Option<Vec<Condition>>,
    pub interventions: Option<Vec<Intervention>>,

    pub contacts: Option<Vec<StudyContact>>,
    pub sponsors: Option<Vec<StudySponsor>>,
    pub funders: Option<Vec<StudyFunder>>,

    pub participant_types: Option<Vec<String>>,
    pub participants: Participants,

    pub recruitment: Recruitment,
    pub centres: Option<Vec<StudyCentre>>,
    pub countries: Option<Vec<String>>,

    pub data_policies: Option<Vec<String>>,
    pub results: Results,
    pub outputs: Option<Vec<StudyOutput>>,
    pub attached_files: Option<Vec<AttachedFile>>,
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
pub struct Summary
{
    pub plain_english_summary: Option<String>,
    pub overall_end_date: Option<String>,
    pub trial_website: Option<String>,
}


#[derive(serde::Serialize)]
pub struct Design
{
    pub study_design: Option<String>,
    pub primary_study_design: Option<String>,
    pub secondary_study_design: Option<String>,
}


#[derive(serde::Serialize)]
pub struct Intervention 
{
    pub description: Option<String>,
    pub int_type: Option<String>,
    pub pharma_study_types: Option<String>,
    pub phase: Option<String>,
    pub drug_names: Option<String>,
}


#[derive(serde::Serialize)]
pub struct Participants
{
    pub age_range: Option<String>,
    pub l_age_limit: Option<String>,
    pub l_age_limit_num: Option<f32>,
    pub l_age_limit_units: Option<String>,
    pub u_age_limit: Option<String>,
    pub u_age_limit_num: Option<f32>,
    pub u_age_limit_units: Option<String>,
    pub gender: Option<String>,
    pub inclusion: Option<String>,
    pub exclusion: Option<String>,
    pub patient_info_sheet: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Recruitment
{
    pub target_enrolment: Option<String>,
    pub total_final_enrolment: Option<String>,
    pub total_target: Option<String>,
    pub recruitment_start: Option<String>,
    pub recruitment_end: Option<String>,
    pub recruitment_start_status_override: Option<String>,
    pub recruitment_status_override: Option<String>,
}

#[derive(serde::Serialize)]
pub struct StudyCentre
{
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Results
{
    pub publication_plan: Option<String>,
    pub intent_to_publish: Option<String>,
    pub publication_details: Option<String>,
    pub publication_stage: Option<String>,
    pub biomed_related: Option<bool>, 
    pub basic_report: Option<String>,
    pub plain_english_report: Option<String>,
}

#[derive(serde::Serialize)]
pub struct IPD
{
    pub ipd_sharing_plan: Option<bool>,    
    pub ipd_sharing_statement: Option<String>,

}

#[derive(serde::Serialize)]
pub struct StudyOutput
{
    pub output_type: Option<String>,
    pub artefact_type: Option<String>,
    pub date_created: Option<String>,
    pub date_uploaded: Option<String>,
    pub peer_reviewed: Option<bool>,  
    pub patient_facing: Option<bool>, 
    pub created_by: Option<String>,
    pub description: Option<String>,
    pub production_notes: Option<String>,
    pub external_link_url: Option<String>,
    pub file_id: Option<String>,
    pub original_filename: Option<String>,
    pub download_filename: Option<String>,
    pub version: Option<String>,
    pub mime_type: Option<String>,
}  

#[derive(serde::Serialize)]
pub struct AttachedFile
{
    pub description: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub is_public: Option<bool>, 
    pub mime_type: Option<String>,
}


*/