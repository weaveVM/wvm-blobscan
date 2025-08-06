use {
    axum::{routing::get, Router},
    std::sync::Arc,
    tokio::sync::RwLock,
    utils::{
        blobscan::get_blobs_of_block,
        constants::FIRST_ETH_L1_EIP4844_BLOCK,
        env_var::load_env_vars,
        eth::Ethereum,
        s3::{get_latest_block_id, insert_block},
        server_handlers::{handle_get_blob, handle_weave_gm},
    },
};

mod utils;

#[tokio::main]
async fn main() {
    load_env_vars();
    let router = Router::new()
        .route("/", get(handle_weave_gm))
        .route("/v1/blob/:versioned_hash", get(handle_get_blob));

    let block_number = Ethereum::get_latest_eth_block().await.unwrap();
    let block_number = Arc::new(RwLock::new(block_number));
    let reader_block_number = block_number.clone();
    let writer_block_number = block_number.clone();

    // backfill_blobscan_blobs(3).await;

    let blobscan_insertion = tokio::spawn(async move {
        let mut latest_archived_block = get_latest_block_id().await;
        loop {
            println!("latest archived block id: {}", latest_archived_block);
            let mut block_number = reader_block_number.read().await;
            if *block_number > FIRST_ETH_L1_EIP4844_BLOCK && latest_archived_block < *block_number {
                let target_block_id = latest_archived_block + 1;
                let blobs = get_blobs_of_block(target_block_id).await;
                println!("GOT BLOBS OF BLOCK #{:?}", target_block_id);
                match blobs {
                    Ok(blobs) => {
                        println!("INSERTING: {:?} BLOBS", blobs.len());
                        println!("BLOBS: {:?}\n\n\n", blobs);
                        let res = insert_block(target_block_id, blobs).await;
                        match res {
                            Ok(_) => latest_archived_block += 1,
                            Err(e) => eprintln!("error updating s3: {}", e),
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
            *block_number = Ethereum::get_latest_eth_block().await.unwrap();
            println!("Updated Ethereum Block Number: {}", *block_number);
            tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
        }
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, router).await.unwrap();
}
