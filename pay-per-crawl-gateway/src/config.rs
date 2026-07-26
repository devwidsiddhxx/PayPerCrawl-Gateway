use std::env;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub target_origin: String,
    pub public_key_hex: String,
    pub private_key_hex: String, // only needed for mock payment generator
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();
        
        // These are hardcoded ONLY for this demo, to ensure it runs easily locally out of the box.
        // In production, these should be generated securely and passed via environment variables.
        let default_pub = "7df0a7b45883ce82046db4ec4e3d30b91e55047b3bd811b7147b19b7ce96dbda";
        let default_priv = "415950d99ba42617f6bdf1a5a8e1dc4a6894c259885834863bc43477ed029e2c";

        let bind_addr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
            .parse()
            .expect("Invalid BIND_ADDR");
            
        let target_origin = env::var("TARGET_ORIGIN")
            .unwrap_or_else(|_| "http://127.0.0.1:8081".to_string());
            
        let public_key_hex = env::var("PUBLIC_KEY_HEX").unwrap_or_else(|_| default_pub.to_string());
        let private_key_hex = env::var("PRIVATE_KEY_HEX").unwrap_or_else(|_| default_priv.to_string());

        Self {
            bind_addr,
            target_origin,
            public_key_hex,
            private_key_hex,
        }
    }
}
