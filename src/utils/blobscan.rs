use foundry_blob_explorers::{BlockResponse, Client};
use eyre::{eyre, Result};
use std::io::{Read, Write};
use serde_json;


pub async fn get_block_by_id(block_id: u32) -> Result<BlockResponse, eyre::Error> {
    let block_id = block_id.to_string();
    let client = Client::mainnet();

    let block = client.block(block_id.parse().unwrap()).await;

    match block {
        Ok(block) => Ok(block),
        Err(e) => {
            eprintln!("Error getting block: {:?}", e);
            Err(eyre!("Error getting block: {:?}", e))
        }
    }
}

pub fn process_blobscan_block(block: BlockResponse) -> Result<Vec<u8>> {
    let data = serde_json::to_vec(&block)?;
    let compressed_data = brotli_compress(&data);
    // println!("data len: before brotli: {} after:{}", data.len(), compressed_data.len());
    Ok(compressed_data)
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
