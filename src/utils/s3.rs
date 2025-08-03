use {
    crate::utils::{
        constants::FIRST_ETH_L1_EIP4844_BLOCK,
        env_var::get_env_var,
        types::BlobInfo,
    },
    aws_config::BehaviorVersion,
    aws_sdk_s3::{Client, Error},
    serde_json::{json, Value},
    std::collections::HashMap,
};

async fn s3_client() -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    Client::new(&config)
}

pub async fn store_blob(versioned_hash: &str, blob_data: &str, block_id: u32) -> Result<(), Error> {
    let client = s3_client().await;
    let bucket = "blobscan";
    let key = format!("/{}", versioned_hash);

    let object_data = json!({"EthereumBlockId": block_id, "VersionedHash": versioned_hash, "BlobData": blob_data});
    let blob  = serde_json::to_vec(&object_data).unwrap();
    
    client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .body(blob.into())
        .content_type("application/json")
        .send()
        .await?;
    
    Ok(())
}

pub async fn get_blob_by_versioned_hash(versioned_hash: &str) -> Option<Value> {
    let client = s3_client().await;
    let bucket = "blobscan";
    
    let objects = client
        .list_objects()
        .bucket(bucket)
        .send()
        .await
        .ok()?;
    
    for object in objects.contents() {
        if let Some(key) = object.key() {
            if key.ends_with(versioned_hash) {
                let parts: Vec<&str> = key.split('/').collect();
                if parts.len() == 2 {
                    let block_id = parts[0].parse::<u32>().ok()?;
                    
                    let blob_data = client
                        .get_object()
                        .bucket(bucket)
                        .key(key)
                        .send()
                        .await
                        .ok()?;
                    
                    let body = blob_data.body.collect().await.ok()?;
                    let data = String::from_utf8(body.to_vec()).ok()?;
                    
                    return Some(serde_json::json!({
                        "EthereumBlockId": block_id,
                        "VersionedHash": versioned_hash,
                        "BlobData": data
                    }));
                }
            }
        }
    }
    
    None
}

pub async fn get_latest_block_id() -> u32 {
    let client = s3_client().await;
    let bucket = "blobscan";
    
    let objects = client
        .list_objects()
        .bucket(bucket)
        .send()
        .await;
    
    match objects {
        Ok(response) => {
            let mut max_block_id = FIRST_ETH_L1_EIP4844_BLOCK;
            
            for object in response.contents() {
                if let Some(key) = object.key() {
                    let parts: Vec<&str> = key.split('/').collect();
                    if parts.len() == 2 {
                        if let Ok(block_id) = parts[0].parse::<u32>() {
                            if block_id > max_block_id {
                                max_block_id = block_id;
                            }
                        }
                    }
                }
            }
            
            max_block_id
        }
        Err(_) => FIRST_ETH_L1_EIP4844_BLOCK,
    }
}

pub async fn get_stats() -> Value {
    let latest_block_id = get_latest_block_id().await;
    let client = s3_client().await;
    let bucket = "blobscan";
    
    let objects = client
        .list_objects()
        .bucket(bucket)
        .send()
        .await;
    
    match objects {
        Ok(response) => {
            for object in response.contents() {
                if let Some(key) = object.key() {
                    let parts: Vec<&str> = key.split('/').collect();
                    if parts.len() == 2 {
                        if let Ok(block_id) = parts[0].parse::<u32>() {
                            if block_id == latest_block_id {
                                return serde_json::json!({
                                    "EthereumBlockId": latest_block_id,
                                    "VersionedHash": parts[1]
                                });
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {}
    }
    
    serde_json::json!({
        "EthereumBlockId": latest_block_id,
        "VersionedHash": null
    })
}

pub async fn insert_block(block_id: u32, blobs: Vec<BlobInfo>) -> Result<(), Box<dyn std::error::Error>> {
    for blob in blobs {
        store_blob(&blob.versioned_hash, &blob.data, block_id).await?;
    }
    Ok(())
}
