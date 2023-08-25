# zabbix_api
A Zabbix API wrapper for Rust

This has been tested for version 6.4 only.

No handling is provided for the result object returned by Zabbix, so the JSON value will need to be handled (i.e. using serde_json::Value).  I doubt it will ever be supported as there are far too many objects and they change with every version.

Only the following methods are supported so far:
* host.create
* host.get
* host.update
* apiinfo.version

The library does allow for custom requests to be made, where the method (string) and parameters (serde_json::Value) can be specified.

It only supports the use of the Authorization header for authentication at this time.  Zabbix only allows for the header to be used for methods that require authentication, so it accordingly will send the headers depending on the method used.