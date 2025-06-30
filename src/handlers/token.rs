use std::str::FromStr;

use axum::Json;
use base64;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use spl_token::instruction::{initialize_mint, mint_to};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenRequest {
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Serialize)]
pub struct TokenInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> Json<ApiResponse<TokenInstructionResponse>> {
    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(m) => m,
        Err(_) => return Json(ApiResponse { success: false, data: TokenInstructionResponse { program_id: "".to_string(), accounts: vec![], instruction_data: "Invalid mint pubkey".to_string() } }),
    };

    let mint_authority = match Pubkey::from_str(&payload.mint_authority) {
        Ok(m) => m,
        Err(_) => return Json(ApiResponse { success: false, data: TokenInstructionResponse { program_id: "".to_string(), accounts: vec![], instruction_data: "Invalid mintAuthority pubkey".to_string() } }),
    };

    let ix = match initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        None,
        payload.decimals,
    ) {
        Ok(ix) => ix,
        Err(e) => return Json(ApiResponse { success: false, data: TokenInstructionResponse { program_id: "".to_string(), accounts: vec![], instruction_data: e.to_string() } }),
    };

    let accounts: Vec<AccountMeta> = ix.accounts.iter().map(|meta| AccountMeta {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();

    let encoded_data = base64::encode(ix.data);

    Json(ApiResponse {
        success: true,
        data: TokenInstructionResponse {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data: encoded_data,
        },
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> Json<ApiResponse<TokenInstructionResponse>> {
    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(p) => p,
        Err(_) => return Json(ApiResponse { success: false, data: TokenInstructionResponse { program_id: "".to_string(), accounts: vec![], instruction_data: "Invalid mint pubkey".to_string() } }),
    };

    let destination = match Pubkey::from_str(&payload.destination) {
        Ok(p) => p,
        Err(_) => return Json(ApiResponse { success: false, data: TokenInstructionResponse { program_id: "".to_string(), accounts: vec![], instruction_data: "Invalid destination pubkey".to_string() } }),
    };

    let authority = match Pubkey::from_str(&payload.authority) {
        Ok(p) => p,
        Err(_) => return Json(ApiResponse { success: false, data: TokenInstructionResponse { program_id: "".to_string(), accounts: vec![], instruction_data: "Invalid authority pubkey".to_string() } }),
    };

    let ix = match mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        payload.amount,
    ) {    Ok(ix) => ix,
        Err(e) => return Json(ApiResponse { success: false, data: TokenInstructionResponse { program_id: "".to_string(), accounts: vec![], instruction_data: e.to_string() } }),
    };

    let accounts: Vec<AccountMeta> = ix.accounts.iter().map(|meta| AccountMeta {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();

    Json(ApiResponse {
        success: true,
        data: TokenInstructionResponse {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data: base64::encode(ix.data),
        },
    })
}
