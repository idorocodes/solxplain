use anyhow::{Ok, Result};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::{CommitmentConfig, RpcAccountInfoConfig, UiAccountEncoding, UiDataSliceConfig}};
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use std::str::FromStr;
use solana_client::rpc_request::Address;


#[derive(Debug)]
struct UserAccount{
    lamorts : u64,
    data_len : usize,
    owner : Address,
    executable : bool,
    rent_epoch : u64}

// #[tokio::main]
// async fn main() -> Result<()> {
//     let rpc_url = "https://api.devnet.solana.com";
//     let client = RpcClient::new(rpc_url.to_string());

//     let signature_str =
//         "3qpbjjdhQ8Qd36vwLvF6u1cv9cz2FvSXtEDK4Z29AG7JDdyxy8dtKc6YeLagptX4FNZ4Xd8yh1nFfrXtECBnuQZZ";
//     let signature = Signature::from_str(signature_str)?;

//     let config = solana_client::rpc_config::RpcTransactionConfig {
//         encoding: Some(UiTransactionEncoding::Json),
//         commitment: Some(CommitmentConfig::confirmed()),
//         max_supported_transaction_version: Some(0),
//     };
//     let tx_with_meta = client
//         .get_transaction_with_config(&signature, config)
//         .await?;

//     println!("Tx Slot : {}", tx_with_meta.slot);

//     if let Some(meta) = tx_with_meta.transaction.meta {
//         println!("Transaction errot {:?}", meta.err);
//         if let OptionSerializer::Some(logs) = meta.log_messages {
//             println!(" --- Logs ----");
//             for log in logs {
//                 println!("{}", log)
//             }
//             println!("--------------")
//         }
//     } else {
//         println!(
//             "Transaction meta data not available (likely failed or not confirmed with the specified commitment)"
//         );
//     }

//     Ok(())
// }



#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let public_key = Pubkey::from_str("H3qTybhEacpa8kQGU7sxocGJuNtFsg4joKHyHcqGKdvE")?;
   
    let tx_with_meta = client.get_account_with_commitment(&public_key, CommitmentConfig::processed()).await?;
    
    
    println!("Tx Slot : {:?}", tx_with_meta.context);
    
    if let Some(account) = tx_with_meta.value{
      let user =  UserAccount{
            lamorts : account.lamports,
            data_len: account.data.len(),
            owner : account.owner,
            executable : account.executable,
            rent_epoch : account.rent_epoch,
        };
        
      println!("{:?}",user)
    }
   
    

    Ok(())
}
