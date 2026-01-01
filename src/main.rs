use clap::{Parser,Subcommand};
mod solana;


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CliCommand{
    #[command(subcommand)]
    command : Commands,
    #[arg(long, global = true)]
    cluster : Option<String>
}

#[derive(Debug, Subcommand)]
enum Commands{
    Tx{
        signature : String
    },
    Account {
        pubkey : String
    }
}

#[derive(Debug,Clone)]
enum Cluster {
    Mainnet,
    TestNet,
    Devnet
}


impl Cluster {
    fn set_rpc (self) -> &'static str{
        match self {
            Cluster::Mainnet => "https://api.mainnet-beta.solana.com",
            Cluster::Devnet => "https://api.devnet-beta.solana.com",
            Cluster::TestNet => "https://api.testnet-beta.solana.com"
        } 
    }
}

fn detect_cluster (cli_cluster : Option<String>) -> Cluster{
    
    if let Some(cluster) = cli_cluster{
        return match cluster.as_str() {
            "devnet" => Cluster::Devnet,
            "testnet" => Cluster::TestNet,
            "mainnet" => Cluster::Mainnet,
             _ => Cluster::Devnet
        };
    }
    else {
        Cluster::Mainnet
    }
}

fn main () {
    let args = CliCommand::parse();
    
    let rpc_url = args.cluster;
    
    match args.command {
        
        Commands::Tx { signature } => println!("Get Tx, {}",signature),
        Commands::Account { pubkey } => println!("Get Account {}", pubkey),
        _ => {},
    }
    
    
    
}


