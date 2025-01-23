DROP TABLE IF EXISTS Blobscan;

CREATE TABLE IF NOT EXISTS Blobscan (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    EthereumBlockId INT,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE,
    VersionedHash TINYTEXT,
    BlobData LONGTEXT
);

CREATE INDEX idx_versioned_hash ON Blobscan(VersionedHash(16));