use rust_crypto_wallet::*;

fn main() {
    let (secret, public) = generate_keypair();
    let address = public_key_to_address(&public);

    println!("Address: {}", address);
}
