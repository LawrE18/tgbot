extern crate hex;
//use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
extern crate ed25519_dalek;
use crate::postgres::crypto::{Ed25519, Sr25519};
extern crate rand;

use rand_core::OsRng;
use schnorrkel::{signing_context, Keypair, PublicKey, Signature};

use ed25519_dalek::ed25519::signature::SignerMut;

use crate::postgres::{crypto::users::User, crypto::users::UserForm};

pub trait CryptoProvider {
    type Transaction;
    type Signature;
    type Error;
    fn generate_keypairs(self) -> Result<String, Self::Error>;
    fn get_public(self) -> Result<String, Self::Error>;
    fn sign_transaction(self, transaction: Self::Transaction) -> Result<String, Self::Error>;
}

impl CryptoProvider for Ed25519 {
    type Transaction = String;
    type Signature = String;
    type Error = String;
    fn generate_keypairs(self) -> Result<String, Self::Error> {
        let mut csprng = rand::rngs::OsRng {};
        let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let public_key: ed25519_dalek::PublicKey = keypair.public;
        let public_key_bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = public_key.to_bytes();
        let keypair_bytes = keypair.to_bytes();
        // let pub_path: String = format!("./db/{}.pub", self.id_);
        // binary_slice_to_file(&public_key_bytes, pub_path.as_str()).expect("error writing file");
        // let keypair_path: String = format!("./db/{}.keypair", self.id_);
        // binary_slice_to_file(&keypair_bytes, keypair_path.as_str()).expect("error writing file");

        let new_user = UserForm {
            username: self.username,
            sig_scheme: "Ed25519".to_string(),
            keypair: hex::encode(keypair_bytes),
            pubkey: hex::encode(public_key_bytes),
        };
        User::insert(new_user).unwrap();
        Ok(hex::encode(public_key_bytes))
        //Ok(keypair.to_bytes().to_vec())
    }

    fn get_public(self) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        Ok(user.pubkey)
        // let pub_path: String = format!("./db/{}.pub", self.id_);
        // match Path::new(pub_path.as_str()).exists() {
        //     true => {
        //         let pub_bytes = read_file_into_binary_vec(pub_path.as_str()).unwrap();
        //         Ok(pub_bytes.to_vec())
        //     }
        //     false => Err("Please create a wallet first (type /createwallet)".to_string()),
        // }
    }

    fn sign_transaction(self, transaction: Self::Transaction) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        let keypair_bytes = hex::decode(user.keypair).unwrap();
        let mut keypair = ed25519_dalek::Keypair::from_bytes(&keypair_bytes).unwrap();
        let signature: ed25519_dalek::Signature = keypair.sign(transaction.as_bytes());
        Ok(hex::encode(signature.to_bytes()))
    }
}

impl CryptoProvider for Sr25519 {
    type Transaction = String;
    type Signature = String;
    type Error = String;
    fn generate_keypairs(self) -> Result<String, Self::Error> {
        let keypair: Keypair = Keypair::generate_with(OsRng);
        let public_key: PublicKey = keypair.public;
        let public_key_bytes = public_key.to_bytes();
        let keypair_bytes = keypair.to_bytes();

        let new_user = UserForm {
            username: self.username,
            sig_scheme: "Sd25519".to_string(),
            keypair: hex::encode(keypair_bytes),
            pubkey: hex::encode(public_key_bytes),
        };
        User::insert(new_user).unwrap();
        Ok(hex::encode(public_key_bytes))
        // let pub_path: String = format!("./db/{}.pub", self.id_);
        // binary_slice_to_file(&public_key_bytes, pub_path.as_str()).expect("error writing file");
        // let keypair_path: String = format!("./db/{}.keypair", self.id_);
        // binary_slice_to_file(&keypair_bytes, keypair_path.as_str()).expect("error writing file");
        // Ok(keypair.to_bytes().to_vec())
    }

    fn get_public(self) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        Ok(user.pubkey)
        // let pub_path: String = format!("./db/{}.pub", self.id_);
        // match Path::new(pub_path.as_str()).exists() {
        //     true => {
        //         let pub_bytes = read_file_into_binary_vec(pub_path.as_str()).unwrap();
        //         Ok(pub_bytes.to_vec())
        //     }
        //     false => Err("Please create a wallet first (type /createwallet)".to_string()),
        // }
    }

    fn sign_transaction(self, transaction: Self::Transaction) -> Result<String, Self::Error> {
        let user = User::find(self.username).unwrap();
        //let keypair_path: String = format!("./db/{}.keypair", self.id_);
        //let keypair_bytes = read_file_into_binary_vec(keypair_path.as_str()).unwrap();
        let keypair_bytes = hex::decode(user.keypair).unwrap();
        let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
        let ctx = signing_context(b"Signing");
        let signature: Signature = keypair.sign(ctx.bytes(transaction.as_bytes()));
        Ok(hex::encode(signature.to_bytes()))
    }
}
