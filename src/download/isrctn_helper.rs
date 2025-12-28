

pub fn count_option<T>(v: Vec<T>) -> Option<Vec<T>> {
    match v.len() {
        0 => None,
        _ => Some(v),
    }
}


#[allow(dead_code)]
pub trait StringExtensions {
    fn tidy(&self) -> Option<String>;
    fn replace_unicodes(&self) -> Option<String>;
    fn replace_tags_and_unicodes(&self) -> Option<String>;
    fn compress_spaces(&self) -> Option<String>;
}

pub trait OptionStringExtensions {
    fn as_text_opt(&self) -> Option<String>;
    fn as_filtered_text_opt(&self) -> Option<String>;
    fn as_date_opt(&self) -> Option<String>;
    fn as_datetime_opt(&self) -> Option<String>;
    fn as_float_opt(&self) -> Option<f32>;
    fn as_bool_opt(&self) -> Option<bool>;
}


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

    fn as_filtered_text_opt(&self) -> Option<String> {
        match self {
            Some(s) => { 
                    let st = s.trim();
                    if st == "" 
                    {
                        None
                    } 
                    else {
                        if st == "N/A" || st.starts_with("Nil ") || st.starts_with("Not ") {
                            None
                        }
                        else {
                            Some(st.to_string())
                        }
                    }
                },
            None => None
        }
    }

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


    fn compress_spaces(&self) -> Option<String> {
    
       let trimmed = self.trim();
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

            let result = output_string.trim();
            Some(result.to_string())
        }
    }
}


