use {
    ethers::{prelude::*, utils},
    crate::utils::{
        eth::Ethereum,
        env_var::get_env_var,
        constants::{WVM_RPC_URL, WVM_CHAIN_ID, WVM_ARCHIVER_ADDRESS}
    }
};


type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn send_wvm_calldata(block_data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let provider = Ethereum::client(WVM_RPC_URL);
    let private_key = get_env_var("blobscan_pk").unwrap();
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(WVM_CHAIN_ID);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let address_from = WVM_ARCHIVER_ADDRESS.parse::<Address>()?;
    let address_to = ethers::addressbook::Address::zero();
    // send calldata tx to WeaveVM
    let txid = send_transaction(&client, &address_from, &address_to, block_data).await?;

    Ok(txid)
}

pub async fn send_transaction(
    client: &Client,
    address_from: &Address,
    address_to: &Address,
    block_data: Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    println!(
        "\nArchiving block data from archiver: {} to archive pool: {}",
        address_from, address_to
    );
    // 2.14 Gwei
    let gas_price = U256::from(2_140_000_000);
    let tx = TransactionRequest::new()
        .to(address_to.clone())
        .value(U256::from(utils::parse_ether(0)?))
        .from(address_from.clone())
        .data(block_data)
        .gas_price(gas_price);

    let tx = client.send_transaction(tx, None).await?.await?;
    let json_tx = serde_json::json!(tx);
    let txid = json_tx["transactionHash"].to_string();

    println!("\nWeaveVM Archiving TXID: {}", txid);
    Ok(txid)
}