use clap::{command, Arg, ArgMatches};
use crate::err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliPars {
    pub importing: bool,
    pub transforming: bool,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args.to_vec())?;

    let mut i_flag = parse_result.get_flag("i_flag");
    let t_flag = parse_result.get_flag("t_flag");

    // If no flagd do the import as the default.

    if i_flag == false && t_flag == false  {
        i_flag = true;
    }

    Ok(CliPars {
        importing: i_flag,
        transforming: t_flag,
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
        .about("Imports data from ROR json file (v2) and imports it into a database")
        .arg(
            Arg::new("i")
           .short('i')
           .long("import")
           .required(false)
           .help("A flag signifying dasta to be imported from CSV files")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("t_flag")
           .short('t')
           .long("transform")
           .required(false)
           .help("A flag signifying sd data to be transformed to mdr schema and put into ad tables")
           .action(clap::ArgAction::SetTrue)
        )
    .try_get_matches_from(args)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cli_no_explicit_params() {
        let target = "dummy target";
        let args: Vec<&str> = vec![target];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, false);
        
    }  

    #[test]
    fn check_cli_with_a_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-a"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, false);
        assert_eq!(res.transforming, true);
    }

    #[test]
    fn check_cli_with_both_flags() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i", "-a"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, true);
    }
   
}

