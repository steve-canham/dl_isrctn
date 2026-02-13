use chrono::{NaiveDate, Local};
use clap::{command, Arg, ArgMatches};
use crate::base_types::{DownloadType, ImportType};
use crate::err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliPars {
    pub import_type: ImportType,
    pub dl_type: DownloadType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args.to_vec())?;

    let i_flag = parse_result.get_flag("i_flag");
    let mut a_flag = parse_result.get_flag("a_flag");
    let mut d_flag = parse_result.get_flag("d_flag");
    let b_flag = parse_result.get_flag("b_flag");
    let w_flag = parse_result.get_flag("w_flag");

    let today = Local::now().date_naive();
    let isrctn_start_date = NaiveDate::from_ymd_opt(2005, 11, 1).unwrap();

    // if no flags set a d_flag is assumed
    // this still requires a start date but that requirement
    // could be dropped if the program can use the day of the last 
    // download date as the start date    

    if  !i_flag && !a_flag  && !d_flag && !b_flag && !w_flag 
    {
        d_flag = true;
    }

    // import and download functions mutually exclusive   

    if i_flag || a_flag {

        if i_flag && a_flag {
            a_flag = false;   // if both true only recent import done
        }

        let import_type = if a_flag{ImportType::All} else {ImportType::Recent};

        Ok(CliPars {
            import_type: import_type,
            dl_type: DownloadType::None,     
            start_date: today,
            end_date: today,
        }) 
    }
    else if d_flag || b_flag {

        let download_type = if d_flag{DownloadType::Recent} else {DownloadType::BetweenDates};

        let start_date_as_string = parse_result.get_one::<String>("start_date").unwrap();
        let mut start_date = match NaiveDate::parse_from_str(start_date_as_string, "%Y-%m-%d") {
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

        // Set end date to today for DownloadType::Recent, 
        // should normally be changed below if DownloadType::BetweenDates

        let mut end_date = today;

        if download_type == DownloadType::BetweenDates {

            let end_date_as_string = parse_result.get_one::<String>("end_date").unwrap();
            end_date = match NaiveDate::parse_from_str(end_date_as_string, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {return Result::Err(AppError::MissingProgramParameter("valid end date".to_string()))},
            };

            if end_date > today {
                end_date = today;
            }
        }

        Ok(CliPars {
            import_type: ImportType::None,
            dl_type: download_type,
            start_date: start_date,
            end_date: end_date,
        }) 
        
    }
    else  {  // w flag must be set - download whole year

        let year_as_string = parse_result.get_one::<String>("download_year").unwrap();  // has a default value
        let year: i32 = year_as_string.parse().unwrap_or_else(|_| 0);

        if year == 0 {
            return Result::Err(AppError::MissingProgramParameter("year for type w download".to_string()));
        }
        else {
            let mut start_date =  NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
            let mut end_date =  NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();

            if start_date < isrctn_start_date {
                start_date = isrctn_start_date;
            }

            if start_date >= today {   // invalid
                return Result::Err(AppError::MissingProgramParameter("valid start date".to_string()));
            }

            if end_date > today {
                end_date = today;
            }

            Ok(CliPars {
            import_type: ImportType::None,
            dl_type: DownloadType::ByYear,     
            start_date: start_date,
            end_date: end_date,
            }) 
        }
    }
      
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
            Arg::new("d_flag")
           .short('d')
           .long("download")
           .required(false)
           .help("")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("b_flag")
           .short('b')
           .long("between")
           .required(false)
           .help("")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("w_flag")
           .short('w')
           .long("whole_year")
           .required(false)
           .help("")
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
            Arg::new("download_year")
           .short('y')
           .long("year")
           .required(false)
           .help("Only data last edited in this year should be downloaded")
           .default_value("")
        )
        .arg(
            Arg::new("i_flag")
           .short('i')
           .long("import")
           .required(false)
           .help("A flag signifying import files downloade since the last import")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("a_flag")
           .short('a')
           .long("import_all")
           .required(false)
           .help("A flag signifying (re-)import all data from source json files")
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
        let args: Vec<&str> = vec![target, "-d", "-s", "2020-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.dl_type, DownloadType::Recent);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, today);
    }  

    #[test]
    fn check_cli_all_type_b_params() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-b", "-s", "2020-12-04", "-e", "2021-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.dl_type, DownloadType::BetweenDates);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, NaiveDate::from_ymd_opt(2021, 2, 6).unwrap());
    }

        #[test]
    fn check_cli_all_type_w_params() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-w", "-y", "2020",];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.dl_type, DownloadType::ByYear);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        assert_eq!(res.end_date, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
    }

    #[test]
    fn check_cli_with_just_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "2020-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.dl_type, DownloadType::Recent);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, today);
    }


    #[test]
    fn check_cli_with_too_early_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-b", "-s", "2002-12-04", "-e", "2021-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let isrctn_start_date = NaiveDate::from_ymd_opt(2005, 11, 1).unwrap();

        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.dl_type, DownloadType::BetweenDates);
        assert_eq!(res.start_date, isrctn_start_date);
        assert_eq!(res.end_date, NaiveDate::from_ymd_opt(2021, 2, 6).unwrap());
    }


    #[test]
    fn check_cli_with_too_late_end_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-b", "-s", "2020-12-04", "-e", "2030-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();
        
        assert_eq!(res.import_type, ImportType::None);
        assert_eq!(res.dl_type, DownloadType::BetweenDates);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, today);
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
    fn check_panics_with_no_year_if_type_w() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-w", "-s", "2032-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let _res = fetch_valid_arguments(test_args).unwrap();
    }

    #[test]
    fn check_correct_pars_for_recent_import() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();
        
        assert_eq!(res.import_type, ImportType::Recent);
        assert_eq!(res.dl_type, DownloadType::None);
        assert_eq!(res.start_date, today);
        assert_eq!(res.end_date, today);
    }

    #[test]
    fn check_correct_pars_for_all_import() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-a"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();
        
        assert_eq!(res.import_type, ImportType::All);
        assert_eq!(res.dl_type, DownloadType::None);
        assert_eq!(res.start_date, today);
        assert_eq!(res.end_date, today);
    }

    #[test]
    fn check_correct_pars_for_both_import_pars() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i", "-a"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();
        
        assert_eq!(res.import_type, ImportType::Recent);
        assert_eq!(res.dl_type, DownloadType::None);
        assert_eq!(res.start_date, today);
        assert_eq!(res.end_date, today);
    }

}
   


