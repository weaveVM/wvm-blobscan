use {
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionedHashOnly {
    pub versioned_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl std::fmt::Display for BlobInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block: {}, Hash: {}, Data: {}",
            self.ethereum_block_number, self.versioned_hash, self.data
        )
    }
}
