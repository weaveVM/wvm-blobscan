use {
    crate::utils::{
        constants::{FIRST_ETH_L1_EIP4844_BLOCK, S3_BUCKET_NAME},
        types::BlobInfo,
    },
    aws_config::BehaviorVersion,
    aws_sdk_s3::{Client, Error},
    serde_json::{json, Value},
};

async fn s3_client() -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    Client::new(&config)
}

pub async fn store_blob(versioned_hash: &str, blob_data: &str, block_id: u64) -> Result<(), Error> {
    let client = s3_client().await;

    let object_data = BlobInfo::from(block_id, versioned_hash.to_string(), blob_data.to_string());
    let blob  = serde_json::to_vec(&object_data).unwrap();
    
    client
        .put_object()
        .bucket(S3_BUCKET_NAME)
        .key(versioned_hash)
        .body(blob.into())
        .content_type("application/json")
        .send()
        .await?;
    
    Ok(())
}

pub async fn get_blob_by_versioned_hash(versioned_hash: &str) -> Option<Value> {
    let client = s3_client().await;

    
    let blob = client
        .get_object()
        .bucket(S3_BUCKET_NAME)
        .key(versioned_hash)
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
