# zabbix-tools

A CLI tool for interacting with Zabbix API built in Rust.  Designed for Zabbix 6.0.

Functions added to test API and add hosts manually or via .csv file.  Assumes SNMPv2 and a number of other host settings.  See /cfg/hosts.csv for the fields that can be adjusted.

## Instructions

Build using cargo from project root directory:
```
cargo build
```

Executable will be located in /target/debug/. 

Copy the config.json file from /cfg/ into the the folder from which you run the executable.  Modify with your server connection details.  API token is not required for testing API.

For .csv import, copy the hosts.csv template from /cfg/ into your root folder and modify and add hosts as needed.  The current template must be used as is.  Adding extra columns will result in an error.

You can post your own JSON request by copying /cfg/request.json to the root directory and entering a valid request.  Be sure to update the auth key.

Note:
Syntax and requirements for Zabbix API seem to change with each version so this may not work for versions other than 6.0  See the Zabbix manual for details: https://www.zabbix.com/documentation/6.0/en/manual/api.

## To-do

Allow for other host settings to be changed such as interface type.

Convert static JSON value text to struct - both work with serde_json but struct will look cleaner and be less error prone.

Prompt for manual input if config.json is not found or contents are invalid (i.e. if initial API test fails or structure cannot be instantiated).

Add additional delimiter inside GroupID and TemplateID.  Currently each device can only be assigned one group and template ID each when added.

Better error handling and debug information for reading in config and hosts files.

Unicode conversion - Zabbix uses \u for formatting unicode for API calls (for example, \u9752 = 青).

Fix unused results and excess mutables.

Other API functions