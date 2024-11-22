use {
    crate::utils::planetscale::ps_get_blob_data_by_versioned_hash,
    axum::{extract::Path, response::Json},
    serde_json::Value,
};

pub async fn handle_weave_gm() -> &'static str {
    "WeaveGM!"
}

pub async fn handle_get_block(Path(versioned_hash): Path<String>) -> Json<Value> {
    let res: Value = ps_get_blob_data_by_versioned_hash(&versioned_hash).await;
    Json(res)
}
