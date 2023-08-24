use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZabbixResponse {
    pub jsonrpc: String,
    pub result: Value,
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub hostid: Option<String>,
    pub host: Option<String>,
    pub description: Option<String>,
    pub flags: Option<i32>,
    pub inventory_mode: Option<i32>,
    pub ipmi_authtype: Option<i32>,
    pub ipmi_password: Option<String>,
    pub ipmi_privilege: Option<i32>,
    pub ipmi_username: Option<String>,
    pub maintenance_from: Option<i64>,
    pub maintenance_status: Option<i32>,
    pub maintenance_type: Option<i32>,
    pub maintenanceid: Option<String>,
    pub name: Option<String>,
    pub proxy_hostid: Option<String>,
    pub status: Option<i32>,
    pub tls_connect: Option<i32>,
    pub tls_accept: Option<i32>,
    pub tls_issuer: Option<String>,
    pub tls_subject: Option<String>,
    pub tls_psk_identity: Option<String>,
    pub tls_psk: Option<String>,
    pub active_available: Option<i32>,
}
