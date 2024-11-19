use foundry_blob_explorers::{BlockResponse, Client};
use alloy_rpc_types_eth::BlockHashOrNumber;
use eyre::{eyre, Result};


pub async fn get_block_by_id(block_id: u32) -> Result<BlockResponse, eyre::Error> {
    let block_id = block_id.to_string();
    let client = Client::mainnet();
    let x: BlockHashOrNumber = block_id.parse().unwrap();

    let block = client.block(x).await;
    println!("{:?}", block);

    match block {
        Ok(block) => Ok(block),
        Err(e) => {
            eprintln!("Error getting block: {:?}", e);
            Err(eyre!("Error getting block: {:?}", e))
        }
    }
}
