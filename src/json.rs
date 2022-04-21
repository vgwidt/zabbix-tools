use serde_json::Value;
use serde::Deserialize;
use serde::Serialize;

pub const JSONRPC : &str = "2.0";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonRoot {
    pub jsonrpc: String,
    pub method: String,
    pub params: JsonParams,
    pub id: i64,
    pub auth: Value
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonParams {
    pub user: String,
    pub password: String,
}

impl JsonRoot {
    pub fn new(method: String, params: JsonParams, id: i64, auth: Value) -> JsonRoot {
        JsonRoot {
            jsonrpc: JSONRPC.to_string(),
            method,
            params,
            id,
            auth
        }
    }
}

impl JsonParams {
    pub fn new(user: String, password: String) -> Self {
        Self { user, password }
    }
}