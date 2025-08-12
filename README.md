## Build & Run

```bash
git clone https://github.com/loadnetwork/blobscan-agent

cd blobscan-agent

cargo +nightly fmt && cargo clippy --all-targets --all-features && cargo run --release
```

## Server Methods

### Retrieve blob versioned hash and the associated ANS-104 dataitem id by VersionedHash

```bash
curl -X GET https://blobscan.load.rs/v1/blob/$BLOB_VERSIONED_HASH
```


### Retrieve Indexer stats

```bash
curl -X GET https://blobscan.load.rs/v1/stats
```

## License
This project is licensed under the [MIT License](./LICENSE)