//! Module that handles the agent's HTTP API server
//!
//! Endpoints:
//! - GET route: `/`
//! - GET blob's associated ANS-104 DataItem ID: `/v1/blob/:versioned_hash`
//! - GET indexer stats: `/v1/stats`
use crate::core::indexer::{get_indexer_stats, get_versioned_hash_value};
use axum::{extract::Path, response::Json};
use serde_json::Value;
/// Agent's server health check.
pub async fn handle_route() -> &'static str {
    "load it up [^^]"
}
/// Agent's server blob's metadata keys retrieval.
pub async fn handle_get_blob(Path(versioned_hash): Path<String>) -> Json<Value> {
    let res = get_versioned_hash_value(&versioned_hash).await;
    Json(res.unwrap_or_else(|_| serde_json::json!({"error": "Blob not found"})))
}
/// Agent's stats retrieval
pub async fn handle_get_stats() -> Json<Value> {
    let res = get_indexer_stats().await;
    Json(res.unwrap_or_default())
}
