use {
    crate::utils::planetscale::ps_get_archived_block_txid,
    axum::{extract::Path, response::Json},
    serde_json::Value,
};

pub async fn handle_weave_gm() -> &'static str {
    "WeaveGM!"
}

pub async fn handle_get_block(Path(id): Path<u64>) -> Json<Value> {
    let res: Value = ps_get_archived_block_txid(id).await;
    Json(res)
}
