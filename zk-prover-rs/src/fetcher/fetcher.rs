use std::{error::Error, fmt::Debug, process::Output, sync::Arc};
use alloy::{primitives::Address, providers::EthGetBlock, rpc::types::EIP1186AccountProofResponse, transports::http::reqwest::Url};
use alloy_eips::BlockNumberOrTag;
use async_trait::async_trait;
use super::{sepolia_fetcher::SepoliaFetcher};
use std::{future::Future, pin::Pin};
use alloy::{rpc::types::Block};

#[async_trait]
pub trait Fetcher {
    fn fetch_block(&self, nft_block_num: BlockNumberOrTag) -> EthGetBlock<Block>;
    async fn fetch_account_data(&self, address: Address) -> EIP1186AccountProofResponse;
}

#[derive(Debug, Clone)]
pub enum FetcherType {
    Sepolia,
}

pub struct FetcherFactory;
impl FetcherFactory {
    // TODO: Check about thread safety Send + Sync traits
    pub fn create(fetcher_type: FetcherType, rpc_url: Url) -> Arc<dyn Fetcher> {
        match fetcher_type {
            FetcherType::Sepolia => Arc::new(SepoliaFetcher::new(rpc_url))
        }
    }
}