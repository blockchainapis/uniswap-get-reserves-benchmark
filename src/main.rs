use std::{env, sync::Arc};

use clap::Parser;
use ethers::providers::{Provider, Http};
use eyre::Result;
use dotenv::dotenv;

use pair_fetching_performance_benchmark::config::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Args::parse();
    dotenv().ok();
    let http_rpc = env::var("HTTP_RPC").expect("Missing HTTP_RPC environment variable");
    let api_url = env::var("BLOCKCHAIN_APIS_URL").expect("Missing BLOCKCHAIN_APIS_URL environment variable");
    let api_key = env::var("BLOCKCHAIN_APIS_KEY").expect("Missing BLOCKCHAIN_APIS_KEY environment variable");

    let provider = Arc::new(Provider::<Http>::try_from(http_rpc)?);

    println!("Hello, world!");
    Ok(())
}
