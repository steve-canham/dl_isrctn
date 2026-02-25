

<h2>Introduction</h2>
This program is designed to download and import ISRCTN data. It uses the ISRCTN API, that provides the data as XML, and generates 
local JSON files from that data. Those files can then be used as the source for importing data into an MDR database.
<br/>During the download, some prelimiinary processing of saecondarty identifiers and link and file data also takes place, to 
make subsequent inmport easier.
<br/>The import takes place in three stages. The first brings the data into a staging database schema (sd), whose structure partly 
reflects that of the original data, partly that of the MDR schema. The second brings it into an 'accumulated data' schema (ad), which fully conforms to the standard MDR schema for source databases. The third attempts to code some of the key entities in the ad data, such as sponsors and conditions under study. The ad data can then be aggregated with data from other MDR sources.


<h2>Using the program</h2>

<h3>Initial Download</h3>
There are three types of download: <br/><br/>
A '-d' flag indicates 'Download Recent'. This identifies and downloads studies edited since a cut-off date, 
usually from the previous week (i.e., the date of the most recent download). It must be 
accompanied by a date parameter in ISO format (e.g. -s "2025-10-18") where the -s flag indicates that the date is the start date.
<br/><br/>
A '-b' flag (Download Between) downloads all records that were last edited between two dates. Calling -b requires two date parmameters, for the start and end dates respectively, e.g. -s "2023-10-01" -e "2023-11-01". Data edited on or after the start date, and up to the end of the day before the end date, is downloaded. This avoids duplications when stepping through the data in the API. It also means that the best time for regular downloading is in the very early morning (European time) as a minimal number of records are missed.
<br/>N.B. The -d procedure also uses a start and end date internally, but in this case the end date is taken as the current date.
<br/><br/>
The '-w' flag (Download Whole Year) can be used to download all records for a specified year, and is designed for bulk download scenarios. It takes a single '-y' parameter that is the year (e.g. -y 2009), and constructs start and end dates for that year, calling the -b routine with those dates.
<br/><br/>
If no download type parameter is provided the program uses '-d' as the default. This means that at its simplest the program can run by just providing a start date -s parameter, and this will download all records edited since that date. In the future, the cut-off date will be taken from a maintained record of downloads, so that not even the cut-off date will be required. This is designed to be used within an environment
when a download of recently edited records takes place weekly.
<br/><br/>
During any download, each period is broken up into periods of 4 days. There does not appear to be a way to rank or 
order results and select from within a returned set, so record sets are returned and processed as a complete block of xml. The program first checks the number of records associated with each 4 day block, however, and if that number is greater than 100 the 4 day period is broken up into separate 
days - i.e. each day's records are downloaded individually. The default number of records provided by the API is 10, so the program requests 100 for each 4 day period, unless it is operating in 'single day' mode, in which case the limit is set as the number last edited on that day.
<br/><br/>
Each xml block is deserialised using the Xml_quick and Serde crates, and the resulting rust struct is processed and serialised back to json and written out.

<h3>Import to the Database</h3>
The import process takes <i>all</i> of the JSON files as input each time. It is fast enough to process all of the files (there are about 28,000 records in the ISRCTN registry) and carry out the latter stages of the import process in about 2 minutes. This greatly simplifies the import process - there is no need to check 'last imported' or 'last downloaded' dates, and all tables in the sd and ad schema tables are recreated each time the import propcess is run.
<br/><br/>
The import command is initiated by the '-a' (import all) flag. Run using cargo, in a rust development environment, the command is therefore simply
<br/> cargo run -- -a
<br/><br/>
The import process uses the downloaded JSON files as input, and reads them in using the Serde crate. It transforms the data into a set of structs corresponding to he database tables (in the staging sd schema), building up vectors of each struct. After a set number of files are read (currently 250) the accumulated objects are stored in the database and the vectors re-initialised. That cycle repeats until all files are processed and al records created. This approach allows data to be stored much more efficiently than storing the data for each file individually.
<br/><br/>
During its initial transfer to the database the data is processed to bring it into a schema that mostly conforms to the MDR schema. Full compliance is not achieved, however, until the second phase of the import, when data in the sd tables is transferred to the ad tables. The transfer is carried out by issueing a series of SQL statements.
<br/><br/>
Some portions of the data in the ad tables then needs to be coded so that the data can be more easily aggregated and searched. For example the organisations listed as study sponsors and funders, or within the affiiations of study leads, receive numeric codes that reference an external 'organisations' table. This allows them to be described and compared consistently, which is not possible using the original string names, because the latter can vary so much between different records. A similar process is applied, as far as possible, to the lists of countries and the lists of conditions being studied. The coding process requires the temporary import of tables from other, 'contextual' databases, as 'Foreign tables'.




