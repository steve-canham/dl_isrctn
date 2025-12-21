use regex::Regex;

pub trait StringExtensions {
    fn tidy(&self) -> Option<String>;
    fn replace_unicodes(&self) -> Option<String>;
    fn replace_tags_and_unicodes(&self) -> Option<String>;
}

pub trait DateExtensions {
    fn as_iso_date(&self) -> Option<String>;
    fn get_time_units(&self) -> String; 
}


impl StringExtensions for String {
    
    fn tidy(&self) -> Option<String> {

        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            None
        }
        else {
            let trimmed: &str;
            if self.starts_with("\"") {
                let complex_trim = |c| c == ' ' || c == ';' || c == '"';
                trimmed = self.trim_matches(complex_trim);
            }
            else {
                let complex_trim = |c| c == ' ' || c == ';';
                trimmed = self.trim_matches(complex_trim);
            }
            if trimmed == "" {
                None
            }
            else {
                Some(trimmed.to_owned())
            }
        }
    }


    fn replace_unicodes(&self) -> Option<String> {
        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            None
        }
        else {
            let trimmed: &str;
            if self.starts_with("\"") {
                let complex_trim = |c| c == ' ' || c == ';' || c == '"';
                trimmed = self.trim_matches(complex_trim);
            }
            else {
                let complex_trim = |c| c == ' ' || c == ';';
                trimmed = self.trim_matches(complex_trim);
            }
            if trimmed == "" {
                None
            }
            else {
                let mut output = trimmed.to_owned();
                output = output.replace("&#32;", " ").replace("&#37;", "%");
                output = output.replace("#gt;", ">").replace("#lt;", "<");       
                output = output.replace("&#39;", "’").replace("&rsquo;", "’");
                output = output.replace("&quot;", "'");
                output = output.replace("&gt;", ">").replace("&lt;", "<");
                output = output.replace("&amp;", "&");
                Some(output)
            }
        }
    }


    fn replace_tags_and_unicodes(&self) -> Option<String> {
        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            None
        }
        else {
            let trimmed: &str;
            if self.starts_with("\"") {
                let complex_trim = |c| c == ' ' || c == ';' || c == '"';
                trimmed = self.trim_matches(complex_trim);
            }
            else {
                let complex_trim = |c| c == ' ' || c == ';';
                trimmed = self.trim_matches(complex_trim);
            }
            if trimmed == "" {
                None
            }
            else {
                let mut output = trimmed.to_owned();
                
                output = output.replace("<p>", "\n");
                output = output.replace("<br>", "\n");
                output = output.replace("<br/>", "\n");
                output = output.replace("<br />", "\n");
                output = output.replace("\n\n", "\n").replace("\n \n", "\n");
                output = output.replace(",,", ",");
                output = output.replace("</p>", "");

                output = output.replace("&#32;", " ").replace("&#37;", "%");
                output = output.replace("#gt;", ">").replace("#lt;", "<");       
                output = output.replace("&#39;", "’").replace("&rsquo;", "’");
                output = output.replace("&quot;", "'");
                output = output.replace("&gt;", ">").replace("&lt;", "<");
                output = output.replace("&amp;", "&");
                Some(output)
            }
        }
    }
}

impl DateExtensions for String {
    
    fn as_iso_date(&self) -> Option<String> {

        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            return None
        }
        
        if self == "1900-01-01" || self == "01/01/1900" || self == "Jan  1 1900" || self == "Jan  1 1900 12:00AM"
        {
            return None
        }
            
        let mut date_string: String;
        let mut iso_date: String;

        date_string = self.trim_matches('"').to_string();
        date_string = date_string.replace("/", "-").replace(".", "-").replace(",", "");   // regularise delimiters

        if date_string.trim() == ""  // check again after changes
        {
            return None
        }
        iso_date =  date_string.clone();   // as the initial default

        let p1 = r#"^(19|20)\d{2}-(0?[1-9]|1[0-2])-(0?[1-9]|1\d|2\d|3[0-1])$"#;
        let re1 = Regex::new(p1).unwrap();
        let p2 = r#"^(0?[1-9]|1\d|2\d|3[0-1])-(0?[1-9]|1[0-2])-(19|20)\d{2}$"#;
        let re2 = Regex::new(p2).unwrap();
        let p3 = r#"^(0?[1-9]|1\d|2\d|3[0-1]) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) (19|20)\d{2}$"#;
        let re3 = Regex::new(p3).unwrap();
        let p4 = r#"^(0?[1-9]|1\d|2\d|3[0-1]) (January|February|March|April|May|June|July|August|September|October|November|December) (19|20)\d{2}$"#;
        let re4 = Regex::new(p4).unwrap();
        let p5 = r#"^(January|February|March|April|May|June|July|August|September|October|November|December) (0?[1-9]|1\d|2\d|3[0-1]) (19|20)\d{2}$"#;
        let re5 = Regex::new(p5).unwrap();
        let p6 = r#"^(January|February|March|April|May|June|July|August|September|October|November|December) (19|20)\d{2}$"#;
        let re6 = Regex::new(p6).unwrap();
        let py = r#"(19|20)\d{2}$"#;
        let re_yr = Regex::new(py).unwrap();
        
        if re1.is_match(&date_string) {   // yyyy_mm_dd
            if date_string.len() == 10
            {
                iso_date = date_string;  // already OK
            }
            else {
                let dash1 = &date_string.find('-').unwrap();
                let dash2 = &date_string.rfind('-').unwrap();
                let year_s = &date_string[..4];
                let month_s = format!("{:0>2}", &date_string[(dash1 + 1)..(dash2 - dash1 - 1)]);
                let day_s =  format!("{:0>2}", &date_string[(dash2 + 1)..]);

                iso_date = format!("{}-{}-{}", year_s, month_s, day_s);
            }
        }
        else if re2.is_match(&date_string) {   // dd_mm_yyyy
            let dash1 = &date_string.find('-').unwrap();
            let dash2 = &date_string.rfind('-').unwrap();
            if re_yr.is_match(&date_string) {
                let caps = re_yr.captures(&date_string).unwrap();
                let year_s = &caps[0];
                let month_s = format!("{:0>2}", &date_string[(dash1 + 1)..*dash2]);
                let day_s =  format!("{:0>2}", &date_string[..*dash1]);
                iso_date = format!("{}-{}-{}", year_s, month_s, day_s);
            }
        }
        else if re3.is_match(&date_string) {        // dd_MMM_yyyy 
            let dash1 = &date_string.find(' ').unwrap();
            let dash2 = &date_string.rfind(' ').unwrap();
            if re_yr.is_match(&date_string) {
                let caps = re_yr.captures(&date_string).unwrap();
                let year_s = &caps[0];
                let month = &date_string[(dash1 + 1)..*dash2];
                let month_s = match month{
                    "Jan" => "01",
                    "Feb" => "02",
                    "Mar" => "03",
                    "Apr" => "04",
                    "May" => "05",
                    "Jun" => "06",
                    "Jul" => "07",
                    "Aug" => "08",
                    "Sep" => "09",
                    "Oct" => "10",
                    "Nov" => "11",
                    "Dec" => "12",
                        _  => "00",
                };
                let day_s =  format!("{:0>2}", &date_string[..*dash1]);
                iso_date = format!("{}-{}-{}", year_s, month_s, day_s);
            }
        }
        else if re4.is_match(&date_string) {   // dd_MMMM_yyyy
            let dash1 = &date_string.find(' ').unwrap();
            let dash2 = &date_string.rfind(' ').unwrap();
            if re_yr.is_match(&date_string) {
                let caps = re_yr.captures(&date_string).unwrap();
                let year_s = &caps[0];
                let month = &date_string[(dash1 + 1)..*dash2];
                let month_s = match month{
                    "January" => "01",
                    "February" => "02",
                    "March" => "03",
                    "April" => "04",
                    "May" => "05",
                    "June" => "06",
                    "July" => "07",
                    "August" => "08",
                    "September" => "09",
                    "October" => "10",
                    "November" => "11",
                    "December" => "12",
                    _ => "00",
                };
                let day_s =  format!("{:0>2}", &date_string[..*dash1]);
                iso_date = format!("{}-{}-{}", year_s, month_s, day_s);
            }
        }
        else if re5.is_match(&date_string) {   // MMMM_dd_yyyy
            let dash1 = &date_string.find(' ').unwrap();
            let dash2 = &date_string.rfind(' ').unwrap();
            if re_yr.is_match(&date_string) {
                let caps = re_yr.captures(&date_string).unwrap();
                let year_s = &caps[0];
                let month = &date_string[..*dash1];
                let month_s = match month{
                    "January" => "01",
                    "February" => "02",
                    "March" => "03",
                    "April" => "04",
                    "May" => "05",
                    "June" => "06",
                    "July" => "07",
                    "August" => "08",
                    "September" => "09",
                    "October" => "10",
                    "November" => "11",
                    "December" => "12",
                    _ => "00",
                };
                let day_s =  format!("{:0>2}", &date_string[(dash1 + 1)..*dash2]);
                iso_date = format!("{}-{}-{}", year_s, month_s, day_s);
            }
        }
        else if re6.is_match(&date_string) {   // MMMM_yyyy
            let dash1 = &date_string.find(' ').unwrap();
            if re_yr.is_match(&date_string) {
                let caps = re_yr.captures(&date_string).unwrap();
                let year_s = &caps[0];
                let month = &date_string[..*dash1];
                let month_s = match month{
                    "January" => "01",
                    "February" => "02",
                    "March" => "03",
                    "April" => "04",
                    "May" => "05",
                    "June" => "06",
                    "July" => "07",
                    "August" => "08",
                    "September" => "09",
                    "October" => "10",
                    "November" => "11",
                    "December" => "12",
                    _ => "00",
                };
                let day_s =  "15".to_string();          // for now **** Need to handle partial dates!
                iso_date = format!("{}-{}-{}", year_s, month_s, day_s);
            }
        }
        
        // Final check in case none of the options above have worked
        
        if re1.is_match(&iso_date) && iso_date.len() == 10 { 
             Some(iso_date)
        }
        else {
            None
        }
    }


    fn get_time_units(&self) -> String {
        if self.trim() == ""
        {
            return "".to_string();
        }

        let time_string = self.to_lowercase();
        let units = match time_string  
        { 
            _ if time_string.contains("year") => "Years",
            _ if time_string.contains("month") => "Months",
            _ if time_string.contains("week") => "Weeks",
            _ if time_string.contains("day") => "Days",
            _ if time_string.contains("hour") => "Hours",
            _ if time_string.contains("min") => "Minutes",
            _ => &("Other (".to_string() + &time_string + ")"),
        };
        return units.to_string()
    }

}



    /* 
    fn replace_nbr_spaces(&self) -> Option<String> {
        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            None
        }
        else {
            Some(self.to_owned())
        }

    }

    fn compress_spaces(&self) -> Option<String> {
        if self == "NULL" ||  self == "null" ||  self == "\"NULL\"" ||  self == "\"null\""
        ||  self.trim() == ""
        {
            None
        }
        else {
            Some(self.to_owned())
        }
    }


    public static string? CompressSpaces(this string? input_string)
    {
        if (string.IsNullOrEmpty(input_string))
        {
            return null;
        }

        string output_string = input_string.Trim();

        output_string = output_string.Replace("\r\n", "\n");    // regularise endings
        output_string = output_string.Replace("\r", "\n");

        while (output_string.Contains("  "))
        {
            output_string = output_string.Replace("  ", " ");
        }
        output_string = output_string.Replace("\n:\n", ":\n");
        output_string = output_string.Replace("\n ", "\n");
        while (output_string.Contains("\n\n"))
        {
            output_string = output_string.Replace("\n\n", "\n");
        }
        output_string = output_string.TrimEnd('\n');
        return output_string;

    }


    public static string? ReplaceNBSpaces(this string? input_string)
    {
        if (string.IsNullOrEmpty(input_string))
        {
            return null;
        }

        string output_string = input_string.Replace('\u00A0', ' ');
        output_string = output_string.Replace('\u2000', ' ').Replace('\u2001', ' ');
        output_string = output_string.Replace('\u2002', ' ').Replace('\u2003', ' ');
        output_string = output_string.Replace('\u2007', ' ').Replace('\u2008', ' ');
        output_string = output_string.Replace('\u2009', ' ').Replace('\u200A', ' ');

        return output_string;
    }
    
       

    public static string? lang_3_to_2(this string input_lang_code)
    {
        if (string.IsNullOrEmpty(input_lang_code))
        {
            return null;
        }
        
        return input_lang_code switch
        {
            "fre" => "fr",
            "ger" => "de",
            "spa" => "es",
            "ita" => "it",
            "por" => "pt",
            "rus" => "ru",
            "tur" => "tr",
            "hun" => "hu",
            "pol" => "pl",
            "swe" => "sv",
            "nor" => "no",
            "dan" => "da",
            "fin" => "fi",
            _ => "??"
        };
    }
}



public static class DateExtensions
{
   
    public static string? GetTimeUnits(this string? input_string)
    {
        if (string.IsNullOrEmpty(input_string))
        {
            return null;
        }

        string time_string = input_string.ToLower();
        return time_string switch 
        { 
            _ when time_string.Contains("year") => "Years",
            _ when time_string.Contains("month") => "Months",
            _ when time_string.Contains("week") => "Weeks",
            _ when time_string.Contains("day") => "Days",
            _ when time_string.Contains("hour") => "Hours",
            _ when time_string.Contains("min") => "Minutes",
            _ => "Other (" + time_string + ")"
        };
    }
    

    public static DateTime? FetchDateTimeFromISO(this string iso_string)
    {
        // iso_string assumed to be in format yyyy-mm-dd.
        if (string.IsNullOrEmpty(iso_string))
        {
            return null;
        }

        if (iso_string.Length > 10)
        {
            iso_string = iso_string[..10];  // if date-time only interested in the date
        }

        int year = int.Parse(iso_string[0..4]);
        int month = int.Parse(iso_string[5..7]);
        int day = int.Parse(iso_string[^2..]);
        return new DateTime(year, month, day);
    }


    public static SplitDate? GetDateParts(this string dateString)
    {
        if (string.IsNullOrEmpty(dateString))
        {
            return null;
        }

        // input date string is in the ISO format of ""
        // or in some cases in the form "<month name> year"
        // split the string on the comma.

        string year_string, month_name, day_string;
        int? year_num, day_num;
        string? month_as3 = null;

        int comma_pos = dateString.IndexOf(',');
        if (comma_pos > 0)
        {
            year_string = dateString[(comma_pos + 1)..].Trim();
            string first_part = dateString[..(comma_pos)].Trim();

            // first part should split on the space
            int space_pos = first_part.IndexOf(' ');
            day_string = first_part[(space_pos + 1)..].Trim();
            month_name = first_part[..(space_pos)].Trim();
        }
        else
        {
            int space_pos = dateString.IndexOf(' ');
            year_string = dateString[(space_pos + 1)..].Trim();
            month_name = dateString[..(space_pos)].Trim();
            day_string = "";
        }

        // convert strings into integers
        if (int.TryParse(year_string, out int y)) year_num = y; else year_num = null;
        int month_num = month_name.GetMonthAsInt();
        if (month_num > 0)
        {
            month_as3 = ((Months3)month_num).ToString();
        }

        if (int.TryParse(day_string, out int d)) day_num = d; else day_num = null;


        // get date as string
        string? date_as_string;
        if (year_num is not null && month_as3 is not null && day_num is not null)
        {
            date_as_string = year_num + " " + month_as3 + " " + day_num;
        }
        else if (year_num is not null && month_as3 is not null && day_num is null)
        {
            date_as_string = year_num + " " + month_as3;
        }
        else if (year_num is not null && month_as3 is null && day_num is null)
        {
            date_as_string = year_num.ToString();
        }
        else
        {
            date_as_string = null;
        }

        return new SplitDate(year_num, month_num, day_num, date_as_string);
    }



    public static DateTime? FetchDateTimeFromDateString(this string dateString)
    {
        if (string.IsNullOrEmpty(dateString))
        {
            return null;
        }
        
        SplitDate? sd = dateString.GetDateParts();
        if (sd is null || sd.year is null || sd.month is null || sd.day is null)
        {
            return null;
        }
        return new DateTime((int)sd.year, (int)sd.month, (int)sd.day);
    }

}


public class SplitDate
{
    public int? year;
    public int? month;
    public int? day;
    public string? date_string;

    public SplitDate(int? _year, int? _month, int? _day, string? _date_string)
    {
        year = _year;
        month = _month;
        day = _day;
        date_string = _date_string;
    }
}

*/