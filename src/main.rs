use std::{env, sync::Arc};

use clap::Parser;
use ethers::{providers::{Provider, Http}, types::H160};
use eyre::Result;
use dotenv::dotenv;

use pair_fetching_performance_benchmark::{config::args::Args, api::get_pairs::get_pairs, blockchain::get_reserves::get_reserves_blockchain};

fn pair_h160_vec_to_string(pairs: &Vec<(H160, H160)>) -> Vec<(String, String)> {
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

    let pairs = get_pairs(&api_url, &api_key, arguments.pair_amount).await?;
    let string_pairs = pair_h160_vec_to_string(&pairs);

    get_reserves_blockchain(provider, pairs, arguments.threads).await?;


    Ok(())
}
