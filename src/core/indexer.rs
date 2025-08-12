//! Agent's indexer module - uses planetscale to index
//! metadata about the Ethereum blobs stored as ANS-104 offchain
//! DataItems on Load's HyperBEAM S3 node.
//!
//! Functionalities:
//! - Map a blob versioned hash to the corresponding offchain ANS-104 DataItem ID
//! - Get the DataItem ID for a given blob versioned hash
//! - Get indexer stats
//! - Get latest indexed (with found blobs) Ethereum block number
//! - Handles the indexer structs
use crate::core::{constants::FIRST_ETH_L1_EIP4844_BLOCK, env_var::get_env_var};
use anyhow::{anyhow, Error};
use planetscale_driver::{query, Database, PSConnection};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Structure used by the indexer to parse the metadata's pair.
#[derive(Debug, Default, Database, Serialize, Deserialize)]
pub(crate) struct GetVersionedHash {
    /// Blob's versioned hash
    pub versioned_hash: String,
    /// ~s3@1.0 ANS-104 DataItem ID
    pub arweave_txid: String,
}
/// Structure to parse the indexer's stats response.
#[derive(Debug, Default, Database, Serialize, Deserialize)]
pub(crate) struct GetIndexerStats {
    /// Latest processed blob's versioned hash
    pub versioned_hash: String,
    /// Latest processed blob's DataItem ID
    pub arweave_txid: String,
    /// Latest processed Etehreum block number containing blobs
    pub ethereum_block_number: u64,
}
/// Initialize Planetscale connection
async fn ps_init() -> PSConnection {
    let host = get_env_var("DATABASE_HOST").unwrap();
    let username = get_env_var("DATABASE_USERNAME").unwrap();
    let password = get_env_var("DATABASE_PASSWORD").unwrap();

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    conn
}
/// Insert the pair of metadata consisting of blob's versioned hash, its
/// corresponding offchain DataItem ID, and the carrier ethereum block number.
pub async fn insert_kv(
    versioned_hash: &str,
    arweave_txid: &str,
    ethereum_block_number: u64,
) -> Result<(), Error> {
    let client = ps_init().await;

    query("INSERT INTO blobscan_arweave_mapping(versioned_hash, arweave_txid, ethereum_block_number) VALUES(\"$0\", \"$1\", $2)",)
    .bind(versioned_hash)
    .bind(arweave_txid)
    .bind(ethereum_block_number)
    .execute(&client)
    .await.map_err(|e| anyhow!(e.to_string()))?;

    Ok(())
}
/// Get the metadata pair (versioned hash, DataItem ID) for a given blob's versioned hash.
pub async fn get_versioned_hash_value(versioned_hash: &str) -> Result<Value, Error> {
    let client = ps_init().await;

    let query_formatted = format!(
        "SELECT versioned_hash, arweave_txid FROM blobscan_arweave_mapping WHERE versioned_hash = '{versioned_hash}' LIMIT 1;"
    );
    let res: GetVersionedHash =
        query(&query_formatted).fetch_one(&client).await.unwrap_or_default();

    Ok(serde_json::to_value(res)?)
}
/// Get the latest processed Ethereum block number that contains an EIP-4844 tx.
pub async fn get_latest_block_id() -> u64 {
    let client = ps_init().await;
    let res: u64 =
        query("SELECT MAX(ethereum_block_number) FROM blobscan_arweave_mapping LIMIT 1;")
            .fetch_scalar(&client)
            .await
            .unwrap_or(FIRST_ETH_L1_EIP4844_BLOCK);
    res
}
/// Get the indexer's stats - it returns the latest fields that contains
/// a blob (a block with EIP-4844 tx).
pub async fn get_indexer_stats() -> Result<Value, Error> {
    let client = ps_init().await;
    let res: GetIndexerStats = query("SELECT versioned_hash, arweave_txid, ethereum_block_number FROM blobscan_arweave_mapping WHERE ethereum_block_number = (SELECT MAX(ethereum_block_number) FROM blobscan_arweave_mapping) LIMIT 1;").fetch_one(&client).await.unwrap();
    Ok(serde_json::to_value(&res).unwrap())
}
