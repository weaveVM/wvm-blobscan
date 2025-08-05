use {
    crate::utils::{
        constants::{FIRST_ETH_L1_EIP4844_BLOCK, S3_BUCKET_NAME},
        types::BlobInfo,
    },
    aws_config::{BehaviorVersion, Region},
    aws_sdk_s3::{Client, Error},
    serde_json::{json, Value},
};

use crate::utils::env_var::get_env_var;
use crate::utils::blobscan::serialize_blobscan_block;

async fn s3_client() -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(get_env_var("AWS_ENDPOINT_URL").unwrap())
        .region(Region::new(get_env_var("AWS_REGION").unwrap())) 
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            get_env_var("AWS_ACCESS_KEY_ID").unwrap(),
            get_env_var("AWS_SECRET_ACCESS_KEY").unwrap(),
            None, 
            None, 
            "custom"
        ))
        .load()
        .await;
    Client::new(&config)
}

pub async fn store_blob(versioned_hash: &str, blob_data: &str, block_id: u64) -> Result<(), Error> {
    let client = s3_client().await;

    let object_data = BlobInfo::from(block_id, versioned_hash.to_string(), blob_data.to_string());
    let blob = serialize_blobscan_block(&object_data).unwrap();
    let key: String = format!("{}/dataitems/{}.ans104", S3_BUCKET_NAME, blob.1);
    
    client
        .put_object()
        .bucket(S3_BUCKET_NAME)
        .key(key)
        .body(blob.0.into())
        .content_type("application/octet-stream")
        .send()
        .await?;
    
    Ok(())
}

pub async fn get_blob_by_versioned_hash(versioned_hash: &str) -> Option<Value> {
    let client = s3_client().await;
    let key: String = format!("{}/dataitems/{}.ans104", S3_BUCKET_NAME, versioned_hash);

    
    let blob = client
        .get_object()
        .bucket(S3_BUCKET_NAME)
        .key(key)
        .send()
        .await
        .ok()?;

    let body = blob.body.collect().await.ok()?.to_vec();
    let data : BlobInfo = serde_json::from_slice(&body).unwrap_or_default();
    let res = serde_json::to_value(&data).unwrap();
    return Some(res)
    
    }

pub async fn get_latest_block_id() -> u64 {
    // todo
    return FIRST_ETH_L1_EIP4844_BLOCK;
}


pub async fn insert_block(block_id: u64, blobs: Vec<BlobInfo>) -> Result<(), Box<dyn std::error::Error>> {
    for blob in blobs {
        store_blob(&blob.versioned_hash, &blob.data, block_id).await?;
    }
    Ok(())
}
