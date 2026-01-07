use std::sync::LazyLock;
use regex::Regex;



#[allow(dead_code)]
pub trait OptionNameExtensions {

    fn tidy_orcid(&self) -> Option<String>;
    fn extract_domain(&self) -> Option<String>;

    fn appears_plausible_org_name(&self) -> bool;
    fn appears_plausible_person_name(&self) -> bool;
    fn appears_plausible_title(&self) -> bool;

    fn tidy_org_name(&self, sd_sid: &String) -> Option<String>;
    fn tidy_person_name(&self) -> Option<String>;
}

impl OptionNameExtensions for Option<String> {

    fn tidy_orcid(&self) -> Option<String> {

        // Repairs common malformed ORCID Ids and checks structure

         match self {
            Some(sf) => { 

                let mut s = sf.to_string();
                s = s.replace("https://orcid.org/", "").replace("http://orcid.org/", "");
                s = s.replace("/", "-").replace(" ", "-");

                if s.len() == 16
                {
                    s = format!("{}-{}-{}-{}", &s[..4], &s[4..8], &s[8..12], &s[12..]);  
                }
                if s.len() == 15 
                {
                    s = format!("0000{}", s);
                }
                if s.len() == 14 {
                    s = format!("0000-{}", s);
                }

                static RE_ORCID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{4}-\d{4}-\d{4}-\d{4}$").unwrap());
                if RE_ORCID.is_match(&s) {
                    Some(s)
                }
                else {
                    None
                }
            },
            None => None
        }
    }


    fn extract_domain(&self) -> Option<String> {

        // Returns the portion of an email address behinmd the '@'

        match self {
            Some(sf) => { 

                match sf.find('@')
                {
                    Some(p) => {
                        Some((&sf[(p + 1)..]).to_string())
                    },
                    None => None,
                }
            },
            None => None
        }
    }


    fn appears_plausible_org_name(&self) -> bool {

        match self {
            Some(sf) => { 

                let lower_s = sf.trim().to_lowercase();
                let low_s = lower_s.as_str();

                if low_s.len() < 3
                {
                    false
                }
                else if low_s == "n.a." || low_s == "n a" || low_s == "n/a" 
                    || low_s == "nil" || low_s == "nill" || low_s == "non" || low_s == "none"
                    || low_s == "not applicable" || low_s == "investigator" || low_s == "self" 
                {
                    false
                }
                else if low_s.starts_with("no ") ||low_s.starts_with("not prov") || low_s.starts_with("non fund")
                    || low_s.starts_with("non spon") || low_s.starts_with("nonfun") || low_s.starts_with("noneno") 
                    || low_s.starts_with("organisation") || low_s.contains("thesis") || low_s.contains(" none.")
                {
                    false
                }
                else if low_s.starts_with("investigator ") ||low_s.starts_with("professor") || low_s.starts_with("prof ")
                    || low_s.starts_with("prof. ") || low_s.starts_with("associate prof") || low_s.starts_with("noneno") 
                    || low_s.starts_with("dr med ") || low_s.starts_with("dr ")
                    || low_s.starts_with("mr ") || low_s.starts_with("ms ") 
                {
                    false
                }
                else if low_s.starts_with("dr") && &sf[..2] == "DR"
                {
                    false
                }
                else {
                    true
                }
            },
            None => false,
        }
    }
     
     
    fn appears_plausible_person_name(&self) -> bool {

        match self {
            Some(sf) => { 

                let lower_s = sf.trim().to_lowercase();
                let low_s = lower_s.as_str();

                if low_s == "" || low_s.contains("research") || low_s.contains("development") ||
                    low_s.contains("trials") || low_s.contains("pharma") ||
                    low_s.contains("ltd") || low_s.contains("inc.") {
                    false
                }
                else {
                    true
                }
            },
            None => false,
        }
    }


    fn appears_plausible_title(&self) -> bool {

        match self {
            Some(sf) => { 

                let lower_s = sf.trim().to_lowercase();
                let low_s = lower_s.as_str();

                if low_s == "" || low_s == "n.a." || low_s == "na" || low_s == "n.a" || low_s == "n/a"
                {
                    false
                }
                else if low_s == "none" || low_s == "not done" || low_s == "same as above" 
                    || low_s == "in preparation" || low_s == "non fornito"
                {
                    false
                }
                else if low_s.starts_with("not applic") || low_s.starts_with("not aplic") || low_s.starts_with("non applic") 
                    || low_s.starts_with("non aplic") || low_s.starts_with("no applic") || low_s.starts_with("no aplic")
                    || low_s.starts_with("see ") || low_s.starts_with("not avail") || low_s.starts_with("non dispo")
                {
                    false
                }
                else {
                    true
                }
            },
            None => false,
        }
    }


    fn tidy_org_name(&self, sd_sid: &String) -> Option<String> {
        
        // string should already have been cleaned 
        // therefore basic trim, apostrophes, escaped characters dealt with

        match self {
            Some(sf) => { 

                let mut s = sf.to_string();

                if s.contains(".")
                {
                    // Protect these exceptions to the remove full stop rule

                    s = s.replace(".com", "|com");
                    s = s.replace(".gov", "|gov");
                    s = s.replace(".org", "|org");

                    s = s.replace(".", "");

                    s = s.replace("|com", ".com");
                    s = s.replace("|gov", ".gov");
                    s = s.replace("|org", ".org");
                }

                s = s.trim_matches(&[',', '-', '*', ';', ' ']).to_string();

                // Deal with some names that can be ambiguous (without country data)
                
                let lower_s = s.trim().to_lowercase();
                let low_s = lower_s.as_str();

                if low_s.contains("newcastle") {
                    if low_s.contains("university") && !low_s.contains("hospital") {
                        if low_s.contains("nsw") || low_s.contains("australia") {
                            s = "University of Newcastle (Australia)".to_string();
                        }
                        else if low_s.contains("uk") || low_s.contains("tyne")
                        {
                            s = "University of Newcastle (UK)".to_string();
                        }
                        else if sd_sid.starts_with("ACTRN")
                        {
                            s = "University of Newcastle (Australia)".to_string();
                        }
                        else
                        {
                            s = "University of Newcastle (UK)".to_string();
                        }
                    }
                }
                if low_s.contains("china medical") {
                    if low_s.contains("taiwan") || low_s.contains("taichung")
                    {
                        s = "China Medical University, Taiwan".to_string();
                    }
                    else if low_s.contains("shenyang") || low_s.contains("prc")
                    {
                        s = "China Medical University".to_string();
                    }
                    else if sd_sid.starts_with("Chi")
                    {
                        s = "China Medical University".to_string();
                    }
                }
                if low_s.contains("cancer center") {
                    if sd_sid.starts_with("KCT")
                    {
                        s = "National Cancer Center, Korea".to_string();
                    }
                    else if sd_sid.starts_with("JPRN")
                    {
                        s = "National Cancer Center, Japan".to_string();
                    }
                }
                   
                Some(s)
            },
            None => None
        }
       
    }


    fn tidy_person_name(&self) -> Option<String> {

        // string should already have been cleaned 
        // therefore basic trim, apostrophes, escaped characters dealt with

        match self {
            Some(sf) => { 

                let mut s = sf.to_string();

                // Remove any periods, then 
                // remove possible professional prefixes

                s = s.replace(".", "");

                let lower_s = s.trim().to_lowercase();
                let low_s = lower_s.as_str();

                if low_s.starts_with("professor ")
                {
                    s = s[10..].to_string();
                }
                else if low_s.starts_with("associate professor ")
                {
                    s = s[20..].to_string();
                }
                else if low_s.starts_with("prof ")
                {
                    s = s[5..].to_string();
                }
                else if low_s.starts_with("dr med ")
                {
                    s = s[7..].to_string()
                }
                else if low_s.starts_with("dr ") || low_s.starts_with("mr ")
                          || low_s.starts_with("ms ")
                {
                    s = s[3..].to_string();
                }
                else if low_s.starts_with("dr") && low_s.len() > 2
                    && s[2..3].to_string() == low_s[2..3].to_string().to_uppercase()
                {
                    s = s[2..].to_string();
                }

                let st = s.trim();
                let lower_st = st.to_lowercase();
                let low_st = lower_st.as_str();

                if low_st == "" || low_st == "dr" || low_st == "mr" || low_st ==  "ms" {
                    None
                }

                else {

                    // remove some excess trailing material, including
                    // initially behind any comma

                    let st = match st.find(',') {
                        Some(p) => &st[..p],
                        None => st,
                    };

                    let lower_st = st.to_lowercase();
                    let low_st = lower_st.as_str();
                    let mut sts = st.to_string();

                    if low_st.ends_with(" phd") || low_st.ends_with(" msc")
                    {
                        sts.truncate(sts.len() - 4);
                    }

                    else if low_st.ends_with(" md") || low_st.ends_with(" ms")
                    {
                        sts.truncate(sts.len() - 3);
                    }

                    else if low_st.ends_with(" ms(ophthal)")
                    {
                        sts.truncate(sts.len() - 12);
                    }

                    Some(sts)

                }
            },
            None => None,

        }
    }

    
}


/* 

    public static string? ExtractOrganisation(this string affiliation, string sid)
    {
        if (string.IsNullOrEmpty(affiliation))
        {
            return null;
        }
        
        string? affil_organisation = "";
        string aff = affiliation.ToLower();

        if (!aff.Contains(","))
        {
            affil_organisation = affiliation;
        }
        else if (aff.Contains("univers"))
        {
            affil_organisation = FindSubPhrase(affiliation, "univers");
        }
        else if (aff.Contains("hospit"))
        {
            affil_organisation = FindSubPhrase(affiliation, "hospit");
        }
        else if (aff.Contains("klinik"))
        {
            affil_organisation = FindSubPhrase(affiliation, "klinik");
        }
        else if (aff.Contains("instit"))
        {
            affil_organisation = FindSubPhrase(affiliation, "instit");
        }
        else if (aff.Contains("nation"))
        {
            affil_organisation = FindSubPhrase(affiliation, "nation");
        }
        else if (aff.Contains(" inc."))
        {
            affil_organisation = FindSubPhrase(affiliation, " inc.");
        }
        else if (aff.Contains(" ltd"))
        {
            affil_organisation = FindSubPhrase(affiliation, " ltd");
        }

        return TidyOrgName(affil_organisation, sid);
    }


    public static string? FindSubPhrase(this string? phrase, string target)
    {
        if (string.IsNullOrEmpty(phrase))
        {
            return null;
        }

        string phrase1 = phrase.Replace("&#44;", ",");
        string p = phrase1.ToLower();
        string t = target.ToLower();

        // ignore trailing commas after some states names.
        p = p.Replace("california,", "california*");
        p = p.Replace("wisconsin,", "wisconsin*");

        // Find target in phrase if possible, and the position
        // of the preceding comma, and the comma after the target (if any)
        // if no preceding comma make start the beginning of the string.
        // if no following comma make end the end of the string
                    
        int startPos = p.IndexOf(t, StringComparison.Ordinal);
        if (startPos == -1)
        {
            return phrase1;
        }

        int commaPos1 = p.LastIndexOf(",", startPos, StringComparison.Ordinal); 
        if (commaPos1 == -1)
        {
            commaPos1 = 0;
        }
        int commaPos2 = p.IndexOf(",", startPos + target.Length - 1, StringComparison.Ordinal);
        if (commaPos2 == -1)
        {
            commaPos2 = p.Length;
        }

        string org_name = phrase1[(commaPos1 + 1)..commaPos2].Trim();

        if (org_name.ToLower().StartsWith("the "))
        {
            org_name = org_name[4..];
        }

        return org_name;
    }


    public static List<string>? SplitStringWithMinWordSize(this string? input_string, char separator, int min_width)
    {
        if (!string.IsNullOrEmpty(input_string))
        {
            return null;
        }
        else
        {
            // try and avoid spurious split string results
            string[] split_strings = input_string!.Split(separator);
            for (int j = 0; j < split_strings.Length; j++)
            {
                if (split_strings[j].Length < min_width)
                {
                    if (j == 0)
                    {
                        split_strings[1] = split_strings[0] + "," + split_strings[1];
                    }
                    else
                    {
                        split_strings[j - 1] = split_strings[j - 1] + "," + split_strings[j];
                    }
                }
            }

            List<string> strings = new();
            foreach (string ss in split_strings)
            {
                if (ss.Length >= min_width)
                {
                    strings.Add(ss);
                }
            }

            return strings;
        }
    }


    public static List<string> GetNumberedStrings(this string input_string, string number_suffix, int max_number)
    {
        List<string> split_strings = new();
        for (int i = max_number; i > 0; i--)
        {
            string string_number = i.ToString() + number_suffix;
            int number_pos = input_string.LastIndexOf(string_number, StringComparison.Ordinal);
            if (number_pos != -1)
            {
                string string_to_store = input_string[(number_pos + string_number.Length)..].Trim();
                split_strings.Add(string_to_store);
                input_string = input_string[..number_pos];
            }
        }

        // Anything left at the front of the string?
        if (input_string != "")
        {
            split_strings.Add(input_string);
        }

        // reverse order before returning
        List<string> reversed_strings = new();
        for (int j = split_strings.Count - 1; j >= 0; j--)
        {
            reversed_strings.Add(split_strings[j]);
        }

        return reversed_strings;
    }



*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::string_extensions::*;

    #[test]
    fn check_name_ext_tidy_orcid() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.tidy_orcid(), None);

        let t_opt = Some("random string".to_string());
        assert_eq!(t_opt.tidy_orcid(), None);

        let t_opt = Some("1234--666-9876".to_string());
        assert_eq!(t_opt.tidy_orcid(), None);

        let t_opt = Some("0000-1234-6666-9876 and the rest".to_string());
        assert_eq!(t_opt.tidy_orcid(), None);

        let t_opt = Some("0000/1234/6666/9876".to_string());
        assert_eq!(t_opt.tidy_orcid(), Some("0000-1234-6666-9876".to_string()));

        let t_opt = Some("0000 1234 6666/9876".to_string());
        assert_eq!(t_opt.tidy_orcid(), Some("0000-1234-6666-9876".to_string()));

        let t_opt = Some("https://orcid.org/0000-1234-6666-9876".to_string());
        assert_eq!(t_opt.tidy_orcid(), Some("0000-1234-6666-9876".to_string()));

        let t_opt = Some("1234-6666-9876".to_string());
        assert_eq!(t_opt.tidy_orcid(), Some("0000-1234-6666-9876".to_string()));

        let t_opt = Some("-1234-6666-9876".to_string());
        assert_eq!(t_opt.tidy_orcid(), Some("0000-1234-6666-9876".to_string()));
    } 


    #[test]
    fn check_name_ext_extract_domain() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.extract_domain(), None);

        let t_opt = Some("random string".to_string());
        assert_eq!(t_opt.extract_domain(), None);

        let t_opt = Some("random@string".to_string());
        assert_eq!(t_opt.extract_domain(), Some("string".to_string()));

        let t_opt = Some("funny_email@domain.com".to_string());
        assert_eq!(t_opt.extract_domain(), Some("domain.com".to_string()));

        let t_opt = Some(r"funny_unicod\u{2022\u{21E7}email@domain.com".to_string());
        assert_eq!(t_opt.extract_domain(), Some("domain.com".to_string()));
    } 


    #[test]
    fn check_name_ext_appears_plausible_org_name() {

        let t_opt = Some(" \n  \n ".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("BT".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("Nil".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("no way".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("Is a thesis".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("Prof smith".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("Dr med".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("DR Smith".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), false);

        let t_opt = Some("Drugs are us".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), true);

        let t_opt = Some("random string ltd".to_string());
        assert_eq!(t_opt.appears_plausible_org_name(), true);

    } 

    #[test]
    fn check_name_ext_appears_plausible_person_name() {

        let t_opt = Some(" \n  \n".to_string());
        assert_eq!(t_opt.appears_plausible_person_name(), false);

        let t_opt = Some("random string".to_string());
        assert_eq!(t_opt.appears_plausible_person_name(), true);

        let t_opt = Some("research head".to_string());
        assert_eq!(t_opt.appears_plausible_person_name(), false);

        let t_opt = Some("development office".to_string());
        assert_eq!(t_opt.appears_plausible_person_name(), false);

        let t_opt = Some("XY pharma".to_string());
        assert_eq!(t_opt.appears_plausible_person_name(), false);

        let t_opt = Some("pfizer inc.".to_string());
        assert_eq!(t_opt.appears_plausible_person_name(), false);
    } 

    #[test]
    fn check_name_ext_appears_plausible_title() {

        let t_opt = Some(" \t \t ".to_string());
        assert_eq!(t_opt.appears_plausible_title(), false);

        let t_opt = Some("N.A.".to_string());
        assert_eq!(t_opt.appears_plausible_title(), false);

        let t_opt = Some("Not done".to_string());
        assert_eq!(t_opt.appears_plausible_title(), false);

        let t_opt = Some("Same as above".to_string());
        assert_eq!(t_opt.appears_plausible_title(), false);

        let t_opt = Some("Not applicable".to_string());
        assert_eq!(t_opt.appears_plausible_title(), false);

        let t_opt = Some("Non disponible".to_string());
        assert_eq!(t_opt.appears_plausible_title(), false);

        let t_opt = Some("random string".to_string());
        assert_eq!(t_opt.appears_plausible_title(), true);
    } 

    #[test]
    fn check_name_ext_tidy_org_name() {

        let t_opt = Some("  ".to_string());
        assert_eq!(t_opt.clean().tidy_org_name(&"NCT12345678".to_string()), None);

        let t_opt = Some("N.A. 'Valdez'".to_string());
        assert_eq!(t_opt.clean().tidy_org_name(&"NCT12345678".to_string()), Some("NA ‘Valdez’".to_string()));

        let t_opt = Some("Not done.com  ;".to_string());
        assert_eq!(t_opt.clean().tidy_org_name(&"NCT12345678".to_string()), Some("Not done.com".to_string()));

        let t_opt = Some("Newcastle University, Newcastle-upon-Tyne".to_string());
        assert_eq!(t_opt.clean().tidy_org_name(&"NCT12345678".to_string()), Some("University of Newcastle (UK)".to_string()));

        let t_opt = Some("**china medical uni, ".to_string());
        assert_eq!(t_opt.clean().tidy_org_name(&"Chi12345678".to_string()), Some("China Medical University".to_string()));

        let t_opt = Some("-china medical uni, Taiwan".to_string());
        assert_eq!(t_opt.clean().tidy_org_name(&"NCT12345678".to_string()), Some("China Medical University, Taiwan".to_string()));

        let t_opt = Some("Nat. Cancer center".to_string());
        assert_eq!(t_opt.clean().tidy_org_name(&"KCT12345678".to_string()), Some("National Cancer Center, Korea".to_string()));
    } 

    #[test]
fn check_name_ext_tidy_person_name() {

        let t_opt = Some(" \t \t ".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), None);

        let t_opt = Some("N.A.".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("NA".to_string()));

        let t_opt = Some("J.S. Smith, MD, BPhil".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("JS Smith".to_string()));

        let t_opt = Some("Dr. Jones".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Jones".to_string()));

        let t_opt = Some("Mr".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), None);

        let t_opt = Some("DrFaustus".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Faustus".to_string()));

        let t_opt = Some("Professor Andrew J. Taylor".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Andrew J Taylor".to_string()));

        let t_opt = Some("Jane Andrews Phd".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Jane Andrews".to_string()));

        let t_opt = Some("Fred Bloggs MS(Ophthal)".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Fred Bloggs".to_string()));

        let t_opt = Some("Freda Bloggs MD".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Freda Bloggs".to_string()));

        let t_opt = Some("Frederika Bloggs MD, DPhil".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Frederika Bloggs".to_string()));

        let t_opt = Some("Frederick Bloggs M.D.".to_string());
        assert_eq!(t_opt.clean().tidy_person_name(), Some("Frederick Bloggs".to_string()));

    } 

}
    