use axum::{Json, response::IntoResponse, extract::State};
use std::time::{SystemTime, UNIX_EPOCH};
use ed25519_dalek::{SigningKey, Signer};
use base64::Engine;
use rand::Rng;
use serde_json::json;
use std::sync::Arc;

pub async fn generate_mock_payment(
    State(signing_key): State<Arc<SigningKey>>,
) -> impl IntoResponse {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    let mut rng = rand::thread_rng();
    let nonce: u32 = rng.gen();
    let nonce_str = format!("{:x}", nonce);

    let payload = json!({
        "amount": 0.001,
        "path": "/",
        "timestamp": now,
        "nonce": nonce_str,
        "expiry": now + 3600 // 1 hour validity
    });

    let payload_bytes = serde_json::to_vec(&payload).unwrap();
    let signature = signing_key.sign(&payload_bytes);
    
    let b64_payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&payload_bytes);
    let hex_sig = hex::encode(signature.to_bytes());
    
    let token = format!("{}.{}", b64_payload, hex_sig);
    
    Json(json!({
        "token": token,
        "header": "X-Payment-Proof",
        "instructions": "Use this token in the X-Payment-Proof header to bypass the AI crawler block."
    }))
}
