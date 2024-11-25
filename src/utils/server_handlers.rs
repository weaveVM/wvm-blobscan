use {
    crate::utils::planetscale::{ps_get_blob_data_by_versioned_hash, ps_get_stats},
    axum::{extract::Path, response::Json},
    serde_json::Value,
};

pub async fn handle_weave_gm() -> &'static str {
    "WeaveGM!"
}

pub async fn handle_get_blob(Path(versioned_hash): Path<String>) -> Json<Value> {
    let res: Value = ps_get_blob_data_by_versioned_hash(&versioned_hash).await;
    Json(res)
}

pub async fn handle_get_stats() -> Json<Value> {
    let res: Value = ps_get_stats().await;
    Json(res)
}
