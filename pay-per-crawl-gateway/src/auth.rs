use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use base64::Engine;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPayload {
    pub amount: f64,
    pub path: String,
    pub timestamp: u64,
    pub nonce: String,
    pub expiry: u64,
}

pub fn verify_payment_proof(proof_header: &str, verifying_key: &VerifyingKey) -> Result<PaymentPayload, String> {
    // Expected format: base64url(json_payload).hex(signature)
    let parts: Vec<&str> = proof_header.split('.').collect();
    if parts.len() != 2 {
        return Err("Invalid token format. Expected payload.signature".to_string());
    }

    let payload_b64 = parts[0];
    let sig_hex = parts[1];

    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| "Invalid base64 payload")?;
        
    let payload: PaymentPayload = serde_json::from_slice(&payload_bytes)
        .map_err(|_| "Invalid JSON payload format")?;
        
    let sig_bytes = hex::decode(sig_hex).map_err(|_| "Invalid signature hex encoding")?;
    let signature = Signature::from_slice(&sig_bytes).map_err(|_| "Invalid signature byte length")?;

    // We verify the signature over the raw JSON bytes
    verifying_key.verify(&payload_bytes, &signature).map_err(|_| "Signature verification failed")?;

    // Check expiry
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    if payload.expiry < now {
        return Err("Payment token has expired".to_string());
    }

    Ok(payload)
}
