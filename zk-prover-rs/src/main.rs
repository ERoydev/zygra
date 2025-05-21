//! Example of creating an HTTP provider using the `connect_http` method on the `ProviderBuilder`.

use alloy::transports::http::reqwest::Url;
use dotenv::dotenv;
use alloy::providers::{Provider, ProviderBuilder}; 
use alloy_eips::BlockNumberOrTag;
use std::{error::Error, ops::Deref};
use std::env;
use zk_prover_rs::fetcher::{FetcherFactory, FetcherType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // loads the .env into env vars
    // Set up the HTTP transport which is consumed by the RPC client.
    let sepolia_rpc: Url = env::var("SEPOLIA_RPC_URL").unwrap().parse()?;
    
    let nft_block_num: BlockNumberOrTag = BlockNumberOrTag::Number(8316688); // Example nft block number

    let sepolia_fetcher = FetcherFactory::create(FetcherType::Sepolia, sepolia_rpc.clone());
    let block = sepolia_fetcher.fetch_block(nft_block_num);
    let block_number = block.await;

    println!("Lets see: {:?}", block_number);
    
    // let provider = ProviderBuilder::new().connect_http(sepolia_rpc); 

    // let block_number = provider.get_block_by_number(nft_block_num).await?;

    // println!("Block number: {:?}", block_number);
    Ok(())
}