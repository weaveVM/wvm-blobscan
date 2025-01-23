DROP TABLE IF EXISTS Blobscan;

CREATE TABLE IF NOT EXISTS Blobscan (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    EthereumBlockId INT,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE,
    VersionedHash TINYTEXT,
    BlobData LONGTEXT
);

CREATE INDEX idx_versioned_hash ON Blobscan(VersionedHash(16));
CREATE INDEX idx_ethereum_block_id ON Blobscan(EthereumBlockId);
CREATE INDEX idx_wvm_txid ON Blobscan(WeaveVMArchiveTxid);