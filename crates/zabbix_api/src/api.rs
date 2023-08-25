use reqwest::{Client, Error, header};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::models::ZabbixResponse;

pub const JSONRPC : &str = "2.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    jsonrpc: String,
    method: String,
    params: Value,
    id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub query: Method,
    pub params: Value,
}

impl Query {
    pub fn new(query: Method) -> Self {
        Query {
            query: query,
            params: json!({}),
        }
    }
    
    // Add search key to query with specified key and value
    // TODO: It should be limited to only certain methods
    pub fn add_search(mut self, key: &str, value: &str) -> Self {
        let mut search = self.params["search"].clone();
        search[key] = json!(value);
        self.params["search"] = search;
        self
    }

    // Set the output parameter for the query (i.e. hostid)
    pub fn set_output(mut self, output: Vec<&str>) -> Self {
        let output_values: Vec<Value> = output.into_iter().map(|s| json!(s)).collect();
        self.params["output"] = json!(output_values);
        self
    }
}


// Zabbix API methods
#[derive(Debug, Serialize, Deserialize)]
pub enum Method {
    HostCreate,
    HostGet,
    HostUpdate,
    APIInfoVersion,
}

impl Method {
    pub fn to_string(&self) -> String {
        match self {
            Method::HostCreate => "host.create".to_string(),
            Method::HostGet => "host.get".to_string(),
            Method::HostUpdate => "host.update".to_string(),
            Method::APIInfoVersion => "apiinfo.version".to_string(),
        }
    }

    // Set whether each method requires authentication or not
    // Zabbix API does not allow methods that do not require authentication to be called with authentication when using a token
    pub fn requires_auth(&self) -> bool {
        match self {
            Method::HostCreate => true,
            Method::HostGet => true,
            Method::HostUpdate => true,
            Method::APIInfoVersion => false,
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

    // Request method
    // pub async fn request(&self, method: Method, params: Value, ) -> Result<ZabbixResponse, Error> {
    //     self.request_internal(method.to_string(), params, method.requires_auth()).await
    // }
    
    // Custom request, for methods not implemented in the library
    pub async fn custom_request(&self, method: &str, params: Value, require_auth: bool) -> Result<ZabbixResponse, Error> {
        self.request_internal(method.to_string(), params, require_auth).await
    }

    // New request that takes Query object
    pub async fn request_query(&self, query: Query) -> Result<ZabbixResponse, Error> {
        self.request_internal(query.query.to_string(), query.params, query.query.requires_auth()).await
    }
    
    // Shared request internals
    async fn request_internal(&self, method: String, params: Value, require_auth: bool) -> Result<ZabbixResponse, Error> {

        // Create the request object
        let request = Request {
            jsonrpc: JSONRPC.to_string(),
            method,
            params,
            id: 1,
        };
        
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json-rpc"));

        if require_auth {
            headers.insert("Authorization", format!("Bearer {}", self.auth_token).parse().unwrap());
        }
    
        let client = Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .http1_title_case_headers() // Zabbix API requires title case authorization header
            .build()?;
    
        let response = client.post(&self.base_url)
            .json(&request)
            .send()
            .await?;
            
            let json_response = response.json().await?;
    
        let zabbix_response: ZabbixResponse = serde_json::from_value(json_response).unwrap();

        Ok(zabbix_response)
    }

}

