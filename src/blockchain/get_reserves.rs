use std::sync::Arc;

use eyre::Result;
use ethers::{providers::{Provider, Http}, types::H160};

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
pub async fn get_reserves_blockchain(provider: Arc<Provider<Http>>, pairs: Vec<(H160, H160)>, thread_amount: usize) -> Result<()> {
    todo!("Not implemented")
}
