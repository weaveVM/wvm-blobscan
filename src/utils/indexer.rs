
use crate::utils::env_var::get_env_var;
use planetscale_driver::{query, Database, PSConnection};
use serde_json::Value;
use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Database, Serialize, Deserialize)]
pub(crate) struct GetVersionedHash {
    pub versioned_hash: String,
    pub arweave_txid: String
} 

async fn ps_init() -> PSConnection {
    let host = get_env_var("DATABASE_HOST").unwrap();
    let username = get_env_var("DATABASE_USERNAME").unwrap();
    let password = get_env_var("DATABASE_PASSWORD").unwrap();

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    conn
}

pub async fn insert_kv(versioned_hash: &str, arweave_txid: &str) -> Result<(), Error> {
    let client = ps_init().await;

    let res = query("INSERT INTO blobscan_arweave_mapping(versioned_hash, arweave_txid) VALUES(\"$0\", \"$1\")",)
    .bind(versioned_hash)
    .bind(arweave_txid)
    .execute(&client)
    .await.map_err(|e| anyhow!(e.to_string()))?;

    Ok(res)
}

pub async fn get_versioned_hash_value(versioned_hash: &str) -> Result<Value, Error> {
    let client = ps_init().await;

    let query_formatted = format!(
        "SELECT versioned_hash, arweave_txid FROM blobscan_arweave_mapping WHERE versioned_hash = '{}' LIMIT 1;",
        versioned_hash
    );
    let res: GetVersionedHash = query(&query_formatted).fetch_one(&client).await.unwrap_or_default();

    Ok(serde_json::to_value(res)?)
}

