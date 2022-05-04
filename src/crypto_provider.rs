extern crate hex;
//use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
extern crate ed25519_dalek;
use rand::rngs::OsRng;
use serde_json::to_vec;
use signature::Signer;
use sp_core::sr25519;
use std::{
    io::{self, BufWriter, Write},
    path::Path,
    str,
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ed25519 {
    id_: i64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Sr25519 {
    id_: i64,
}

trait CryptoProvider {
    type Transaction;
    type Signature;
    type Error;
    fn generate_keypairs(self) -> Result<Vec<u8>, Self::Error>;
    fn get_public(self) -> Result<Vec<u8>, Self::Error>;
    fn sign_transaction(
        self,
        transaction: Self::Transaction,
    ) -> Result<Self::Signature, Self::Error>;
}

impl CryptoProvider for Ed25519 {
    type Transaction = String;
    type Signature = String;
    type Error = String;
    fn generate_keypairs(self) -> Result<Vec<u8>, Self::Error> {
        let mut csprng = OsRng {};
        let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let public_key: ed25519_dalek::PublicKey = keypair.public;
        let public_key_bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = public_key.to_bytes();
        let keypair_bytes = keypair.to_bytes();
        let pub_path: String = format!("./db/{}.pub", self.id_);
        binary_slice_to_file(&public_key_bytes, pub_path.as_str()).expect("error writing file");
        let keypair_path: String = format!("./db/{}.keypair", self.id_);
        binary_slice_to_file(&keypair_bytes, keypair_path.as_str()).expect("error writing file");
        Ok(keypair.to_bytes().to_vec())
    }

    fn get_public(self) -> Result<Vec<u8>, Self::Error> {
        let pub_path: String = format!("./db/{}.pub", self.id_);
        match Path::new(pub_path.as_str()).exists() {
            true => {
                let pub_bytes = read_file_into_binary_vec(pub_path.as_str()).unwrap();
                Ok(pub_bytes.to_vec())
            }
            false => Err("Please create a wallet first (type /createwallet)".to_string()),
        }
    }

    fn sign_transaction(
        self,
        transaction: Self::Transaction,
    ) -> Result<Self::Signature, Self::Error> {
        let keypair_path: String = format!("./db/{}.keypair", self.id_);
        let keypair_bytes = read_file_into_binary_vec(keypair_path.as_str()).unwrap();
        let keypair = ed25519_dalek::Keypair::from_bytes(&keypair_bytes).unwrap();
        let signature = keypair.sign(transaction.as_bytes());
        Ok(signature.to_vec())
    }
}

impl CryptoProvider for Sr25519 {
    type Transaction = String;
    type Signature = String;
    type Error = String;
    fn generate_keypairs(self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    fn get_public(self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    fn sign_transaction(
        self,
        transaction: Self::Transaction,
    ) -> Result<Self::Signature, Self::Error> {
        todo!()
    }
}
