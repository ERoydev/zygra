use alloy::primitives::Address;
use super::types::AssetTypes;
use serde::{Serialize, Deserialize};
use axum::{http::StatusCode, Json};


pub async fn generate_proof(
    Json(payload): Json<GenerateProof>,
) -> (StatusCode, Json<Proof>) {

    let proof = Proof { 
        mocked: String::from("mocked")
    };

    (StatusCode::CREATED, Json(proof))
}   

// The input parameters for `generate_proof` endpoint
#[derive(Deserialize)]
pub struct GenerateProof {
    eth_address: Address,
    asset_type: AssetTypes, 
    collateral_amount: i32
}

#[derive(Serialize)]
pub struct Proof {
    mocked: String
}   