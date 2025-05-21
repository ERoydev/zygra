use std::{error::Error, fmt::Debug, sync::Arc};
use alloy::{providers::EthGetBlock, transports::http::reqwest::Url};
use alloy_eips::BlockNumberOrTag;
use super::{sepolia_fetcher::SepoliaFetcher, types::FutureOutputType};
use std::{future::Future, pin::Pin};
use alloy::{rpc::types::Block};

pub trait Fetcher {
    fn fetch_block(&self, nft_block_num: BlockNumberOrTag) -> EthGetBlock<Block>;
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