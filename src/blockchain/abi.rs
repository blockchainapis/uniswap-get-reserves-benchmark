use ethers::contract::abigen;

abigen!(
    UniswapV2Factory,
    "src/blockchain/abi_files/UniswapV2FactoryAbi.json"
);

abigen!(
    UniswapV2Pair,
    "src/blockchain/abi_files/UniswapV2PairAbi.json"
);
