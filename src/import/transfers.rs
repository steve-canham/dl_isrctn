use crate::AppError;
use sqlx::{Pool, Postgres};


pub async fn execute_sql(sql: &str, src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    sqlx::raw_sql(sql)
        .execute(src_pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;       
        
    Ok(())
}


pub async fn transfer_study_core_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.studies (sd_sid, display_title, brief_description, 
                type_id, status_id, is_ipd_sharing, ipd_sharing_plan, date_last_revised,
                dt_of_data_fetch)
                select sd_sid, display_title, brief_description, 
                type_id, status_id, 
                case 
                when is_ipd_sharing = true then 'Yes' 
                when is_ipd_sharing = false then 'No' 
                else 'Not provided' end, 
                ipd_sharing_plan, date_last_revised,
                dt_of_data_fetch
                from sd.studies
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_date_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_dates (sd_sid, reg_year, reg_month, reg_date_type, 
                start_year, start_month, start_date_type, 
                comp_year, comp_month, comp_date_type, 
                res_year, res_month, res_date_type)
                select sd_sid, reg_year, reg_month, reg_date_type, 
                start_year, start_month, start_date_type, 
                comp_year, comp_month, comp_date_type, 
                res_year, res_month, res_date_type
                from sd.study_dates
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_participants_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_participants (sd_sid, enrolment, enrolment_type, 
                gender_flag, min_age, min_age_units_id, 
                max_age, max_age_units_id, age_group_flag, iec_flag)
                select sd_sid, enrolment, enrolment_type, 
                gender_flag, min_age, min_age_units_id, 
                max_age, max_age_units_id, age_group_flag, iec_flag
                from sd.study_participants
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_titles_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_titles (sd_sid, title_text, is_public
                    , is_scientific, is_acronym, is_display, comments)
                select sd_sid, title_text, is_public
                    , is_scientific, is_acronym, is_display, comments
                from sd.study_titles
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_identifiers_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_identifiers (sd_sid, id_value, id_type_id, id_type)
                select sd_sid, id_value, id_type_id, id_type
                from sd.study_identifiers
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}

// to use org_cref_id??

pub async fn transfer_study_orgs_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_orgs (sd_sid, org_name, org_country,
                    org_ror_id, is_sponsor, is_funder, is_collaborator)
                select sd_sid, org_name, org_country,
                    org_ror_id, is_sponsor, is_funder, is_collaborator
                from sd.study_orgs
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}

// to use email_domain??

pub async fn transfer_study_people_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_people (sd_sid, full_name, listed_as, orcid_id
                    , affiliation)
                select sd_sid, full_name, listed_as, orcid_id, affiliation
                from sd.study_people
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_iec_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_iec (sd_sid, seq_num, ie_type_id, split_type, tag
                    , indent_level, indent_seq_num, sequence_string, criterion)
                select sd_sid, seq_num, ie_type_id, split_type, tag
                    , indent_level, indent_seq_num, sequence_string, criterion
                from sd.study_iec
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}

// relevance if any of fac_address, may be the fac sometimes?

pub async fn transfer_study_locations_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_locations (sd_sid, fac_name, city_name
                        , disamb_name, country_name)
                select sd_sid, fac_name, city_name
                        , disamb_name, country_name
                from sd.study_locations
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_countries_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_countries (sd_sid, country_name)
                select sd_sid, country_name
                from sd.study_countries
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_topics_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_topics (sd_sid, topic_type, original_value)
                select sd_sid, topic_type, topic_value
                from sd.study_topics
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}

// convert 'Topic...' garbage first in sd table
// usefulness of class1???
/* 
pub async fn transfer_study_conditions_data1(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_conditions (sd_sid, original_value)
                select sd_sid, class1
                from sd.study_conditions
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}
*/

pub async fn transfer_study_conditions_data2(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_conditions (sd_sid, original_value)
                select sd_sid, class2
                from sd.study_conditions
                where class2 is not null
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_conditions_data3(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_conditions (sd_sid, original_value)
                select sd_sid, specific
                from sd.study_conditions
                where specific is not null
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_features_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_features (sd_sid, feature_type, feature_value)
                select sd_sid, feature_type, feature_value
                from sd.study_features
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}

/*

pub async fn transfer_study_objects_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_objects (sd_sid, object_class, object_type
                        , object_name, object_version, date_created, date_updated
                        , yr_of_pub, notes)
                select sd_sid, artefact_type, output_type, output_description, output_version
                        , date_created , date_uploaded, 'year published', mime_type
                
                from sd.study_objects
                where output_type not ilike '%erticle%'
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_obj_instances_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_obj_instances (
        )
                select 
                from sd.study_obj_instances
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_pubs_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_pubs (sd_sid, )
                select sd_sid, 
                from sd.study_pubs
                where output_type ilike '%erticle%'
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}


pub async fn transfer_study_pub_instances_data(src_pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into ad.study_pub_instances (
                    )
                select 
                from sd.study_pub_instances
                order by sd_sid"#;
    execute_sql(sql, src_pool).await
}

     */
