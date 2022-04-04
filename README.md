# zabbix-tools

A CLI tool for interacting with Zabbix API built in Rust.  Designed for Zabbix 6.0.

API test call implemented.  Add host function has been included for prototyping, but largely relies on static JSON.

Copy the config.json file from /cfg/ into the root of the project folder and modify with your server connection details.  API token is not required for testing API.

To-do:
Convert static JSON value text to struct - both work with serde_json but struct will look cleaner and be less error prone.
Prompt for manual input if config.json is not found or contents are invalid (i.e. if initial API test fails or structure cannot be instantiated).
Add additional delimiter inside GroupID and TemplateID.  Currently each device can only be assigned one group and template ID each when added.
Unicode conversion - Zabbix uses \u for formatting unicode for API calls (for example, \u9752 = 青).

Note:
Syntax and requirements for Zabbix API seem to change with each version so this likely will not work for versions prior to 6.0.  See the Zabbix manual for details: https://www.zabbix.com/documentation/6.0/en/manual/api.