
use alloy::{
    providers::{EthGetBlock, Provider, ProviderBuilder},
    rpc::types::Block, 
    transports::{http::reqwest::Url, RpcError}
};
use alloy_eips::BlockNumberOrTag;
use super::{fetcher::Fetcher, types::FutureOutputType};
use super::types::ProviderType;
use std::{error::Error, pin::Pin};


#[derive(Debug, Clone)]
pub struct SepoliaFetcher {
    pub rpc_url: Url,
    pub provider: ProviderType
}

impl SepoliaFetcher {
    pub fn new(rpc_url: Url) -> Self {
        let provider = ProviderBuilder::new().connect_http(rpc_url.clone());

        SepoliaFetcher { 
            rpc_url, 
            provider
        }
    }
}
 
impl Fetcher for SepoliaFetcher {
    fn fetch_block(&self, nft_block_num: BlockNumberOrTag) -> EthGetBlock<Block> {
        let block_num = self.provider
            .get_block_by_number(nft_block_num);

        block_num
    }
}