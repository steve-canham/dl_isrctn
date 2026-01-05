use std::sync::LazyLock;
use regex::Regex;


#[allow(dead_code)]
pub trait OptionStringExtensions {
    fn as_text_opt(&self) -> Option<String>;
    fn as_tidied_text_opt(&self) -> Option<String>;
    fn as_filtered_ident_opt(&self) -> Option<String>;

    fn as_date_opt(&self) -> Option<String>;
    fn as_datetime_opt(&self) -> Option<String>;
    fn as_i32_opt(&self) -> Option<i32>;
    fn as_f32_opt(&self) -> Option<f32>;
    fn as_bool_opt(&self) -> Option<bool>;

    fn clean(&self) -> Option<String>;
    fn multiline_clean(&self) -> Option<String>;

    fn replace_unicodes(&self) -> Option<String>;
    // fn replace_tags_and_unicodes(&self) -> Option<String>;
    fn compress_spaces(&self) -> Option<String>;
    fn replace_apostrophes(&self) -> Option<String>;
    fn replace_tags(&self) -> Option<String>;

    fn regularise_hyphens(&self) -> Option<String>;
    fn regularise_nb_spaces(&self) -> Option<String>;
}

// Extensions for Option<String>, some specific to 
// the ISRCTN data derived from deserialisation of its XML.

// The XML deserialises to Option<String> because most elements
// and attributes are optional, and may be empty or completely missing.
// The generated json also has to support Options, both to make missing 
// data clearer, and for it to be more easily transferred to a database.
// It is useful, however, to introduce different types as appropriate, 
// (e.g. Option<bool>, Option<f32>), and also to put dates into 
// appropriate levels of accuracy, by truncating the over precise 
// ISO strings. In the json dates are still strings, but
// in a form more easily convertable to the correct DB type.

impl OptionStringExtensions for Option<String> {

    fn as_text_opt(&self) -> Option<String> {
         match self {
            Some(s) => { 
                    let st = s.trim();  // trims all whitespace
                    if st == "" 
                    {
                        None
                    } else {
                        Some(st.to_string())
                    }
                },
            None => None
        }
    }

    fn as_tidied_text_opt(&self) -> Option<String> {

        match self {
            Some(s) => {
                
                // Trim all whitespace and then any enclosing quotes

                let quoteless = s.trim().trim_matches('"');
                let lower = quoteless.to_lowercase();
                
                // Check for common 'null value' values

                if lower == "null" || lower == "n/a"
                || lower == "na" || lower == "none"
                || lower == ""
                {
                    None
                }
                else {
                    let complex_trim = |c| c == ' ' || c == ';';
                    let trimmed = quoteless.trim_matches(complex_trim);
                    if trimmed == "" {
                        None
                    }
                    else {
                        Some(trimmed.to_string())
                    }
                }
            },
        None => None
        }

    }

    fn as_filtered_ident_opt(&self) -> Option<String> {

        // Filtering here is to translate 'n/a', 'null' or 'nil'
        // type entries with None. the options used are ISRCTN specific -
        // other choices might be necessary in other systems.
        
        match self {
            Some(s) => { 
                let st = s.trim();
                if st == "" || st.len() < 2  // 1 character ids not meaningful or useful
                {
                    None
                } 
                else {
                    let stl = st.to_ascii_lowercase();
                    if stl == "n/a" || stl == "na" || stl == "no" || stl == "none"
                    || stl.starts_with("nil ") || stl.starts_with("not ") {
                        None
                    }
                    else {
                        static RE_ONE_AND_ZEROS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[01\. -]+$").unwrap());
                        if RE_ONE_AND_ZEROS.is_match(st) {  // ids just with 1s and 0s rarely meaningful or useful
                            None
                        }
                        else {
                            Some(st.to_string())
                        }
                    }
                }
            },
            None => None
        }
    }

    fn as_date_opt(&self) -> Option<String> {

    // dates are kept as strings but truncated to the 
    // short ISO YYYY-MM-DD format. It is assumed that the
    // fields using this extension are written as short ISO dates.
    // The regex checks that this is the case.
    // N.B. Only checks foremat is correvt - may be invalid as a date

        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        static ISO_DATE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap());
                        if ISO_DATE.is_match(st) {
                            Some(st[0..10].to_string())
                        }
                        else {
                            None
                        }
                    }
                },
            None => None
        }
    }

    fn as_datetime_opt(&self) -> Option<String> {

    // datetimes are kept as strings but truncated to the 
    // ISO YYY-MM-DDThh:mm::ss format. It is assumed that the
    // fields using this extension are written as long ISO dates.
    // The regex checks that this is the case.
    // N.B. Only checks foremat is correvt - may be invalid as a datetime

        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        static ISO_DATETIME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap());
                        if ISO_DATETIME.is_match(st) {
                            Some(st[0..19].to_string())
                        }
                        else {
                            None
                        }
                    }
                },
                None => None
        }
    }
    
    fn as_i32_opt(&self) -> Option<i32> {
        match self {
            Some(s) => { 
                let st = s.trim();
                if st == "" 
                {
                    None
                } 
                else 
                {
                    match st.parse::<i32>() 
                    {
                        Ok(n) => Some(n),
                        Err(_e) => None
                    }
                }
            },
            None => None,
        }
    }

    fn as_f32_opt(&self) -> Option<f32> {
        match self {
            Some(s) => { 
                let st = s.trim();
                if st == "" 
                {
                    None
                } 
                else 
                {
                    match st.parse::<f32>() 
                    {
                        Ok(n) => Some(n),
                        Err(_e) => None
                    }
                }
            },
            None => None,
        }
    }

    fn as_bool_opt(&self) -> Option<bool> {
        match self {
            Some(s) => { 
                let st = s.trim();
                if st == "" 
                {
                    None
                } 
                else {
                    let stl = st.to_ascii_lowercase();
                    if stl == "true" || stl == "yes" {
                        Some(true)
                    }
                    else if stl == "false" || stl == "no" {
                        Some(false)
                    }
                    else {
                        None
                    }
                }
            },
            None => None
        }
    }


    fn clean(&self) -> Option<String> {
       
        let mut s_opt = self.as_tidied_text_opt();
        s_opt = s_opt.replace_unicodes();
        s_opt = s_opt.replace_tags();
        s_opt.replace_apostrophes()
    }    

    fn multiline_clean(&self) -> Option<String> {

        let mut s_opt = self.as_tidied_text_opt();
        s_opt = s_opt.replace_unicodes();
        s_opt = s_opt.replace_tags();
        s_opt = s_opt.replace_apostrophes();
        s_opt.compress_spaces()
     
    }

    fn replace_unicodes(&self) -> Option<String> {

        match self {
            Some(s) => {
                let quoteless = s.trim_matches('"');
                let lower = quoteless.to_lowercase();
                if lower == "null" || lower == "n/a"
                || lower == "na" || lower == "none"
                {
                    None
                }
                else {
                    let complex_trim = |c| c == ' ' || c == ';';
                    let trimmed = quoteless.trim_matches(complex_trim);
                    if trimmed == "" {
                        None
                    }
                    else {
                        let mut output = trimmed.to_owned();
                        output = output.replace("&#32;", " ").replace("&#37;", "%");
                        output = output.replace("&#39;", "’").replace("&rsquo;", "’");
                        output = output.replace("&#44;", ",");

                        output = output.replace("&quot;", "'");
                        output = output.replace("#gt;", ">").replace("#lt;", "<");       
                        output = output.replace("&gt;", ">").replace("&lt;", "<");
                        output = output.replace("&amp;", "&");

                        Some(output.trim().to_string())
                    }
                }
            },
        None => None,
    }

}
/* 
    fn replace_tags_and_unicodes(&self) -> Option<String> {
         match self {
            Some(s) => {
                let quoteless = s.trim_matches('"');
                let lower = quoteless.to_lowercase();
                if lower == "null" || lower == "n/a"
                || lower == "na" || lower == "none"
                {
                    None
                }
                else {
                    let complex_trim = |c| c == ' ' || c == ';';
                    let trimmed = quoteless.trim_matches(complex_trim);
                    if trimmed == "" {
                        None
                    }
                    else {
                        let mut output = trimmed.to_owned();

                        output = output.replace("&#32;", " ").replace("&#37;", "%");
                        output = output.replace("&#39;", "’").replace("&rsquo;", "’");
                        output = output.replace("&#44;", ",");

                        output = output.replace("&quot;", "'");
                        output = output.replace("#gt;", ">").replace("#lt;", "<");       
                        output = output.replace("&gt;", ">").replace("&lt;", "<");
                        output = output.replace("&amp;", "&");


                        output = output.replace("<p>", "\n");
                        output = output.replace("<br>", "\n");
                        output = output.replace("<br/>", "\n");
                        output = output.replace("<br />", "\n");
                        output = output.replace("\n\n", "\n").replace("\n \n", "\n");
                        output = output.replace(",,", ",");
                        output = output.replace("</p>", "");

                        Some(output)
                    }
                }
            },
            None => None,
        }
    }
*/
    fn regularise_hyphens(&self) -> Option<String> {

        // assumed this call is immediately after a 'tidy' call, either
        // on a string or string option, so only basic null check required
        match self.clone() {
            Some(s) => {
                let mut st = s.trim().to_string();
                if st == "".to_string() {
                    None
                }
                else {
                    st = st.replace("\u{2010}", "-"); 
                    st = st.replace("\u{2011}", "-"); 
                    st = st.replace("\u{2012}", "-"); 
                    st = st.replace("\u{2013}", "-"); 
                    st = st.replace("\u{2212}", "-"); 

                    Some(st)
                }
            },
            None => None,
        }
    }

    fn regularise_nb_spaces(&self) -> Option<String> {

        // assumed this call is immediately after a 'tidy' call, either
        // on a string or string option, so only basic null check required
        match self.clone(){
            Some(s) => {
                let mut st = s.trim().to_string();
                if st == "".to_string() {
                    None
                }
                else {
                    st = st.replace("\u{00A0}", " ");
                    st = st.replace("\u{2000}", " ").replace("\u{2001}", " ");
                    st = st.replace("\u{2002}", " ").replace("\u{2003}", " ");
                    st = st.replace("\u{2007}", " ").replace("\u{2008}", " ");
                    st = st.replace("\u{2009}", " ").replace("\u{200A}", " ");

                    Some(st)
                }

            },
            None => None,
        }
    }

    fn compress_spaces(&self) -> Option<String> {
    
       match self {
            Some(s) => {
            let trimmed = s.trim();
            if trimmed == "NULL" ||  trimmed == "null" ||  trimmed == "\"NULL\"" ||  trimmed == "\"null\""
                    ||  trimmed == ""
                {
                    None
                }
                else {
                    
                    let mut output_string = trimmed.replace("\r\n", "\n");    // regularise endings
                    output_string = output_string.replace("\r", "\n");

                    while output_string.contains("  ")
                    {
                        output_string = output_string.replace("  ", " ");
                    }
                    output_string = output_string.replace("\n:\n", ":\n");
                    output_string = output_string.replace("\n ", "\n");
                    while output_string.contains("\n\n")
                    {
                        output_string = output_string.replace("\n\n", "\n");
                    }

                    Some(output_string.trim().to_string())
                }
            },
            None => None,
       }
    }

    fn replace_apostrophes(&self) -> Option<String> {
    
         match self {
            Some(s) => {
                
                // Trim all whitespace and then any enclosing quotes
                let quoteless = s.trim().trim_matches('"');
                let lower = quoteless.to_lowercase();
                
                // Check for common 'null value' values

                if lower == "" || lower == "null" || lower == "n/a"
                || lower == "na" || lower == "none"
                {
                    None
                }
                else {

                    let mut a_name = quoteless.replace("&#44;", ","); // unusual but it can occur
                    a_name = a_name.replace("&#39;", "'"); // unusual but it can occur

                    if a_name.contains('\'') {

                        // Do a blanket replacement of apostrophes to RSQs.
                        // Then deal with situations where a LSQ applies

                        a_name = a_name.replace("'", "’");
                        
                        if a_name.starts_with('’') {
                            let mut chars = a_name.chars();
                            chars.next();
                            a_name = format!("‘{}", chars.as_str());
                        }

                        a_name = a_name.replace(" ’", " ‘");
                        a_name = a_name.replace("(’", "(‘");
                    }

                    Some(a_name)
                
                }
            },
            None => None,
        }
    }

    fn replace_tags(&self) -> Option<String> {
    
       let tidied_self = self.replace_unicodes();
       match tidied_self {
            Some(mut s) => {

                // needs to include both opening and closing tags to be processed.
               
                if !(s.contains('<') && s.contains('>')) {
                    Some(s)
                }
                else {  // Consider the commonest case and then check if that has removed tags

                    s = s.replace("<br>", "\n").replace("<br/>", "\n")
                        .replace("<br />", "\n").replace("<br/ >", "\n")
                        .replace("< br / >", "\n");

                    if !(s.contains('<') && s.contains('>')) {
                        Some(s)
                    }
                    else {    

                        // Need to go through the characters and remove the 'islands' of tags
                        // and their included text, but - - consider
                        // a) genuine < and > signs; b) sub and superscripted text, and 
                        // c) the need to make bullet tags into text based bullets 

                        s = s.replace("<li", "\n\u{2022} <li");  // to solve bullet issue
                        s = s.replace("<p", "\n<p");  // to ensure line breaks are conserved

                        // When the tags above are removed the \n and bullets will now be left

                        // replace and <sub>, </sub>, <sup>, </sup> tags with single chars

                        s = s.replace("<sub>", "\u{21E9}"); // fat arrow down
                        s = s.replace("</sub>", "\u{21D1}"); // open fat arrow up
                        s = s.replace("<sup>", "\u{21E7}");  // fat arrow up
                        s = s.replace("</sup>", "\u{21D3}"); // open fat arrow down

                        //  use regex to find and 'protect' standalone < signs

                        static RE_LT_ARROW: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<(?<n>( |[0-9\.]))").unwrap());
                        s = (RE_LT_ARROW.replace_all(&s, "\u{222E}$n")).to_string();   // line integral symbol
                                                                       
                        // Now go through characters and create new string (new_s)

                        let mut inside = false;
                        let mut in_sub = false;
                        let mut in_sup = false;
                        let mut new_s = "".to_string();

                        // loop over string chars.

                        for c in s.chars() {

                            // Detect tag starts and ends, and skip over tag edge chars.

                            match c {
                                '<' => {inside = true;  continue;},
                                '>' => {if inside {inside = false;  continue;}}
                                '\u{21E9}'  => {in_sub = true; continue;},
                                '\u{21E7}'  => {in_sup = true; continue;},
                                '\u{21D1}'  => {in_sub = false; continue;},
                                '\u{21D3}'  => {in_sup = false; continue;},
                                _ => {},
                            }

                            if in_sub {
                                let subc = match c {
                                    '0' => '\u{2080}',
                                    '1' => '\u{2081}',
                                    '2' => '\u{2082}',
                                    '3' => '\u{2083}',
                                    '4' => '\u{2084}',
                                    '5' => '\u{2085}',
                                    '6' => '\u{2086}',
                                    '7' => '\u{2087}',
                                    '8' => '\u{2088}',
                                    '9' => '\u{2089}',
                                    '+' => '\u{208A}',
                                    '-' => '\u{208B}',
                                    '=' => '\u{208C}',
                                    '(' => '\u{208D}',
                                    ')' => '\u{208E}',
                                    'a' => '\u{2090}',
                                    'e' => '\u{2091}',
                                    'o' => '\u{2092}',
                                    'x' => '\u{2093}',
                                    'h' => '\u{2095}',
                                    'k' => '\u{2096}',
                                    'l' => '\u{2097}',
                                    'm' => '\u{2098}',
                                    'n' => '\u{2099}',
                                    'p' => '\u{209A}',
                                    's' => '\u{209B}',
                                    't' => '\u{209C}',
                                    _ => c
                                };
                                new_s.push(subc);

                            }
                            else if in_sup {
                                let supc = match c {
                                    '0' => '\u{2070}',
                                    '1' => '\u{00B9}',
                                    '2' => '\u{00B2}',
                                    '3' => '\u{00B3}',
                                    '4' => '\u{2074}',
                                    '5' => '\u{2075}',
                                    '6' => '\u{2076}',
                                    '7' => '\u{2077}',
                                    '8' => '\u{2078}',
                                    '9' => '\u{2079}',
                                    'i' => '\u{2071}',
                                    '+' => '\u{207A}',
                                    '-' => '\u{207B}',
                                    '=' => '\u{207C}',
                                    '(' => '\u{207D}',
                                    ')' => '\u{207E}',
                                    'n' => '\u{207F}',
                                    _ => c
                                };
                                new_s.push(supc);
                            }
                            else if inside {
                                // do nothing
                            }
                            else {
                                // 'normal' outside
                                new_s.push(c);
                            }
                        }

                        new_s = new_s.replace("\u{222E}", "<");  // put any lt signs back
                        
                        Some(new_s)

                    }
                                
                }
            },
            None => None,
       }
    }



}




#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    /*
    fn clean(&self) -> Option<String>;
    fn multiline_clean(&self) -> Option<String>;
    fn compress_spaces(&self) -> Option<String>;
     */

    #[test]
    fn check_as_text_opt() {

        let t_opt = Some("".to_string());
        assert_eq!(t_opt.as_text_opt(), None);

        let t_opt = Some("   \n   ".to_string());
        assert_eq!(t_opt.as_text_opt(), None);

        let t_opt = Some("\t \t foo \r\n     ".to_string());
        assert_eq!(t_opt.as_text_opt(), Some("foo".to_string()));
    } 
    
    #[test]
    fn check_as_tidied_text_opt() {

        let t_opt = Some("N/A".to_string());
        assert_eq!(t_opt.as_tidied_text_opt(), None);

        let t_opt = Some("none ".to_string());
        assert_eq!(t_opt.as_tidied_text_opt(), None);

        let t_opt = Some("\"foo  \"  ".to_string());
        assert_eq!(t_opt.as_tidied_text_opt(), Some("foo".to_string()));

        let t_opt = Some("   foo  ; \n".to_string());
        assert_eq!(t_opt.as_tidied_text_opt(), Some("foo".to_string()));
    } 

    #[test]
    fn check_as_filtered_text_opt() {

        let t_opt = Some("N/A".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), None);

        let t_opt = Some("none ".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), None);

        let t_opt = Some(" nil provided".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), None);

        let t_opt = Some(" 1 ".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), None);

        let t_opt = Some("1.0 ".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), None);

        let t_opt = Some("1111-000".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), None);
       
        let t_opt = Some("foo  \n".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), Some("foo".to_string()));

        let t_opt = Some("   foo  ; \n".to_string());
        assert_eq!(t_opt.as_filtered_ident_opt(), Some("foo  ;".to_string()));
    }

    #[test]
    fn check_as_date_opt() {

        let t_opt = Some("random_string".to_string());
        assert_eq!(t_opt.as_date_opt(), None);

        let t_opt = Some("20-04-23".to_string());
        assert_eq!(t_opt.as_date_opt(), None);

        let t_opt = Some("2020-04-23".to_string());
        assert_eq!(t_opt.as_date_opt(), Some("2020-04-23".to_string()));

        let t_opt = Some("2020-04-66".to_string());
        assert_eq!(t_opt.as_date_opt(), Some("2020-04-66".to_string()));

        let t_opt = Some("2020-04-23T12:34:45".to_string());
        assert_eq!(t_opt.as_date_opt(), Some("2020-04-23".to_string()));
    } 

    #[test]
    fn check_as_datetime_opt() {

        let t_opt = Some("random_string".to_string());
        assert_eq!(t_opt.as_datetime_opt(), None);

        let t_opt = Some("20-04-23".to_string());
        assert_eq!(t_opt.as_datetime_opt(), None);

        let t_opt = Some("2020-04-23".to_string());
        assert_eq!(t_opt.as_datetime_opt(), None);

        let t_opt = Some("2020-04-23T12:34:45".to_string());
        assert_eq!(t_opt.as_datetime_opt(), Some("2020-04-23T12:34:45".to_string()));

        let t_opt = Some("2020-04-23T12:34:45.12345".to_string());
        assert_eq!(t_opt.as_datetime_opt(), Some("2020-04-23T12:34:45".to_string()));

        let t_opt = Some("2020-04-23T33:99:99.12345".to_string());
        assert_eq!(t_opt.as_datetime_opt(), Some("2020-04-23T33:99:99".to_string()));
    } 

     #[test]
    fn check_as_i32_opt() {

        let t_opt = Some("random_string".to_string());
        assert_eq!(t_opt.as_i32_opt(), None);

        let t_opt = Some("    \n".to_string());
        assert_eq!(t_opt.as_i32_opt(), None);

        let t_opt = Some("13.2".to_string());
        assert_eq!(t_opt.as_i32_opt(), None);

        let t_opt = Some("13".to_string());
        assert_eq!(t_opt.as_i32_opt(), Some(13));

        let t_opt = Some("-145.23".to_string());
        assert_eq!(t_opt.as_i32_opt(), None);

        let t_opt = Some("0".to_string());
        assert_eq!(t_opt.as_i32_opt(), Some(0));

        let t_opt = Some("-12".to_string());
        assert_eq!(t_opt.as_i32_opt(), Some(-12));
    } 

    #[test]
    fn check_as_f32_opt() {

        let t_opt = Some("random_string".to_string());
        assert_eq!(t_opt.as_f32_opt(), None);

        let t_opt = Some("    \n".to_string());
        assert_eq!(t_opt.as_f32_opt(), None);

        let t_opt = Some("13.2".to_string());
        assert_eq!(t_opt.as_f32_opt(), Some(13.2));

        let t_opt = Some("13".to_string());
        assert_eq!(t_opt.as_f32_opt(), Some(13.0));

        let t_opt = Some("-145.23".to_string());
        assert_eq!(t_opt.as_f32_opt(), Some(-145.23));

        let t_opt = Some("0".to_string());
        assert_eq!(t_opt.as_f32_opt(), Some(0.0));

        let t_opt = Some("-12".to_string());
        assert_eq!(t_opt.as_f32_opt(), Some(-12.0));
    } 

    #[test]
    fn check_as_bool_opt() {

        let t_opt = Some("random_string".to_string());
        assert_eq!(t_opt.as_bool_opt(), None);

        let t_opt = Some("    ".to_string());
        assert_eq!(t_opt.as_bool_opt(), None);

        let t_opt: Option<String> = Some("yes".to_string());
        assert_eq!(t_opt.as_bool_opt(), Some(true));

        let t_opt = Some("tRue".to_string());
        assert_eq!(t_opt.as_bool_opt(), Some(true));

        let t_opt: Option<String> = Some("NO".to_string());
        assert_eq!(t_opt.as_bool_opt(), Some(false));

        let t_opt = Some("False".to_string());
        assert_eq!(t_opt.as_bool_opt(), Some(false));
    } 

    #[test]
    fn check_regularise_hyphens() {

        let t_opt = Some("".to_string());
        assert_eq!(t_opt.regularise_hyphens(), None);

        let t_opt = Some("  \u{2010}  ".to_string());
        assert_eq!(t_opt.regularise_hyphens(), Some("-".to_string()));

        let t_opt = Some("foo\u{2012}bar".to_string());
        assert_eq!(t_opt.regularise_hyphens(), Some("foo-bar".to_string()));
    } 
    
    #[test]
    fn check_regularise_nb_spaces() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.regularise_nb_spaces(), None);

        let t_opt = Some("foo\u{00A0}\u{2000}\u{2009}   ".to_string());
        assert_eq!(t_opt.regularise_nb_spaces(), Some("foo".to_string()));

        let t_opt = Some("foo\u{2009}bar".to_string());
        assert_eq!(t_opt.regularise_nb_spaces(), Some("foo bar".to_string()));
    } 
    
    #[test]
    fn check_replace_unicodes() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.replace_unicodes(), None);

        let t_opt = Some("&#32;foo&#44;&#32;&amp;&#32;bar".to_string());
        assert_eq!(t_opt.replace_unicodes(), Some("foo, & bar".to_string()));

        let t_opt = Some("foo &gt; fie and foe #lt; fum".to_string());
        assert_eq!(t_opt.replace_unicodes(), Some("foo > fie and foe < fum".to_string()));
    } 


    #[test]
    fn check_replace_apostrophes() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.replace_apostrophes(), None);

        let t_opt = Some("Fred's bar".to_string());
        assert_eq!(t_opt.replace_apostrophes(), Some("Fred’s bar".to_string()));

        let t_opt = Some("'it's peculiar', he said, but we can't really do the 'right thing'".to_string());
        assert_eq!(t_opt.replace_apostrophes(), Some("‘it’s peculiar’, he said, but we can’t really do the ‘right thing’".to_string()));

        let t_opt = Some("They call it 'el grande' ('the big one')".to_string());
        assert_eq!(t_opt.replace_apostrophes(), Some("They call it ‘el grande’ (‘the big one’)".to_string()));
    } 

    #[test]
    fn check_replace_tags() {

        let t_opt = Some("   ".to_string());
        assert_eq!(t_opt.replace_tags(), None);

        let t_opt = Some("<p> this is a broken <br /> sentence, <i>emphatically so</i></p>".to_string());
        assert_eq!(t_opt.replace_tags(), Some("\n this is a broken \n sentence, emphatically so".to_string()));

        let t_opt = Some("<ul>a list<li>item 1</li><li>item 2</li><li>item 3 has a thing < 0.4 in it</li></ul>, to be more interesting".to_string());
        assert_eq!(t_opt.replace_tags(), Some("a list\n\u{2022} item 1\n\u{2022} item 2\n\u{2022} item 3 has a thing < 0.4 in it, to be more interesting".to_string()));

        let t_opt = Some("this is <emphatic>both</emphatic> > 32 and < 29, which is impossible, <br/><br/> surely that will be clear to <span class=\"foo\">ALL</span>".to_string());
        info!("{}", t_opt.replace_tags().unwrap());
        assert_eq!(t_opt.replace_tags(), Some("this is both > 32 and < 29, which is impossible, \n\n surely that will be clear to ALL".to_string()));

        let t_opt = Some("this is <b class=\"foo\">about</b> 29kgm<sup>-3</sup>s<sup>-1</sup>, and it applies to K<sub>0</sub> and K<sub>max</sub>".to_string());
        info!("{}", t_opt.replace_tags().unwrap());
        assert_eq!(t_opt.replace_tags(), Some("this is about 29kgm\u{207B}\u{00B3}s\u{207B}\u{00B9}, and it applies to K\u{2080} and K\u{2098}\u{2090}\u{2093}".to_string()));
    } 




}

