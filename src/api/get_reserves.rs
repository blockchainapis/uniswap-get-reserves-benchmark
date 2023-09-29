use std::{sync::Arc, time::Instant};

use eyre::Result;
use indicatif::ProgressBar;
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use tokio::sync::Semaphore;

use crate::api::models::Reserve;

/// Make api calls in order to get the reserves inside of every pair
/// 
/// # Arguments
/// 
/// * `api_url` - The url of Blockchain APIs, for example: https://api.blockchainapis.io
/// * `api_key` - The API key to use
/// * `pairs` - The list of pairs as String that we have to use
/// * `thread_amount` - The amount of threads that we create
/// 
/// This method will make API calls as fast as possible to Blockchain APIs in order
/// to get the reserves. It will also print in the terminal the duration that it took
pub async fn get_reserves_blockchain_apis(api_url: String, api_key: String, pairs: Vec<(String, String)>, thread_amount: usize) -> Result<()> {
    println!("Getting reserves using Blockchain APIs");

    let pb = Arc::new(ProgressBar::new(pairs.len() as u64));
    let semaphore = Arc::new(Semaphore::new(thread_amount));
    let mut join_handles = Vec::new();

    let start = Instant::now();
    let mut headers = HeaderMap::new();
    headers.insert("api-key", HeaderValue::from_str(&api_key)?);
    let client_build = Client::builder()
        .default_headers(headers)
        .build()?;
    let client = Arc::new(client_build);
    let api_url_arc = Arc::new(api_url);

    for i in 0..pairs.len() {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let curr_pair = pairs.get(i).unwrap().clone();
        let copy_pb = pb.clone();
        let client_copy = client.clone();
        let api_url_clone = api_url_arc.clone();
        join_handles.push(tokio::spawn(async move {
            let _: Vec<Reserve> = client_copy.get(format!("{api_url_clone}/v0/exchanges/pairs/reserves?blockchain=ethereum&exchange=uniswapv2_ethereum&token0={}&token1={}", curr_pair.0, curr_pair.1))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            copy_pb.inc(1);
            drop(permit)
        }))
    }

    for handler in join_handles {
        handler.await?;
    }

    let duration = start.elapsed();
    println!("Took {:?} to gather uniswap reserves using Blockchain APIs", duration);

    Ok(())
}
