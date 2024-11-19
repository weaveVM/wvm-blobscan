use crate::utils::{
    blobscan::{get_block_by_id, process_blobscan_block},
    eth::get_latest_eth_block
};

mod utils;

#[tokio::main]
async fn main() {
    let block_id = get_latest_eth_block().await.unwrap();
    println!("block id: {}", block_id);
    let block = get_block_by_id(block_id).await;
    process_blobscan_block(block.as_ref().ok().unwrap().clone()).unwrap();
    match block {
        Ok(block) => println!("{:#?}", block.transactions.len()),
        _ => eprint!("error")
    }
}

