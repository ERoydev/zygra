
use alloy::{
    primitives::{Address}, providers::{EthGetBlock, Provider, ProviderBuilder}, rpc::types::{Block, EIP1186AccountProofResponse}, transports::{http::reqwest::Url, RpcError}
};
use alloy_eips::BlockNumberOrTag;
use async_trait::async_trait;
use super::{fetcher::Fetcher};
use super::types::ProviderType;

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

#[async_trait]
impl Fetcher for SepoliaFetcher {
    fn fetch_block(&self, nft_block_num: BlockNumberOrTag) -> EthGetBlock<Block> {
        let block_num = self.provider
            .get_block_by_number(nft_block_num);

        block_num
    }

    async fn fetch_account_data(&self, address: Address) -> EIP1186AccountProofResponse {
        self.provider
            .raw_request(
                "eth_getProof".into(),
                (
                    address,
                    Vec::<String>::new(),
                    "latest",
                ),
            )
            .await
            .expect("RPC call failed")
    }
}
