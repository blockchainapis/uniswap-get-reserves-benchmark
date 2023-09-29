use eyre::Result;

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
    todo!("Not implemented");
}
