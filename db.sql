DROP TABLE IF EXISTS Blobscan;

CREATE TABLE IF NOT EXISTS Blobscan (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    EthereumBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);