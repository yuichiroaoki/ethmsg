use clap::{Parser, Subcommand};
use dotenv::dotenv;

mod request;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show hello message
    Hello {
        #[arg(default_value = "Hello World!")]
        message: String,
    },
    /// Show env
    Env {
        #[arg(default_value = "USER")]
        key: String,
    },
    /// Send get request
    Request { url: String },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();
    match args.command {
        Commands::Hello { message } => {
            println!("{}", message);
        }
        Commands::Env { key } => {
            let val = utils::get_env(&key);
            println!("{} = {}", key, val);
        }
        Commands::Request { url } => {
            let res = request::send_get_request(&url).await;
            assert!(res.is_ok());
        }
    }
}
