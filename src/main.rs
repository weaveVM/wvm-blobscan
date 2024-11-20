use serde::Serialize;

use crate::utils::{
    blobscan::{get_block_by_id, process_blobscan_block},
    eth::Ethereum,
    constants::FIRST_ETH_L1_EIP4844_BLOCK,
    planetscale::ps_archive_block,
    wvm::send_wvm_calldata
};

mod utils;


#[tokio::main]
async fn main() {
    // let block_id = get_latest_eth_block().await.unwrap();
    // println!("block id: {}", block_id);
    let block = get_block_by_id(FIRST_ETH_L1_EIP4844_BLOCK).await;
    let wvm_data_input = process_blobscan_block(block.as_ref().ok().unwrap().clone()).unwrap();
    let raw_data = serde_json::to_string(&block.unwrap()).unwrap();
    let wvm_txid = send_wvm_calldata(wvm_data_input).await.unwrap();
    let _ = ps_archive_block(&FIRST_ETH_L1_EIP4844_BLOCK, &wvm_txid, &raw_data).await.unwrap();
    // match block {
    //     Ok(block) => println!("{:#?}", block),
    //     _ => eprint!("error")
    // }
}

