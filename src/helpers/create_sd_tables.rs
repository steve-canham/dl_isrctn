use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use crate::err::AppError;
use log::info;

pub async fn build_sd_tables (pool: &Pool<Postgres>) -> Result<(), AppError> {  
    
    execute_sql(get_schema_sql(), pool).await?;
    execute_sql(get_studies_sql(), pool).await?;
    execute_sql(get_study_dates_sql(), pool).await?;
    execute_sql(get_study_partics_sql(), pool).await?;
    execute_sql(get_titles_sql(), pool).await?;
    execute_sql(get_idents_sql(), pool).await?;
    execute_sql(get_orgs_sql(), pool).await?;
    execute_sql(get_people_sql(), pool).await?;
    execute_sql(get_ie_sql(), pool).await?;
    execute_sql(get_locations_sql(), pool).await?;
    execute_sql(get_countries_sql(), pool).await?;
    execute_sql(get_topics_sql(), pool).await?;
    execute_sql(get_conditions_sql(), pool).await?;
    execute_sql(get_features_sql(), pool).await?;
    execute_sql(get_rels_sql(), pool).await?;
    execute_sql(get_refs_sql(), pool).await?;
    execute_sql(get_ipd_available_sql(), pool).await?;

    execute_sql(get_data_objects_sql(), pool).await?;
    execute_sql(get_datasets_sql(), pool).await?;
    execute_sql(get_obj_dates_sql(), pool).await?;
    execute_sql(get_obj_instances_sql(), pool).await?;
    execute_sql(get_obj_titles_sql(), pool).await?;

    info!("all sd tables recreated");

    Ok(())
}


pub async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<PgQueryResult, AppError> {
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
}

fn get_schema_sql <'a>() -> &'a str {
    r#"SET client_min_messages TO WARNING; 
    create schema if not exists sd;"#
}

fn get_studies_sql <'a>() -> &'a str {
    r#"SET client_min_messages TO WARNING; 
	DROP TABLE IF EXISTS sd.studies;
	CREATE TABLE sd.studies(
	  id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY  (start with 1000001 increment by 1)
	, sd_sid                 VARCHAR         NOT NULL
	, display_title          VARCHAR         NULL
	, brief_description      VARCHAR         NULL
	, type_id                INT             NOT NULL default 0
	, status_id        	     INT             NOT NULL default 0
  , status_override        VARCHAR         NULL
  , start_status_override  VARCHAR         NULL
	, iec_flag               INT             NOT NULL default 0 
	, ipd_sharing		         VARCHAR         NULL
  , ipd_sharing_plan   	   VARCHAR         NULL
  , date_last_revised      Date            NULL
	, dt_of_data_fetch 	     TIMESTAMP       NULL
	, added_on               TIMESTAMPTZ     NOT NULL default now()
	);
	CREATE INDEX studies_sid ON sd.studies(sd_sid);"#
}

fn get_study_dates_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
	DROP TABLE IF EXISTS sd.study_dates;
	CREATE TABLE sd.study_dates(
	  id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY  (start with 1000001 increment by 1)
	, sd_sid                 VARCHAR         NOT NULL
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
	, added_on               TIMESTAMPTZ     NOT NULL default now()
	);
	CREATE INDEX study_dates_sid ON sd.study_dates(sd_sid);"#
}

fn get_study_partics_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
	DROP TABLE IF EXISTS sd.study_partics;
	CREATE TABLE sd.study_partics(
	  id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY  (start with 1000001 increment by 1)
	, sd_sid                 VARCHAR         NOT NULL
    , enrolment_target       VARCHAR         NULL
    , enrolment_final        VARCHAR         NULL
    , enrolment_total        VARCHAR         NULL
	, enrolment              VARCHAR         NULL
	, enrolment_type         CHAR(1)         NULL
	, gender_flag            CHAR(1)         NULL
	, min_age_as_string      VARCHAR         NULL
	, min_age                INT             NULL
	, min_age_units_id       INT             NULL
	, max_age_as_string      VARCHAR         NULL
	, max_age                INT             NULL
	, max_age_units_id       INT             NULL
	, age_group_flag         INT             NOT NULL default 0
	, added_on               TIMESTAMPTZ     NOT NULL default now()
	);
	CREATE INDEX study_partics_sid ON sd.study_partics(sd_sid);"#
}

fn get_titles_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_titles;
    CREATE TABLE sd.study_titles(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , title_type_id          INT
    , title_text             VARCHAR
    , is_default             BOOL
    , comments               VARCHAR
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_titles_sid ON sd.study_titles(sd_sid);"#
}

fn get_idents_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_identifiers;
    CREATE TABLE sd.study_identifiers(
      id                     INT             NOT NULL GENERATED BY DEFAULT AS IDENTITY
    , sd_sid                 VARCHAR         NOT NULL
    , id_value               VARCHAR         NULL
    , id_type_id             INT             NULL
    , id_type                VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_identifiers_sid ON sd.study_identifiers(sd_sid);"#
}

fn get_orgs_sql<'a>() -> &'a str {
    
    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_orgs;
    CREATE TABLE sd.study_orgs(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , contrib_type_id        INT             NULL
    , name                   VARCHAR         NULL
    , ror_id                 VARCHAR         NULL
    , cross_ref_id           VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_organisations_sid ON sd.study_organisations(sd_sid);"#
}

fn get_people_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_people;
    CREATE TABLE sd.study_people(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , contrib_type_id        INT             NULL
    , given_name             VARCHAR         NULL
    , family_name            VARCHAR         NULL
    , full_name              VARCHAR         NULL
    , orcid_id               VARCHAR         NULL
    , affiliation            VARCHAR         NULL
    , email_domain           VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_people_sid ON sd.study_people(sd_sid);"#
}

fn get_ie_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_iec;
    CREATE TABLE sd.study_iec(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , ie_type_id             INT                
    , ie_num                 int          
    , criterion              VARCHAR
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_iec_sid ON sd.study_iec(sd_sid);"#
}

fn get_locations_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_locations;
    CREATE TABLE sd.study_locations(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , facility               VARCHAR         NULL
    , city_name              VARCHAR         NULL
    , disamb_name            VARCHAR         NULL
    , country_name           VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_locations_sid ON sd.study_locations(sd_sid);"#
}

fn get_countries_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_countries;
    CREATE TABLE sd.study_countries(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , country_name           VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
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
    , added_on               TIMESTAMPTZ     NOT NULL default now()
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
    , original_class1        VARCHAR         NULL
    , original_class2        VARCHAR         NULL
    , original_ct_type_id    INT             NULL
    , original_ct_code       VARCHAR         NULL                 
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX study_conditions_sid ON sd.study_conditions(sd_sid);"#
}


fn get_features_sql<'a>() -> &'a str {

    r#"SET client_min_messages TO WARNING; 
    DROP TABLE IF EXISTS sd.study_features;
    CREATE TABLE sd.study_features(
      id                     INT             PRIMARY KEY GENERATED ALWAYS AS IDENTITY (start with 10000001 increment by 1)
    , sd_sid                 VARCHAR         NOT NULL
    , original_value         VARCHAR         NULL
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
    , ipd_name               VARCHAR         NULL
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
    , publication_year       INT             NULL
    , object_class_id        INT             NULL
    , object_type_id         INT             NULL
    , managing_org           VARCHAR         NULL
    , lang_code              VARCHAR         NULL
    , access_type_id         INT             NULL
    , access_details         VARCHAR         NULL
    , access_details_url     VARCHAR         NULL
    , eosc_category          INT             NULL
    , datetime_of_data_fetch TIMESTAMPTZ     NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
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
    , deident_details        VARCHAR         NULL    
    , consent_type_id        INT             NULL  
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
    , system                 VARCHAR         NULL
    , url                    VARCHAR         NULL
    , url_accessible         BOOLEAN         NULL
    , resource_type_id       INT             NULL
    , resource_size          VARCHAR         NULL
    , resource_size_units    VARCHAR         NULL
    , resource_comments      VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
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
    , is_default             BOOLEAN         NULL
    , comments               VARCHAR         NULL
    , added_on               TIMESTAMPTZ     NOT NULL default now()
    );
    CREATE INDEX object_titles_oid ON sd.object_titles(sd_oid);"#
}

