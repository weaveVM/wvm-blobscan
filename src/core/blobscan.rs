use bundles_rs::ans104::data_item::DataItem;
use bundles_rs::ans104::tags::Tag;
use bundles_rs::crypto::arweave::ArweaveSigner;
use anyhow::Error;
use reqwest;
use serde_json::{self, Value};
use crate::core::env_var::get_env_var;
use crate::core::types::BlobInfo;

pub async fn get_blobs_versioned_hashes_of_block(
    block_id: u64,
) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://api.blobscan.com/blocks/{}?type=canonical",
        block_id
    );
    let req: Value = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    let versioned_hashes: Vec<String> = req
        .pointer("/transactions")
        .and_then(|txs| txs.as_array())
        .map(|txs| {
            txs.iter()
                .filter_map(|tx| tx.pointer("/blobs"))
                .filter_map(|blobs| blobs.as_array())
                .flat_map(|blobs| {
                    blobs
                        .iter()
                        .filter_map(|blob| blob.pointer("/versionedHash"))
                        .filter_map(|hash| hash.as_str())
                        .map(String::from)
                        .collect::<Vec<String>>()
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(versioned_hashes)
}

async fn get_blob_data(versioned_hash: &str) -> Result<String, Error> {
    let url = format!("https://api.blobscan.com/blobs/{}/data", versioned_hash);
    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await
        .unwrap_or_default();
    Ok(res)
}

pub async fn get_blobs_of_block(block_id: u64) -> Result<Vec<BlobInfo>, Error> {
    let versioned_hashes = get_blobs_versioned_hashes_of_block(block_id)
        .await
        .unwrap_or_default();
    let mut res: Vec<BlobInfo> = Vec::new();
    for hash in versioned_hashes {
        let blob_data = get_blob_data(&hash).await.unwrap();

        let blob = BlobInfo {
            ethereum_block_number: block_id as u64,
            versioned_hash: hash,
            data: blob_data,
        };

        res.push(blob);
    }

    Ok(res)
}

pub fn serialize_blobscan_block(block: &BlobInfo) -> Result<(Vec<u8>, String), Error> {
    let data = serde_json::to_vec(&block)?;
    let tags = vec![
        Tag::new("content-type", "application/json"),
        Tag::new("Protocol", "Load-Blobscan"),
    ];
    let jwk = get_env_var("blobscan_agent_pk")?;
    let signer = ArweaveSigner::from_jwk_str(&jwk).unwrap();
    let dataitem = DataItem::build_and_sign(&signer, None, None, tags, data).unwrap();
    Ok((dataitem.to_bytes().unwrap(), dataitem.arweave_id()))
}

pub async fn send_blob_to_blobscan(blob_hash: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let key = get_env_var("blobscan_api_key").unwrap();
    let response = client
        .post("https://api.blobscan.com/blobs/weavevm-references")
        .header("Authorization", key)
        .json(&serde_json::json!({
            "blobHashes": [blob_hash]
        }))
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());

    Ok(())
}

// pub async fn send_blobs_to_blobscan(blob_hash: Vec<&str>) -> Result<(), Error> {
//     let client = reqwest::Client::new();
//     let key = get_env_var("blobscan_api_key").unwrap();
//     let response = client
//         .post("https://api.blobscan.com/blobs/weavevm-references")
//         .header("Authorization", key)
//         .json(&serde_json::json!({
//             "blobHashes": blob_hash
//         }))
//         .send()
//         .await?;

//     println!("Status: {}", response.status());
//     println!("Headers: {:?}", response.headers());

//     Ok(())
// }

// pub async fn backfill_blobscan_blobs(page: u32) {
//     for page in 0..page {
//         let mut temp_hashes: Vec<&str> = vec![];
//         let mut i = 0;
//         let batch = ps_get_all_versioned_hashes_paginated(page).await;

//         for el in &batch {
//             let hash = &el.versioned_hash;
//             temp_hashes.push(&hash);
//         }
//         println!("Fetched {} blobs", temp_hashes.len());
//         println!("sending blobs on 10ks to blobscan");

//         while i < 100_000 {
//             println!("{} {}", i, i + 10_000);
//             let _ = send_blobs_to_blobscan(temp_hashes[i..i + 10_000].to_vec()).await.unwrap();
//             i += 10_000;
//         }
//      }
// }
