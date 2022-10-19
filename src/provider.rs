use ethers::providers::Http;
use ethers::providers::Provider;

use crate::utils::get_env;

pub fn get_http_provider() -> Result<Provider<Http>, Box<dyn std::error::Error>> {
    let json_rpc_url = get_env("JSON_RPC_URL");
    let provider =
        Provider::<Http>::try_from(json_rpc_url).expect("could not instantiate HTTP Provider");
    Ok(provider)
}
