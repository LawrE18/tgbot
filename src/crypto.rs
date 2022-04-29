extern crate rand;
extern crate ed25519_dalek;
extern crate hex;

use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signer};
use ed25519_dalek::{PublicKey, SecretKey};
use ed25519_dalek::Signature;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, KEYPAIR_LENGTH, SIGNATURE_LENGTH};
use std::io::{BufWriter, Result, Write};
use std::cmp::max;
use std::str;
use std::error::Error;
use std::path::Path;

pub fn read_file_into_binary_vec(file_path: &str) -> Result<Vec<u8>> {
    std::fs::read(file_path)
}

pub fn get_address(id_: i64) -> String {
    let pub_path: String = format!("./out/{}.pub", id_);
    match Path::new(pub_path.as_str()).exists() {
        true => {
            let pub_bytes = read_file_into_binary_vec(pub_path.as_str()).unwrap();
            let pub_hex = hex::encode(pub_bytes);
            pub_hex
        }
        false => {
            "Please create a wallet first (type /createwallet)".to_string()
        }
    }
}

fn binary_slice_to_file(data: &[u8], file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    let file = std::fs::File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(data)
}

pub fn gen_key_pair(id_: i64) -> [u8; PUBLIC_KEY_LENGTH] {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key: PublicKey = keypair.public;
    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = public_key.to_bytes();
    // let secret_key_bytes: [u8; SECRET_KEY_LENGTH] = keypair.secret.to_bytes();
    let keypair_bytes = keypair.to_bytes();
    let pub_path: String = format!("./out/{}.pub", id_);
    binary_slice_to_file(&public_key_bytes, pub_path.as_str());
    let keypair_path: String = format!("./out/{}.keypair", id_);
    binary_slice_to_file(&keypair_bytes, keypair_path.as_str());
    public_key_bytes
}


pub fn sign(id_: i64, data: String) -> [u8; SIGNATURE_LENGTH]{
    let keypair_path: String = format!("./out/{}.keypair", id_);
    let keypair_bytes = read_file_into_binary_vec(keypair_path.as_str()).unwrap();
    let keypair: Keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
    let signature: Signature = keypair.sign(data.as_bytes());
    signature.to_bytes()
}