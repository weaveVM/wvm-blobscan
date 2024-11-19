use ethers::{
    providers::{Provider, Http},
    middleware::Middleware, 
};
use crate::utils::constants::ETH_RPC_URL;
use eyre::Result;

pub async fn get_latest_eth_block() -> Result<u32> {
    let provider = Provider::<Http>::try_from(
        ETH_RPC_URL
    )?;
    
    let latest_block_number = provider.get_block_number().await?;
    
    Ok(latest_block_number.as_u32())
}