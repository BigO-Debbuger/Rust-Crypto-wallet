# Design Document - Rust Crypto Wallet

## Overview

This project implements a simple cryptocurrency wallet in Rust using secp256k1 for key generation and signing, with Bitcoin-style address derivation.

## Cryptography

- Private/Public keys use the secp256k1 curve.
- Address derived by SHA256 + RIPEMD160 hashing of compressed public key.
- Address encoded with Base58Check encoding (Bitcoin mainnet prefix 0x00).

## Project Structure

- `src/main.rs` - main wallet logic
- `examples/` - usage examples
- `tests/` - integration and unit tests

## Future Work

- Add Ethereum address support (Keccak256 + EIP-55)
- Encrypted key storage
- Transaction building and broadcasting
- CLI interface
