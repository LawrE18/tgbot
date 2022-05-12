extern crate anyhow;
extern crate hex;

pub mod crypto_provider;
pub mod users;

#[derive(Debug, Clone)]
pub struct Ed25519 {
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Sr25519 {
    pub username: String,
}
