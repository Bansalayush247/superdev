use axum::Json;
use bs58;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};

#[derive(Serialize)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub async fn generate_keypair() -> Json<ApiResponse<KeypairData>> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    Json(ApiResponse {
        success: true,
        data: KeypairData { pubkey, secret },
    })
}
