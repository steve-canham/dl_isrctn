use crate::data_models::json_models::Study;
use crate::data_models::db_models::*;
use chrono::Utc; 
//use crate::AppError;
//use log::info;

 
// The processor needs to creat a full DB version of eacgh study's data,
// working on the whole of the data to present it in aa 'database ready' form.

#[allow(dead_code)]
pub fn get_study_data(s: &Study) -> DBSummary {
    
    let status_id = Some(0);
    let type_id = Some(1);
    let iec_flag = Some(0);
    let display_title = Some("title".to_string());
    let dt_of_data = Utc::now();
    
    DBSummary {
        sd_sid: s.sd_sid.clone(),
        display_title: display_title,
        brief_description: s.summary.plain_english_summary.clone(),
        type_id: type_id,
        status_id: status_id,
        iec_flag: iec_flag,
        ipd_sharing: s.ipd.ipd_sharing_plan,
        ipd_sharing_plan: s.ipd.ipd_sharing_statement.clone(), 
        dt_of_data: dt_of_data, 
    }

}

#[allow(dead_code)]
pub fn get_study_dates_data(s: &Study) -> DBStudyDates {
      
    let reg_year = Some(2020);  
    let reg_month = Some(6);  
    let reg_date_type = Some("E".to_string());           
    let start_year = Some(2020);
    let start_month = Some(6);    
    let start_date_type = Some("E".to_string());     
    let comp_year = None;
    let comp_month = None; 
    let comp_date_type = None;     
    let res_year = None; 
    let res_month = None;  
    let res_date_type = None;



    DBStudyDates {
        sd_sid: s.sd_sid.clone(),
        reg_year: reg_year,  
        reg_month: reg_month,
        reg_date_type: reg_date_type,         
        start_year: start_year, 
        start_month: start_month,   
        start_date_type: start_date_type,       
        comp_year: comp_year,
        comp_month: comp_month,  
        comp_date_type: comp_date_type,      
        res_year: res_year,  
        res_month: res_month,   
        res_date_type: res_date_type,    
    }

}

/*
pub fn get_study_partics_data(s: &Study) -> DBStudyPartics {


}

pub fn get_study_titles_data(s: &Study) -> Vec<DBTitle> {


}

pub fn get_study_idents_data(s: &Study) -> Vec<DBIdentifier> {


}
 */