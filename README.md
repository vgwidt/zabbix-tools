# zabbix-tools

A CLI tool for interacting with Zabbix API built in Rust.

API test call implemented.  Add host function has been included, but my instance of Zabbix is rejecting it with an invalid parameter error (-32500).

To-do:
Convert static JSON text to struct - both work with serde_json but struct will look cleaner and be less error prone.
