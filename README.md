# 🦀 rust-crypto-wallet

`rust-crypto-wallet` is a lightweight and secure cryptocurrency wallet built in Rust. It enables the generation of private/public key pairs using the `secp256k1` elliptic curve, derives wallet addresses in a Bitcoin-like format, and allows users to sign and verify messages.

This project is designed as a minimal and beginner-friendly introduction to building wallets and understanding cryptographic operations in Rust. It demonstrates secure key handling, address generation, and digital signatures using modern Rust crates.

---

## 🔐 Features

- ✅ Generate ECDSA key pairs (secp256k1 curve)
- ✅ Derive wallet addresses (SHA-256 + RIPEMD-160 + Base58Check)
- ✅ Sign and verify messages
- ✅ Command-line output for testing and learning

---

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** – Systems programming language
- [`secp256k1`](https://docs.rs/secp256k1) – Bitcoin/Ethereum ECDSA signing
- `sha2`, `ripemd160` – Hashing algorithms for wallet address derivation
- `base58` – Base58Check address encoding

---

## 🚀 Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install)

### Clone the Repository

```bash
git clone https://github.com/yourusername/rust-crypto-wallet.git
cd rust-crypto-wallet
