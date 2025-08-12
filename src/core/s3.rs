//! Module to interact with Load's S3 HyperBEAM device (~s3@1.0).
//! The module store the serialized Ethereum blobs as ANS-104 DataItems
//! in a a location in the HyperBEAM device where it can be retrieved back from 
//! the Load HyperBEAM Hybrid Gateway as if it is an onchain Arweave DataItem
//! To learn more about Hybrid Gateway and retrieval logic, check the load_hb
//! documentation: https://github.com/loadnetwork/load_hb/tree/s3-edge/native/s3_nif#hybrid-gateway 
//! 
//! Functionalities: 
//! - Initialize ~s3@1.0 device connection
//! - Store Ethereum blob as BlobInfo struct, serialized as ANS-104 DataItem
//!  - Retrieve a blob and its data (deserialized) back from the ~s3@1.0 for a given versione hash
use crate::core::{
    blobscan::serialize_blobscan_block, env_var::get_env_var, indexer::insert_kv, types::BlobInfo,
};
use anyhow::{anyhow, Error};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client;
use serde_json::Value;

async fn s3_client() -> Result<Client, Error> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(get_env_var("AWS_ENDPOINT_URL").unwrap())
        .region(Region::new(get_env_var("AWS_REGION").unwrap()))
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            get_env_var("AWS_ACCESS_KEY_ID").unwrap(),
            get_env_var("AWS_SECRET_ACCESS_KEY").unwrap(),
            None,
            None,
            "custom",
        ))
        .load()
        .await;
    Ok(Client::new(&config))
}

pub async fn store_blob(versioned_hash: &str, blob_data: &str, block_id: u64) -> Result<(), Error> {
    let client = s3_client().await;
    let s3_bucket_name = get_env_var("S3_BUCKET_NAME").unwrap();
    let s3_dir_name = get_env_var("S3_DIR_NAME").unwrap();

    let object_data = BlobInfo::from(block_id, versioned_hash.to_string(), blob_data.to_string());
    let blob = serialize_blobscan_block(&object_data).unwrap();
    let key: String = format!("{}/{}/{}.ans104", s3_bucket_name, s3_dir_name, blob.1);

    client?
        .put_object()
        .bucket(s3_bucket_name)
        .key(key)
        .body(blob.0.into())
        .content_type("application/octet-stream")
        .send()
        .await?;

    let _ =
        insert_kv(versioned_hash, &blob.1, block_id).await.map_err(|e| anyhow!(e.to_string()))?;

    Ok(())
}

pub async fn get_blob_by_versioned_hash(versioned_hash: &str) -> Result<Value, Error> {
    let client = s3_client().await;
    let s3_bucket_name = get_env_var("S3_BUCKET_NAME").unwrap();
    let s3_dir_name = get_env_var("S3_DIR_NAME").unwrap();
    let key: String = format!("{}/{}/{}.ans104", s3_bucket_name, s3_dir_name, versioned_hash);

    let blob = client?.get_object().bucket(s3_bucket_name).key(key).send().await?;

    let body = blob.body.collect().await?.to_vec();
    let data: BlobInfo = serde_json::from_slice(&body)?;
    let res = serde_json::to_value(&data)?;
    Ok(res)
}

pub async fn insert_block(block_id: u64, blobs: Vec<BlobInfo>) -> Result<(), Error> {
    let mut hashes: Vec<String> = Vec::new();
    for blob in blobs {
        if !hashes.contains(&blob.versioned_hash) {
            store_blob(&blob.versioned_hash, &blob.data, block_id).await?;
            hashes.push(blob.versioned_hash);
        }
    }
    Ok(())
}
