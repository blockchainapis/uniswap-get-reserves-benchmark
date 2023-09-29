use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pair {
    pub blockchain: String,
    pub exchange: String,
    pub token0: String,
    pub token1: String,
    pub fee: u16
}

#[derive(Debug, Deserialize)]
pub struct Pairs {
    pub page: u32,
    pub total_pages: u64,
    pub data: Vec<Pair>
}

#[derive(Debug, Deserialize)]
pub struct Reserve {
    pub blockchain: String,
    pub exchange: String,
    pub token0: String,
    pub token1: String,
    pub reserve0: u128,
    pub reserve1: u128
}
