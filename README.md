

<h2>Introduction</h2>
This program is designed to download ISRCTN data, using the ISRCTN API, that provides the data as XML. The
download generates JSON files, stored locally, which can thenm be used for further processing, including
import into a database. The JSON files include most of the same data points as the original XML, minus the 
internal ISRCTN identifiers, but with a structure more easily assimilated into a database. Some prelimiinary 
processing of saecondarty identifiers also takes place, to improve the characterisation of these identifiers.

<h2>Using the program</h2>
There are three types of download: <br/>

identifies and downloads studies edited since a cut-off date, 
usually from the previous week (i.e., the date of the most recent download). It must be 
accompanied by a date parameter in ISO format (e.g. -s "2025-10-18")
<br/><br/>
 downloads all records that were last edited
between two dates. Running against this type in batches allows all ISRCTN records to be
re-downloaded, if and when necessary. Calling -t 115 requires two date
parmameters, for the start and end dates respectively, e.g. -s "2023-10-01" -e "2023-10-31"
<br/><br/>
Both procedures use a start and end date internally, but in the case of type 111 the
end date is taken as the current date.
<br/><br/>

can be used to download all records for a specified year,
and is designed for bulk download scenarios. It takes a single parameter (e.g. -y 2009),
and constructs start and end dates for that year, calling the -t 115 routine with those dates.
It therefore wraps the -t 115 download type.
<br/><br/>
If no type parameter is provided the program uses 111 as the default. This means that at its 
simplest the program can run by just providing a start date -s parameter, and this will 
download all records edited since that date.
<br/><br/>
In the future, the cut-off date will be taken from a maintained record of downloads, so that 
not even the cut-off date will be required. This is designed to be used within an environment
when a download of recently edited records takes place weekly.

<h2>Program functioning</h2>
In all cases the program will have start and end dates, whether provided explicitly or as derived 
automatically. Parameters such as the location of the json files and a log file, plus database 
connection parameters, are obtained from a configuration file (app_config.toml). The download 
process downloads all records with last edited dates >= start date and < end date. 
Records last edited on the start date are therefore included, but the set only includes studies
last edited up to the (end date - 1). This avoids duplications when stepping through the data 
in the API. It also means that the best time for regular downloading is in the very 
early morning (European time) as a minimal number of records are missed.
<br/><br/>
Each period is broken up into periods of 4 days. There does not appear to be a way to rank or 
order results and select from within a returned set, so record sets are returned and processed as
a complete block of xml. The program first checks the number of records associated with each 4 day
block, however, and if that number is greater than 100 the 4 day period is broken up into separate 
days - i.e. each day's records are downloaded individually. The default number of records provided 
by the API is 10, so the program requests 100 for each 4 day period, unless it is operating in
'single day' mode, in which case the limit is set as the number last edited on that day.
<br/><br/>
Each xml block is deserialised using the Xml_quick and Serde crates, and the resulting rust struct
is processed and serialised back to json and written out.
