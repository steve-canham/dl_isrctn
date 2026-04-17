use chrono::{NaiveDate, Utc, Datelike};
use clap::{command, Arg, ArgMatches};
use crate::base_types::{DownloadType, ImportType, EncodingType};
use crate::err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliPars {
    pub import_type: ImportType,
    pub download_type: DownloadType,
    pub encoding_type: EncodingType,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_test: bool,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args.to_vec())?;

    let do_all_recent = parse_result.get_flag("do_all_recent");

    let mut dl_updated_recently = parse_result.get_flag("dl_recent");
    let dl_updated_between_dates = parse_result.get_flag("dl_updated_between_dates");
    let dl_updated_in_year = parse_result.get_flag("dl_updated_in_year");

    let mut imp_recent_flag = parse_result.get_flag("imp_flag");
    let mut imp_all_flag = parse_result.get_flag("imp_all_flag");
    let mut code_recent_flag = parse_result.get_flag("code_flag");
    let mut code_all_flag = parse_result.get_flag("code_all_flag");

    let test_flag = parse_result.get_flag("test_flag");

    // dates have default values of "" so can be unwrapped

    let start_date_as_string = parse_result.get_one::<String>("start_date").unwrap();
    let end_date_as_string = parse_result.get_one::<String>("end_date").unwrap();

    if do_all_recent {
        dl_updated_recently = true;
        imp_recent_flag = true;
        code_recent_flag = true;
    }

    // if no (non-test) flags set a 'dl_updated_recently' flag is assumed.
    // This still requires a start date but that requirement
    // could be dropped if the program can use the day of the last 
    // download date as the start date    

    if !imp_recent_flag && !imp_all_flag  
    && !dl_updated_recently && !dl_updated_between_dates && !dl_updated_in_year 
    && !code_recent_flag && !code_all_flag
    {
        dl_updated_recently = true;
    }

    let mut download_type = DownloadType::None;
    let mut import_type = ImportType::None;
    let mut encoding_type = EncodingType::None;
    let mut start_date = None;
    let mut end_date = None;
    let today = Utc::now().date_naive();
    let isrctn_start_date = NaiveDate::from_ymd_opt(2005, 11, 1).unwrap();

    if imp_recent_flag || imp_all_flag {

        if imp_recent_flag && imp_all_flag {
            imp_all_flag = false;   // if both true only recent import done
        }
        import_type = if imp_all_flag {ImportType::All} else {ImportType::Recent};
    }

    if code_recent_flag || code_all_flag {

        if code_recent_flag && code_all_flag {
            code_all_flag = false;   // if both true only recent code done
        }
        encoding_type = if code_all_flag {EncodingType::All} else {EncodingType::Recent};
    }

    if dl_updated_recently || dl_updated_between_dates || dl_updated_in_year {

        if dl_updated_recently {
            
            start_date = get_start_date(start_date_as_string, today, isrctn_start_date)?;
            end_date = Some(Utc::now().date_naive());
            download_type = DownloadType::Recent;
        }

        if dl_updated_between_dates {

            // Both start and end date parameters are essential.

            start_date = get_start_date(start_date_as_string, today, isrctn_start_date)?;
            end_date = get_end_date(end_date_as_string, today)?;
            download_type = DownloadType::BetweenDates;
        }

        if dl_updated_in_year {

            download_type = DownloadType::ByYear;

            let year: i32 = start_date_as_string.parse().unwrap_or_else(|_| 0);
            let current_year = Utc::now().year();

            if year == 0 {
                return Result::Err(AppError::MissingProgramParameter("year not provided for download".to_string()));
            }
            else if year < 2005 || year > current_year {
                return Result::Err(AppError::MissingProgramParameter("year provided is invalid".to_string()));
            }
            else {
                let mut s_date =  NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
                let mut e_date =  NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();
                           
                if s_date < isrctn_start_date {
                    s_date = isrctn_start_date
                }

                if e_date > today {
                    e_date = today;
                }

                start_date = Some(s_date);
                end_date = Some(e_date);
            }
        }
 
    }
        
    Ok(CliPars {
        download_type: download_type,
        import_type: import_type,
        encoding_type: encoding_type,
        start_date: start_date,
        end_date: end_date,
        is_test: test_flag,
    }) 
        
}


fn get_start_date(sd_param: &String, today: NaiveDate, isrctn_start_date: NaiveDate) -> Result<Option<NaiveDate>, AppError> {

    let mut start_date = match NaiveDate::parse_from_str(sd_param, "%Y-%m-%d") {
        Ok(date) => {if date >= today {   // invalid
                                    return Result::Err(AppError::MissingProgramParameter("valid start date".to_string()));
                                }
                                else {date}},
        Err(_) => {
            return Result::Err(AppError::MissingProgramParameter("valid start date".to_string()))},
    };
    if start_date < isrctn_start_date {
        start_date = isrctn_start_date;
    }
    Ok(Some(start_date))
}


fn get_end_date(ed_param: &String, today: NaiveDate) -> Result<Option<NaiveDate>, AppError> {

    let mut end_date = match NaiveDate::parse_from_str(ed_param, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {return Result::Err(AppError::MissingProgramParameter("valid end date".to_string()))},
    };
    if end_date > today {
        end_date = today;
    }
    Ok(Some(end_date))
}


pub fn config_file_exists()-> bool {
    let config_path = PathBuf::from("./app_config.toml");
    let res = match config_path.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}


fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {

    command!()
        .about("Imports data from ISRCTN API and transforms it into local json files")
        .arg(
            Arg::new("do_all_recent")
           .short('a')
           .long("recent_data")
           .required(false)
           .help("")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("dl_recent")
           .short('r')
           .long("download")
           .required(false)
           .help("")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("dl_updated_between_dates")
           .short('b')
           .long("between")
           .required(false)
           .help("")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("dl_updated_in_year")
           .short('y')
           .long("year")
           .required(false)
           .help("Only data last edited in this year should be downloaded")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("start_date")
           .short('s')
           .long("start_date")
           .required(false)
           .help("Only data last edited on or after this date should be downloaded")
           .default_value("")
        )
        .arg(
            Arg::new("end_date")
           .short('e')
           .long("end_date")
           .required(false)
           .help("Only data last edited before this date should be downloaded")
           .default_value("")
        )
        .arg(
            Arg::new("imp_flag")
           .short('i')
           .long("import")
           .required(false)
           .help("A flag signifying import files downloade since the last import")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("imp_all_flag")
           .short('I')
           .long("import_all")
           .required(false)
           .help("A flag signifying (re-)import all data from source json files")
           .action(clap::ArgAction::SetTrue)
        )
         .arg(
            Arg::new("code_flag")
            .short('c')
            .long("coderecent")
            .required(false)
            .help("A flag signifying code all data downloaded since the last coding process")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("code_all_flag")
            .short('C')
            .long("codeall")
            .required(false)
            .help("A flag indicating signifying (re)code all data")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("test_flag")
             .short('z')
             .long("test")
             .required(false)
             .help("A flag signifying that this is part of a test run - suppresses logs")
             .action(clap::ArgAction::SetTrue)
        )
    .try_get_matches_from(args)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_cli_all_type_d_params() {
        let target = "dummy target";
        let args: Vec<&str> = vec![target, "-r", "-s", "2020-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Utc::now().date_naive(); 

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::Recent);
        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.encoding_type, EncodingType::None);
        assert_eq!(res.start_date, Some(NaiveDate::from_ymd_opt(2020, 12, 4).unwrap()));
        assert_eq!(res.end_date, Some(today));

    }  

    #[test]
   fn check_cli_all_type_b_params() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-b", "-s", "2020-12-04", "-e", "2021-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::BetweenDates);
        assert_eq!(res.start_date, Some(NaiveDate::from_ymd_opt(2020, 12, 4).unwrap()));
        assert_eq!(res.end_date, Some(NaiveDate::from_ymd_opt(2021, 2, 6).unwrap()));
    }


    #[test]
    fn check_cli_all_type_y_params() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-y", "-s", "2020",];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::ByYear);
        assert_eq!(res.start_date, Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()));
        assert_eq!(res.end_date, Some(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()));
    }

    #[test]
    fn check_cli_with_just_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "2020-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Utc::now().date_naive();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::Recent);
        assert_eq!(res.start_date, Some(NaiveDate::from_ymd_opt(2020, 12, 4).unwrap()));
        assert_eq!(res.end_date, Some(today));
    }


    #[test]
    fn check_cli_with_too_early_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-b", "-s", "2002-12-04", "-e", "2021-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        let isrctn_start_date = NaiveDate::from_ymd_opt(2005, 11, 1).unwrap();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::BetweenDates);
        assert_eq!(res.start_date, Some(isrctn_start_date));
        assert_eq!(res.end_date, Some(NaiveDate::from_ymd_opt(2021, 2, 6).unwrap()));
    }


    #[test]
    fn check_cli_with_too_late_end_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-b", "-s", "2020-12-04", "-e", "2030-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Utc::now().date_naive();
        
        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.download_type, DownloadType::BetweenDates);
        assert_eq!(res.start_date, Some(NaiveDate::from_ymd_opt(2020, 12, 4).unwrap()));
        assert_eq!(res.end_date, Some(today));
    }


    #[test]
    #[should_panic]
    fn check_panics_with_no_valid_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-d"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let _res = fetch_valid_arguments(test_args).unwrap();

    }

    #[test]
    #[should_panic]
    fn check_panics_with_future_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-d", "-s", "2032-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let _res = fetch_valid_arguments(test_args).unwrap();
    }

    #[test]
    #[should_panic]
    fn check_panics_with_no_year_if_type_y() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-y", "-s", "2032-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let _res = fetch_valid_arguments(test_args).unwrap();
    }

    #[test]
    fn check_correct_pars_for_recent_import() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        
        assert_eq!(res.import_type, ImportType::Recent);
        assert_eq!(res.download_type, DownloadType::None);
        assert_eq!(res.start_date, None);
        assert_eq!(res.end_date, None);
    }

    #[test]
    fn check_correct_pars_for_all_import() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-I"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        
        assert_eq!(res.import_type, ImportType::All);
        assert_eq!(res.download_type, DownloadType::None);
        assert_eq!(res.start_date, None);
        assert_eq!(res.end_date, None);
    }

    #[test]
    fn check_correct_pars_for_both_import_pars() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i", "-I"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        
        assert_eq!(res.import_type, ImportType::Recent);
        assert_eq!(res.download_type, DownloadType::None);
        assert_eq!(res.start_date, None);
        assert_eq!(res.end_date, None);
    }

}
   


