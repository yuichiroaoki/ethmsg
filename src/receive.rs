use crate::provider;
use ethers::{providers::Middleware, types::H256};

pub fn tx_hash_str_to_h256(tx_hash_str: &str) -> Result<H256, Box<dyn std::error::Error>> {
    let tx_hash: H256 = tx_hash_str.parse().unwrap();
    Ok(tx_hash)
}

pub async fn get_tx_input(tx_hash: H256) {
    let provider = provider::get_http_provider().unwrap();
    let tx = provider.get_transaction(tx_hash).await.unwrap();
    match tx {
        Some(tx) => {
            println!("block number: {}", &tx.block_number.unwrap());
            println!("from: {:?}", &tx.from);
            println!("to: {:?}", &tx.to.unwrap());
            let input = tx.input;
            let message = String::from_utf8(input.0.to_vec()).unwrap();
            println!("message: {}", message);
        }
        None => println!("tx not found"),
    };
}
