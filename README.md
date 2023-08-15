# file-encryption-decryption

## Description

This is a simple file encryption/decryption program written in Rust. It is using the XChaCha20-Poly1305 AEAD cipher. The
key is randomly generated as is the nonce. Please don't use this for anything serious. It is just a fun project to learn
Rust. It is not ready for production use.

## Usage

Edit the text you want to encrypt/decrypt in the plaintext.txt file. Then run the program with:

```
cargo run
```
