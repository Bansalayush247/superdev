use axum::Json;
use base64;
use bs58;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};

/// ------------------ /message/sign ------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Debug, Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> Json<ApiResponse<SignMessageResponse>> {
    let secret_bytes = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) if bytes.len() == 64 => bytes,
        _ => {
            return Json(ApiResponse {
                success: false,
                data: SignMessageResponse {
                    signature: "".into(),
                    public_key: "".into(),
                    message: "Invalid or malformed secret key (expected 64-byte base58)".into(),
                },
            })
        }
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Json(ApiResponse {
                success: false,
                data: SignMessageResponse {
                    signature: "".into(),
                    public_key: "".into(),
                    message: "Failed to parse secret key into Keypair".into(),
                },
            })
        }
    };

    let message_bytes = payload.message.as_bytes();
    let signature = keypair.sign(message_bytes);

    Json(ApiResponse {
        success: true,
        data: SignMessageResponse {
            signature: base64::encode(signature.to_bytes()),
            public_key: bs58::encode(keypair.public).into_string(),
            message: payload.message,
        },
    })
}

/// ------------------ /message/verify ------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyMessageData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> Json<ApiResponse<VerifyMessageData>> {
    let pubkey_bytes = match bs58::decode(&payload.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(ApiResponse {
                success: false,
                data: VerifyMessageData {
                    valid: false,
                    message: "Invalid base58 pubkey".into(),
                    pubkey: payload.pubkey,
                },
            })
        }
    };

    let public_key = match PublicKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(ApiResponse {
                success: false,
                data: VerifyMessageData {
                    valid: false,
                    message: "Failed to parse pubkey".into(),
                    pubkey: payload.pubkey,
                },
            })
        }
    };

    let signature_bytes = match base64::decode(&payload.signature) {
        Ok(sig) => sig,
        Err(_) => {
            return Json(ApiResponse {
                success: false,
                data: VerifyMessageData {
                    valid: false,
                    message: "Invalid base64 signature".into(),
                    pubkey: payload.pubkey,
                },
            })
        }
    };

    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            return Json(ApiResponse {
                success: false,
                data: VerifyMessageData {
                    valid: false,
                    message: "Failed to parse signature".into(),
                    pubkey: payload.pubkey,
                },
            })
        }
    };

    let message_bytes = payload.message.as_bytes();
    let is_valid = public_key.verify_strict(message_bytes, &signature).is_ok();

    Json(ApiResponse {
        success: true,
        data: VerifyMessageData {
            valid: is_valid,
            message: payload.message,
            pubkey: payload.pubkey,
        },
    })
}
