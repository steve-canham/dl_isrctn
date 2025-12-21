use sqlx::{Pool, Postgres};
use crate::AppError;

pub async fn create_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    execute_sql(get_trials_sql(), pool).await?;
    execute_sql(get_sec_ids_sql(), pool).await?;
    execute_sql(get_health_conditions_sql(), pool).await?;
    execute_sql(get_condition_codes_sql(), pool).await?;    
    execute_sql(get_interventions_sql(), pool).await?;
    execute_sql(get_pri_outcomes_sql(), pool).await?;
    execute_sql(get_sec_outcomes_sql(), pool).await?;
    execute_sql(get_sites_sql(), pool).await?;
    execute_sql(get_postcodes_sql(), pool).await?;
    execute_sql(get_countries_sql(), pool).await?;
    execute_sql(get_funders_sql(), pool).await?;
    execute_sql(get_sec_sponsors_sql(), pool).await?;
    execute_sql(get_other_collabs_sql(), pool).await?;
    execute_sql(get_ethics_comms_sql(), pool).await?;
    execute_sql(get_contacts_sql(), pool).await?;
    execute_sql(get_dss_sql(), pool).await?;
    execute_sql(get_results_sql(), pool).await?;
    execute_sql(get_supp_docs_sql(), pool).await?;
    execute_sql(get_ext_pubs_sql(), pool).await?;
      
    Ok(())
}
    

async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {
    
    sqlx::raw_sql(&sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


fn get_trials_sql <'a>() -> &'a str {
    r#"drop table if exists sd.trials;
    create table sd.trials (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                    int not null
      , actrn_id                    varchar null
      , submit_date                 datetime null
      , approval_date               datetime null
      , study_title                 varchar null
      , scientific_title            varchar null
      , utn                         varchar null                   
      , trial_acronym               varchar null
      , linked_study                varchar null
      , interventions               varchar null
      , comparator                  varchar null
      , control                     varchar null
      , inclusion_criteria          varchar null
      , min_age                     int null
      , min_age_type                varchar null
      , max_age                     int null
      , max_age_type                varchar null
      , gender                      varchar null
      , healthy_volunteers          varchar null
      , exclusion_criteria          varchar null
      , study_type                  varchar null
      , purpose                     varchar null
      , allocation                  varchar null
      , concealment                 varchar null
      , sequence                    varchar null
      , masking                     varchar null
      , assignment                  varchar null
      , other_design                varchar null
      , endpoint                    varchar null
      , phase                       varchar null
      , stat_methods                varchar null
      , masked_participant          bool null
      , masked_clinician            bool null
      , masked_assessors            bool null
      , masked_analyst              bool null
      , pt_registry                 bool null
      , registry_followup           int null
      , registry_fu_type            varchar null
      , purpose_obs                 varchar null
      , duration_obs                int null
      , selection_obs               varchar null
      , timing_obs                  varchar null
      , antic_start_date            date null
      , actual_start_date           date null
      , antic_end_date              date null
      , actual_end_date             date null
      , target_sample_size          int null
      , final_sample_size           int null
      , current_sample_size         int null
      , antic_last_visit_date       date null
      , actual_last_visit_date      date null
      , recruitment_status          varchar null
      , analysis_status             varchar null
      , withdrawn_reason            varchar null
      , wd_reason_other             varchar null
      , recruitment_country         varchar null
      , recruitment_state           varchar null
      , sponsor_type                varchar null
      , sponsor_name                varchar null
      , sponsor_country             varchar null
      , ethics_status               varchar null
      , brief_summary               varchar null
      , trial_website               varchar null
      , publication                 varchar null
      , public_notes                varchar null
    );"#
}


fn get_sec_ids_sql <'a>() -> &'a str {
    r#"drop table if exists sd.sec_ids;
    create table sd.sec_ids (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , secondary_id            archar null
    );"#
}


fn get_health_conditions_sql <'a>() -> &'a str {
    r#"drop table if exists sd.health_conditions;
    create table sd.health_conditions (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , health_condition        varchar null
    );"#
}


fn get_condition_codes_sql <'a>() -> &'a str {
    r#"drop table if exists sd.condition_codes;
    create table sd.condition_codes (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , condition_category      varchar null
      , condition_code          varchar null
    );"#
}


fn get_interventions_sql <'a>() -> &'a str {
    r#"drop table if exists sd.interventions;
    create table sd.interventions (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , intervention_code       varchar null
    );"#
}


fn get_pri_outcomes_sql <'a>() -> &'a str {
    r#"drop table if exists sd.pri_outcomes;
    create table sd.pri_outcomes (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , outcome                 varchar null
      , outcome_assessment      varchar null
      , timepoint               varchar null
    );"#
}


fn get_sec_outcomes_sql <'a>() -> &'a str {
    r#"drop table if exists sd.sec_outcomes;
    create table sd.sec_outcomes (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , outcome                 varchar null
      , outcome_assessment      varchar null
      , timepoint               varchar null
    );"#
}


fn get_sites_sql <'a>() -> &'a str {
    r#"drop table if exists sd.sites;
    create table sd.sites (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , hospital                varchar null
    );"#
}


fn get_postcodes_sql <'a>() -> &'a str {
    r#"drop table if exists sd.postcodes;
    create table sd.postcodes (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , postcode                varchar null
    );"#
}


fn get_countries_sql <'a>() -> &'a str {
    r#"drop table if exists sd.countries;
    create table sd.countries (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , country                 varchar null
      , state                   varchar null
    );"#
}


fn get_funders_sql <'a>() -> &'a str {
    r#"drop table if exists sd.funders;
    create table sd.funders (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , funding_source_type     varchar null
      , funding_source_name     varchar null
      , funding_source_country  varchar null
    );"#
}


fn get_sec_sponsors_sql <'a>() -> &'a str {
    r#"drop table if exists sd.sec_sponsors;
    create table sd.sec_sponsors (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , sec_sponsor_type        varchar null
      , sec_sponsor_name        varchar null
      , sec_sponsor_country     varchar null
    );"#
}


fn get_other_collabs_sql <'a>() -> &'a str {
    r#"drop table if exists sd.other_collabs;
    create table sd.other_collabs (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , other_collab_type       varchar null
      , other_collab_name       varchar null
      , other_collab_country    varchar null
    );"#
}


fn get_ethics_comms_sql <'a>() -> &'a str {
    r#"drop table if exists sd.ethics_comms;
    create table sd.ethics_comms (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , ethics_comm_name        varchar null
      , ethics_comm_contact     varchar null
      , ethics_comm_country     varchar null
      , ethics_submit_date      date null
      , ethics_approval_date    date null
      , hrec_approval_id        varchar null
    );"#
}


fn get_contacts_sql <'a>() -> &'a str {
    r#"drop table if exists sd.contacts;
    create table sd.contact (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , type                    varchar null
      , title                   varchar null
      , name                    varchar null
      , address                 varchar null
      , country                 varchar null
      , phone                   varchar null
      , fax                     varchar null
      , email                   varchar null
    );"#
}


fn get_dss_sql <'a>() -> &'a str {
    r#"drop table if exists sd.dss;
    create table sd.dss (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , ipd_availability        varchar null
      , to_whom                 varchar null
      , conditions              varchar null
      , data_to_be_shared       varchar null
      , types_of_analysis       varchar null
      , timeframe_from          varchar null
      , timeframe_to            varchar null
      , mechanism               varchar null
      , extra_considerations    varchar null
    );"#
}

fn get_results_sql <'a>() -> &'a str {
    r#"drop table if exists sd.results;
    create table sd.results (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , type                    varchar null
      , doi                     varchar null
      , citation                varchar null
      , attachment              varchar null
    );"#
}


fn get_supp_docs_sql <'a>() -> &'a str {
    r#"drop table if exists sd.supp_docs;
    create table sd.supp_docs (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , type                    varchar null
      , citation                varchar null
      , link                    varchar null
      , email                   varchar null
      , details                 varchar null
      , attachment              varchar null
    );"#
}

fn get_ext_pubs_sql <'a>() -> &'a str {
    r#"drop table if exists sd.ext_pubs;
    create table sd.ext_pubs (
        id int GENERATED ALWAYS AS IDENTITY (START WITH 10000) PRIMARY KEY
      , trial_id                int not null
      , source                  varchar null
      , doi                     varchar null
      , title                   varchar null
      , year_of_pub             varchar null

    );"#
}
