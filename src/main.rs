use std::{env, sync::Arc};

use clap::Parser;
use ethers::{providers::{Provider, Http}, types::H160};
use eyre::Result;
use dotenv::dotenv;

use pair_fetching_performance_benchmark::{config::args::Args, api::{get_pairs::get_pairs, get_reserves::get_reserves_blockchain_apis}, blockchain::get_reserves::get_reserves_blockchain};

fn pair_string_to_h160_pairs(pairs: &Vec<(String, String)>) -> Vec<(H160, H160)> {
    todo!("Not implemented")
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Args::parse();
    dotenv().ok();
    let http_rpc = env::var("HTTP_RPC").expect("Missing HTTP_RPC environment variable");
    let api_url = env::var("BLOCKCHAIN_APIS_URL").expect("Missing BLOCKCHAIN_APIS_URL environment variable");
    let api_key = env::var("BLOCKCHAIN_APIS_KEY").expect("Missing BLOCKCHAIN_APIS_KEY environment variable");

    let provider = Arc::new(Provider::<Http>::try_from(http_rpc)?);

    let string_pairs = get_pairs(&api_url, &api_key, arguments.pair_amount).await?;
    let pairs = pair_string_to_h160_pairs(&string_pairs);

    get_reserves_blockchain(provider, pairs, arguments.threads).await?;

    get_reserves_blockchain_apis(api_url, api_key, string_pairs, arguments.threads).await?;

    Ok(())
}
