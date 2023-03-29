Raspberry pi 2 pinouts: https://www.raspberry-pi-geek.com/howto/GPIO-Pinout-Rasp-Pi-1-Model-B-Rasp-Pi-2-Model-B

SQLite3 commands:
`sqlite3 <db_file_path>/<db_name>.db`

`.schema` => prints out the table schema
`.help` => you know what it is for ;-)
`.exit` => does what it says
`.output <path>/<filename>.sql` => sets the path for the consequent commands output to be recorded in 
`.dump` prints out the table schema and all of the data with insert statements

Sample query:
`select id, datetime(timestamp, 'localtime'), notes from logs;`

Deployment steps:
1. Push the changes to the Repo (git push origin master)
2. Pull it on the server (git pull origin master) (legacypi - 252)
3. Build the project and move the binary to `/usr/local/bin/`
4. Update the environment variables if necessary from the Supervisor config
5. Run `reread` and `update` commands on the supervisorctl