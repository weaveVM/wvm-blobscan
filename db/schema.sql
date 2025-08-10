CREATE TABLE blobscan_arweave_mapping (
    id INT AUTO_INCREMENT PRIMARY KEY,
    versioned_hash VARCHAR(66) NOT NULL UNIQUE,
    arweave_txid VARCHAR(43) NOT NULL,
    ethereum_block_number INT NOT NULL,
    INDEX idx_versioned_hash (versioned_hash),
    INDEX idx_arweave_txid (arweave_txid),
    INDEX idx_ethereum_block_number (ethereum_block_number)
);
