use clap::{Parser, Subcommand};
use dotenv::dotenv;

mod provider;
mod receive;
mod send;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show env
    Env { key: String },
    /// Decode tx input message
    Decode { tx_hash: String },
    /// Send a message
    Send {
        msg: String,
        to: String,
        #[arg(default_value = "0.0")]
        value: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();
    match args.command {
        Commands::Decode { tx_hash } => {
            let tx_hash = receive::tx_hash_str_to_h256(&tx_hash).unwrap();
            receive::get_tx_input(tx_hash).await;
        }
        Commands::Env { key } => {
            let val = utils::get_env(&key);
            println!("{} = {}", key, val);
        }
        Commands::Send { msg, to, value } => {
            println!("to: {}, value: {}", to, value);
            send::send_msg(msg, to, value).await.unwrap();
        }
    }
}
