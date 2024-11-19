use ethers::{
    providers::{Provider, Http},
    middleware::Middleware, 
};
use eyre::Result;

pub async fn get_latest_block() -> Result<u32> {
    let provider = Provider::<Http>::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY"
    )?;
    
    let latest_block_number = provider.get_block_number().await?;
    
    
    Ok(latest_block_number.as_u32())
}