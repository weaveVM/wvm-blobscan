use {
    axum::{routing::get, Router},
    std::sync::Arc,
    tokio::sync::RwLock,
    utils::{
        blobscan::{get_block_by_id, insert_block}, // backfill_blobscan_blobs
        constants::FIRST_ETH_L1_EIP4844_BLOCK,
        eth::Ethereum,
        planetscale::get_latest_block_id,
        server_handlers::{handle_get_blob, handle_get_stats, handle_weave_gm},
    },
};

mod utils;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // load secrets from Shuttle.toml into env var;
    secrets.into_iter().for_each(|(key, val)| {
        std::env::set_var(key, val);
    });
    let router = Router::new()
        .route("/", get(handle_weave_gm))
        .route("/v1/blob/:versioned_hash", get(handle_get_blob))
        .route("/v1/stats", get(handle_get_stats));

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

    Ok(router.into())
}
