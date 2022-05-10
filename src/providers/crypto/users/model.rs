use crate::providers::db::HASHMAP;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub sig_scheme: String,
    pub keypair: String,
    pub pubkey: String,
}

impl User {
    pub fn find(username: String) -> anyhow::Result<Self> {
        let map = HASHMAP.lock().unwrap();
        let user: User = map.get(&username).unwrap().to_owned();
        Ok(user)
    }

    pub fn insert(username: String, user: User) -> anyhow::Result<Self> {
        let mut map = HASHMAP.lock().unwrap();
        let user1 = user.clone();
        let user2 = user.clone();
        match map.entry(username) {
            Entry::Vacant(e) => {
                e.insert(user);
                Ok(user1)
            }
            Entry::Occupied(mut e) => { let u = e.get_mut(); *u = user1; Ok(user2) },
        }
    }
}
