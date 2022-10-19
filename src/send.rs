use crate::{provider, utils::get_env};
use ethers::{
    providers::Middleware,
    signers::{LocalWallet, Signer},
    types::{Address, TransactionRequest},
    utils::parse_ether,
};

pub async fn send_msg(
    msg: String,
    to: String,
    value: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = provider::get_http_provider().unwrap();
    let to = to.parse::<Address>()?;
    let value = parse_ether(value)?;
    let wallet: LocalWallet = get_env("PRIVATE_KEY").parse()?;
    println!("from: {:?}", wallet.address());
    println!("to: {:?}", to);
    println!("message: {}", msg);
    let tx_request = TransactionRequest {
        from: Some(wallet.address()),
        to: Some(ethers::types::NameOrAddress::Address(to)),
        value: Some(value),
        data: Some(msg.into_bytes().into()),
        ..Default::default()
    };
    let tx = provider.send_transaction(tx_request, None).await?.await?;
    match tx {
        Some(tx) => {
            println!("block number: {}", &tx.block_number.unwrap());
            println!("tx hash: {:?}", &tx.transaction_hash);
        }
        None => todo!(),
    }
    Ok(())
}
