use {
    planetscale_driver::Database,
    serde::{Deserialize, Serialize},
    serde_json::Value,
};
#[derive(Debug, Serialize, Deserialize, Database)]
pub struct PsGetBlockByVersionedHash {
    pub ethereum_block_number: u64,
    pub wvm_archive_txid: String,
    pub versioned_hash: String,
    pub blob_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlobInfo {
    pub ethereum_block_number: u64,
    pub versioned_hash: String,
    pub data: String,
}

impl BlobInfo {
    pub fn from(ethereum_block_number: u64, versioned_hash: String, data: String) -> Self {
        Self {
            ethereum_block_number,
            versioned_hash,
            data,
        }
    }
}
