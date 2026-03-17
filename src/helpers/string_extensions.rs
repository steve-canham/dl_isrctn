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

    fn regularise_hyphens(&self) -> Option<String>;
    fn regularise_nb_spaces(&self) -> Option<String>;

    fn replace_escaped(&self) -> Option<String>;
    fn replace_apostrophes(&self) -> Option<String>;
    fn replace_tags(&self) -> Option<String>;
    fn replace_gaps(&self) -> Option<String>;

    fn clean(&self) -> Option<String>;
    fn clean_multiline(&self) -> Option<String>;

    fn is_not_a_place_holder(&self) -> bool;
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
                let low_ref = lower.as_str();
                
                // Check for common 'null value' values

                if low_ref == "null" || low_ref == "n/a"
                || low_ref == "na" || low_ref == "none"
                || low_ref == ""
                {
                    None
                }
                else {
                    let trimmed = quoteless.trim_matches(&[' ', '-']);
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

        // Applies chiefly to filtering secondary identifiers.
        // Filtering here is to translate 'n/a', 'null' or 'nil'
        // type entries as None. the options used are ISRCTN specific -
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


    fn regularise_hyphens(&self) -> Option<String> {

        // assumed this call is immediately after a 'tidy' call, 
        // on a string option, so only basic null check required.
        // Mostly applicable to identifiers where consistency 
        // hyphens is needed for comparison purposes.


        match self.clone() {
            Some(s) => {
                let mut st = s.trim().to_string();
                if st == "".to_string() {
                    None
                }
                else {
                    st = st.replace("\u{00AD}", "");  // soft hyphen
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

        // Assumed this call is immediately after a 'tidy' call
        // on a string option, so only basic null check required.

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



    fn replace_escaped(&self) -> Option<String> {
        
        match self {
            Some(s) => {
                
                // Top portion is the same as 'as_tidied_text_opt'
                // Trim all whitespace and then any enclosing quotes

                let quoteless = s.trim().trim_matches('"');
                let lower = quoteless.to_lowercase();
                let low_ref = lower.as_str();
                
                // Check for common 'null value' values as well  as empty string

                if low_ref == "" || low_ref == "null" || low_ref == "n/a"
                || low_ref == "na" || low_ref == "none"
                {
                    None
                }
                else {
                    let t = quoteless.trim_matches(&[' ', '-']);
                    if t == "" {
                        None
                    }
                    else {

                        // Start with replacing non breaking spaces

                        let mut tr = t.replace('\u{00A0}', " ");
                        tr = tr.replace('\u{2000}', " ").replace('\u{2001}', " ");
                        tr = tr.replace('\u{2002}', " ").replace('\u{2003}', " ");
                        tr = tr.replace('\u{2007}', " ").replace('\u{2008}', " ");
                        tr = tr.replace('\u{2009}', " ").replace('\u{200A}', " ");

                        tr = tr.replace('\u{00AE}', " ").replace('\u{2122}', " ");

                        // Can sometimes be in as explicit unicode codes

                        if t.contains(r"\u")
                        {
                            tr = tr.replace(r"\u00A0", " ");
                            tr = tr.replace(r"\u2000", " ").replace(r"\u2001", " ");
                            tr = tr.replace(r"\u2002", " ").replace(r"\u2003", " ");
                            tr = tr.replace(r"\u2007", " ").replace(r"\u2008", " ");
                            tr = tr.replace(r"\u2009", " ").replace(r"\u200A", " ");

                            tr = tr.replace(r"\u00AE", " ").replace(r"\u2122", " ");
                        }

                        // Some codes can be included in their escaped form

                        if t.contains(';') {

                            // Do these two first as they can impact the replacements below.

                            tr= tr.replace("&#38;", "&").replace("&amp;", "&");

                            tr = tr.replace("&#32;", " ").replace("&#37;", "%");
                            tr = tr.replace("&#44;", ",").replace("&#45;", "-");
                            tr = tr.replace("&#39;", "'").replace("&#8217;", "'");
                            tr = tr.replace("&quot;", "'").replace("&rsquo;", "’");
                            tr = tr.replace("#gt;", ">").replace("#lt;", "<");       
                            tr = tr.replace("&gt;", ">").replace("&lt;", "<");

                            tr = tr.trim_matches(&[';', ' ']).to_string()
                            
                        }

                        tr = tr.replace("â??", "");  // remove combination sometimes used to denote an 'unprintable character'
                        tr = tr.replace(r"\\", "");  // remove 'double escape' sequence now found in some CTG text
                        tr = tr.replace('\u{0081}', "");   // remove control character that can (very rarely) appear in string

                        Some(tr)
   
                    }
                }
            },

            None => None

        }
        
    }


    fn replace_apostrophes(&self) -> Option<String> {
    
          match self {
            Some(s) => {
                
                // Top portion is the same as 'as_tidied_text_opt'
                // Trim all whitespace and then any enclosing quotes

                let quoteless = s.trim().trim_matches('"');
                let lower = quoteless.to_lowercase();
                let low_ref = lower.as_str();
                
                // Check for common 'null value' values as well  as empty string

                if low_ref == "" || low_ref == "null" || low_ref == "n/a"
                || low_ref == "na" || low_ref == "none"
                {
                    None
                }
                else {
                    let t = quoteless.trim_matches(&[' ', '-']);
                    if t == "" {
                        None
                    }
                    else {

                        // This part replicates 'replace_escaped'

                        // Start with replacing non breaking spaces

                        let mut tr = t.replace('\u{00A0}', " ");
                        tr = tr.replace('\u{2000}', " ").replace('\u{2001}', " ");
                        tr = tr.replace('\u{2002}', " ").replace('\u{2003}', " ");
                        tr = tr.replace('\u{2007}', " ").replace('\u{2008}', " ");
                        tr = tr.replace('\u{2009}', " ").replace('\u{200A}', " ");

                        tr = tr.replace('\u{00AE}', " ").replace('\u{2122}', " ");

                        // Can sometimes be in as explicit unicode codes

                        if t.contains(r"\u")
                        {
                            tr = tr.replace(r"\u00A0", " ");
                            tr = tr.replace(r"\u2000", " ").replace(r"\u2001", " ");
                            tr = tr.replace(r"\u2002", " ").replace(r"\u2003", " ");
                            tr = tr.replace(r"\u2007", " ").replace(r"\u2008", " ");
                            tr = tr.replace(r"\u2009", " ").replace(r"\u200A", " ");

                            tr = tr.replace(r"\u00AE", " ").replace(r"\u2122", " ");
                        }

                        // Some codes can be included in their escaped form

                        if t.contains(';') {

                            // Do these two first as they can impact the replacements below.

                            tr = tr.replace("&#38;", "&").replace("&amp;", "&");

                            tr = tr.replace("&#32;", " ").replace("&#37;", "%");
                            tr = tr.replace("&#44;", ",").replace("&#45;", "-");
                            tr = tr.replace("&#39;", "'").replace("&#8217;", "'");
                            tr = tr.replace("&quot;", "'").replace("&rsquo;", "’");
                            tr = tr.replace("#gt;", ">").replace("#lt;", "<");       
                            tr = tr.replace("&gt;", ">").replace("&lt;", "<");

                            tr = tr.trim_matches(&[';', ' ']).to_string()
                            
                        }

                        tr = tr.replace("â??", "");  // remove combination sometimes used to denote an 'unprintable character'
                        tr = tr.replace(r"\\", "");  // remove 'double escape' sequence now found in some CTG text
                        tr = tr.replace('\u{0081}', "");  // remove control character that can (very rarely) appear in string
                        tr = tr.replace("\u{00AD}", "");  // remove soft hyphen

                        // Now (!) do the replace apostrophes part

                        if tr.contains('\'') {

                            // Do a blanket replacement of apostrophes to RSQs.
                            // Then deal with situations where a LSQ applies

                            tr = tr.replace("'", "’");
                            
                            if tr.starts_with('’') {
                                let mut chars = tr.chars();
                                chars.next();
                                tr = format!("‘{}", chars.as_str());
                            }

                            tr = tr.replace(" ’", " ‘");
                            tr = tr.replace("(’", "(‘");
                        }

                        Some(tr)
                
                    }
                }
            },
            None => None,
        }
    }



    fn replace_tags(&self) -> Option<String> {
    
        // Assumed will normally be called in the context of 'clean' or 'clean_multiline'
        // and therefore string will already have been tidied, apostrophes sorted etc.
        // The null check is therefore rudimentary.

        match self {
            Some(sf) => {
               
               let mut s= sf.trim().to_string();

               if s == "".to_string()
               {
                    None
               }
               else {

                // String must include both opening and closing tags to be processed.

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

                            // replace any <sub>, </sub>, <sup>, </sup> tags with single chars

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
                }
            },
            None => None,
       }
    }



    fn replace_gaps(&self) -> Option<String> {
    
        // Assummed normally called after a clean process, as the final stage for trimming multiline
        // strings. Null check is therefore basic. Regularises line endings and removes double spaces
        // and double carriage returns.

        match self {
            Some(sf) => {

                let mut s = sf.trim().to_string();

                if s == "".to_string() {
                    None
                }
                else {

                    // Regularise endings

                    s = s.replace("\r\n", "\n").replace("\r", "\n");    

                    // Regularise carriage returns

                    while s.contains("\n:\n")
                    {
                        s = s.replace("\n:\n", ":\n");
                    }
                    while s.contains("\n\n")
                    {
                        s = s.replace("\n\n", "\n");
                    }
                    s = s.replace("\n ", "\n");

                    // Remove extended spaces

                    while s.contains("  ")
                    {
                        s = s.replace("  ", " ");
                    }

                    Some(s)
                }
            },
            None => None,
       }
    }
  
  
    fn clean(&self) -> Option<String> {
       
       // replace_apostrophes includes 'as_tidied_text_opt' and 
       // replace unicodes as a second step. There is therefore
       // no need to call these routines before hand. 

       // replace tags normally assumed to be used in this context
       // rather than called independently. If it is then
       // dome initial cleaning may be required.

       self.replace_apostrophes().replace_tags()
    }    

    fn clean_multiline(&self) -> Option<String> {

        // Extends the single line clean by chaining
        // the replace gaps function, which compresses
        // multiple spaces and multiple carriage returns.

        self.clean().replace_gaps()
    }


    fn is_not_a_place_holder(&self) -> bool {

        match self {
            
            Some(s) => {

                let mut result = true;

                let lower_s = s.trim().to_lowercase();
                let low_s = lower_s.as_str();

                if s.len() < 3 {
                    result = false;
                }

                else if low_s.starts_with("n") {

                    if low_s == "n.a." || low_s == "na" || low_s == "n/a"  
                    || low_s == "no" || low_s == "nil"  || low_s == "nill" || low_s == "non" {
                        result = false;
                    }
                    else if low_s.starts_with("not ") || low_s.starts_with("non ")
                    || low_s.starts_with("no ") 
                    {
                        result = false;
                    }
                    else if low_s == "none" || low_s == "nd" 
                    || low_s == "nothing" || low_s == "n.a" || low_s == "n/a."
                    {
                        result = false;
                    }
                    else if low_s.starts_with("not-") || low_s.starts_with("not_")
                    || low_s.starts_with("notapplic") ||  low_s.starts_with("notavail") 
                    || low_s.starts_with("nonfun") || low_s.starts_with("noneno")
                    {
                        result = false;
                    }
                }

                else if low_s == "same as above" || low_s == "in preparation"
                || low_s == "other" || low_s == "pending" || low_s.contains(" none.")
                {
                    result = false;
                }
                else if low_s.starts_with("organisation name ") || low_s.starts_with("to be ")
                || low_s.starts_with("tobealloc") || low_s.starts_with("see ")
                {
                    result = false;
                }

                result
            },
            None => false,
        }

    }

}


pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    let mut s = match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    };

    if s.starts_with("P") && s.len() > 1 {

        // if second character is a number or a dot more likely to be a page reference

        if s.chars().nth(1).unwrap_or_default() == '.' ||  s.chars().nth(1).unwrap_or_default().is_digit(10) {
            s = format!("p{}", &s[1..]);
        }
    }
    s

}



#[cfg(test)]
mod tests {
    use super::*;

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

        let t_opt = Some("   -foo  - \n".to_string());
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
    fn check_replace_escaped() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.replace_escaped(), None);

        let t_opt = Some("&#32;foo&#44;&#32;&amp;&#32;bar".to_string());
        assert_eq!(t_opt.replace_escaped(), Some("foo, & bar".to_string()));

        let t_opt = Some("foo &gt; fie and foe #lt; fum".to_string());
        assert_eq!(t_opt.replace_escaped(), Some("foo > fie and foe < fum".to_string()));
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
        assert_eq!(t_opt.replace_tags(), Some("this is both > 32 and < 29, which is impossible, \n\n surely that will be clear to ALL".to_string()));

        let t_opt = Some("it may be < 9, <.8, or even <0.02, <p>which one should be clearer <span class=\"foo\">after</span> the experiment".to_string());
        assert_eq!(t_opt.replace_tags(), Some("it may be < 9, <.8, or even <0.02, \nwhich one should be clearer after the experiment".to_string()));

        let t_opt = Some("this is <b class=\"foo\">about</b> 29kgm<sup>-3</sup>s<sup>-1</sup>, and it applies to K<sub>0</sub> and K<sub>max</sub>".to_string());
        assert_eq!(t_opt.replace_tags(), Some("this is about 29kgm\u{207B}\u{00B3}s\u{207B}\u{00B9}, and it applies to K\u{2080} and K\u{2098}\u{2090}\u{2093}".to_string()));
    } 

    #[test]
    fn check_replace_gaps() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.replace_gaps(), None);

        let t_opt = Some("long    gaps and    other \n\n\n\n:\n  stuff   \r\n".to_string());
        assert_eq!(t_opt.replace_gaps(), Some("long gaps and other :\n stuff".to_string()));

        let t_opt = Some("one funny line \r\n second    line \r\n  third    bit \n\n:\n:  \r".to_string());
        assert_eq!(t_opt.replace_gaps(), Some("one funny line \nsecond line \n third bit :\n:".to_string()));
    } 
  
}

