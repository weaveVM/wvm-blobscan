use {
    crate::utils::{
        env_var::get_env_var,
        planetscale::{ps_archive_block, ps_get_all_versioned_hashes_paginated},
        types::BlobInfo,
        wvm::send_wvm_calldata,
    },
    eyre::{eyre, Error, Result},
    reqwest,
    serde_json::{self, Value},
    std::io::{Read, Write},
};

pub async fn get_blobs_versioned_hashes_of_block(
    block_id: u32,
) -> Result<Vec<String>, eyre::Error> {
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

async fn get_blob_data(versioned_hash: &str) -> Result<String, eyre::Error> {
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

pub async fn get_blobs_of_block(block_id: u32) -> Result<Vec<BlobInfo>> {
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

pub fn serialize_blobscan_block(block: &BlobInfo) -> Result<Vec<u8>> {
    let data = serde_json::to_vec(&block)?;
    let compressed_data = brotli_compress(&data);
    Ok(compressed_data)
}

pub async fn insert_block(block_id: u32, blobs: Vec<BlobInfo>) -> Result<(), Error> {
    for blob in blobs {
        let wvm_data_input = serialize_blobscan_block(&blob)?;
        let wvm_txid = send_wvm_calldata(wvm_data_input).await.unwrap();
        let _res = ps_archive_block(&block_id, &wvm_txid, &blob.versioned_hash, &blob.data)
            .await
            .unwrap();
        let _send_to_blobscan = send_blob_to_blobscan(&blob.versioned_hash).await.unwrap();
    }

    Ok(())
}

fn brotli_compress(input: &[u8]) -> Vec<u8> {
    let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
    writer.write_all(input).unwrap();
    writer.into_inner()
}

fn brotli_decompress(input: Vec<u8>) -> Vec<u8> {
    let mut decompressed_data = Vec::new();
    let mut decompressor = brotli::Decompressor::new(input.as_slice(), 4096); // 4096 is the buffer size

    decompressor
        .read_to_end(&mut decompressed_data)
        .expect("Decompression failed");
    decompressed_data
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
