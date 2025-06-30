use std::str::FromStr;

use axum::Json;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use solana_sdk::system_instruction;
use spl_token::instruction::transfer_checked;

//
// REQUEST TYPES
//

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

//
// RESPONSE STRUCTS
//

#[derive(Debug, Serialize)]
pub struct ApiSuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct SolInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    #[serde(rename = "instruction_data")]
    pub instruction_data: String,
}

#[derive(Debug, Serialize)]
pub struct CompactAccountMeta {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Debug, Serialize)]
pub struct TokenInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<CompactAccountMeta>,
    #[serde(rename = "instruction_data")]
    pub instruction_data: String,
}

//
// HANDLER: /send/sol
//

pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> Json<ApiSuccessResponse<SolInstructionResponse>> {
    let from = match Pubkey::from_str(&payload.from) {
        Ok(p) => p,
        Err(_) => {
            return Json(ApiSuccessResponse {
                success: false,
                data: SolInstructionResponse {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: base64::encode("Invalid 'from' pubkey"),
                },
            })
        }
    };

    let to = match Pubkey::from_str(&payload.to) {
        Ok(p) => p,
        Err(_) => {
            return Json(ApiSuccessResponse {
                success: false,
                data: SolInstructionResponse {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: base64::encode("Invalid 'to' pubkey"),
                },
            })
        }
    };

    let ix = system_instruction::transfer(&from, &to, payload.lamports);

    let accounts: Vec<String> = ix.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();

    Json(ApiSuccessResponse {
        success: true,
        data: SolInstructionResponse {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data: base64::encode(ix.data),
        },
    })
}

//
// HANDLER: /send/token
//

pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> Json<ApiSuccessResponse<TokenInstructionResponse>> {
    let destination = match Pubkey::from_str(&payload.destination) {
        Ok(p) => p,
        Err(_) => return Json(ApiSuccessResponse {
            success: false,
            data: TokenInstructionResponse {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: base64::encode("Invalid destination pubkey"),
            },
        }),
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(p) => p,
        Err(_) => return Json(ApiSuccessResponse {
            success: false,
            data: TokenInstructionResponse {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: base64::encode("Invalid mint pubkey"),
            },
        }),
    };

    let owner = match Pubkey::from_str(&payload.owner) {
        Ok(p) => p,
        Err(_) => return Json(ApiSuccessResponse {
            success: false,
            data: TokenInstructionResponse {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: base64::encode("Invalid owner pubkey"),
            },
        }),
    };

    let decimals: u8 = 6; // Adjust if your mint uses a different value

    let ix = match transfer_checked(
        &spl_token::id(),
        &owner,        // source
        &mint,
        &destination,  // destination
        &owner,        // authority
        &[],           // signers
        payload.amount,
        decimals,
    ) {
        Ok(ix) => ix,
        Err(e) => return Json(ApiSuccessResponse {
            success: false,
            data: TokenInstructionResponse {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: STANDARD.encode(e.to_string()),
            },
        }),
    };

    let accounts: Vec<CompactAccountMeta> = ix.accounts.iter().map(|meta| CompactAccountMeta {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
    }).collect();

    Json(ApiSuccessResponse {
        success: true,
        data: TokenInstructionResponse {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data: base64::encode(ix.data),
        },
    })
}

//
// HELPER
//

fn error_response(msg: &str) -> ApiSuccessResponse<ApiErrorResponse> {
    ApiSuccessResponse {
        success: false,
        data: ApiErrorResponse {
            success: false,
            error: msg.to_string(),
        },
    }
}
