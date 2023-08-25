# zabbix-tools

A CLI tool for interacting with Zabbix API built in Rust.  Designed for Zabbix 6. 

This was quickly written up for a couple of mass host update features, there is not much for error checking.

Features:
* Bulk Add Hosts via CSV
* Bulk update host names
* Send custom JSON requests

Adding hosts can be done manually or via .csv file.  Assumes SNMPv2 and a number of other host settings.  See /cfg/hosts.csv for the fields that can be adjusted.

An API token is required for requests that require authentication.

## Instructions

Build using cargo from project root directory:
```
cargo build
```

Executable will be located in /target/debug/. 

Copy the config.json file from /cfg/ into the the folder from which you run the executable.  Modify with your server connection details.  API token is not required for testing API.

For .csv import, copy the hosts.csv template from /cfg/ into your root folder and modify and add hosts as needed.  The current template must be used as is.  Adding extra columns will result in an error.

Note:
This is designed for Zabbix 6.*.  The API may not be compatible with other version.