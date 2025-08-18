## About

`blobscan-agent` is a hybrid data agent built at the intersection of ao compute, HyperBEAM's temporal S3 data storage, and Arweave's permanent storage.

The agent stores Ethereum's blobs temporarily on the [~s3@1.0 HyperBEAM device](https://github.com/loadnetwork/load_hb/tree/s3-edge/native/s3_nif), serialized as [ANS-104 DataItems](https://github.com/ArweaveTeam/arweave-standards/blob/master/ans/ANS-104.md). Based on need/demand, DataItems can be deterministically pushed to Arweave while maintaining integrity and provenance.

DataItems stored on the `~s3@1.0` device can be retrieved from the Hybrid Gateway as if they are Arweave txs: https://github.com/loadnetwork/load_hb/tree/s3-edge/native/s3_nif#hybrid-gateway  

## Build & Run

```bash
git clone https://github.com/loadnetwork/blobscan-agent

cd blobscan-agent

cargo +nightly fmt && cargo clippy --all-targets --all-features && cargo run --release
```

## Agent Server Methods

### Retrieve blob versioned hash and the associated ANS-104 dataitem id by versioned hash

```bash
curl -X GET https://load-blobscan-agent.load.network/v1/blob/$BLOB_VERSIONED_HASH
```

### Retrieve Indexer stats

```bash
curl -X GET https://load-blobscan-agent.load.network/v1/stats
```

### Agent's server info

```bash
curl -X GET https://load-blobscan-agent.load.network/v1/info
```

## License
This project is licensed under the [MIT License](./LICENSE)