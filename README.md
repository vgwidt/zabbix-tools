# zabbix-tools

A CLI tool for interacting with Zabbix API built in Rust.  Designed for Zabbix 6.

Functions added to test API and add hosts manually or via .csv file.  Assumes SNMPv2 and a number of other host settings.  See /cfg/hosts.csv for the fields that can be adjusted.

There are issues using Authorization bearer (https://support.zabbix.com/browse/ZBX-22952) so this may not work right now.  Library should be switched to offer both credentials and auth key in cases where the authorization header doesn't work.

## Instructions

Build using cargo from project root directory:
```
cargo build
```

Executable will be located in /target/debug/. 

Copy the config.json file from /cfg/ into the the folder from which you run the executable.  Modify with your server connection details.  API token is not required for testing API.

For .csv import, copy the hosts.csv template from /cfg/ into your root folder and modify and add hosts as needed.  The current template must be used as is.  Adding extra columns will result in an error.

Note:
This is designed for Zabbix 6.*.  The API may not be compatible with other versions

## To-do

Allow for other host settings to be changed such as interface type.

Convert static JSON value text to struct - both work with serde_json but struct will look cleaner and be less error prone.

Add additional delimiter inside GroupID and TemplateID.  Currently each device can only be assigned one group and template ID each when added.

Better error handling and debug information for reading in config and hosts files.

Unicode conversion - Zabbix uses \u for formatting unicode for API calls (for example, \u9752 = 青).

Other API functions