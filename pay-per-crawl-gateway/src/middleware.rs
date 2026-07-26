use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;
use ed25519_dalek::VerifyingKey;
use crate::{crawler::is_ai_crawler, auth::verify_payment_proof, cache::NonceCache, metrics::record_request};

#[derive(Clone)]
pub struct GatewayState {
    pub verifying_key: Arc<VerifyingKey>,
    pub nonce_cache: NonceCache,
}

pub async fn crawler_auth_middleware(
    State(state): State<Arc<GatewayState>>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_agent = req
        .headers()
        .get(axum::http::header::USER_AGENT)
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    let is_bot = is_ai_crawler(user_agent);

    if !is_bot {
        record_request(false, false);
        return Ok(next.run(req).await);
    }

    // It's a bot, check payment
    let proof_header = req
        .headers()
        .get("X-Payment-Proof")
        .and_then(|val| val.to_str().ok());

    match proof_header {
        Some(proof) => {
            match verify_payment_proof(proof, &state.verifying_key) {
                Ok(payload) => {
                    // Check replay attack using nonce cache
                    if !state.nonce_cache.insert_and_check(payload.nonce.clone()) {
                        record_request(true, true);
                        return Err((StatusCode::PAYMENT_REQUIRED, "Replay attack detected: Nonce already used".to_string()));
                    }

                    record_request(true, false);
                    Ok(next.run(req).await)
                }
                Err(e) => {
                    record_request(true, true);
                    Err((StatusCode::PAYMENT_REQUIRED, format!("Invalid payment proof: {}", e)))
                }
            }
        }
        None => {
            record_request(true, true);
            Err((StatusCode::PAYMENT_REQUIRED, "AI Crawler detected. Payment required via X-Payment-Proof header.".to_string()))
        }
    }
}
