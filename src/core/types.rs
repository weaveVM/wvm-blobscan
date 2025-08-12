//! A Module that contains some of the repositories structs.
//! Data types mostly associated with the Ethereum blobs
use serde::{Deserialize, Serialize};

/// Used by the agent's indexer to handle the
/// versioned hash DB response.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionedHashOnly {
    /// Blob's versioned hash
    pub versioned_hash: String,
}
/// Blob's main data structure. Represents the
/// ANS-104 DataItem data field.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobInfo {
    /// EIP-4844 tx's block number
    pub ethereum_block_number: u64,
    /// Blob's versioned hash
    pub versioned_hash: String,
    /// Blob's hex data field
    pub data: String,
}

impl BlobInfo {
    /// Creates a BlobInfo instance from given struct field params.
    pub fn from(ethereum_block_number: u64, versioned_hash: String, data: String) -> Self {
        Self { ethereum_block_number, versioned_hash, data }
    }
}

impl std::fmt::Display for BlobInfo {
    /// BlobInfo custom display formatted function
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block: {}, Hash: {}, Data: {}",
            self.ethereum_block_number, self.versioned_hash, self.data
        )
    }
}
