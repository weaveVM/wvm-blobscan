//! Module to interact with an Ethereum JSON-RPC client
//! 
//! Functionalities:
//! - Get the latest Ethereum block number
//! 
//! Note: this modules should migrate to alloy-rs instead of using ethers
use crate::core::constants::ETH_RPC_URL;
use anyhow::Error;
use ethers::{
    middleware::Middleware,
    providers::{Http, Provider},
};

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
