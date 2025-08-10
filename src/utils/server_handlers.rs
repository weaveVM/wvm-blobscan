use crate::utils::indexer::{get_indexer_stats, get_versioned_hash_value};
use axum::extract::Path;
use axum::response::Json;
use serde_json::Value;

pub async fn handle_route() -> &'static str {
    "load it up [^^]"
}

pub async fn handle_get_blob(Path(versioned_hash): Path<String>) -> Json<Value> {
    let res = get_versioned_hash_value(&versioned_hash).await;
    Json(res.unwrap_or_else(|_| serde_json::json!({"error": "Blob not found"})))
}

pub async fn handle_get_stats() -> Json<Value> {
    let res = get_indexer_stats().await;
    Json(res.unwrap_or_default())
}
