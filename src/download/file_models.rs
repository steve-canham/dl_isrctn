use chrono::NaiveDate;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct WHOLine
{
    pub trial_id: String,                           // 0
    pub last_updated: String,                       // 1
    pub sec_ids: String,                            // 2
    pub pub_title: String,                          // 3
    pub scientific_title: String,                   // 4
    pub url: String,                                // 5

    pub pub_contact_first_name: String,             // 6
    pub pub_contact_last_name: String,              // 7
    pub pub_contact_address: String,                // 8
    pub pub_contact_email: String,                  // 9
    pub pub_contact_tel: String,                    // 10
    pub pub_contact_affiliation: String,            // 11

    pub sci_contact_first_name: String,             // 12
    pub sci_contact_last_name: String,              // 13
    pub sci_contact_address: String,                // 14
    pub sci_contact_email: String,                  // 15
    pub sci_contact_tel: String,                    // 16
    pub sci_contact_affiliation: String,            // 17

    pub study_type: String,                         // 18
    pub study_design: String,                       // 19
    pub phase: String,                              // 20
    pub date_registration: String,                  // 21
    pub date_enrollement: String,                   // 22
    pub target_size: String,                        // 23
    pub recruitment_status:String,                  // 24

    pub primary_sponsor: String,                    // 25
    pub secondary_sponsors: String,                 // 26
    pub source_support: String,                     // 27
    pub countries: String,                          // 28
    pub conditions: String,                         // 29
    pub interventions: String,                      // 30

    pub age_min: String,                            // 31
    pub age_max: String,                            // 32
    pub gender: String,                             // 33
    pub inclusion_criteria: String,                 // 34
    pub exclusion_criteria: String,                 // 35

    pub primary_outcome: String,                    // 36
    pub secondary_outcomes: String,                 // 37

    pub bridging_flag: String,                      // 38
    pub bridged_type: String,                       // 39
    pub childs: String,                             // 40
    pub type_enrolment: String,                     // 41
    pub retrospective_flag: String,                 // 42

    pub results_actual_enrollment: String,          // 43
    pub results_url_link: String,                   // 44
    pub results_summary: String,                    // 45
    pub results_date_posted: String,                // 46
    pub results_date_first_pub: String,             // 47
    pub results_baseline_char: String,              // 48
    pub results_participant_flow: String,           // 49
    pub results_adverse_events: String,             // 50
    pub results_outcome_measures: String,           // 51
    pub results_url_protocol: String,               // 52

    pub results_ipd_plan: String,                   // 53
    pub results_ipd_description: String,            // 54
    pub results_date_completed: String,             // 55
    pub results_yes_no: String,                     // 56

    pub ethics_status: String,                      // 57
    pub ethics_approval_date: String,               // 58
    pub ethics_contact_name: String,                // 59
    pub ethics_contact_address:String,              // 60
    pub ethics_contact_phone: String,               // 61
    pub ethics_contact_email: String,               // 62

}


/* 
    [Index(0)] 
    pub string TrialID: Option<String>, = null!;
ACTRN12625000115437,
    [Index(1)]
    pub string? last_updated: Option<String>,
"""17 February 2025""",
    [Index(2)]
    pub string? SecondaryIDs: Option<String>,
"""NCRC-AU-2024/003""",    
    [Index(3)]
    pub string? pub_title: Option<String>,
"""Sub-Protocol #1 of Umbrella Protocol Study:  Limit of Blank Characterization of Vancomycin Biosensor Nutromics Device. ""","""Sub-Protocol #1 of Umbrella Protocol Study:  Limit of Blank Characterization of Vancomycin Biosensor Nutromics Device.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        """,
    [Index(4)]
    pub string? Scientific_title: Option<String>,
"""Sub-Protocol #1 of Umbrella Protocol Study:  Limit of Blank Characterization of Vancomycin Biosensor Nutromics Device. ""","""Sub-Protocol #1 of Umbrella Protocol Study:  Limit of Blank Characterization of Vancomycin Biosensor Nutromics Device.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        """,
    [Index(5)]
    pub string? url: Option<String>,
"""https://anzctr.org.au/ACTRN12625000115437.aspx""",    
    [Index(6)]
    pub string? pub_Contact_Firstname: Option<String>, NULL
    [Index(7)]
    pub string? pub_Contact_Lastname: Option<String>, NULL
    [Index(8)]
    pub string? pub_Contact_Address: Option<String>, NULL
    [Index(9)]
    pub string? pub_Contact_Email: Option<String>, NULL
    [Index(10)]
    pub string? pub_Contact_Tel: Option<String>, NULL
    [Index(11)]
    pub string? pub_Contact_Affiliation: Option<String>, NULL
    [Index(12)]
    pub string? Scientific_Contact_Firstname: Option<String>, NULL
    [Index(13)]
    pub string? Scientific_Contact_Lastname: Option<String>, NULL
    [Index(14)]
    pub string? Scientific_Contact_Address: Option<String>, NULL
    [Index(15)]
    pub string? Scientific_Contact_Email: Option<String>, NULL
    [Index(16)]
    pub string? Scientific_Contact_Tel: Option<String>, NULL
    [Index(17)]
    pub string? Scientific_Contact_Affiliation: Option<String>, NULL
    [Index(18)]
    pub string? study_type: Option<String>,
    """Interventional""", 
    [Index(19)]
    pub string? study_design: Option<String>,
    """Purpose: Diagnosis; Allocation: Non-randomised trial; Masking: Open (masking not used);"""
    [Index(20)]
    pub string? phase: Option<String>,
    """Not Applicable""",  
    [Index(21)]
    pub string? Date_registration: Option<String>,
    """31/01/2025""", 
    [Index(22)]
    pub string? Date_enrollement: Option<String>,
    """03/02/2025""",
    [Index(23)]
    pub string? Target_size: Option<String>,
    """50"""
    [Index(24)]
    pub string? Recruitment_status: Option<String>,
    """Not yet recruiting"""
    [Index(25)]
    pub string? Primary_sponsor: Option<String>,
    """Nutromics Operations"""
    [Index(26)]
    pub string? Secondary_sponsors: Option<String>,
    NULL
    [Index(27)]
    pub string? Source_Support: Option<String>,
    """Nutromics Operations"""
    [Index(28)]
    pub string? Countries: Option<String>,
    """Australia""",
    [Index(29)]
    pub string? Conditions: Option<String>,
    """Therapeutic Drug Monitoring; <br>Therapeutic Drug Monitoring;Infection - Studies of infection and infectious agents""",
    [Index(30)]
    pub string? Interventions: Option<String>,
    """This is a prospective study with an Umbrella Protocol; where each sub-Protocol investigates a particular condition(s) and challenges to various Vancomycin biosensors and electrode properties. The study will enroll healthy participants in the community. Participants will be recruited for participation in stages depending on the Research and Development needs. This registration described Sub-Protocol #1.<br><br>The study will enroll no more than 50 participants. Participants may participate in more than one sub-Protocol of the Study, subject to their continued eligibility. <br><br>Potential participants will be recruited by flyers on community, university, and  <br>Nutromics Operations noticeboards. Prospective participants will express interest via email to  <br>clinical.researchcentre@nutromics.com. <br><br>Participants who have previously participated or expressed interested in studies at the Nutromics Clinical Research Centre may also be contacted and asked if they are interested in participating. Prospective participants will be sent an Online Health Screening. Nutromics employees can participate in this study. <br><br>Participants who meet the inclusion criteria (as per the Online Health Screening Survey) will be contacted for a brief phone screening for inclusion/exclusion criteria, including an explanation of the study details. A copy of the patient information sheet and consent form will be given to the participants who meet the inclusion criteria and still express interest in participating in the study following the phone discussion. Eligible participants who agree to participate in the study will be given time to consider their decision. Eligible participants who agree to participate in the study will be asked to attend an on-site Visit, where written consent w""",
    [Index(31)]
    pub string? Age_min: Option<String>,
    """18 Years""",
    [Index(32)]
    pub string? Age_max: Option<String>,
    """60 Years""",
    [Index(33)]
    pub string? Gender: Option<String>,
    """Both males and females""", 
    [Index(34)]
    pub string? Inclusion_Criteria: Option<String>,
    """Inclusion criteria: Participants self-declaring healthy without any disease, condition or syndrome or on any current prescription medical treatments (other than contraception medication) <br><br>Aged 18-60 years """,
    [Index(35)]
    pub string? Exclusion_Criteria: Option<String>,
    """Exclusion criteria: Participants who are pregnant, lactating, planning to become pregnant, breastfeeding, or donating ova.<br><br>Participants who declare previous allergic reactions to metals, plastics, and adhesives. <br><br>Participants who have a history of fainting or experiencing vasovagal reactions during blood draws.<br><br>Non-English-speaking participants.<br><br>Participants with small children in their household, where the Investigator believes that the device falling off could pose a biohazard risk if a child were to pick it up (Sub-Protocol #1 only). <br>""",
    [Index(36)]
    pub string? Primary_Outcome: Option<String>,
    """Establish the highest reported concentration Limit of Blank (LoB)  likely to be observed for a blank sample using the Nutromics Sensor Device.[In-vivo data collection will be calibrated against a standard titration of known values on analogous devices to report concentrations. The workflow to achieve this involves benchtop testing of Nutromics aptamer sensors in buffer and/or biofluid with a known concentration of vancomcyin across a range of temperatures anticipated in-vivo (30-42C). Addition of known values of vancomycin and subsequent measurements. Fitting a calibration equation, and finally using data collected from the device (electrochemical measurements & temperature measurements) in this equation to predict concentration.  <br><br>Limit of Blank will be reported using the predicted mean concentration + 2 standard deviations for a given device’s data ( during the first and last hour of data collection.  <br> 24 hour wear period.];To assess the Limit of Blank (LoB) of the Nutromics Sensor Device regarding its performance and stability over extended periods of wear. This includes evaluating any potential degradation in accuracy or precision of the measurements when the Nutromics Sensor Device is used continuously over a long duration.[The accuracy of the device from interstitial fluid measurements will be assessed as the deviation of vancomycin concentrations from 0 mg/L. The precision will be assessed as the variability (standard deviation) of vancomycin concentrations (if detected) over the 24-hour duration of the study. <br> Across wear period where the Nutromics Sensor Device will collect data on vancomycin concentrations in interstitial fluid every 5 minutes.]""",
    [Index(37)]
    pub string? Secondary_Outcomes: Option<String>,
    """Evaluate the variability and error associated with Vancomycin biosensors when the target analyte is not present. [The accuracy of the device from interstitial fluid measurements will be assessed as the deviation of vancomycin concentrations from 0 mg/L. The precision will be assessed as the variability (standard deviation) of vancomycin concentrations (if detected) across devices and across participants. This data will be evaluated as a composite outcome.<br><br>Signal to noise will be calculated from voltammogram data using the root-mean-squared error (RSME) of the raw signal versus the smoothed (Savitzky-Golay) data. Across wear period where the Nutromics Sensor Device will collect data on vancomycin concentrations in interstitial fluid every 5 minutes.];To evaluate the safety of the Nutromics sensor device. [Monitoring full blood count (FBC), urea electrolytes creatinine (UEC), and liver function test (LFT) results and C-reactive Protein (CRP), prior to the application, and following removal of the Investigational Device. <br><br>Examining digitally captured images of the skin surface at the sensor application site(s) for signs of irritation and allergic reactions. <br><br>Observing participants pain using the scales provided (Harvard pain Scale). <br><br>Assess adverse events using the MedDRA class system. Blood will be collected at three occasions to assess safety of the Nutromics Sensor Device; immediately prior to application of the Devices, immediately prior to the participant being discharged whilst wearing the Device, and immediately following the removal of the Device the following day. Images of the application site will be taken prior to the application and removal of the Device. A pain score is obtained from the participant no more than 15 minutes following application of each Nutromics Sensor Device. Adverse Events will be assessed across the wear time of the Device.];Characterise the impact of hydration on the LoB of Vancomycin biosensors (serum osmolality) [Blood will be tested for serum osmolality. Blood will be collected at three occasions to assess serum osmolality; immediately prior to application of the Devices, immediately prior to the participant being discharged whilst wearing the Device, and immediately following the removal of the Device the following day,]""",
    [Index(38)]
    pub string? Bridging_flag: Option<String>,
    """                                                  """,
    [Index(39)]
    pub string? Bridged_type: Option<String>,
    """          """,
    [Index(40)]
    pub string? Childs: Option<String>,
    NULL,
    [Index(41)]
    pub string? type_enrolment: Option<String>,
    """Anticipated""",
    [Index(42)]
    pub string? Retrospective_flag: Option<String>,
    NULL,
    [Index(43)]
    pub string? results_actual_enrollment: Option<String>,
    """""",
    [Index(44)]
    pub string? results_url_link: Option<String>,
    """""",
    [Index(45)]
    pub string? results_summary: Option<String>,
    """""",
    [Index(46)]
    pub string? results_date_posted: Option<String>,
    NULL,
    [Index(47)]
    pub string? results_date_first_pubation: Option<String>,
    NULL,
    [Index(48)]
    pub string? results_baseline_char: Option<String>,
    """""",
    [Index(49)]
    pub string? results_participant_flow: Option<String>,
    """""",
    [Index(50)]
    pub string? results_adverse_events: Option<String>,
    """""",
    [Index(51)]
    pub string? results_outcome_measures: Option<String>,
    """""",
    [Index(52)]
    pub string? results_url_protocol: Option<String>,
    """""",
    [Index(53)]
    pub string? results_IPD_plan: Option<String>,
    """Yes""",
    [Index(54)]
    pub string? results_IPD_description: Option<String>,
    """""",
    [Index(55)]
    pub string? results_date_completed: Option<String>,
    NULL,
    [Index(56)]
    pub string? results_yes_no: Option<String>,
    NULL,
    [Index(57)]
    pub string? Ethics_Status: Option<String>,
    """Not approved""",
    [Index(58)]
    pub string? Ethics_Approval_Date: Option<String>,
    """Jan  1 1900 12:00AM""",
    [Index(59)]
    pub string? Ethics_Contact_Name: Option<String>,
    """""",
    [Index(60)]
    pub string? Ethics_Contact_Address: Option<String>,
    """Nutromics Diagnostics HREC""",
    [Index(61)]
    pub string? Ethics_Contact_Phone: Option<String>,
    """""",
    [Index(62)]
    pub string? Ethics_Contact_Email: Option<String>,
    """"""
   

    [Index(0)] 
    pub string TrialID: Option<String>, = null!;
EUCTR2020-001039-29-GR
    [Index(1)]
    pub string? last_updated: Option<String>,
,"""17 February 2025""",
    [Index(2)]
    pub string? SecondaryIDs: Option<String>,
    """ESCAPE""",
    [Index(3)]
    pub string? pub_title: Option<String>,
"""MANAGEMENT OF NOVEL SARS CORONAVIRUS""",
    [Index(4)]
    pub string? Scientific_title: Option<String>,
"""EFFICIENCY IN MANAGEMENT OF ORGAN DYSFUNCTION ASSOCIATED WITH INFECTION BY THE NOVEL SARS-CoV-2 VIRUS (COVID-19) THROUGH A PERSONALIZED IMMUNOTHERAPY APPROACH: THE ESCAPE CLINICAL TRIAL - PERSONALIZED IMMUNOTHERAPY FOR SARS-CoV-2 ASSOCIATED ORGAN DYSFUCTION                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               """,
    [Index(5)]
    pub string? url: Option<String>,
"""https://www.clinicaltrialsregister.eu/ctr-search/search?query=eudract_number:2020-001039-29""",
    [Index(6)]
    pub string? pub_Contact_Firstname: Option<String>, 
"""President of the Board""",
    [Index(7)]
    pub string? pub_Contact_Lastname: Option<String>, 
"""""",
    [Index(8)]
    pub string? pub_Contact_Address: Option<String>, 
"""88 Michalakopoulou Street""",
    [Index(9)]
    pub string? pub_Contact_Email: Option<String>,
"""insepsis@otenet.gr"""
    [Index(10)]
    pub string? pub_Contact_Tel: Option<String>, 
"""302107480662""",
    [Index(11)]
    pub string? pub_Contact_Affiliation: Option<String>,
"""HELLENIC INSTITUTE FOR THE STUDY OF SEPSIS""",
    [Index(12)]
    pub string? Scientific_Contact_Firstname: Option<String>,
"""President of the Board""",
    [Index(13)]
    pub string? Scientific_Contact_Lastname: Option<String>,
"""""",
    [Index(14)]
    pub string? Scientific_Contact_Address: Option<String>,
    """88 Michalakopoulou Street""",
    [Index(15)]
    pub string? Scientific_Contact_Email: Option<String>,
    """insepsis@otenet.gr"""
    [Index(16)]
    pub string? Scientific_Contact_Tel: Option<String>, 
    """302107480662""",
    [Index(17)]
    pub string? Scientific_Contact_Affiliation: Option<String>,
    """HELLENIC INSTITUTE FOR THE STUDY OF SEPSIS""",
    [Index(18)]
    pub string? study_type: Option<String>,
"""Interventional clinical trial of medicinal product""",
    [Index(19)]
    pub string? study_design: Option<String>,
"""Controlled: no Randomised: no Open: yes Single blind: no Double blind: no Parallel group: no Cross over: no Other: no If controlled, specify comparator, Other Medicinial Product:  Placebo:  Other:  Number of treatment arms in the trial: 2 """,
    [Index(20)]
    pub string? phase: Option<String>,
 """Human pharmacology (Phase I): noTherapeutic exploratory (Phase II): yesTherapeutic confirmatory - (Phase III): noTherapeutic use (Phase IV): no""",
    [Index(21)]
    pub string? Date_registration: Option<String>,
"""31/03/2020""",
    [Index(22)]
    pub string? Date_enrollement: Option<String>,
    """01/04/2020""",
    [Index(23)]
    pub string? Target_size: Option<String>,
    """40""",
    [Index(24)]
    pub string? Recruitment_status: Option<String>,
   """Not Recruiting""",
    [Index(25)]
    pub string? Primary_sponsor: Option<String>,
   """HELLENIC INSTITUTE FOR THE STUDY OF SEPSIS""",
    [Index(26)]
    pub string? Secondary_sponsors: Option<String>,
   NULL,
    [Index(27)]
    pub string? Source_Support: Option<String>,
    """HELENIC INSTITUTE FOR THE STUDY OF SEPSIS""",
    [Index(28)]
    pub string? Countries: Option<String>,
   """Greece""",
    [Index(29)]
    pub string? Conditions: Option<String>,
   """Organ dysfunction by the novel SARS-Cov-2 virus <br>MedDRA version: 20.0Level: LLTClassification code 10035738Term: Pneumonia viral NOSSystem Organ Class: 100000004862;Therapeutic area: Diseases [C] - Virus Diseases [C02]""",
    [Index(30)]
    pub string? Interventions: Option<String>,
   """<br>Trade Name: Kineret<br>Product Name: Anakinra<br>Pharmaceutical Form: <br><br>Trade Name: RoActemra<br>Product Name: Tocilizumab<br>Pharmaceutical Form: Concentrate for solution for infusion<br><br>""",
   [Index(31)]
    pub string? Age_min: Option<String>,
  """""",
    [Index(32)]
    pub string? Age_max: Option<String>,
    """""",
    [Index(33)]
    pub string? Gender: Option<String>,
   """<br>Female: yes<br>Male: yes<br>""",
    [Index(34)]
    pub string? Inclusion_Criteria: Option<String>,
"""Inclusion criteria: <br>• Age equal to or above 18 years<br>• Male or female gender<br>• In case of women, unwillingness to remain pregnant during the study period.<br>• Written informed consent provided by the patient or by one first-degree relative/spouse in case of patients unable to consent<br>• Confirmed infection by SARS-CoV-2 virus using molecular techniques as defined by the World Health Organization11<br>• Organ dysfunction defined as the presence of at least one of the following conditions: <br> - Total SOFA score greater than or equal to 2; <br> - Involvement of the lower respiratory tract<br>• Laboratory documentation of MAS or immune dysregulation. MAS is documented by the findings of any serum ferritin greater than 4,420ng/ml. immune dysregulation is documented by the combination of two findings: a) serum ferritin equal to or lower than 4,420ng/ml; and b) less than 5,000 receptors of the membrane molecule of HLA-DR on the cell membrane of blood CD14-monocytes or less than 30 MFI of HLA-DR on the cell membrane of blood CD14-monocytes as counted by flow cytometry.<br>Are the trial subjects under 18? no<br>Number of subjects for this age range: <br>F.1.2 Adults (18-64 years) yes<br>F.1.2.1 Number of subjects for this age range 20<br>F.1.3 Elderly (>=65 years) yes<br>F.1.3.1 Number of subjects for this age range 20<br>""",
    [Index(35)]
    pub string? Exclusion_Criteria: Option<String>,
  """Exclusion criteria: <br>• Age below 18 years<br>• Denial for written informed consent<br>• Any stage IV malignancy<br>• Any do not resuscitate decision<br>• Active tuberculosis (TB) as defined by the co-administration of drugs for the treatment of TB<br>• Infection by the human immunodeficiency virus (HIV)<br>• Any primary immunodeficiency<br>• Oral or IV intake of corticosteroids at a daily dose equal or greater than 0.4 mg prednisone or greater the last 15 days.<br>• Any anti-cytokine biological treatment the last one month<br>• Medical history of systemic lupus erythematosus<br>• Medical history of multiple sclerosis or any other demyelinating disorder.<br>• Pregnancy or lactation. Women of child-bearing potential will be screened by a urine pregnancy test before inclusion in the study<br>""","""Main Objective: Our aim is to conduct one trial of personalized immunotherapy in patients with SARS-CoV-2 associated with organ dysfunction and with laboratory findings of macrophage activation syndrome or immune dysregulation. These patients will be selected by the use of a panel of biomarkers and laboratory findings and they will be allocated to immunotherapy treatment according to their needs.  ;Secondary Objective: Not applicable;Primary end point(s): The study primary endpoint is composite and contains the achievement of at least one of the following goals or both goals after 7 days (study visit of day 8):<br>• At least 25% decrease of baseline total SOFA score or increase of the pO2/FiO2 ratio by at least 50%<br>• Clinical improvement of lung involvement<br>Patients discharged from hospital alive before study visit of day 8 are considered achieving the primary endpoint. Patients dying before study visit of day 8 are considered non-achieving the primary endpoint.;Timepoint(s) of evaluation of this end point: Visit study day 8""","""Secondary end point(s): • Comparison of the primary endpoint with historical comparators<br>• Change of SOFA score on day 28<br>• Mortality on day 28<br>• Mortality on day 90<br>• Change of cytokine stimulation between days 0 and 4 <br>• Change of gene expression between days 0 and 4<br>• Change of serum/plasma proteins between days 0 and 4<br>• Classification of immune function of screened patients who are not enrolled in study drug since they do not have MAS or immune dysregulation <br>The above secondary endpoints will also be analyzed separately to study the specific effect of anakinra and of tocilizumab.;Timepoint(s) of evaluation of this end point: Screening<br>Day 4<br>Day 15<br>Day 28<br>Day 90""",
   [Index(36)]
    pub string? Primary_Outcome: Option<String>,
"""Main Objective: Our aim is to conduct one trial of personalized immunotherapy in patients with SARS-CoV-2 associated with organ dysfunction and with laboratory findings of macrophage activation syndrome or immune dysregulation. These patients will be selected by the use of a panel of biomarkers and laboratory findings and they will be allocated to immunotherapy treatment according to their needs.  ;Secondary Objective: Not applicable;Primary end point(s): The study primary endpoint is composite and contains the achievement of at least one of the following goals or both goals after 7 days (study visit of day 8):<br>• At least 25% decrease of baseline total SOFA score or increase of the pO2/FiO2 ratio by at least 50%<br>• Clinical improvement of lung involvement<br>Patients discharged from hospital alive before study visit of day 8 are considered achieving the primary endpoint. Patients dying before study visit of day 8 are considered non-achieving the primary endpoint.;Timepoint(s) of evaluation of this end point: Visit study day 8""",
    [Index(37)]
    pub string? Secondary_Outcomes: Option<String>,
 """Secondary end point(s): • Comparison of the primary endpoint with historical comparators<br>• Change of SOFA score on day 28<br>• Mortality on day 28<br>• Mortality on day 90<br>• Change of cytokine stimulation between days 0 and 4 <br>• Change of gene expression between days 0 and 4<br>• Change of serum/plasma proteins between days 0 and 4<br>• Classification of immune function of screened patients who are not enrolled in study drug since they do not have MAS or immune dysregulation <br>The above secondary endpoints will also be analyzed separately to study the specific effect of anakinra and of tocilizumab.;Timepoint(s) of evaluation of this end point: Screening<br>Day 4<br>Day 15<br>Day 28<br>Day 90""",
     [Index(38)]
    pub string? Bridging_flag: Option<String>,
"""NCT04339712                                       """,
    [Index(39)]
    pub string? Bridged_type: Option<String>,
"""parent    """,
    [Index(40)]
    pub string? Childs: Option<String>,
"""NCT04339712""",
    [Index(41)]
    pub string? type_enrolment: Option<String>,
"""Date trial authorised""",
    [Index(42)]
    pub string? Retrospective_flag: Option<String>,
NULL,
    [Index(43)]
    pub string? results_actual_enrollment: Option<String>,
"""""",
    [Index(44)]
    pub string? results_url_link: Option<String>,
"""""",
    [Index(45)]
    pub string? results_summary: Option<String>,
"""""",
    [Index(46)]
    pub string? results_date_posted: Option<String>,
NULL,
    [Index(47)]
    pub string? results_date_first_pubation: Option<String>,
NULL,
    [Index(48)]
    pub string? results_baseline_char: Option<String>,
    [Index(49)]
    pub string? results_participant_flow: Option<String>,
"""No results available""",
    [Index(50)]
    pub string? results_adverse_events: Option<String>,
"""No results available""",
    [Index(51)]
    pub string? results_outcome_measures: Option<String>,
 """No results available""",
    [Index(52)]
    pub string? results_url_protocol: Option<String>,
 """""",
    [Index(53)]
    pub string? results_IPD_plan: Option<String>,
"""""",
    [Index(54)]
    pub string? results_IPD_description: Option<String>,
"""""",
    [Index(55)]
    pub string? results_date_completed: Option<String>,
 NULL,
    [Index(56)]
    pub string? results_yes_no: Option<String>,
  NULL,
    [Index(57)]
    pub string? Ethics_Status: Option<String>,
"""Approved""",
    [Index(58)]
    pub string? Ethics_Approval_Date: Option<String>,
"""Mar 27 2020 12:00AM""",
    [Index(59)]
    pub string? Ethics_Contact_Name: Option<String>,
"""""",
    [Index(60)]
    pub string? Ethics_Contact_Address: Option<String>,
"""""",
    [Index(61)]
    pub string? Ethics_Contact_Phone: Option<String>,
"""""",
    [Index(62)]
    pub string? Ethics_Contact_Email: Option<String>,
"""""",



*/


#[derive(Debug, serde::Serialize)]
pub struct WHORecord
{
    pub source_id: i32, 
    pub record_date: Option<String>,
    pub sd_sid: String, 
    pub pub_title: Option<String>,
    pub scientific_title: Option<String>,
    pub remote_url: Option<String>,
    pub pub_contact_givenname: Option<String>,
    pub pub_contact_familyname: Option<String>,
    pub pub_contact_email: Option<String>,
    pub pub_contact_affiliation: Option<String>,
    pub scientific_contact_givenname: Option<String>,
    pub scientific_contact_familyname: Option<String>,
    pub scientific_contact_email: Option<String>,
    pub scientific_contact_affiliation: Option<String>,
    pub study_type_orig: Option<String>,
    pub study_type: i32,
    pub study_status_orig: Option<String>,
    pub study_status: i32,
    pub date_registration: Option<String>,
    pub date_enrolment: Option<String>,
    pub target_size: Option<String>,
    pub primary_sponsor: Option<String>,
    pub secondary_sponsors: Option<String>,
    pub source_support: Option<String>,
    pub interventions: Option<String>,
    pub agemin: Option<String>,
    pub agemin_units: Option<String>,
    pub agemax: Option<String>,
    pub agemax_units: Option<String>,
    pub gender: Option<String>,
    pub inclusion_criteria: Option<String>,
    pub exclusion_criteria: Option<String>,
    pub primary_outcome: Option<String>,
    pub secondary_outcomes: Option<String>,
    pub bridging_flag: Option<String>,
    pub bridged_type: Option<String>,
    pub childs: Option<String>,
    pub type_enrolment: Option<String>,
    pub retrospective_flag: Option<String>,
    pub results_actual_enrollment: Option<String>,
    pub results_url_link: Option<String>,
    pub results_summary: Option<String>,
    pub results_date_posted: Option<String>,
    pub results_date_first_pub: Option<String>,
    pub results_url_protocol: Option<String>,
    pub ipd_plan: Option<String>,
    pub ipd_description: Option<String>,
    pub results_date_completed: Option<String>,
    pub results_yes_no: Option<String>,

    pub design_string: Option<String>,
    pub phase_string: Option<String>,

    pub country_list: Option<Vec<String>>,
    pub secondary_ids: Option<Vec<SecondaryId>>,
    pub study_features: Option<Vec<WhoStudyFeature>>,
    pub condition_list: Option<Vec<String>>,
    pub meddra_condition_list: Option<Vec<MeddraCondition>>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[allow(dead_code)]
pub struct SecondaryId
{
    pub source_field: String,
    pub sec_id: String,
    pub processed_id: String,
    pub sec_id_source: usize,
    pub sec_id_type_id: usize,
    pub sec_id_type: String,
}

#[allow(dead_code)]
impl SecondaryId {

    pub fn new(source_field: String, sec_id: String, 
        processed_id: String, sec_id_source: usize,
        sec_id_type_id: usize, sec_id_type: String)
         -> Self {SecondaryId {  
                source_field,
                sec_id,
                processed_id,
                sec_id_source,
                sec_id_type_id,
                sec_id_type,
            }
    }
  
    pub fn new_from_base(source_field: String, sec_id: String,
                sid: SecIdBase)
         -> Self {SecondaryId {  
            source_field,
            sec_id,
            processed_id: sid.processed_id,
            sec_id_source: sid.sec_id_source,
            sec_id_type_id: sid.sec_id_type_id,
            sec_id_type: sid.sec_id_type,
         }
    }

    pub fn clone(&self) -> SecondaryId {
        SecondaryId {
            source_field: self.source_field.clone(),
            sec_id: self.sec_id.clone(),
            processed_id: self.processed_id.clone(),
            sec_id_source: self.sec_id_source,
            sec_id_type_id: self.sec_id_type_id,
            sec_id_type: self.sec_id_type.clone(),
        }
    }
    
}


#[allow(dead_code)]
pub struct SecIdBase
{
    pub processed_id: String,
    pub sec_id_source: usize,
    pub sec_id_type_id: usize,
    pub sec_id_type: String,
} 

#[allow(dead_code)]
impl SecIdBase {

    pub fn new(processed_id: String, sec_id_source: usize,
        sec_id_type_id: usize , sec_id_type: String) -> Self {
        SecIdBase {  
            processed_id,
            sec_id_source,
            sec_id_type_id,
            sec_id_type,
         }
    }
} 


#[derive(Debug, serde::Serialize)]
#[allow(dead_code)]
pub struct WhoStudyFeature
{
    pub ftype_id: usize,
    pub ftype: String,
    pub fvalue_id: usize,
    pub fvalue: String,
}

#[allow(dead_code)]
impl WhoStudyFeature {

    pub fn new(ftype_id: usize, ftype: &str,
        fvalue_id: usize, fvalue: &str) -> Self {
        WhoStudyFeature {
            ftype_id,
            ftype: ftype.to_string(),
            fvalue_id,
            fvalue: fvalue.to_string(),
        }
    }
}


#[derive(Debug, serde::Serialize)]
#[allow(dead_code)]
pub struct MeddraCondition
{
    pub version: String,
    pub level: String,
    pub code: String,
    pub term: String,
    pub soc_code: String,
    pub soc_term: String,
}

#[allow(dead_code)]
impl MeddraCondition {

    pub fn new(version: String, level: String,
        code: String, term: String, soc_code: String, 
        soc_term: String) -> Self {
            MeddraCondition {
            version,
            level,
            code,
            term,
            soc_code,
            soc_term,
        }
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct WHOSummary
{
    pub source_id: i32, 
    pub sd_sid: String, 
    pub title: Option<String>,
    pub remote_url: Option<String>,
    pub study_type: i32,
    pub study_status: i32,
    pub secondary_ids: Option<Vec<SecondaryId>>,
    pub date_registration: Option<String>,
    pub date_enrolment: Option<String>,
    pub results_yes_no: Option<String>,
    pub results_url_link: Option<String>,
    pub results_url_protocol: Option<String>,
    pub results_date_posted: Option<NaiveDate>,
    pub results_date_first_pub: Option<NaiveDate>,
    pub results_date_completed: Option<NaiveDate>,
    pub table_name: String,
    pub country_list: Option<Vec<String>>,
    pub date_last_rev: Option<NaiveDate>,
}
