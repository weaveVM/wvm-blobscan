use {
    std::sync::Arc,
    tokio::sync::RwLock,
    utils::{
        blobscan::{get_block_by_id, insert_block},
        constants::FIRST_ETH_L1_EIP4844_BLOCK,
        eth::Ethereum,
        planetscale::{ps_get_archived_block_txid, ps_get_latest_block_id},
    },
};

mod utils;

#[tokio::main]
async fn main() {
    let _ = ps_get_archived_block_txid(19824701).await;
    let block_number = Ethereum::get_latest_eth_block().await.unwrap();
    let block_number = Arc::new(RwLock::new(block_number));
    let reader_block_number = block_number.clone();
    let writer_block_number = block_number.clone();

    let blobscan_insertion = tokio::spawn(async move {
        let mut latest_archived_block = ps_get_latest_block_id().await;
        loop {
            println!("latest archived block id: {}", latest_archived_block);
            let mut block_number = reader_block_number.read().await;
            if *block_number > FIRST_ETH_L1_EIP4844_BLOCK && latest_archived_block < *block_number {
                let block = get_block_by_id(latest_archived_block + 1).await;
                match block {
                    Ok(block) => {
                        println!("block response: {:?}", block);
                        let res = insert_block(block).await;
                        match res {
                            Ok(res) => latest_archived_block += 1,
                            _ => eprintln!("error updating planetscale"),
                        }
                    }
                    Err(e) => {
                        eprintln!("no blobs found in block {}", latest_archived_block + 1);
                        latest_archived_block += 1
                    }
                }
            }
        }
    });

    let eth_block_updater = tokio::spawn(async move {
        loop {
            let mut block_number = writer_block_number.write().await;
            *block_number += Ethereum::get_latest_eth_block().await.unwrap();
            println!("Updated Ethereum Block Number: {}", *block_number);
            tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
        }
    });

    tokio::try_join!(eth_block_updater, blobscan_insertion);
}
