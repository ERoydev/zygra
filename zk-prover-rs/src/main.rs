use alloy::primitives::Address;
use alloy::rpc::types::EIP1186AccountProofResponse;
use alloy::transports::http::reqwest::Url;
use dotenv::dotenv;
use alloy::providers::{Provider, ProviderBuilder}; 
use alloy_eips::BlockNumberOrTag;
use zk_prover_rs::generate_proof::generate_proof;
use std::{error::Error, ops::Deref};
use std::env;
use zk_prover_rs::fetcher::{FetcherFactory, FetcherType};
use zk_prover_rs::types::AssetTypes;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Axum

    dotenv().ok(); // loads the .env into env vars
    // Set up the HTTP transport which is consumed by the RPC client.
    // let sepolia_rpc: Url = env::var("SEPOLIA_RPC_URL").unwrap().parse();
    // // let provider = ProviderBuilder::new().connect_http(sepolia_rpc.clone());
    
    // let nft_block_num: BlockNumberOrTag = BlockNumberOrTag::Number(8316688); // Example nft block number

    // let sepolia_fetcher = FetcherFactory::create(FetcherType::Sepolia, sepolia_rpc.clone());
    // let block_fut = sepolia_fetcher.fetch_block(nft_block_num);
    // let block = block_fut.await;


    // let address: Address = "0x9f4BF05551f5cB1a2BF3cba1d27a4e025FF2E716".parse();
    // let account_data = sepolia_fetcher.fetch_account_data(address).await;

    // println!("Balance: {}", account_data.balance);
    // println!("Proof: {:?}", account_data.account_proof);

    // Axum stuff
    let app = Router::new()
        .route("/", get(root))
        .route("/generate_proof", post(generate_proof));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("\nWeb Server is listening on: 0.0.0.0:3000 port");
    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}

