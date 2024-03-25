use anyhow::{bail, Result};
use reqwest::StatusCode;
use starknet::core::types::{Transaction, TransactionStatus};

use crate::utils::{call_endpoint, get_body};

pub async fn get_block_with_txs(txn_hash: String) -> Result<Transaction> {
    let params = serde_json::json!({ "transaction_hash": txn_hash});
    let response = call_endpoint("starknet_getTransactionByHash".to_string(), params).await?;
    if response.status() == StatusCode::OK {
        let body = get_body(response).await?;
        Ok(serde_json::from_value(body.result).unwrap())
    } else {
        let body = get_body(response).await?;
        bail!(format!("failed to fetch: {:?}", body))
    }
}

pub async fn get_transaction_status(txn_hash: String) -> Result<TransactionStatus> {
    let params = serde_json::json!({ "transaction_hash": txn_hash});
    let response = call_endpoint("starknet_getTransactionStatus".to_string(), params).await?;
    if response.status() == StatusCode::OK {
        let body = get_body(response).await?;
        Ok(serde_json::from_value(body.result).unwrap())
    } else {
        let body = get_body(response).await?;
        bail!(format!("failed to fetch: {:?}", body))
    }
}
