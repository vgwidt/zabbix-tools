use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::ZabbixResponse;

pub const JSONRPC : &str = "2.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    jsonrpc: String,
    method: String,
    params: Value,
    id: usize,
}

// Zabbix API methods
pub enum Method {
    HostGet
}

impl Method {
    pub fn to_string(&self) -> String {
        match self {
            Method::HostGet => "host.get".to_string(),
        }
    }
}

// Connection parameters for the Zabbix API
#[derive(Debug, Serialize, Deserialize)]
pub struct ZabbixApi {
    base_url: String,
    auth_token: String,
}

impl ZabbixApi {
    pub fn new(base_url: &str, auth_token: &str) -> Self {
        ZabbixApi {
            base_url: base_url.to_string(),
            auth_token: auth_token.to_string(),
        }
    }

    pub async fn request(&self, method: &str, params: Value) -> Result<ZabbixResponse, Error> {
        let request = Request {
            jsonrpc: JSONRPC.to_string(),
            method: method.to_string(),
            params,
            id: 1,
        };

        let client = Client::new();

        let response = client.post(&self.base_url)
        .header("Content-Type", "application/json-rpc")
        .header("Authorization", format!("Bearer {}", self.auth_token))
        .json(&request)
        .send()
        .await?;

        let zabbix_response: ZabbixResponse = response.json().await?;

        Ok(zabbix_response)
    }
}

