

<h2>Introduction</h2>
This program is designed to download and import ISRCTN data. It uses the ISRCTN API, that provides the data as XML, and generates local JSON files from that data. Those files can then be used as the source for importing data into an MDR database.
<br/>During the download, some prelimiinary processing of secondarty identifiers and link and file data also takes place, to make subsequent inmport easier.
<br/>The import takes place in three stages. The first brings the data into a staging database schema (sd), whose structure partly reflects that of the original data, partly that of the MDR schema. The second brings it into an 'accumulated data' schema (ad), which fully conforms to the standard MDR schema for source databases. The third attempts to code some of the key entities in the ad data, such as sponsors and conditions under study. The ad data can then be aggregated with data from other MDR sources.

<h2>Using the program</h2>

<h3>Initial Download</h3>

The program uses the API of the ISRCTN web site (https://www.isrctn.com/)
to download data about the trials registered on the site.<br/>
There are four types of download:<br/><br/>
'Recent' (-r in the CLI) identifies and downloads studies edited since a cut-off date, 
usually from the previous week (i.e., the date of the most recent download). It must be 
accompanied by a date parameter in ISO format (e.g. -s "2025-10-18"), or be able to obtain such
a parameter from the database record of previous downloads of the same type. 
If no parameters are provided to the program, '-r' is applied as the default operation. This 
allows weekly updates to the local json file store to be iomplemented easily.
<br/><br/>
'UdBetweenDates' (-b in the CLI) downloads all records that were last edited
between two dates. <br/>
'CrBetweenDates' (-c in the CLI) downloads all records that were created (more exactly,
applied for inclusion in ISRCTN) between two dates. <br/>
In both of the cases above the two date parameters must be supplied (as  -s and -t parameters) in ISO format.<br/><br/>
'ByYear' (-y in the CLI) can be used to download all studies that applied for inclusion 
to ISRCTN in a specified year, and is designed for bulk download scenarios, such as 
rebuilding the whole ISRCTN dataset from scratch.<br/><br/>
In fact all procedures work in a similar way and need a start and end date, but in the case of 
type 'Recent' the end date is taken as the current date, and in the case of 'ByYear' the dates are the first date of the year, and the first date of the following year.<br/><br/>
During any download, each period is broken up into periods of 4 days. The API does not appear to offer a way to rank or order results and select from within a returned set, so record sets are returned and processed as a complete block of xml. The program first checks the number of records associated with each 4 day block, however, and if that number is greater than 100 the 4 day period is broken up into separate days - i.e. each day's records are downloaded individually. The default number of records provided by the API is 10, so the program requests 100 for each 4 day period, unless it is operating in 'single day' mode, in which case the limit is set as the number last edited / created on that day.
Each xml block is deserialised using the Xml_quick and Serde crates, and the resulting rust struct is processed and serialised back to json and written out.

<h3>Import to the Database</h3>
The import process can take either recently downloaded (-i) or <i>all</i> (-I) of the JSON files as input each time. <br/>
Running -i imports data from any json files downloaded on or after the date of the last import process, and puts this data into the sd staging schema. It is then used to replace the corresponding records in the ad schema. The sd schema is therefore re-created, but the ad schema is updated.<br/>
Running -I recreates both the sd and ad schema and imports the data from all the json files. It is fast enough to process all of the files (there are about 28,000 records in the ISRCTN registry) and carry out the latter stages of the import process in about 2 minutes, so the usual practice is to use -I. This simplifies the process - there is no need to check 'last imported' and 'last downloaded' dates. It can be run using cargo, in a rust development environment, by
<br/> cargo run -- -I<br/><br/>
Either import process uses the downloaded JSON files as input, and reads them in using the Serde crate. It transforms the data into a set of structs corresponding to he database tables (in the staging sd schema), building up vectors of each struct. After a set number of files are read (currently 250) the accumulated objects are stored in the database and the vectors re-initialised. That cycle repeats until all files are processed and al records created. This approach allows data to be stored much more efficiently than storing the data for each file individually.
<br/><br/>
During its initial transfer to the database the data is processed to bring it into a schema that mostly conforms to the MDR schema. Full compliance is not achieved, however, until the second phase of the import, when data in the sd tables is transferred to the ad tables. The transfer is carried out by a series of SQL statements.

<h3>Coding the Database</h3>
Some portions of the data in the ad tables then needs to be coded so that the data can be more easily aggregated and searched. For example the organisations listed as study sponsors and funders, or within the affiiations of study leads, receive numeric codes that reference an external 'organisations' table. This allows them to be described and compared consistently, which is not possible using the original string names, because the latter can vary so much between different records. A similar process is applied, as far as possible, to the lists of countries and the lists of conditions being studied. The coding process requires the temporary import of tables from other, 'contextual' databases, as 'Foreign tables'.




