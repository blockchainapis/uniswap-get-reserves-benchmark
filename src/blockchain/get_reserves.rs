use std::{str::FromStr, sync::Arc, time::Instant};

use ethers::{
    providers::{Http, Provider},
    types::H160,
};
use eyre::Result;
use indicatif::ProgressBar;
use tokio::sync::Semaphore;

use crate::blockchain::abi::UniswapV2Pair;

use super::abi::UniswapV2Factory;

/// Make calls to the blockchain in order to get the reserves for the given pairs
///
/// # Arguments
///
/// * `provider` - The HTTP provider that we use to make calls to the node
/// * `pairs` - The list of pairs that we have to fetch
/// * `thread_amount` - The amount of threads that we should use
///
/// This method will make multithreaded calls to the blockchain in order to get
/// the pair reserves as fast as possible.
/// It will also print the terminal the time took to make the API calls
pub async fn get_reserves_blockchain(
    provider: Arc<Provider<Http>>,
    pairs: Vec<(H160, H160)>,
    thread_amount: usize,
) -> Result<()> {
    println!("Getting reserves using the node");

    let pb = Arc::new(ProgressBar::new(pairs.len() as u64));
    let semaphore = Arc::new(Semaphore::new(thread_amount));
    let mut join_handles = Vec::new();
    let start = Instant::now();

    let factory = Arc::new(UniswapV2Factory::new(
        H160::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap(),
        provider.clone(),
    ));

    for i in 0..pairs.len() {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let curr_pair = pairs.get(i).unwrap().clone();
        let factory_clone = factory.clone();
        let copy_pb = pb.clone();
        let provider_clone = provider.clone();
        join_handles.push(tokio::spawn(async move {
            let pair_address = factory_clone.get_pair(curr_pair.0, curr_pair.1).await.unwrap();
            let pair_contract = UniswapV2Pair::new(pair_address, provider_clone);
            let _ = pair_contract.get_reserves().await.unwrap();
            copy_pb.inc(1);
            drop(permit)
        }));
    }


    pb.finish_and_clear();
    let duration = start.elapsed();
    println!("Took {:?} to gather uniswap reserves using the node", duration);

    Ok(())
}
