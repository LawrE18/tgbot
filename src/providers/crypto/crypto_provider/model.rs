extern crate ed25519_dalek;
extern crate hex;
use crate::providers::crypto::{users::User, Ed25519, Sr25519};
extern crate rand;

use rand_core::OsRng;
use schnorrkel::{signing_context, Keypair, PublicKey, Signature};

use ed25519_dalek::ed25519::signature::SignerMut;

impl crate::CryptoProvider for Ed25519 {
    type Transaction = String;
    type Signature = String;
    type Error = String;
    fn generate_keypairs(self) -> Result<String, Self::Error> {
        let mut csprng = rand::rngs::OsRng {};
        let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let public_key: ed25519_dalek::PublicKey = keypair.public;
        let public_key_bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = public_key.to_bytes();
        let keypair_bytes = keypair.to_bytes();

        let new_user = User {
            sig_scheme: "Ed25519".to_string(),
            keypair: hex::encode(keypair_bytes),
            pubkey: hex::encode(public_key_bytes),
        };
        User::insert(self.username, new_user).unwrap();
        Ok(hex::encode(public_key_bytes))
    }

    fn get_public(self) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        Ok(user.pubkey)
    }

    fn sign_transaction(self, transaction: Self::Transaction) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        let keypair_bytes = hex::decode(user.keypair).unwrap();
        let mut keypair = ed25519_dalek::Keypair::from_bytes(&keypair_bytes).unwrap();
        let signature: ed25519_dalek::Signature = keypair.sign(transaction.as_bytes());
        Ok(hex::encode(signature.to_bytes()))
    }
}

impl crate::CryptoProvider for Sr25519 {
    type Transaction = String;
    type Signature = String;
    type Error = String;
    fn generate_keypairs(self) -> Result<String, Self::Error> {
        let keypair: Keypair = Keypair::generate_with(OsRng);
        let public_key: PublicKey = keypair.public;
        let public_key_bytes = public_key.to_bytes();
        let keypair_bytes = keypair.to_bytes();

        let new_user = User {
            sig_scheme: "Sr25519".to_string(),
            keypair: hex::encode(keypair_bytes),
            pubkey: hex::encode(public_key_bytes),
        };
        User::insert(self.username, new_user).unwrap();
        Ok(hex::encode(public_key_bytes))
    }

    fn get_public(self) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        Ok(user.pubkey)
    }

    fn sign_transaction(self, transaction: Self::Transaction) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        let keypair_bytes = hex::decode(user.keypair).unwrap();
        let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
        let ctx = signing_context(b"Signing");
        let signature: Signature = keypair.sign(ctx.bytes(transaction.as_bytes()));
        Ok(hex::encode(signature.to_bytes()))
    }
}
