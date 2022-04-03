# zabbix-tools

A CLI tool for interacting with Zabbix API built in Rust.

API test call implemented.  Add host function has been included for prototyping, but largely relies on static JSON.

Copy the config.json file from /cfg/ into the root of the project folder and modify with your server connection details.  API token is not required for testing API.

To-do:
-Convert static JSON text to struct - both work with serde_json but struct will look cleaner and be less error prone.
-Prompt for manual input if config.json is not found or contents are invalid (i.e. if initial API test fails or structure cannot be instantiated).
