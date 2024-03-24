use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

fn create_rpc_endpoint() -> String {
    let env_endpoint =
        env::var("RPC_ENDPOINT").unwrap_or("https://free-rpc.nethermind.io/mainnet-juno".into());
    env_endpoint
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCResponse {
    pub jsonrpc: String,
    pub result: Value,
    pub id: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: u16,
}

fn create_rpc_call(method: String, params: Value) -> RPCRequest {
    RPCRequest {
        jsonrpc: "2.0".to_string(),
        method,
        params,
        id: 1,
    }
}

type Result = std::result::Result<Response, reqwest::Error>;

pub async fn call_endpoint(method: String, params: Value) -> Result {
    let endpoint = create_rpc_endpoint();
    let rpc_request = create_rpc_call(method, params);
    println!("{:?}", rpc_request);
    let client = Client::new();
    Ok(client.post(endpoint).json(&rpc_request).send().await?)
}

type BodyResult = std::result::Result<RPCResponse, reqwest::Error>;
pub async fn get_body(response: reqwest::Response) -> BodyResult {
    Ok(response.json().await?)
}
