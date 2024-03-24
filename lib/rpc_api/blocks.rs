use anyhow::{bail, Error, Result};
use reqwest::StatusCode;
use serde_json::Number;
use starknet::core::types::{
    requests::{BlockHashAndNumberRequest, BlockNumberRequest},
    BlockWithTxHashes,
};

use crate::utils::{call_endpoint, get_body};

pub async fn get_block_with_txs(block_number: u64) -> Result<BlockWithTxHashes> {
    let params = serde_json::json!({
            "block_id": {
            "block_number": block_number
        }
    });
    let response = call_endpoint("starknet_getBlockWithTxHashes".to_string(), params).await?;
    if response.status() == StatusCode::OK {
        let body = get_body(response).await?;
        Ok(serde_json::from_value(body.result).unwrap())
    } else {
        let body = get_body(response).await?;
        bail!(format!("failed to fetch: {:?}", body))
    }
}

pub async fn get_latest_block() -> Result<u64> {
    let params = serde_json::json!({});
    let response = call_endpoint("starknet_blockNumber".to_string(), params).await?;
    if response.status() == StatusCode::OK {
        let body = get_body(response).await?;
        Ok(serde_json::from_value(body.result).unwrap())
    } else {
        let body = get_body(response).await?;
        bail!(format!("failed to fetch: {:?}", body))
    }
}
