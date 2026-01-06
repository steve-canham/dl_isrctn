use std::sync::LazyLock;
use regex::Regex;


#[allow(dead_code)]
pub trait OptionNameExtensions {

    fn tidy_orcid(&self) -> Option<String>;

    fn appears_plausible_org_name(&self) -> bool;
    fn appears_plausible_person_name(&self) -> bool;
    fn appears_plausible_title(&self) -> bool;


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

    
}


/* 

    
    public static string? TidyOrgName(this string? in_name, string sid)
    {
        if (string.IsNullOrEmpty(in_name))
        {
            return null;
        }

        string? name = in_name;

        if (name.Contains("."))
        {
            // protect these exceptions to the remove full stop rule
            name = name.Replace(".com", "|com");
            name = name.Replace(".gov", "|gov");
            name = name.Replace(".org", "|org");

            name = name.Replace(".", "");

            name = name.Replace("|com", ".com");
            name = name.Replace("|gov", ".gov");
            name = name.Replace("|org", ".org");
        }

        // Replace any apostrophes

        name = name.ReplaceApos();

        // Trim any odd' characters

        name = name!.Trim(',', '-', '*', ';', ' ');

        // try and deal with possible ambiguities (organisations with genuinely the same name)

        string nLower = name.ToLower();
        if (nLower.Contains("newcastle") && nLower.Contains("university")
                                         && !nLower.Contains("hospital"))
        {
            if (nLower.Contains("nsw") || nLower.Contains("australia"))
            {
                name = "University of Newcastle (Australia)";
            }
            else if (nLower.Contains("uk") || nLower.Contains("tyne"))
            {
                name = "University of Newcastle (UK)";
            }
            else if (sid.StartsWith("ACTRN"))
            {
                name = "University of Newcastle (Australia)";
            }
            else
            {
                name = "University of Newcastle (UK)";
            }
        }

        if (nLower.Contains("china medical") && nLower.Contains("university"))
        {
            if (nLower.Contains("taiwan") || nLower.Contains("taichung"))
            {
                name = "China Medical University, Taiwan";
            }
            else if (nLower.Contains("Shenyang") || nLower.Contains("prc"))
            {
                name = "China Medical University";
            }
            else if (sid.StartsWith("Chi"))
            {
                name = "China Medical University";
            }
        }

        if (nLower.Contains("national") && nLower.Contains("cancer center"))
        {
            if (sid.StartsWith("KCT"))
            {
                name = "National Cancer Center, Korea";
            }
            else if (sid.StartsWith("JPRN"))
            {
                name = "National Cancer Center, Japan";
            }
        }

        return name;
    }


    public static string? TidyPersonName(this string? in_name)
    {
        // Replace apostrophes and remove periods.
        
        string? name1 = in_name.ReplaceApos();
        string? pName = name1?.Replace(".", "");
        
        if (string.IsNullOrEmpty(pName))
        {
            return null;
        }

        // Check for professional titles

        string low_name = pName.ToLower();

        if (low_name.StartsWith("professor "))
        {
            pName = pName[10..];
        }
        else if (low_name.StartsWith("associate professor "))
        {
            pName = pName[20..];
        }
        else if (low_name.StartsWith("prof "))
        {
            pName = pName[5..];
        }
        else if (low_name.StartsWith("dr med "))
        {
            pName = pName[7..];
        }
        else if (low_name.StartsWith("dr ") || low_name.StartsWith("mr ")
                                            || low_name.StartsWith("ms "))
        {
            pName = pName[3..];
        }
        else if (low_name.StartsWith("dr") && low_name.Length > 2
                                           && pName[2].ToString() == low_name[2].ToString().ToUpper())
        {
            pName = pName[2..];
        }
        else if (low_name is "dr" or "mr" or "ms")
        {
            pName = "";
        }
       
        if (pName == "")
        {
            return pName;
        }
        
        // remove some trailing qualifications

        int comma_pos = pName.IndexOf(',', StringComparison.Ordinal);
        if (comma_pos > -1)
        {
            pName = pName[..comma_pos];
        }

        string low_name2 = pName.ToLower();
        if (low_name2.EndsWith(" phd") || low_name2.EndsWith(" msc"))
        {
            pName = pName[..^3];
        }
        else if (low_name2.EndsWith(" ms"))
        {
            pName = pName[..^2];
        }
        else if (low_name2.EndsWith(" ms(ophthal)"))
        {
            pName = pName[..^12];
        }

        return pName.Trim(' ', '-');
    }


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




}
    

