use crate::data_models::db_models::*;
use crate::AppError;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use chrono::{NaiveDate, NaiveDateTime};

pub struct StudyVecs {
    pub sd_sids: Vec<String>,
    pub display_titles: Vec<String>,
    pub brief_descriptions: Vec<String>,
    pub type_ids: Vec<i32>,
	pub status_ids: Vec<i32>,
    pub status_overrides: Vec<Option<String>>,
    pub start_status_overrides: Vec<Option<String>>,
    pub iec_flags: Vec<i32>,
    pub ipd_sharings: Vec<bool>,
	pub ipd_sharing_plans: Vec<Option<String>>,
    pub date_last_reviseds: Vec<Option<NaiveDate>>,
	pub dt_of_datas: Vec<NaiveDateTime>,

}

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
            date_last_reviseds: Vec::with_capacity(vsize),
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
        self.date_last_reviseds.push(r.date_last_revised);
        self.dt_of_datas.push(r.dt_of_data_fetch);
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.studies (sd_sid, display_title, brief_description, type_id, status_id, status_override, start_status_override,
                        iec_flag, ipd_sharing, ipd_sharing_plan, date_last_revised, dt_of_data_fetch) 
                        SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::int[], $5::int[], $6::text[], $7::text[], 
                        $8::int[], $9::text[], $10::text[], $11::date[], $12::timestamp[])"#;

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
        .bind(&self.date_last_reviseds)
        .bind(&self.dt_of_datas)


        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

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

pub struct StudyParticsVecs {
    pub sd_sids: Vec<String>,
    pub enrolment_targets: Vec<Option<String>>, 
    pub enrolment_finals: Vec<Option<String>>, 
    pub enrolment_totals: Vec<Option<String>>, 
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

impl StudyParticsVecs{
    pub fn new(vsize: usize) -> Self {
        StudyParticsVecs { 
            sd_sids: Vec::with_capacity(vsize),
            enrolment_targets: Vec::with_capacity(vsize), 
            enrolment_finals: Vec::with_capacity(vsize),
            enrolment_totals: Vec::with_capacity(vsize),
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
        self.enrolment_targets.push(r.enrolment_target.clone());
        self.enrolment_finals.push(r.enrolment_final.clone());
        self.enrolment_totals.push(r.enrolment_total.clone());
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

        let sql = r#"INSERT INTO sd.study_partics (sd_sid, enrolment_target, enrolment_final,
                enrolment_total, enrolment, enrolment_type, gender_flag,  
                min_age_as_string, min_age, min_age_units_id, 
                max_age_as_string, max_age, max_age_units_id, age_group_flag) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[], 
                    $8::text[], $9::float[], $10::int[], $11::text[], $12::float[], $13::int[], $14::int[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.enrolment_targets)
        .bind(&self.enrolment_finals)
        .bind(&self.enrolment_totals)
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

pub struct TitleVecs {
    pub sd_sids: Vec<String>,
    pub title_texts: Vec<String>,
    pub is_defaults: Vec<bool>,
    pub is_publics: Vec<bool>,
    pub is_scientifics: Vec<bool>,
    pub is_acronyms: Vec<bool>,
    pub comments: Vec<Option<String>>,
}

impl TitleVecs{
    pub fn new(vsize: usize) -> Self {
        TitleVecs { 
            sd_sids: Vec::with_capacity(vsize),
            title_texts: Vec::with_capacity(vsize),
            is_defaults: Vec::with_capacity(vsize),
            is_publics: Vec::with_capacity(vsize),
            is_scientifics: Vec::with_capacity(vsize),
            is_acronyms: Vec::with_capacity(vsize),           
            comments: Vec::with_capacity(vsize),
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.title_texts.shrink_to_fit();
        self.is_defaults.shrink_to_fit();
        self.is_publics.shrink_to_fit();
        self.is_scientifics.shrink_to_fit();
        self.is_acronyms.shrink_to_fit();       
        self.comments.shrink_to_fit();
    }


    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBTitle>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.title_texts.push(r.title_text.clone());
            self.is_defaults.push(r.is_default);
            self.is_publics.push(r.is_public);
            self.is_scientifics.push(r.is_scientific);
            self.is_acronyms.push(r.is_acronym);
            self.comments.push(r.comment.clone());
        }
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_titles (sd_sid, title_text, is_default, is_public, is_scientific, is_acronym, comments) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::bool[], $4::bool[], $5::bool[], $6::bool[], $7::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.title_texts)
        .bind(&self.is_defaults)
        .bind(&self.is_publics)
        .bind(&self.is_scientifics)
        .bind(&self.is_acronyms)       
        .bind(&self.comments)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct IdentifierVecs {
    pub sd_sids: Vec<String>,
    pub id_values: Vec<String>,
    pub id_type_ids: Vec<i32>,
    pub id_types: Vec<String>,
}

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


#[allow(dead_code)]
pub struct OrgVecs {
    pub sd_sids: Vec<String>,
    pub contrib_types: Vec<String>,
    pub org_names: Vec<Option<String>>,
    pub countries: Vec<Option<String>>,
    pub org_ror_ids: Vec<Option<String>>,
    pub org_cref_ids: Vec<Option<String>>,
    pub sponsor_types: Vec<Option<String>>,
}

impl OrgVecs{
    pub fn new(vsize: usize) -> Self {
        OrgVecs { 
            sd_sids: Vec::with_capacity(vsize),
            contrib_types: Vec::with_capacity(vsize),
            org_names: Vec::with_capacity(vsize),
            countries: Vec::with_capacity(vsize),
            org_ror_ids: Vec::with_capacity(vsize),
            org_cref_ids: Vec::with_capacity(vsize),
            sponsor_types: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBOrganisation>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.contrib_types.push(r.contrib_type.clone());
            self.org_names.push(r.org_name.clone());
            self.countries.push(r.country.clone());
            self.org_ror_ids.push(r.org_ror_id.clone());
            self.org_cref_ids.push(r.org_cref_id.clone());
            self.sponsor_types.push(r.sponsor_type.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.contrib_types.shrink_to_fit();
        self.org_names.shrink_to_fit();
        self.countries.shrink_to_fit();
        self.org_ror_ids.shrink_to_fit();
        self.org_cref_ids.shrink_to_fit();
        self.sponsor_types.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_orgs (sd_sid, contrib_type, name, country, ror_id, cross_ref_id, sponsor_type) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.contrib_types)
        .bind(&self.org_names)
        .bind(&self.countries)
        .bind(&self.org_ror_ids)
        .bind(&self.org_cref_ids)
        .bind(&self.sponsor_types)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct PeopleVecs {
    pub sd_sids: Vec<String>,
    pub contrib_types: Vec<String>,
    pub given_names: Vec<Option<String>>,
    pub family_names: Vec<Option<String>>,
    pub orcid_ids: Vec<Option<String>>,
    pub affiliations: Vec<Option<String>>,
    pub email_domains: Vec<Option<String>>,
}

impl PeopleVecs{
    pub fn new(vsize: usize) -> Self {
        PeopleVecs { 
            sd_sids: Vec::with_capacity(vsize),
            contrib_types: Vec::with_capacity(vsize),
            given_names: Vec::with_capacity(vsize),
            family_names: Vec::with_capacity(vsize),
            orcid_ids: Vec::with_capacity(vsize),
            affiliations: Vec::with_capacity(vsize),
            email_domains: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBPerson>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.contrib_types.push(r.contrib_type.clone());
            self.given_names.push(r.given_name.clone());
            self.family_names.push(r.family_name.clone());
            self.orcid_ids.push(r.orcid_id.clone());
            self.affiliations.push(r.affiliation.clone());
            self.email_domains.push(r.email_domain.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.contrib_types.shrink_to_fit();
        self.given_names.shrink_to_fit();
        self.family_names.shrink_to_fit();
        self.orcid_ids.shrink_to_fit();
        self.affiliations.shrink_to_fit();
        self.email_domains.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_people (sd_sid, contrib_type, given_name, family_name,
                        orcid_id, affiliation, email_domain) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[],
                                 $6::text[], $7::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.contrib_types)
        .bind(&self.given_names)
        .bind(&self.family_names)
        .bind(&self.orcid_ids)
        .bind(&self.affiliations)
        .bind(&self.email_domains)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}

pub struct LocationVecs {
    pub sd_sids: Vec<String>,
    pub facilities: Vec<Option<String>>,
    pub addresses: Vec<Option<String>>,
    pub city_names: Vec<Option<String>>,
    pub disamb_names: Vec<Option<String>>,
    pub country_names: Vec<Option<String>>,
}

impl LocationVecs{
    pub fn new(vsize: usize) -> Self {
        LocationVecs { 
            sd_sids: Vec::with_capacity(vsize),
            facilities: Vec::with_capacity(vsize),
            addresses: Vec::with_capacity(vsize),
            city_names: Vec::with_capacity(vsize),
            disamb_names: Vec::with_capacity(vsize),
            country_names: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBLocation>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.facilities.push(r.facility.clone());
            self.addresses.push(r.address.clone());
            self.city_names.push(r.city_name.clone());
            self.disamb_names.push(r.disamb_name.clone());
            self.country_names.push(r.country_name.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.facilities.shrink_to_fit();
        self.addresses.shrink_to_fit();
        self.city_names.shrink_to_fit();
        self.disamb_names.shrink_to_fit();
        self.country_names.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_locations (sd_sid, facility, address, city_name, disamb_name, country_name) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.facilities)
        .bind(&self.addresses)
        .bind(&self.city_names)
        .bind(&self.disamb_names)
        .bind(&self.country_names)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct CountryVecs {
    pub sd_sids: Vec<String>,
    pub country_names: Vec<String>,
}


impl CountryVecs{
    pub fn new(vsize: usize) -> Self {
        CountryVecs { 
            sd_sids: Vec::with_capacity(vsize),
            country_names: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBCountry>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.country_names.push(r.country_name.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.country_names.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_countries (sd_sid, country_name) 
            SELECT * FROM UNNEST($1::text[], $2::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.country_names)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


#[allow(dead_code)]
pub struct ConditionVecs {
    pub sd_sids: Vec<String>,
    pub original_values: Vec<Option<String>>,
    pub original_class1s: Vec<Option<String>>,
    pub original_class2s: Vec<Option<String>>,
    pub ct_type_ids: Vec<Option<i32>>,
    pub ct_codes: Vec<Option<String>>,
}


#[allow(dead_code)]
impl ConditionVecs {

    pub fn new(vsize: usize) -> Self {
        ConditionVecs { 
            sd_sids: Vec::with_capacity(vsize),
            original_values: Vec::with_capacity(vsize),
            original_class1s: Vec::with_capacity(vsize),
            original_class2s: Vec::with_capacity(vsize),
            ct_type_ids: Vec::with_capacity(vsize),
            ct_codes: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBCondition>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.original_values.push(r.original_value.clone());
            self.original_class1s.push(r.original_class1.clone());
            self.original_class2s.push(r.original_class2.clone());
            self.ct_type_ids.push(r.ct_type_id);
            self.ct_codes.push(r.ct_code.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.original_values.shrink_to_fit();
        self.original_class1s.shrink_to_fit();
        self.original_class2s.shrink_to_fit();
        self.ct_type_ids.shrink_to_fit();
        self.ct_codes.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_conditions (sd_sid, original_value, original_class1, original_class2) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.original_values)
        .bind(&self.original_class1s)
        .bind(&self.original_class2s)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


#[allow(dead_code)]
pub struct FeatureVecs {
    pub sd_sids: Vec<String>,
    pub country_names: Vec<String>,
}

#[allow(dead_code)]
impl FeatureVecs{
    pub fn new(vsize: usize) -> Self {
        FeatureVecs { 
            sd_sids: Vec::with_capacity(vsize),
            country_names: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBCountry>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.country_names.push(r.country_name.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.country_names.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_countries (sd_sid, country_name) 
            SELECT * FROM UNNEST($1::text[], $2::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.country_names)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


#[allow(dead_code)]
pub struct TopicVecs {
    pub sd_sids: Vec<String>,
    pub country_names: Vec<String>,
}

#[allow(dead_code)]
impl TopicVecs{
    pub fn new(vsize: usize) -> Self {
        TopicVecs { 
            sd_sids: Vec::with_capacity(vsize),
            country_names: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, sd_sid:&String, v: &Vec<DBCountry>) 
    {
        for r in v {
            self.sd_sids.push(sd_sid.clone());
            self.country_names.push(r.country_name.clone());
        }
    }

    pub fn shrink_to_fit(&mut self) -> () {
        self.sd_sids.shrink_to_fit();
        self.country_names.shrink_to_fit();
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO sd.study_countries (sd_sid, country_name) 
            SELECT * FROM UNNEST($1::text[], $2::text[])"#;

        sqlx::query(sql)
        .bind(&self.sd_sids)
        .bind(&self.country_names)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}






