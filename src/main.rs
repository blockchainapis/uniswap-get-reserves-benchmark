use std::{env, str::FromStr, sync::Arc};

use clap::Parser;
use dotenv::dotenv;
use ethers::{
    providers::{Http, Provider},
    types::H160,
};
use eyre::Result;

use pair_fetching_performance_benchmark::{
    api::{get_pairs::get_pairs, get_reserves::get_reserves_blockchain_apis},
    blockchain::get_reserves::get_reserves_blockchain,
    config::args::Args,
};

/// Transform a list of pairs as String to a list of H160 pairs
///
/// # Arguments
///
/// * `pairs` - The vector containing the pairs
///
/// # Returns
///
/// The pairs vector but only with H160
fn pair_string_to_h160_pairs(pairs: &Vec<(String, String)>) -> Vec<(H160, H160)> {
    pairs
        .iter()
        .map(|pair| {
            (
                H160::from_str(&pair.0).unwrap(),
                H160::from_str(&pair.1).unwrap(),
            )
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Args::parse();
    dotenv().ok();
    let http_rpc = env::var("HTTP_RPC").expect("Missing HTTP_RPC environment variable");
    let api_url =
        env::var("BLOCKCHAIN_APIS_URL").expect("Missing BLOCKCHAIN_APIS_URL environment variable");
    let api_key =
        env::var("BLOCKCHAIN_APIS_KEY").expect("Missing BLOCKCHAIN_APIS_KEY environment variable");

    let provider = Arc::new(Provider::<Http>::try_from(http_rpc)?);

    let string_pairs = get_pairs(&api_url, &api_key, arguments.pair_amount).await?;
    let pairs = pair_string_to_h160_pairs(&string_pairs);

    get_reserves_blockchain(provider, pairs, arguments.threads).await?;

    get_reserves_blockchain_apis(api_url, api_key, string_pairs, arguments.threads).await?;

    Ok(())
}
