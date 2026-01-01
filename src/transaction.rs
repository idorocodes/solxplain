use anyhow::{Ok, Result};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::signature::Signature;
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use std::str::FromStr;

pub async fn transaction(rpc_url: &str, signature: String) -> Result<()> {
    let rpc_url = rpc_url;
    let client = RpcClient::new(rpc_url.to_string());

    let signature_str = signature;
    let signature = Signature::from_str(signature_str.as_str())?;

    let config = solana_client::rpc_config::RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    let tx_with_meta = client
        .get_transaction_with_config(&signature, config)
        .await?;

    println!("Tx Slot : {}", tx_with_meta.slot);

    if let Some(meta) = tx_with_meta.transaction.meta {
        println!("Transaction error {:?}", meta.err);
        if let OptionSerializer::Some(logs) = meta.log_messages {
            println!(" ---- Logs ----");
            for log in logs {
                println!("{}", log)
            }
            println!("--------------")
        }
    } else {
        println!(
            "Transaction meta data not available (likely failed or not confirmed with the specified commitment)"
        );
    }

    Ok(())
}
