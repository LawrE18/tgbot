use crate::providers::db::HASHMAP;
use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub sig_scheme: String,
    pub keypair: String,
    pub pubkey: String,
}

impl User {
    pub fn find(username: &String) -> Result<Self, anyhow::Error> {
        let map = HASHMAP.lock().expect("error lock");
        match map.get(username) {
            Some(u) => Ok(u.to_owned()),
            None => bail!("error get user"),
        }
    }

    pub fn insert(username: String, user: User) -> Result<Self> {
        let mut map = HASHMAP.lock().expect("error lock");
        match map.entry(username) {
            Entry::Vacant(e) => {
                e.insert(user.clone());
            }
            Entry::Occupied(_) => {}
        };
        Ok(user)
    }
}
