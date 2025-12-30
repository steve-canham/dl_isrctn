use chrono::{NaiveDate, Local};
use clap::{command, Arg, ArgMatches};
use crate::err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliPars {
    pub dl_type: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args.to_vec())?;

    let dl_type_as_string = parse_result.get_one::<String>("dl_type").unwrap();
    let dl_type: i32 = dl_type_as_string.parse().unwrap_or_else(|_| 0);   // N.B. default is 111

    let year_as_string = parse_result.get_one::<String>("download_year").unwrap();
    let year: i32 = year_as_string.parse().unwrap_or_else(|_| 0);

    if dl_type == 117 && year == 0 {
        return Result::Err(AppError::MissingProgramParameter("year for type 117 download".to_string()));
    }

    let isrctn_start_date = NaiveDate::from_ymd_opt(2005, 11, 1).unwrap();
    let today = Local::now().date_naive();

    let start_date_as_string = parse_result.get_one::<String>("start_date").unwrap();
    let start_date_opt = match NaiveDate::parse_from_str(start_date_as_string, "%Y-%m-%d") {
        Ok(date) => Some(date),
        Err(_) => None,
    };
    
    let mut start_date = match start_date_opt {  
        Some(d) => if dl_type == 117 {  // year known to be present
                                    NaiveDate::from_ymd_opt(year, 1, 1).unwrap()
                                }
                                else if d >= today {   // invalid
                                    return Result::Err(AppError::MissingProgramParameter("valid start date".to_string()));
                                }
                                else {
                                    d
                                },
        None => { 
            if dl_type != 117 {  
                return Result::Err(AppError::MissingProgramParameter("valid start date".to_string()));
            }
            else {  // year known to be present
                NaiveDate::from_ymd_opt(year, 1, 1).unwrap()
            }
        }
    };

    if start_date < isrctn_start_date {
        start_date = isrctn_start_date;
    }
  
    let end_date_as_string = parse_result.get_one::<String>("end_date").unwrap();
    let end_date_opt = match NaiveDate::parse_from_str(end_date_as_string, "%Y-%m-%d") {
        Ok(date) => Some(date),
        Err(_) => None,
    };

    let mut end_date = match end_date_opt {
        Some(d) => if dl_type == 117 {  // year known to be present
                                  NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
                              }
                              else {
                                 d
                              },
        None => if dl_type != 117 { 
                today
            }
            else {   // year known to be present
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
            },
    };

    if end_date > today {
        end_date = today;
    }

    Ok(CliPars {
        dl_type: dl_type,
        start_date: start_date,
        end_date: end_date,
    }) 
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
             Arg::new("dl_type")
            .short('t')
            .long("type")
            .help("An integer indicating the type of download required")
            .default_value("111")   // Note default value
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
    .try_get_matches_from(args)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cli_all_type_111_params() {
        let target = "dummy target";
        let args: Vec<&str> = vec![target, "-t", "111", "-s", "2020-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();

        assert_eq!(res.dl_type, 111);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, today);
    }  

    #[test]
    fn check_cli_all_type_115_params() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t", "115", "-s", "2020-12-04", "-e", "2021-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.dl_type, 115);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, NaiveDate::from_ymd_opt(2021, 2, 6).unwrap());
    }

        #[test]
    fn check_cli_all_type_117_params() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t", "117", "-y", "2020",];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.dl_type, 117);
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
        assert_eq!(res.dl_type, 111);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, today);
    }


    #[test]
    fn check_cli_with_too_early_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "2002-12-04", "-e", "2021-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let isrctn_start_date = NaiveDate::from_ymd_opt(2005, 11, 1).unwrap();

        assert_eq!(res.dl_type, 111);
        assert_eq!(res.start_date, isrctn_start_date);
        assert_eq!(res.end_date, NaiveDate::from_ymd_opt(2021, 2, 6).unwrap());
    }


    #[test]
    fn check_cli_with_too_late_end_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "2020-12-04", "-e", "2030-02-06"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        let today = Local::now().date_naive();
        assert_eq!(res.dl_type, 111);
        assert_eq!(res.start_date, NaiveDate::from_ymd_opt(2020, 12, 4).unwrap());
        assert_eq!(res.end_date, today);
    }


    #[test]
    #[should_panic]
    fn check_panics_with_no_valid_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t", "111"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let _res = fetch_valid_arguments(test_args).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_panics_with_future_start_date() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t", "111", "-s", "2032-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let _res = fetch_valid_arguments(test_args).unwrap();
    }

    #[test]
    #[should_panic]
    fn check_panics_with_no_year_if_type_117() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t", "117", "-s", "2032-12-04"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let _res = fetch_valid_arguments(test_args).unwrap();
    }

}
   


