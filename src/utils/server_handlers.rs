use crate::utils::s3::get_blob_by_versioned_hash;
use axum::extract::Path;
use axum::response::Json;
use serde_json::Value;

pub async fn handle_weave_gm() -> &'static str {
    "load it up [^^]"
}

pub async fn handle_get_blob(Path(versioned_hash): Path<String>) -> Json<Value> {
    let res = get_blob_by_versioned_hash(&versioned_hash).await;
    Json(res.unwrap_or_else(|| serde_json::json!({"error": "Blob not found"})))
}
