use crate::providers::crypto::users::User;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<String, User>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}
