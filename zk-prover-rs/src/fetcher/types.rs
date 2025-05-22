use std::{error::Error, pin::Pin};

use alloy::providers::{fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller}, RootProvider};



pub type ProviderType = FillProvider<JoinFill<alloy::providers::Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, RootProvider>;