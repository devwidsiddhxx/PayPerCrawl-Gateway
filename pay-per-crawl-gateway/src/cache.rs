use dashmap::DashSet;
use std::sync::Arc;

#[derive(Clone)]
pub struct NonceCache {
    seen_nonces: Arc<DashSet<String>>,
}

impl NonceCache {
    pub fn new() -> Self {
        Self {
            seen_nonces: Arc::new(DashSet::new()),
        }
    }

    pub fn insert_and_check(&self, nonce: String) -> bool {
        // returns true if the nonce was NOT present (i.e. it is a new valid nonce)
        self.seen_nonces.insert(nonce)
    }
}
