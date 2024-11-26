use {
    planetscale_driver::Database,
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Serialize, Deserialize, Database)]
pub struct PsGetBlockByVersionedHash {
    pub ethereum_block_number: u64,
    pub wvm_archive_txid: String,
    pub versioned_hash: String,
    pub blob_data: String,
}

#[derive(Debug, Serialize, Deserialize, Database)]
pub struct PsGetLatestArchivedBlock {
    pub last_archived_eth_block: u64,
    pub blob_versioned_hash: String,
    pub wvm_archive_txid: String,
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
