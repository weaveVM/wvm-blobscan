<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About
`wvm-blobscan` is a data bridge built to archive Ethereum's blob data on [WeaveVM](https://wvm.dev). It grabs blocks containing eip-4844 transactions from [Blobscan](https://blobscan.com) and archive them permanently on WeaveVM..

### Prerequisites & Dependencies

While the core functionality of this ETL codebase can run without web2 component dependencies, this node implementation uses [planetscale](https://planetscale.com) for cloud indexing and [shuttle.rs](https://shuttle.rs) for backend hosting. Check [.env.example](./env.example) to set up your environment variables.

```js
blobscan_pk="" // WeaveVM Blobscan archiver PK

DATABASE_HOST="" // planetscale
DATABASE_USERNAME="" // planetscale
DATABASE_PASSWORD="" // planetscale
```

## Build & Run

```bash
git clone https://github.com/weaveVM/wvm-blobscan.git

cd wvm-blobscan

cargo shuttle run
```
## Workflow

```mermaid
graph TD
    A[Ethereum Block]
    B[Fetch Block Object from Blobscan API]
    C[Archive on WeaveVM]
    D[Index on PlanetScale]

    A --> B
    B --> C
    C --> D
    C -- Archive TxId --> D
```
## Server Methods

### Retrieve full block data and the associated archive txid

```bash
curl -X GET https://blobscan.shuttleapp.rs/v1/block/$ETH_BLOCK_NUMBER
```

Returns the res in the format as in below:

```rs
pub struct GetBlockByIdRes {
    pub ethereum_block_number: u64,
    pub wvm_archiver_txid: String,
    pub data: Value,
}
```

## License
This project is licensed under the [MIT License](./LICENSE)