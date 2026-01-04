use std::sync::LazyLock;
use regex::Regex;


#[allow(dead_code)]
pub trait OptionStringExtensions {
    fn as_text_opt(&self) -> Option<String>;
    fn as_filtered_text_opt(&self) -> Option<String>;
    fn as_date_opt(&self) -> Option<String>;
    fn as_datetime_opt(&self) -> Option<String>;
    fn as_float_opt(&self) -> Option<f32>;
    fn as_bool_opt(&self) -> Option<bool>;

    fn tidy(&self) -> Option<String>;
    fn replace_unicodes(&self) -> Option<String>;
    fn replace_tags_and_unicodes(&self) -> Option<String>;
    fn compress_spaces(&self) -> Option<String>;
    fn replace_apostrophes(&self) -> Option<String>;
    fn replace_tags(&self) -> Option<String>;

    fn regularise_hyphens(&self) -> Option<String>;
    fn regularise_nb_spaces(&self) -> Option<String>;
}

// Extensions for Option<String>, largely specific to 
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
                    let st = s.trim();
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

    fn tidy(&self) -> Option<String> {

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
                        Some(trimmed.to_string())
                    }
                }
            },
        None => None
        }

        /*
        
    public static string? TrimPlus(this string? input_string)
    {
        // removes beginning or trailing carriage returns, tabs and spaces
        if (string.IsNullOrEmpty(input_string))
        {
            return null;
        }
        else
        {
            return input_string.Trim('\r', '\n', '\t', ' ');
        }
    }
         */
    }

    // Filtering here is to translate 'n/a', 'null' or 'nil'
    // type entries with None. the options used are ISRCTN specific -
    // other choices might be necessary in other systems.

    fn as_filtered_text_opt(&self) -> Option<String> {
        
        static RE_ONE_AND_ZEROS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[01\. -]+$").unwrap());
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

    // dates are kept as strings but truncated to the 
    // short ISO YYYY-MM-DD format. It is assumed that the
    // fields using this extension are written as long ISO dates.
    // It may be that a Regexp check shopuld be added to ensure 
    // that this is the case.

    fn as_date_opt(&self) -> Option<String> {
        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        let st2 = st.to_string();
                        if st2.len() > 10 {
                            let date_string = &st2[0..10];
                            Some(date_string.to_string())
                        }
                        else {
                            None
                        }
                    }
                },
            None => None
        }
    }

    // datetimes are kept as strings but truncated to the 
    // ISO YYY-MM-DDThh:mm::ss format. It is assumed that the
    // fields using this extension are written as long ISO dates.
    // It may be that a Regexp check shopuld be added to ensure 
    // that this is the case.

    fn as_datetime_opt(&self) -> Option<String> {
        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        let st2 = st.to_string();
                        if st2.len() > 19 {
                            let date_string = &st2[0..19];
                            Some(date_string.to_string())
                        }
                        else {
                            None
                        }
                    }
                },
                None => None
        }
    }
    
    fn as_float_opt(&self) -> Option<f32> {
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

                        Some(output)
                    }
                }
            },
        None => None,
    }

}

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

    fn regularise_hyphens(&self) -> Option<String> {

        // assumed this call is immediately after a 'tidy' call, either
        // on a string or string option, so only basic null check required
        match self {
            Some(s) => {
                if s.trim() == "" {
                    None
                }
                else {

                    let mut output_string = s.replace("\u{2010}", "-"); 
                    output_string = output_string.replace("\u{2011}", "-"); 
                    output_string = output_string.replace("\u{2012}", "-"); 
                    output_string = output_string.replace("\u{2013}", "-"); 
                    output_string = output_string.replace("\u{2212}", "-"); 

                    Some(output_string)
                }
            },
            None => None,
        }
    }

    fn regularise_nb_spaces(&self) -> Option<String> {

        // assumed this call is immediately after a 'tidy' call, either
        // on a string or string option, so only basic null check required
        match self {
            Some(s) => {
                if s.trim() == "" {
                    None
                }
                else {
                    let mut output_string = s.replace("\u{00A0}", " ");
                    output_string = output_string.replace("\u{2000}", " ").replace("\u{2001}", " ");
                    output_string = output_string.replace("\u{2002}", " ").replace("\u{2003}", " ");
                    output_string = output_string.replace("\u{2007}", " ").replace("\u{2008}", " ");
                    output_string = output_string.replace("\u{2009}", " ").replace("\u{200A}", " ");

                    Some(output_string)
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
            Some(s) => {let quoteless = s.trim_matches('"');
                let lower = quoteless.to_lowercase();
                if lower == "null" || lower == "n/a"
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
                // needs to include opening and closing tags to be processed.
                // except in a few cases commas may be in a string as "&#44;". 
               
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

                        // Need to go through the characters and remove the 'islands' of tagged 
                        // text, but - - consider
                        // a) genuine < and > signs; b) sub and superscripted text, and 
                        // c) the need to make bullet tags into bullets - solved below
     
                        s = s.replace("<li", "\n\u{2022} <li");  // to solve c)

                        // for b) consider these seperately

                        while s.contains ("<sub>") {
                            // Try to substitute characters
                            // If not possible just remove tags

                        } 
                        
                        while s.contains ("<sup>") {
                            // Try to substitute characters
                            // If not possible just remove tags
                        } 

                        s = s.replace("<p>", "\n").replace("</p>", "")
                             .replace("<li>", "").replace("</li>", "")
                             .replace("<ul>", "").replace("</ul>", "")
                             .replace("<ol>", "").replace("</ol>", "");
                        
                        s = s.replace("<div>", "").replace("</div>", "")
                             .replace("<span>", "").replace("</span>", "");

                        s = s.replace("<b>", "").replace("</b>", "")
                             .replace("<i>", "").replace("</i>", "")
                             .replace("<u>", "").replace("</u>", "")
                             .replace("<em>", "").replace("</em>", "")
                             .replace("<strong>", "").replace("</strong>", "");

                        if !(s.contains('<') && s.contains('>')) {
                                Some(s)
                        }
                        else {
                            let mut new_s = String::new();
                            let mut temp_tag_name = String::new();
                            let mut inside = false;
                            let mut really_inside = false;
                            let mut in_sub = false;
                            let mut in_sup = false;

                            // Step 1: loop over string chars.
                            for c in s.chars() {
                                // Step 2: detect markup start and end, and skip over markup chars.
                                if c == '<' {
                                    if in_sub || in_sup {
                                        really_inside = true;
                                    }
                                    inside = true;
                                    continue;
                                }
                                if really_inside && c == '>' {
                                    inside = false;
                                    really_inside = false;
                                    
                                    continue;
                                }

                                if in_sub {
                                    // try to add small subscript version of char
                                }

                                if in_sup {
                                    // try to add small superscript version of char
                                }

                                if inside {

                                    // what comes after the <?
                                    if !really_inside {
                                        temp_tag_name.push(c);

                                        if temp_tag_name == "<p " || temp_tag_name == "<a "
                                        || temp_tag_name == "<div " || temp_tag_name == "<span " 
                                        || temp_tag_name =="<ol " || temp_tag_name == "<ul " 
                                        || temp_tag_name =="<sub>" || temp_tag_name == "<sup>" 
                                        {
                                            really_inside = true;
                                        }

                                        if temp_tag_name == "<sub>" {
                                            in_sub = true;
                                        }

                                        if temp_tag_name == "<sup>" {
                                            in_sup = true;
                                        }
                                    }

                                }

                                if !inside {
                                    // Step 3: push other  characters to the result string.
                                    new_s.push(c);
                                }

                            }
                
                            Some(new_s)

                        }
                            
                       


                                // Remaining are likely to be tags with attributes, 
                                // or tags signifying super or sub scripts
/*
                                while s.contains ("<p ") {
                                    // replace whole tag by \n
                                } 

                                while s.contains ("<li ") {
                                    // replace whole tag by \n, bullet, space
                                } 

                                while s.contains ("<ol ") {
                                    // remove whole tag 
                                } 

                                while s.contains ("<ul ") {
                                    // remove whole tag 
                                } 

                                while s.contains ("<div ") {
                                    // remove whole tag 
                                } 

                                while s.contains ("<span ") {
                                    // remove whole tag 
                                } 

                                while s.contains ("<a ") {
                                    // remove whole tag 
                                } 

                                while s.contains ("<sub>") {
                                    // Try to substitute characters
                                    // If not possible just remove tags

                                } 
                                
                                while s.contains ("<sup>") {
                                    // Try to substitute characters
                                    // If not possible just remove tags
                                } 
                                
                                Some(s)
                                */

                      

                        
                    }
                }
            },
            None => None,
       }
    }



}


/*
            
        while (output_string.Contains("<p"))
        {
            // replace any p start tags with a carriage return

            int start_pos = output_string.IndexOf("<p", StringComparison.Ordinal);
            int end_pos = output_string.IndexOf(">", start_pos, StringComparison.Ordinal);
            output_string = output_string[..start_pos] + "\n" + output_string[(end_pos + 1)..];
        }

        // Check for any list structures

        if (output_string.Contains("<li"))
        {
            while (output_string.Contains("<li"))
            {
                // replace any li start tags with a carriage return and bullet

                int start_pos = output_string.IndexOf("<li", StringComparison.Ordinal);
                int end_pos = output_string.IndexOf(">", start_pos, StringComparison.Ordinal);
                output_string = output_string[..start_pos] + "\n\u2022 " + output_string[(end_pos + 1)..];
            }

            // remove any list start and end tags

            while (output_string.Contains("<ul"))
            {
                int start_pos = output_string.IndexOf("<ul", StringComparison.Ordinal);
                int end_pos = output_string.IndexOf(">", start_pos, StringComparison.Ordinal);
                output_string = output_string[..start_pos] + output_string[(end_pos + 1)..];
            }

            while (output_string.Contains("<ol"))
            {
                int start_pos = output_string.IndexOf("<ol", StringComparison.Ordinal);
                int end_pos = output_string.IndexOf(">", start_pos, StringComparison.Ordinal);
                output_string = output_string[..start_pos] + output_string[(end_pos + 1)..];
            }

            output_string = output_string.Replace("</li>", "").Replace("</ul>", "").Replace("</ol>", "");
        }

        while (output_string.Contains("<div"))
        {
            // remove any div start tags
            int start_pos = output_string.IndexOf("<div", StringComparison.Ordinal);
            int end_pos = output_string.IndexOf(">", start_pos, StringComparison.Ordinal);
            output_string = output_string[..start_pos] + output_string[(end_pos + 1)..];
        }

        while (output_string.Contains("<span"))
        {
            // remove any span start tags
            int start_pos = output_string.IndexOf("<span", StringComparison.Ordinal);
            int end_pos = output_string.IndexOf(">", start_pos, StringComparison.Ordinal);
            output_string = output_string[..start_pos] + output_string[(end_pos + 1)..];
        }

        // check need to continue

        if (!(output_string.Contains('<') && output_string.Contains('>')))
        {
            return output_string;
        }

        while (output_string.Contains("<a"))
        {
            // remove any link start tags - appears to be very rare
            int start_pos = output_string.IndexOf("<a", StringComparison.Ordinal);
            int end_pos = output_string.IndexOf(">", start_pos, StringComparison.Ordinal);
            output_string = output_string[..start_pos] + output_string[(end_pos + 1)..];
        }

        output_string = output_string.Replace("</a>", "");

        // try and replace sub and super scripts

        while (output_string.Contains("<sub>"))
        {
            int start_pos = output_string.IndexOf("<sub>", StringComparison.Ordinal);
            int end_string = output_string.IndexOf("</sub>", start_pos, StringComparison.Ordinal);
            if (end_string != -1) // would indicate a non matched sub entry
            {
                int end_pos = end_string + 5;
                string string_to_change = output_string[(start_pos + 5)..end_string];
                string new_string = "";
                for (int i = 0; i < string_to_change.Length; i++)
                {
                    new_string += string_to_change[i].ChangeToSubUnicode();
                }

                if (end_pos > output_string.Length - 1)
                {
                    output_string = output_string[..start_pos] + new_string;
                }
                else
                {
                    output_string = output_string[..start_pos] + new_string + output_string[(end_pos + 1)..];
                }
            }
            else
            {
                // drop any that are left (to get out of the loop)
                output_string = output_string.Replace("</sub>", "");
                output_string = output_string.Replace("<sub>", "");
            }
        }

        while (output_string.Contains("<sup>"))
        {
            int start_pos = output_string.IndexOf("<sup>", StringComparison.Ordinal);
            int end_string = output_string.IndexOf("</sup>", start_pos, StringComparison.Ordinal);
            if (end_string != -1) // would indicate a non matched sup entry
            {
                int end_pos = end_string + 5;
                string string_to_change = output_string[(start_pos + 5)..end_string];
                string new_string = "";
                for (int i = 0; i < string_to_change.Length; i++)
                {
                    new_string += string_to_change[i].ChangeToSupUnicode();
                }

                if (end_pos > output_string.Length - 1)
                {
                    output_string = output_string[..start_pos] + new_string;
                }
                else
                {
                    output_string = output_string[..start_pos] + new_string + output_string[(end_pos + 1)..];
                }
            }
            else
            {
                // drop any that are left (to ensure getting out of the loop)
                output_string = output_string.Replace("</sup>", "");
                output_string = output_string.Replace("<sup>", "");
            }
        }

        return output_string;
    }


    public static string? RegulariseStringEndings(this string? input_string)
    {
        if (string.IsNullOrEmpty(input_string))
        {
            return null;
        }

        string output_string = input_string.Replace("\r\n", "|@@|");
        output_string = output_string.Replace("\r", "\n");
        return output_string.Replace("|@@|", "\r\n");
 }


    public static string? StringClean(this string? input_string)
    {
        if (string.IsNullOrEmpty(input_string))
        {
            return null;
        }

        string? output_string = input_string.TrimPlus();
        output_string = output_string.ReplaceTags();
        output_string = output_string.ReplaceApos();
        return output_string.RegulariseStringEndings();
    }


    private static char ChangeToSupUnicode(this char a)
    {
        return a switch
        {
            '0' => '\u2070',
            '1' => '\u0B09',
            '2' => '\u0B02',
            '3' => '\u0B03',
            '4' => '\u2074',
            '5' => '\u2075',
            '6' => '\u2076',
            '7' => '\u2077',
            '8' => '\u2078',
            '9' => '\u2079',
            'i' => '\u2071',
            '+' => '\u207A',
            '-' => '\u207B',
            '=' => '\u207C',
            '(' => '\u207D',
            ')' => '\u207E',
            'n' => '\u207F',
            _ => a
        };
    }

    private static char ChangeToSubUnicode(this char a)
    {
        return a switch
        {
            '0' => '\u2080',
            '1' => '\u2081',
            '2' => '\u2082',
            '3' => '\u2083',
            '4' => '\u2084',
            '5' => '\u2085',
            '6' => '\u2086',
            '7' => '\u2087',
            '8' => '\u2088',
            '9' => '\u2089',
            '+' => '\u208A',
            '-' => '\u208B',
            '=' => '\u208C',
            '(' => '\u208D',
            ')' => '\u208E',
            'a' => '\u2090',
            'e' => '\u2091',
            'o' => '\u2092',
            'x' => '\u2093',
            'h' => '\u2095',
            'k' => '\u2096',
            'l' => '\u2097',
            'm' => '\u2098',
            'n' => '\u2099',
            'p' => '\u209A',
            's' => '\u209B',
            't' => '\u209C',
            _ => a
        };

    }

*/


/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_can_identify_iras_number() {
        let (type_id, type_string, id) = classify_identifier("IRAS 123456".to_string());

        assert_eq!(type_id, 303);
        assert_eq!(type_string, "IRAS ID".to_string());
        assert_eq!(id, "123456".to_string());
    } 
    

}

    */
