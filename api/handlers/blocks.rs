use std::u64::MAX;

use axum::{extract::Path, Extension};
use serde_json::json;
use starknet::core::types::FieldElement;

use crate::{
    db::{read_block_with_txns, read_block_with_txns_from_block_number},
    SharedState,
};

use super::utils::{ApiError, ApiResponse};

pub async fn get_block(
    Path(block_id): Path<FieldElement>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<ApiResponse, ApiError> {
    let state = shared_state.read().await;
    let db = state.db.clone();
    match read_block_with_txns(db, block_id.to_string()).await {
        Ok(Some(v)) => Ok(ApiResponse::JsonData(serde_json::to_value(v).unwrap())),
        Ok(None) => Err(ApiError::NotFound(Some(
            json!({"error": format!("block hash {} not found!!", block_id)}),
        ))),
        Err(_e) => Err(ApiError::InternalServerError),
    }
}

pub async fn get_block_number(
    Path(block_number): Path<u64>,
    Extension(shared_state): Extension<SharedState>,
) -> Result<ApiResponse, ApiError> {
    let state = shared_state.read().await;
    let db = state.db.clone();
    match read_block_with_txns_from_block_number(db.clone(), block_number).await {
        Ok(Some(v)) => Ok(ApiResponse::JsonData(serde_json::to_value(v).unwrap())),
        Ok(None) => Err(ApiError::NotFound(Some(
            json!({"error": format!("block number {} not found!!", block_number)}),
        ))),
        Err(_e) => Err(ApiError::InternalServerError),
    }
}
