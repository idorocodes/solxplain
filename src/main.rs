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
    Testnet,
    Devnet
}


impl Cluster {
    fn set_rpc (&self) -> &'static str{
        match self {
            Cluster::Mainnet => "https://api.mainnet-beta.solana.com",
            Cluster::Devnet => "https://api.devnet.solana.com",
            Cluster::Testnet => "https://api.testnet.solana.com"
        } 
    }
    fn detect_cluster(cli_cluster: Option<String>) -> Cluster {
        match cli_cluster.as_deref() {
            Some("devnet")  => Cluster::Devnet,
            Some("testnet") => Cluster::Testnet,
            Some("mainnet") => Cluster::Mainnet,
            _               => Cluster::Mainnet,
        }
    }
}

  

    
    
fn main() {
    let args = CliCommand::parse();

    let cluster_hint = args.cluster;
    let cluster = Cluster::detect_cluster(cluster_hint).set_rpc();

    println!("cluster: {}", cluster);

    match args.command {
        Commands::Tx { signature } => {
            println!("Get Tx: {}", signature);
        }
        Commands::Account { pubkey } => {
            println!("Get Account: {}", pubkey);
        }
    }
}


