use {
    crate::utils::{planetscale::ps_archive_block, types::BlobInfo, wvm::send_wvm_calldata},
    eyre::{eyre, Error, Result},
    foundry_blob_explorers::{BlockResponse, Client},
    serde_json,
    std::io::{Read, Write},
};

pub async fn get_block_by_id(block_id: u32) -> Result<BlockResponse, Error> {
    let block_id = block_id.to_string();
    let client = Client::mainnet();

    let block = client.block(block_id.parse().unwrap()).await;

    match block {
        Ok(block) => Ok(block),
        Err(e) => {
            eprintln!("Error getting block: {:?}", e);
            return Err(eyre!("Error getting block: {:?}", e));
        }
    }
}

pub fn get_blobs_of_block(block: BlockResponse) -> Result<Vec<BlobInfo>> {
    let mut res: Vec<BlobInfo> = Vec::new();
    let txs = block.transactions;
    for _tx in txs {
        let blobs = _tx.blobs;

        println!("BLOCKS COUNT IN BLOCK #{} is {}", block.number, blobs.len());

        for _blob in blobs {
            let to_blob_info = BlobInfo::from(
                block.number,
                _blob.versioned_hash.to_string(),
                _blob.data.to_string(),
            );
            println!("{:?}", to_blob_info);
            res.push(to_blob_info);
        }
    }

    Ok(res)
}

pub fn serialize_blobscan_block(block: &BlobInfo) -> Result<Vec<u8>> {
    let data = serde_json::to_vec(&block)?;
    let compressed_data = brotli_compress(&data);
    Ok(compressed_data)
}

pub async fn insert_block(block: BlockResponse) -> Result<(), Error> {
    let blobs = get_blobs_of_block(block.clone())?;

    for blob in blobs {
        let wvm_data_input = serialize_blobscan_block(&blob)?;
        let raw_block_data = serde_json::to_string(&blob)?;
        let wvm_txid = send_wvm_calldata(wvm_data_input).await.unwrap();
        let res = ps_archive_block(
            &(block.clone().number as u32),
            &wvm_txid,
            &blob.versioned_hash,
            &blob.data,
        )
        .await
        .unwrap();
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
