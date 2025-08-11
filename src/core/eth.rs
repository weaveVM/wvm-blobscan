use crate::core::constants::ETH_RPC_URL;
use ethers::middleware::Middleware;
use ethers::providers::{Http, Provider};
use anyhow::Error;

pub struct Ethereum {}

impl Ethereum {
    pub fn client(url: &str) -> Provider<Http> {
        Provider::<Http>::try_from(url).unwrap()
    }

    pub async fn get_latest_eth_block() -> Result<u64, Error> {
        let provider = Self::client(ETH_RPC_URL);
        let latest_block_number = provider.get_block_number().await?;

        Ok(latest_block_number.as_u64())
    }
}
