use {
    planetscale_driver::Database,
    serde::{Deserialize, Serialize},
    serde_json::{from_str, Value},
};
#[derive(Debug, Serialize, Deserialize, Database)]
pub struct PsGetBlockById {
    pub ethereum_block_number: u64,
    pub wvm_archive_txid: String,
    pub raw_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockByIdRes {
    pub ethereum_block_number: u64,
    pub wvm_archiver_txid: String,
    pub data: Value,
}

impl GetBlockByIdRes {
    pub fn from_ps_result(obj: PsGetBlockById) -> Result<Self, serde_json::Error> {
        let data = serde_json::from_str(&obj.raw_data).unwrap();
        Ok(Self {
            data,
            ethereum_block_number: obj.ethereum_block_number,
            wvm_archiver_txid: obj.wvm_archive_txid,
        })
    }
}
