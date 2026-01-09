pub mod monitoring;
mod processor;
mod support_fns;
use std::fs;
use std::path::PathBuf;

use crate::data_models::json_models::*;
use crate::data_models::data_vecs::*;

use crate::AppError;
use sqlx::{Pool, Postgres};
use crate::base_types::{ImportType, ImportResult};
use chrono::Local;
use log::info;


#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct FilePath {
    sd_sid: String,
    local_path: String,
}

pub async fn import_data(import_type: &ImportType, _imp_event_id:i32, src_pool: &Pool<Postgres>) -> Result<ImportResult, AppError> {

    // First recreate the staging schema tables - sqlscript in file (path is relative)

    let sql = include_str!("../../sql/sd_tables.sql");
    sqlx::raw_sql(sql).execute(src_pool)
        .await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    
    // get the total number of records to be processed (depends on import type)

    let count_sql = match import_type {
        ImportType::Recent => {
            r#"select count(*) from mn.source_data
            where last_imported is null 
            or last_downloaded > last_imported"#
        },
        ImportType::All => {
            "select count(*) from mn.source_data;"
        }
        ImportType::None => ""
    };

    let num_files: i64 = sqlx::query_scalar(count_sql).fetch_one(src_pool).await
                    .map_err(|e| AppError::SqlxError(e, count_sql.to_string()))?;

    // Set the loops rolling, through the batches of records.
    // Re-iniitalise the data vectors at the start of each batch
    // After each batch store the data vectors into the database.

    let batch_size = 250;

    for n in (0..num_files).step_by(batch_size) { 

        // iniitalise the data vectors

        let mut studies_dv = StudyVecs::new(batch_size);
        let mut study_dates_dv = StudyDatesVecs::new(batch_size);
        let mut study_partics_dv = StudyParticsVecs::new(batch_size);
        let mut study_titles_dv = TitleVecs::new(3*batch_size);
        let mut study_idents_dv = IdentifierVecs::new(3*batch_size);
        let mut study_orgs_dv = OrgVecs::new(3*batch_size);
        let mut study_people_dv = PeopleVecs::new(3*batch_size);
        let mut study_locs_dv = LocationVecs::new(3*batch_size);
        let mut study_cnts_dv = CountryVecs::new(3*batch_size);
        let mut study_conds_dv = ConditionVecs::new(2*batch_size);
        let mut study_feats_dv = FeatureVecs::new(4*batch_size);
        let mut study_tops_dv = TopicVecs::new(3*batch_size);
        let mut study_iec_dv = IECVecs::new(10*batch_size);

        // get the list of json files relevant to this pass

        let file_sql = match import_type {
            ImportType::Recent => {
                    format!(r#"select sd_sid, local_path from mn.source_data
                    where last_imported is null 
                    or last_downloaded > last_imported
                    ORDER BY sd_sid
                    offset {} limit {}"#, n, batch_size)
            },
            ImportType::All  => {
                    format!(r#"select sd_sid, local_path from mn.source_data
                    ORDER BY sd_sid
                    offset {} limit {}"#, n, batch_size)
            }
            ImportType::None => "".to_string()
        };

        let file_list: Vec<FilePath> = sqlx::query_as(&file_sql).fetch_all(src_pool).await
                        .map_err(|e| AppError::SqlxError(e, file_sql))?;      
        //let mut i = 0;

        for path in file_list {
            
            // Deserialise the file beiung referenhced and pass for processing

            let p= PathBuf::from(&path.local_path);
            let json_data = fs::read_to_string(&p)?;
            let s: Study = serde_json::from_str(&json_data)?;

            // pass s to the procesor and receive a 'database friendly' version, 
            // with the data arranged to match the tables in the DB.
            // Rather than immediately store for each study individually,
            // acummulate the data intothe data vectgors and store them
            // as a bulk insert below.

            let dbs = processor::process_study_data(&s);
            let sd_sid = &dbs.sd_sid;
            studies_dv.add(sd_sid,&dbs.summary);
            study_dates_dv.add(sd_sid, &dbs.dates);
            study_partics_dv.add(sd_sid, &dbs.participants);

            if let Some(ts) = dbs.titles { study_titles_dv.add(sd_sid, &ts); }
            if let Some(ids) = dbs.identifiers { study_idents_dv.add(sd_sid, &ids); }
            if let Some(orgs) = dbs.orgs { study_orgs_dv.add(sd_sid, &orgs); }
            if let Some(peop) = dbs.people { study_people_dv.add(sd_sid, &peop); }
            if let Some(locs) = dbs.locations { study_locs_dv.add(sd_sid, &locs); }
            if let Some(cies) = dbs.countries { study_cnts_dv.add(sd_sid, &cies); }
            if let Some(conds) = dbs.conditions { study_conds_dv.add(sd_sid, &conds); }
            if let Some(feats) = dbs.features { study_feats_dv.add(sd_sid, &feats); }
            if let Some(tops) = dbs.topics { study_tops_dv.add(sd_sid, &tops); }
            if let Some(iecs) = dbs.ie_crit { study_iec_dv.add(sd_sid, &iecs); }
                    
            //i += 1;
            //if i > 40 { break;}
        } 

        study_titles_dv.shrink_to_fit();
        study_idents_dv.shrink_to_fit();
        study_orgs_dv.shrink_to_fit();
        study_people_dv.shrink_to_fit();
        study_locs_dv.shrink_to_fit();
        study_cnts_dv.shrink_to_fit();
        study_conds_dv.shrink_to_fit();
        study_feats_dv.shrink_to_fit();
        study_tops_dv.shrink_to_fit();
        study_iec_dv.shrink_to_fit();


        studies_dv.store_data(src_pool).await?;
        study_dates_dv.store_data(src_pool).await?;
        study_partics_dv.store_data(src_pool).await?;
        study_titles_dv.store_data(src_pool).await?;
        study_idents_dv.store_data(src_pool).await?;
        study_orgs_dv.store_data(src_pool).await?;
        study_people_dv.store_data(src_pool).await?;
        study_locs_dv.store_data(src_pool).await?;
        study_cnts_dv.store_data(src_pool).await?;
        study_conds_dv.store_data(src_pool).await?;
        study_feats_dv.store_data(src_pool).await?;
        study_tops_dv.store_data(src_pool).await?;
        study_iec_dv.store_data(src_pool).await?;


        if n > 1000 {
            break;
        }

    }

    info!("number of files found: {}",  num_files);

    Ok(ImportResult {
        num_available: num_files,
        num_imported: 0,
        earliest_dl_date: Local::now().date_naive(),
        latest_dl_date: Local::now().date_naive(),
    })
}
