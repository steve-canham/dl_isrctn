use crate::data_models::db_models::*;
use crate::AppError;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use chrono::{NaiveDateTime};

#[allow(dead_code)]
pub struct StudyVecs {
    pub sd_sids: Vec<String>,
    pub display_titles: Vec<Option<String>>,
    pub brief_descriptions: Vec<Option<String>>,
    pub type_ids: Vec<i32>,
	pub status_ids: Vec<i32>,
    pub status_overrides: Vec<Option<String>>,
    pub start_status_overrides: Vec<Option<String>>,
    pub iec_flags: Vec<Option<i32>>,
    pub ipd_sharings: Vec<Option<bool>>,
	pub ipd_sharing_plans: Vec<Option<String>>,
	pub dt_of_datas: Vec<NaiveDateTime>,

}

#[allow(dead_code)]
impl StudyVecs{
    pub fn new(vsize: usize) -> Self {
        StudyVecs { 
            sd_sids: Vec::with_capacity(vsize),
            display_titles: Vec::with_capacity(vsize),
            brief_descriptions: Vec::with_capacity(vsize),
            type_ids: Vec::with_capacity(vsize),
            status_ids: Vec::with_capacity(vsize),
            status_overrides: Vec::with_capacity(vsize),
            start_status_overrides: Vec::with_capacity(vsize),
            iec_flags: Vec::with_capacity(vsize),
            ipd_sharings: Vec::with_capacity(vsize),
	        ipd_sharing_plans: Vec::with_capacity(vsize),
	        dt_of_datas: Vec::with_capacity(vsize),
        }
    }
    

    pub fn add(&mut self, sd_sid:&String, r: &DBSummary) 
    {
        self.sd_sids.push(sd_sid.clone());
        self.display_titles.push(r.display_title.clone());
        self.brief_descriptions.push(r.brief_description.clone());
        self.type_ids.push(r.type_id);
        self.status_ids.push(r.status_id);
        self.status_overrides.push(r.status_override.clone());
        self.start_status_overrides.push(r.start_status_override.clone());
        self.iec_flags.push(r.iec_flag);
        self.ipd_sharings.push(r.ipd_sharing);
        self.ipd_sharing_plans.push(r.ipd_sharing_plan.clone());
        self.dt_of_datas.push(r.dt_of_data);
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.studies (sd_sid, display_title, brief_description, type_id, status_id, status_override, start_status_override,
                        iec_flag, ipd_sharing, ipd_sharing_plan, dt_of_data) 
                        SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::int[], $5::int[], $6::text[], $7::text[], 
                        $8::int[], $9::text[], $10::text[], $11::timestamp[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.display_titles)
        .bind(&self.brief_descriptions)
        .bind(&self.type_ids)
        .bind(&self.status_ids)
        .bind(&self.status_overrides)
        .bind(&self.start_status_overrides)
        .bind(&self.iec_flags)
        .bind(&self.ipd_sharings)
        .bind(&self.ipd_sharing_plans)
        .bind(&self.dt_of_datas)


        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

#[allow(dead_code)]
pub struct StudyDatesVecs {
    pub sd_sids: Vec<String>,
    pub reg_years: Vec<Option<i32>>,  
	pub reg_months: Vec<Option<i32>>,
    pub reg_date_types: Vec<Option<String>>,         
	pub start_years: Vec<Option<i32>>, 
	pub start_months: Vec<Option<i32>>,   
    pub start_date_types: Vec<Option<String>>,       
	pub comp_years: Vec<Option<i32>>,
	pub comp_months: Vec<Option<i32>>,  
	pub comp_date_types: Vec<Option<String>>,      
	pub res_years: Vec<Option<i32>>,  
	pub res_months: Vec<Option<i32>>,   
    pub res_date_types: Vec<Option<String>>,    
}

#[allow(dead_code)]
impl StudyDatesVecs{
    pub fn new(vsize: usize) -> Self {
        StudyDatesVecs { 
            sd_sids: Vec::with_capacity(vsize),
            reg_years: Vec::with_capacity(vsize),  
            reg_months: Vec::with_capacity(vsize),
            reg_date_types: Vec::with_capacity(vsize),        
            start_years: Vec::with_capacity(vsize),
            start_months: Vec::with_capacity(vsize),   
            start_date_types: Vec::with_capacity(vsize),       
            comp_years: Vec::with_capacity(vsize),
            comp_months: Vec::with_capacity(vsize),
            comp_date_types: Vec::with_capacity(vsize),     
            res_years: Vec::with_capacity(vsize), 
            res_months: Vec::with_capacity(vsize),   
            res_date_types: Vec::with_capacity(vsize), 
        }
    }

    pub fn add(&mut self, sd_sid:&String, r: &DBStudyDates) 
    {
        self.sd_sids.push(sd_sid.clone());
        self.reg_years.push(r.reg_year);
        self.reg_months.push(r.reg_month);
        self.reg_date_types.push(r.reg_date_type.clone());
        self.start_years.push(r.start_year);
        self.start_months.push(r.start_month);
        self.start_date_types.push(r.start_date_type.clone());
        self.comp_years.push(r.comp_year);
        self.comp_months.push(r.comp_month);
        self.comp_date_types.push(r.comp_date_type.clone());
        self.res_years.push(r.res_year);
        self.res_months.push(r.res_month);
        self.res_date_types.push(r.res_date_type.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_dates (sd_sid, reg_year, reg_month, reg_date_type, start_year, start_month, start_date_type,
                        comp_year, comp_month, comp_date_type, res_year, res_month, res_date_type) 
            SELECT * FROM UNNEST($1::text[], $2::int[], $3::int[], $4::text[], $5::int[], $6::int[], $7::text[], 
                                 $8::int[], $9::int[], $10::text[], $11::int[], $12::int[], $13::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.reg_years)
        .bind(&self.reg_months)
        .bind(&self.reg_date_types)
        .bind(&self.start_years)
        .bind(&self.start_months)
        .bind(&self.start_date_types)
        .bind(&self.comp_years)
        .bind(&self.comp_months)
        .bind(&self.comp_date_types)
        .bind(&self.res_years)
        .bind(&self.res_months)
        .bind(&self.res_date_types)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}



#[allow(dead_code)]
pub struct StudyParticsVecs {
    pub sd_sids: Vec<String>,
    pub enrolments: Vec<Option<String>>, 
	pub enrolment_types: Vec<Option<String>>,
	pub gender_flags: Vec<Option<String>>,
    pub min_age_as_strings: Vec<Option<String>>,
	pub min_ages: Vec<Option<f32>>,  
	pub min_age_units_ids: Vec<Option<i32>>,
    pub max_age_as_strings: Vec<Option<String>>,
	pub max_ages: Vec<Option<f32>>,  
	pub max_age_units_ids: Vec<Option<i32>>, 
	pub age_group_flags: Vec<Option<i32>>, 
}

#[allow(dead_code)]
impl StudyParticsVecs{
    pub fn new(vsize: usize) -> Self {
        StudyParticsVecs { 
            sd_sids: Vec::with_capacity(vsize),
            enrolments: Vec::with_capacity(vsize),
            enrolment_types: Vec::with_capacity(vsize),
            gender_flags: Vec::with_capacity(vsize),
            min_age_as_strings: Vec::with_capacity(vsize),
            min_ages: Vec::with_capacity(vsize), 
            min_age_units_ids: Vec::with_capacity(vsize),
            max_age_as_strings: Vec::with_capacity(vsize),
            max_ages: Vec::with_capacity(vsize),  
            max_age_units_ids: Vec::with_capacity(vsize), 
            age_group_flags: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, r: &DBStudyPartics) 
    {
        self.sd_sids.push(sd_sid.clone());
        self.enrolments.push(r.enrolment.clone());
        self.enrolment_types.push(r.enrolment_type.clone());
        self.gender_flags.push(r.gender_flag.clone());
        self.min_age_as_strings.push(r.min_age_as_string.clone());
        self.min_ages.push(r.min_age);
        self.min_age_units_ids.push(r.min_age_units_id);
        self.max_age_as_strings.push(r.max_age_as_string.clone());
        self.max_ages.push(r.max_age);
        self.max_age_units_ids.push(r.max_age_units_id.clone());
        self.age_group_flags.push(r.age_group_flag);

    }
    
    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_partics (sd_sid, enrolment, enrolment_type, gender_flag, 
                min_age_as_string, min_age, min_age_units_id, max_age_as_string, max_age, max_age_units_id, age_group_flag) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], 
                    $5::text[], $6::float[], $7::int[], $8::text[], $9::float[], $10::int[], $11::int[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.enrolments)
        .bind(&self.enrolment_types)
        .bind(&self.gender_flags)
        .bind(&self.min_age_as_strings)
        .bind(&self.min_ages)
        .bind(&self.min_age_units_ids)
        .bind(&self.max_age_as_strings)
        .bind(&self.max_ages)
        .bind(&self.max_age_units_ids)
        .bind(&self.age_group_flags)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}



#[allow(dead_code)]
pub struct TitleVecs {
    pub sd_sids: Vec<String>,
    pub title_type_ids: Vec<i32>,
    pub title_texts: Vec<String>,
    pub is_defaults: Vec<bool>,
    pub comments: Vec<Option<String>>,
}

#[allow(dead_code)]
impl TitleVecs{
    pub fn new(vsize: usize) -> Self {
        TitleVecs { 
            sd_sids: Vec::with_capacity(vsize),
            title_type_ids: Vec::with_capacity(vsize),
            title_texts: Vec::with_capacity(vsize),
            is_defaults: Vec::with_capacity(vsize),
            comments: Vec::with_capacity(vsize),
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.title_type_ids.shrink_to_fit();
        self.title_texts.shrink_to_fit();
        self.is_defaults.shrink_to_fit();
        self.comments.shrink_to_fit();
    }


    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBTitle>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.title_type_ids.push(r.title_type_id);
            self.title_texts.push(r.title_text.clone());
            self.is_defaults.push(r.is_default);
            self.comments.push(r.comment.clone());
        }
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_titles (sd_sid, title_type_id, title_text, is_default, comments) 
            SELECT * FROM UNNEST($1::text[], $2::int[], $3::text[], $4::bool[], $5::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.title_type_ids)
        .bind(&self.title_texts)
        .bind(&self.is_defaults)
        .bind(&self.comments)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


#[allow(dead_code)]
pub struct IdentifierVecs {
    pub sd_sids: Vec<String>,
    pub id_values: Vec<String>,
    pub id_type_ids: Vec<i32>,
    pub id_types: Vec<String>,
}

#[allow(dead_code)]
impl IdentifierVecs{
    pub fn new(vsize: usize) -> Self {
        IdentifierVecs { 
            sd_sids: Vec::with_capacity(vsize),
            id_values: Vec::with_capacity(vsize),
            id_type_ids: Vec::with_capacity(vsize),
            id_types: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBIdentifier>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.id_values.push(r.id_value.clone());
            self.id_type_ids.push(r.id_type_id);
            self.id_types.push(r.id_type.clone());
        }
    }


    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.id_values.shrink_to_fit();
        self.id_type_ids.shrink_to_fit();
        self.id_types.shrink_to_fit();
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_identifiers (sd_sid, id_value, id_type_id, id_type) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::int[], $4::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.id_values)
        .bind(&self.id_type_ids)
        .bind(&self.id_types)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

