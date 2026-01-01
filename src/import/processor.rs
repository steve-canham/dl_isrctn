use crate::data_models::json_models::*;
use crate::AppError;
use log::info;

pub fn process_study_json(sd_sid: &String, s: Study) -> Result<(), AppError> {

    let vvv = s.sd_sid;
    info!("{},   {}", sd_sid, vvv);

    Ok(())

}