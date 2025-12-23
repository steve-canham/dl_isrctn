mod file_models;
mod processor;
pub mod data_access;
pub mod gen_helper;
mod who_helper;

//use std::collections::HashMap;
//use std::path::PathBuf;
use crate::{AppError};
use super::setup::InitParams;
use super::DownloadResult;
use chrono::NaiveDate;

//use data_access::{update_who_study_mon, add_new_single_file_record, 
// add_contents_record, store_who_summary};
//use file_models::WHOLine;
//use super::setup::config_reader::fetch_src_db_name;
//use std::fs;
//use std::io::BufReader;
//use std::fs::File;
//use csv::ReaderBuilder;
//use std::io::Write;
//use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};
use log::info;


pub async fn process_data(params: &InitParams, _dl_id:i32, _mon_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    // The base url, json file folder, log folder, and start and end dates have
    // already been checked as being present and reasonable.

    let res = DownloadResult::new();
    //let res = DwnloadRevisedRecords(params.base_url, params.start_date, params.end_date, dl_id, mon_pool)?;

    // initially get a single study to indicate total number to be downloaded.

   let url = get_url( &params.base_url, &params.start_date, &params.end_date, 1);
   let response = reqwest::get(url.clone()).await.map_err(|e| AppError::ReqwestError(url.clone(), e))?;
   let xml_content = response.text().await.map_err(|e| AppError::ReqwestError(url, e))?;

   //let initial_result = Deserialize<allTrials?>(responseBodyAsString);

   info!("{}", xml_content);

    /*
       

    // DownloadRevisedRecords returns all records that have been revised on or since 
    // the cutoff date, including today's records. This means that successive calls will
    // often overlap on the day of the call. This is by design as the call day's records will
    // not necessarily be complete when the call is made.

    public async Task<DownloadResult> DownloadRevisedRecords(string file_base, DateTime cut_off_date, int source_id, int saf_id)
    {
        DownloadResult res = new();
        ScrapingHelpers ch = new(_loggingHelper); 

        // initially get a single study to indicate total number to be downloaded.

        string url = GetUrl(1, cut_off_date);
        string? responseBodyAsString = await ch.GetAPIResponseAsync(url);
        allTrials? initial_result = Deserialize<allTrials?>(responseBodyAsString, _loggingHelper);

        if (initial_result is not null)
        {
            int record_num = initial_result.totalCount;
            if (record_num > 0)
            {
                if (record_num <= 100)
                {
                    // Do a single call but with an increased limit.

                    url = GetUrl(record_num, cut_off_date);
                    responseBodyAsString = await ch.GetAPIResponseAsync(url);
                    if (responseBodyAsString is not null)
                    {
                        DownloadResult batch_res = await DownloadBatch(responseBodyAsString, file_base, source_id,
                               saf_id);
                        res.num_checked += batch_res.num_checked;
                        res.num_downloaded += batch_res.num_downloaded;
                        res.num_added += batch_res.num_added;
                    }
                }
                else
                { 
                    // Split the calls to a per day basis.

                    DateTime date_to_check = cut_off_date;
                    while (date_to_check.Date <= DateTime.Now.Date)
                    {
                        DownloadResult day_res = await DownloadStudiesFromSingleDay(date_to_check, file_base, 
                            source_id, saf_id);
                        res.num_checked += day_res.num_checked;
                        res.num_downloaded += day_res.num_downloaded;
                        res.num_added += day_res.num_added;

                        string feedback = $"{day_res.num_downloaded} studies downloaded, for {date_to_check.ToShortDateString()}.";
                        feedback += $" Total downloaded: {res.num_downloaded}";
                        _loggingHelper.LogLine(feedback);

                        date_to_check = date_to_check.AddDays(1);
                        Thread.Sleep(800);  // Add a pause between calls.
                    }
                }
            }
        }

        return res;
    }

    // Downloads all studies last edited from the first date (inclusively) and the 
    // last date (exclusively) - i.e. GE date 1 and LT date 2.
    // By default the download is done in batches of 4 days. If the end date is included
    // in a batch, the batch is made up to the end date.

    public async Task<DownloadResult> DownloadRecordsBetweenDates(string file_base, DateTime start_date, 
        DateTime end_date, int source_id, int saf_id,
        IMonDataLayer mon_data_layer, ILoggingHelper logging_helper)
    {
        DownloadResult res = new();
        ScrapingHelpers ch = new(logging_helper);

        // If the start date is earlier than 10/11/2005 it is made into 10/11/2005,
        // the earliest date in the ISRCTN system for 'date last edited'.
        // If the end date is later than today it is made today.
        // Dates are transformed into number of days post 01/01/2005.
        // Day numbers are then used to loop through the requested period.

        start_date = start_date < new DateTime(2005, 11, 10) ? new DateTime(2005, 11, 10) : start_date;
        end_date = end_date > DateTime.Now ? DateTime.Now.Date : end_date;

        DateTime baseDate = new DateTime(2005, 1, 1);
        int startday = (start_date - baseDate).Days;
        int endday = (end_date - baseDate).Days;

        for (int d = startday; d < endday; d += 4)
        {
            // The 4 days being considered are the start date
            // and the following three days. 

            DateTime date_GE = baseDate.AddDays(d);
            DateTime date_LT = date_GE.AddDays(4);

            // Must end on the correct day, therefore
            // check and truncate end of period if necessary.

            date_LT = date_LT > end_date ? end_date : date_LT;

            // Initial call to get number of studies in this period

            string url = GetUrl(1, date_GE, date_LT);
            string? responseBodyAsString = await ch.GetAPIResponseAsync(url);
            allTrials? result = Deserialize<allTrials?>(responseBodyAsString, logging_helper);
            if (result is not null)
            {
                int record_num = result.totalCount;
                if (record_num > 0)
                {
                    if (record_num <= 100)
                    {
                        // Do a single call but with the increased limit.

                        url = GetUrl(record_num, date_GE, date_LT);
                        responseBodyAsString = await ch.GetAPIResponseAsync(url);
                        if (responseBodyAsString is not null)
                        {
                            DownloadResult batch_res = await DownloadBatch(responseBodyAsString, file_base, source_id, saf_id);
                            res.num_checked += batch_res.num_checked;
                            res.num_downloaded += batch_res.num_downloaded;
                            res.num_added += batch_res.num_added;

                            string feedback = $"{batch_res.num_downloaded} studies downloaded, ";
                            feedback += $"with last edited GE { date_GE.ToShortDateString()} and LT { date_LT.ToShortDateString()}. ";
                            feedback += $"Total downloaded: {res.num_downloaded}";
                            logging_helper.LogLine(feedback);
                            Thread.Sleep(800);  // Add a pause between calls.
                        }
                    }
                    else
                    { 
                        // Split the calls to a per day basis.

                        DateTime date_to_check = date_GE;
                        while (date_to_check.Date < date_LT)
                        {
                            DownloadResult day_res = await DownloadStudiesFromSingleDay(date_to_check, file_base, source_id, saf_id);
                            res.num_checked += day_res.num_checked;
                            res.num_downloaded += day_res.num_downloaded;
                            res.num_added += day_res.num_added;

                            string feedback = $"{day_res.num_downloaded} studies downloaded, for {date_to_check.ToShortDateString()}.";
                            feedback += $" Total downloaded: {res.num_downloaded}";
                            logging_helper.LogLine(feedback);
                            date_to_check = date_to_check.AddDays(1);
                            Thread.Sleep(800);  // Add a pause between calls.
                        }
                    }
                }
            }
        }

        return res;
    }

    // Downloads the study records where day = last edited is a single designated day
    // Called from both DownloadRevisedRecords and DownloadRecordsBetweenDates when amounts for
    // a period exceed 100 and the system switches to getting records one day at a time.
    // First gets a single record to calculate total amount to be retrieved, and
    // then sets the limit in a following call to retrieve all records.

    private async Task<DownloadResult> DownloadStudiesFromSingleDay(DateTime date_to_check, string file_base, 
        int source_id, int saf_id)
    {
        DownloadResult res = new();
        ScrapingHelpers ch = new(_loggingHelper);
        DateTime next_day_date = date_to_check.AddDays(1);

        string url = GetUrl(1, date_to_check, next_day_date);
        string? responseBodyAsString = await ch.GetAPIResponseAsync(url);
        allTrials? day_result = Deserialize<allTrials?>(responseBodyAsString, _loggingHelper);

        if (day_result is not null)
        {
            int day_record_num = day_result.totalCount;
            if (day_record_num > 0)
            {
                Thread.Sleep(300);
                url = GetUrl(day_record_num, date_to_check, next_day_date);
                responseBodyAsString = await ch.GetAPIResponseAsync(url);
                if (responseBodyAsString is not null)
                {
                    DownloadResult batch_res = await DownloadBatch(responseBodyAsString, file_base, source_id, saf_id);
                    res.num_checked += batch_res.num_checked;
                    res.num_downloaded += batch_res.num_downloaded;
                    res.num_added += batch_res.num_added;
                }
            }
        }
        return res;
    }

    // Batch download, called by other functions whenever a set of study records has been obtained, (as a string
    // from an API call). The string first needs deserializing to the response object, and then each individual 
    // study needs to be transformed into the json file model, and saved as a json file in the appropriate folder.

    private async Task<DownloadResult> DownloadBatch(string responseBodyAsString, string file_base, 
                        int source_id, int saf_id)
    {
        DownloadResult res = new();
        allTrials? result = Deserialize<allTrials?>(responseBodyAsString, _loggingHelper);
        if(result is null)
        {
            _loggingHelper.LogError("Error de-serialising " + responseBodyAsString);
            return res;
        }

        ISRCTN_Processor isrctn_processor = new();
        int number_returned = result.totalCount;
        if (number_returned > 0 && result.fullTrials?.Any() is true) 
        { 
            foreach (FullTrial f in result.fullTrials)
            {
                res.num_checked++;
                Study? s = await isrctn_processor.GetFullDetails(f, _loggingHelper);
                if (s is not null)
                {
                    string full_path = await WriteOutFile(s, s.sd_sid, file_base);
                    if (full_path != "error")
                    {
                        string remote_url = "https://www.isrctn.com/" + s.sd_sid;
                        DateTime? last_updated = s.lastUpdated?.FetchDateTimeFromISO();
                        bool added = _monDataLayer.UpdateStudyDownloadLog(source_id, s.sd_sid, remote_url, saf_id,
                                                last_updated, full_path);
                        res.num_downloaded++;
                        if (added) res.num_added++;
                    }
                }
            }
        }
        return res;
    }

    // Writes out the file with the correct name to the correct folder, as indented json.
    // Called from the DownloadBatch function.
    // Returns the full file path as constructed, or an 'error' string if an exception occurred.

    private async Task<string> WriteOutFile(Study s, string sd_sid, string file_base)
    {
        string file_name = sd_sid + ".json";
        string full_path = Path.Combine(file_base, file_name);
        try
        {
            await using FileStream jsonStream = File.Create(full_path);
            await JsonSerializer.SerializeAsync(jsonStream, s, _json_options);
            await jsonStream.DisposeAsync();
            return full_path;
        }
        catch (Exception e)
        {
            _loggingHelper.LogLine("Error in trying to save file at " + full_path + ":: " + e.Message);
            return "error";
        }
    }

    // String function that constructs the required URL for the ISRCTN API.

    private string GetUrl(int limit, DateTime startdate, DateTime? enddate = null)
    {
        string start_date_param, id_params;
        if (enddate is null)
        {
            start_date_param = $"{startdate.Year}-{startdate.Month:00}-{startdate.Day:00}";
            id_params = "lastEdited%20GE%20" + start_date_param + "T00:00:00%20";
        }
        else
        {
            DateTime end_date = (DateTime)enddate;
            start_date_param = $"{startdate.Year}-{startdate.Month:00}-{startdate.Day:00}";
            string end_date_param = $"{end_date.Year}-{end_date.Month:00}-{end_date.Day:00}";
            id_params = "lastEdited%20GE%20" + start_date_param + "T00:00:00%20AND%20lastEdited%20LT%20" + end_date_param + "T00:00:00";
        }
        string end_url = $"&limit={limit}";
        return _base_url + id_params + end_url;
    }


    
    */

        Ok(res)

}


fn get_url(base_url: &String, start_date: &NaiveDate, end_date: &NaiveDate, limit: i32) -> String
{
    let start_date_param = start_date.format("%Y-%m-%d").to_string();
    let query_start_param = format!("lastEdited%20GE%20{}T00:00:00%20AND%20", start_date_param);

    let end_date_param = end_date.format("%Y-%m-%d").to_string();
    let query_end_param = format!("lastEdited%20LE%20{}T00:00:00%20&limit=", end_date_param);

    format!("{}{}{}{}", base_url, query_start_param, query_end_param, limit)

    /*
    https://www.isrctn.com/api/query/format/default?q=lastEdited%20GT%202020-01-01T00:00:00%20AND%20lastEdited%20LE%202020-01-05T00:00:00%20&limit=1
     */
}


/* 
fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
    */
