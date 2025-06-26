use secp256k1::SecretKey;
use rust_crypto_wallet::{generate_keypair, public_key_to_address};

#[test]
fn test_keypair_and_address() {
    let (sk, pk) = generate_keypair();
    let addr = public_key_to_address(&pk);

    assert_eq!(addr.len() > 0, true);
    // Further tests for address format and key validity can be added here
}
