use chrono::NaiveDate;


pub trait OptionDateExtensions {
    fn as_string_opt(&self) -> Option<String>;
    
}

// Extensions for Option<NaiveDate>.

impl OptionDateExtensions for Option<NaiveDate> {

    // Simply converts the Option<NaiveDate> to
    // an Option<String>, with the date in ISO format.
    // Can be useful when a date is stored in a 
    // varchar field, e.g. when a general parameter 
    // happens to be, and be used as, a date.

    fn as_string_opt(&self) -> Option<String> {
         match self {
            Some(s) => { 
                    Some(s.format("%Y-%m-%d").to_string())
                },
            None => None
        }
    }



}

