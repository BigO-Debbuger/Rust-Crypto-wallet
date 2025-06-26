//! rust-crypto-wallet: Minimal cryptocurrency wallet demo.
//! Features: key generation, address derivation, and signing.

use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use ripemd160::Ripemd160;
use base58::ToBase58;

fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::default();
    let secret_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    (secret_key, public_key)
}

fn public_key_to_address(public_key: &PublicKey) -> String {
    let serialized = public_key.serialize();
    let sha256 = Sha256::digest(&serialized);
    let ripemd = Ripemd160::digest(&sha256);

    let mut address_bytes = vec![0x00]; // Bitcoin mainnet prefix
    address_bytes.extend(&ripemd);

    let checksum = Sha256::digest(&Sha256::digest(&address_bytes));
    address_bytes.extend(&checksum[0..4]);

    address_bytes.to_base58()
}

fn main() {
    let (secret_key, public_key) = generate_keypair();
    let address = public_key_to_address(&public_key);

    println!("🔐 Private Key: {}", secret_key.display_secret());
    println!("🔓 Public Key:  {}", public_key);
    println!("🏷️ Wallet Address: {}", address);
}
