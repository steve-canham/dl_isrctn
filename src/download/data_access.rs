use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use chrono::{NaiveDate, Utc};
use std::collections::HashMap;
use crate::{err::AppError, DownloadResult};
use crate:: download::file_models::{WHOSummary, SecondaryId};

pub async fn update_who_study_mon(db_name: &String, sd_sid: &String, remote_url: &Option<String>, dl_id: i32,
                     record_date: &Option<NaiveDate>, full_path: &PathBuf, pool: &Pool<Postgres>) -> Result<bool, AppError> {

        let mut added = false;          // indicates if will be a new record or update of an existing one
        let now = Utc::now();
              
        let local_path = full_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/");     // assumes utf-8 characters
        let sql = format!("SELECT EXISTS(SELECT 1 from mon.{} where sd_sid = '{}')", db_name, sd_sid); 
        let mon_record_exists = sqlx::query_scalar(&sql).fetch_one(pool).await
                        .map_err(|e| AppError::SqlxError(e, sql))?;
        if mon_record_exists {
            
            // Row already exists - update with new details.
           
            let sql = "Update mon.".to_string() + db_name + r#" set 
                        remote_src_url = $1,
                        last_who_revised = $2,
                        local_path = $3,
                        last_who_dl_id = $4,
                        last_who_downloaded = $5
                        where sd_sid = $6;"#;
            sqlx::query(&sql).bind(remote_url).bind(record_date).bind(local_path) 
                    .bind(dl_id).bind(now).bind(sd_sid).execute(pool).await
                    .map_err(|e| AppError::SqlxError(e, sql))?;       
        }
        else {
            
            // Create as a new record.
            
            let sql = "Insert into mon.".to_string() + db_name + r#"(sd_sid, remote_src_url, last_who_revised,
	                    local_path, last_who_dl_id, last_who_downloaded) values ($1, $2, $3, $4, $5, $6)"#;
            sqlx::query(&sql).bind(sd_sid).bind(remote_url).bind(record_date)    
            .bind(local_path).bind(dl_id).bind(now).execute(pool).await
                    .map_err(|e| AppError::SqlxError(e, sql))?;     
            added = true;  
        }

        Ok(added)
}


pub async fn get_next_download_id(pool: &Pool<Postgres>) -> Result<i32, AppError>{

    let sql = "select max(id) from evs.dl_events ";
    let last_id: i32 = sqlx::query_scalar(sql).fetch_one(pool)
                      .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    let new_id = last_id + 1;
    
    // Create the new record (to be updated later).
    let now = Utc::now();
    let sql = "Insert into evs.dl_events(id, source_id, time_started) values ($1, $2, $3)";
    sqlx::query(sql).bind(new_id).bind(100115).bind(now).execute(pool)
             .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(new_id)
}


pub async fn update_dl_event_record (dl_id: i32, type_id: i32, dl_res: DownloadResult, pool: &Pool<Postgres>) ->  Result<bool, AppError> {
     
    let now = Utc::now();
    let sql = r#"Update evs.dl_events set 
             num_records_checked = $1,
             num_records_downloaded = $2,
             num_records_added = $3,
             time_ended = $4,
             type_id = $5
             where id = $6"#;
    let res = sqlx::query(sql).bind(dl_res.num_checked).bind(dl_res.num_downloaded).bind(dl_res.num_added)
          .bind(now).bind(type_id).bind(dl_id).execute(pool)
             .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}


pub async fn add_new_single_file_record(dl_id: i32, file_path: &PathBuf, file_res: &DownloadResult, pool: &Pool<Postgres>) -> Result<bool, AppError> {

    let source_path = file_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/");     // assumes utf-8 characters
    let date_dl = Utc::now().date_naive();
    let sql = r#"Insert into der.who_file_dls(dl_id, file_path, date_dl, 
                num_checked, num_downloaded, num_added) 
                values($1, $2, $3, $4, $5, $6)"#;
    let res = sqlx::query(sql).bind(dl_id).bind(source_path).bind(date_dl)
                .bind(file_res.num_checked).bind(file_res.num_downloaded).bind(file_res.num_added)
                .execute(pool).await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}

pub async fn add_contents_record(file_path: &PathBuf, source_tots: &mut HashMap<i32, i32>, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let source_path = file_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/");   
    let mut source_ids = Vec::<i32>::new();
    let mut tots = Vec::<i32>::new();
    for (k, v) in source_tots.drain() {
        source_ids.push(k);
        tots.push(v)
    }

    // ?? delete existing first ?? 
    let sql = r#"Insert into der.who_file_contents (file_path, source_id, num_found)
                 select $1, a.*
                    from
                    (select * from UNNEST($2::int[], $3::int[])) as a"#;
               let res = sqlx::query(sql).bind(source_path)
               .bind(source_ids).bind(tots)
               .execute(pool).await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 

    Ok(res.rows_affected())
}

pub async fn store_who_summary(rec: WHOSummary, pool: &Pool<Postgres>) -> Result<bool, AppError> {

    // WHO summary data needs to be modified before storage in db.

    let reg_year: Option<i32>;
    let reg_month: Option<i32>;
    let reg_day: Option<i32>;
    (reg_year, reg_month, reg_day) = split_iso_date(rec.date_registration);

    let enrol_year: Option<i32>;
    let enrol_month: Option<i32>;
    let enrol_day: Option<i32>;
    (enrol_year, enrol_month, enrol_day) = split_iso_date(rec.date_enrolment);

    let reg_ids: Option<Vec::<String>>;
    let oth_ids: Option<Vec::<String>>;
    (reg_ids, oth_ids) = split_secids(rec.secondary_ids);
     
    let now = Utc::now();
    let sql_prefix = "INSERT INTO bas.".to_string() + &rec.table_name;
    let sql = sql_prefix + r#" (source_id, sd_sid, title, remote_url, study_type, study_status, 
                    reg_sec_ids, oth_sec_ids, 
                    reg_year, reg_month, reg_day, enrol_year, enrol_month, enrol_day,
                    results_yes_no, results_url_link, results_url_protocol, 
                    results_date_posted, results_date_first_pub, results_date_completed,
                    country_list, date_last_rev, date_last_edited)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23)
            ON CONFLICT (sd_sid)
            DO UPDATE SET
                source_id = $1, 
                title = $3, 
                remote_url = $4, 
                study_type = $5, 
                study_status = $6, 
                reg_sec_ids = $7, 
                oth_sec_ids = $8, 
                reg_year = $9, 
                reg_month = $10, 
                reg_day = $11, 
                enrol_year = $12, 
                enrol_month = $13, 
                enrol_day = $14,
                results_yes_no = $15,  
                results_url_link = $16,  
                results_url_protocol = $17,  
                results_date_posted = $18,  
                results_date_first_pub = $19,  
                results_date_completed = $20, 
                country_list = $21,  
                date_last_rev = $22,  
                date_last_edited = $23"#;

    let res = sqlx::query(&sql)
        .bind(rec.source_id).bind(rec.sd_sid).bind(rec.title)
        .bind(rec.remote_url).bind(rec.study_type).bind(rec.study_status)
        .bind(reg_ids).bind(oth_ids)
        .bind(reg_year).bind(reg_month).bind(reg_day)
        .bind(enrol_year).bind(enrol_month).bind(enrol_day)
        .bind(rec.results_yes_no).bind(rec.results_url_link).bind(rec.results_url_protocol)
        .bind(rec.results_date_posted).bind(rec.results_date_first_pub).bind(rec.results_date_completed)
        .bind(rec.country_list).bind(rec.date_last_rev).bind(now)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql))?;

    Ok(res.rows_affected() == 1)

} 


fn split_iso_date (dt: Option<String>) -> (Option<i32>, Option<i32>, Option<i32>) {

    match dt {
        Some(d) => {
            if d.len() != 10 {
                println!("Odd iso date: {}", d);
            }
            let year: i32 = d[0..4].parse().unwrap_or(0);
            let month: i32 = d[5..7].parse().unwrap_or(0);
            let day: i32 = d[8..].parse().unwrap_or(0);
            if year != 0 && month != 0 && day != 0 {
                (Some(year), Some(month), Some(day))           
            }
            else {
                (None, None, None)      
            }
         },
         None => (None, None, None),     
    }
}

fn split_secids (ids: Option<Vec<SecondaryId>>) -> (Option<Vec<String>>, Option<Vec<String>>) {
    
    let mut reg_ids = Vec::<String>::new();
    let mut oth_ids = Vec::<String>::new();

    match ids {
        Some(sids) => {
            if sids.len() > 0 {
                for secid in sids {
                   if secid.sec_id_type_id == 11 {
                       reg_ids.push(format!("{}::{}", secid.sec_id_source, secid.processed_id))
                   }
                   else {
                       oth_ids.push(secid.sec_id)
                   }
                }

                let reg_sec_ids = match reg_ids.len() {
                    0 => None,
                  _ => Some(reg_ids)
                };
                let oth_sec_ids = match oth_ids.len() {
                    0 => None,
                    _ => Some(oth_ids)
                };
                (reg_sec_ids, oth_sec_ids)
           }
           else {
            (None, None)
           }
        },
        None => (None, None),
    }
    
    

}




/*

pub fn is_who_test_study() -> bool {

    public bool IsWHOTestStudy(string dbname, string sd_sid)
    {
        string whoConnString = _credentials.GetConnectionString(dbname);
        using NpgsqlConnection conn = new(whoConnString);
        string sql_string = @$"select for_testing
                    from mn.source_data where sd_sid = '{sd_sid}';";
        bool? res = conn.QueryFirstOrDefault<bool?>(sql_string);
        return res == true;
    }
    false
}

public bool WriteFile(string sid, string jsonString, string folder_path)
{
    try
    {
        // Write out study record as json.

        string full_path = Path.Combine(folder_path, sid + ".json");
        File.WriteAllText(full_path, jsonString);

        if (IsTestStudy(sid))
        {
            // write out copy of the file in the test folder

            string test_path = _logging_helper.TestFilePath;
            string full_test_path = Path.Combine(test_path, sid + ".json");
            File.WriteAllText(full_test_path, jsonString);
        }
        return true;
    }

}


*/