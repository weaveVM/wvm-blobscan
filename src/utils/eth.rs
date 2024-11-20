use ethers::{
    providers::{Provider, Http},
    middleware::Middleware, 
};
use crate::utils::constants::ETH_RPC_URL;
use eyre::Result;

pub struct Ethereum {}

impl Ethereum {
    pub fn client(url: &str) -> Provider<Http> {
        Provider::<Http>::try_from(url).unwrap()
    }
    
    pub async fn get_latest_eth_block(self) -> Result<u32> {
        let provider = Self::client(ETH_RPC_URL);        
        let latest_block_number = provider.get_block_number().await?;
        
        Ok(latest_block_number.as_u32())
    }
}