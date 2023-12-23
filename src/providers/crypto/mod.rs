use serde::{Deserialize, Serialize};

extern crate anyhow;
extern crate hex;

pub mod crypto_provider;
pub mod users;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ed25519 {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sr25519 {
    pub username: String,
}

#[cfg(test)]
pub mod test {
    use crate::Sr25519;

    #[test]
    fn test_ser() {
        let s_rhs = Sr25519 {
            username: "username".to_string(),
        };
        let s_lhs = r#"{
            "username": "username"
        }"#;
        let s_lhs: Sr25519 = serde_json::from_str(s_lhs).unwrap();

        assert_eq!(s_lhs, s_rhs);
    }
}
