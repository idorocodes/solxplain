use anyhow::{Ok, Result};
use solana_client::rpc_request::Address;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::CommitmentConfig};
use solana_sdk::{
     pubkey::Pubkey,
};
use std::str::FromStr;

#[derive(Debug)]
struct UserAccount {
    lamports: u64,
    data_len: usize,
    owner: Address,
    executable: bool,
    rent_epoch: u64,
}

pub async fn account(rpc_url: &str, pubkey: String) -> Result<()> {
    let rpc_url = rpc_url;
    let client = RpcClient::new(rpc_url.to_string());

    let public_key = Pubkey::from_str(pubkey.as_str())?;

    let tx_with_meta = client
        .get_account_with_commitment(&public_key, CommitmentConfig::processed())
        .await?;

    println!("Account Slot : {:?}", tx_with_meta.context.slot);
    

    if let Some(account) = tx_with_meta.value {
        let user = UserAccount {
            lamports: account.lamports,
            data_len: account.data.len(),
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        };
        println!(" --- Logs ----");
        println!("Lamports: {:?} ", user.lamports);
        println!("Data Len:{:?} ", user.data_len);
        println!("Owner: {:?}", user.owner);
        println!("Executable: {:?}", user.executable);
        println!("Rent Epoch: {:?}", user.rent_epoch);
        println!("--------------")
    }
    else {
        println!("Account fetch error !")
    }


    Ok(())
}
