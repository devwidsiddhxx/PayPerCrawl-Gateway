mod auth;
mod cache;
mod config;
mod crawler;
mod metrics;
mod middleware;
mod payment_mock;
mod proxy;

use axum::{
    routing::{any, get},
    Router,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use reqwest::Client;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{
    cache::NonceCache,
    config::Config,
    middleware::{crawler_auth_middleware, GatewayState},
    proxy::{reverse_proxy_handler, ProxyState},
};

#[tokio::main]
async fn main() {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::load();
    let metrics_handle = metrics::setup_metrics();

    // Load Cryptographic keys
    let verifying_key_bytes = hex::decode(&config.public_key_hex).expect("Invalid public key hex");
    let verifying_key = VerifyingKey::from_bytes(verifying_key_bytes.as_slice().try_into().unwrap())
        .expect("Invalid public key bytes format");

    let signing_key_bytes = hex::decode(&config.private_key_hex).expect("Invalid private key hex");
    let signing_key = SigningKey::from_bytes(signing_key_bytes.as_slice().try_into().unwrap());

    // Initialize shared state components
    let nonce_cache = NonceCache::new();

    let gateway_state = Arc::new(GatewayState {
        verifying_key: Arc::new(verifying_key),
        nonce_cache,
    });

    let proxy_state = Arc::new(ProxyState {
        client: Client::new(),
        target_origin: config.target_origin.clone(),
    });

    // Sub-router for the mock payment generator
    let mock_payment_router = Router::new()
        .route("/mock-payment", get(payment_mock::generate_mock_payment))
        .with_state(Arc::new(signing_key));

    // Sub-router for the actual reverse proxy and auth middleware
    let proxy_router = Router::new()
        .fallback(any(reverse_proxy_handler))
        .with_state(proxy_state)
        .layer(axum::middleware::from_fn_with_state(
            gateway_state,
            crawler_auth_middleware,
        ));

    // Main application router combining everything
    let app = Router::new()
        .route("/metrics", get(move || std::future::ready(metrics_handle.render())))
        .merge(mock_payment_router)
        .merge(proxy_router);

    println!("PayPerCrawl Gateway starting on http://{}", config.bind_addr);
    println!("Proxying to origin {}", config.target_origin);

    let listener = TcpListener::bind(config.bind_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
