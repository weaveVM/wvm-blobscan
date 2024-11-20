use crate::utils::env_var::get_env_var;
use anyhow::Error;
use planetscale_driver::{query, PSConnection};
use serde_json::Value;

use super::constants::FIRST_ETH_L1_EIP4844_BLOCK;

async fn ps_init() -> PSConnection {
    let host = get_env_var("DATABASE_HOST").unwrap();
    let username = get_env_var("DATABASE_USERNAME").unwrap();
    let password = get_env_var("DATABASE_PASSWORD").unwrap();

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    conn
}

pub async fn ps_archive_block(
    network_block_id: &u32,
    wvm_calldata_txid: &str,
    raw_data: &str,
) -> Result<(), Error> {
    // format to the table VAR(66) limitation
    let wvm_calldata_txid = wvm_calldata_txid.trim_matches('"');
    let conn = ps_init().await;

    let res = query(
        "INSERT INTO Blobscan(EthereumBlockId, WeaveVMArchiveTxid, RawData) VALUES($0, \"$1\", \"$2\")",
    )
    .bind(network_block_id)
    .bind(wvm_calldata_txid)
    .bind(raw_data)
    .execute(&conn)
    .await;

    match res {
        Ok(result) => {
            println!("Insert operation was successful: {:?}", result);
            Ok(result)
        }
        Err(e) => {
            println!("Error occurred during insert operation: {:?}", e);
            Err(e)
        }
    }
}

pub async fn ps_get_latest_block_id() -> u32 {
    let conn = ps_init().await;
    let latest_archived: u64 =
        query("SELECT MAX(EthereumBlockId) AS LatestNetworkBlockId FROM Blobscan;")
            .fetch_scalar(&conn)
            .await
            .unwrap_or(FIRST_ETH_L1_EIP4844_BLOCK as u64);
    latest_archived as u32
}

// pub async fn ps_get_archived_block_txid(id: u64) -> Value {
//     let conn = ps_init().await;

//     let query_formatted = format!(
//         "SELECT WeaveVMArchiveTxid FROM WeaveVMArchiverMetis WHERE NetworkBlockId = {}",
//         id
//     );
//     let txid: PsGetBlockTxid = query(&query_formatted).fetch_one(&conn).await.unwrap();

//     let res = serde_json::json!(txid);
//     res
// }

// pub async fn ps_get_blocks_extremes(extreme: &str) -> Value {
//     let conn = ps_init().await;

//     let query_type = match extreme {
//         "first" => "ASC",
//         "last" => "DESC",
//         _ => panic!("invalid extreme value. Use 'first' or 'last'."),
//     };

//     let query_formatted = format!(
//         "SELECT NetworkBlockId FROM WeaveVMArchiverMetis ORDER BY NetworkBlockId {} LIMIT 1;",
//         query_type
//     );

//     let query: PsGetExtremeBlock = query(&query_formatted).fetch_one(&conn).await.unwrap();

//     let res = serde_json::json!(query);
//     res
// }

// pub async fn ps_get_archived_blocks_count() -> PsGetTotalBlocksCount {
//     let conn = ps_init().await;

//     let query_formatted = "SELECT MAX(Id) FROM WeaveVMArchiverMetis;";
//     let count: PsGetTotalBlocksCount = query(&query_formatted).fetch_one(&conn).await.unwrap();
//     count
// }
