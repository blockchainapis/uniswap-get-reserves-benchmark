use eyre::Result;

use super::models::Pairs;

async fn get_pair(api_url: &String, api_key: &String, page: u32) -> Result<Pairs> {
    let client = reqwest::Client::new();
    let response: Pairs = client.get(format!("{api_url}/v0/exchanges/pairs/?page={page}&blockchain=ethereum&exchange=uniswapv2_ethereum"))
        .header("api-key", api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

/// Get the pairs using Blockchain APIs
///
/// # Arguments
///
/// * `api_url` - The url of Blockchain APIs
/// * `api_key` - The key that we use
/// * `pair_amount` - The amount of pairs that we have to get
///
/// # Returns
///
/// A vector containing the pairs that we got
pub async fn get_pairs(
    api_url: &String,
    api_key: &String,
    pair_amount: u64,
) -> Result<Vec<(String, String)>> {
    let mut pairs = Vec::new();
    let mut curr_amount = 0;
    let mut page = 1_u32;
    while curr_amount < pair_amount {
        let mut result = get_pair(api_url, api_key, page).await?;
        pairs.append(&mut result.data);
        curr_amount += 100;
        page += 1;
    }

    let mut ret = Vec::new();

    for i in 0..(pair_amount as usize) {
        let curr_pair = &pairs[i];
        ret.push((curr_pair.token0.clone(), curr_pair.token1.clone()));
    }

    Ok(ret)
}
