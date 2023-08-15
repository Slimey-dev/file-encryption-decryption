use std::fs::OpenOptions;
use std::io::prelude::*;

use anyhow::Result;
use chacha20poly1305::{aead::Aead, KeyInit, XChaCha20Poly1305, XNonce};
use rand_core::{OsRng, RngCore};

fn create_key_and_cipher() -> (XChaCha20Poly1305, [u8; 24]) {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut nonce);

    (XChaCha20Poly1305::new(&key.into()), nonce)
}

fn encrypt_content(cipher: XChaCha20Poly1305, nonce: &[u8; 24], contents: &[u8]) -> Vec<u8> {
    cipher
        .encrypt(&XNonce::from_slice(nonce), contents)
        .expect("Encryption failure!")
}

fn decrypt_content(cipher: XChaCha20Poly1305, nonce: &[u8; 24], contents: &[u8]) -> Vec<u8> {
    cipher
        .decrypt(&XNonce::from_slice(nonce), contents)
        .expect("Decryption failure!")
}

fn read_from_file(file_name: &str) -> Result<Vec<u8>> {
    let mut file_contents = Vec::new();
    OpenOptions::new()
        .read(true)
        .open(file_name)?
        .read_to_end(&mut file_contents)?;
    Ok(file_contents)
}

fn write_to_file(file_name: &str, contents: &[u8]) -> Result<()> {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file_name)?
        .write_all(contents)?;
    Ok(())
}

fn main() -> Result<()> {
    let (cipher, nonce) = create_key_and_cipher();
    let cipher_clone = cipher.clone();

    let file_contents = read_from_file("plaintext.txt")?;
    let ciphertext = encrypt_content(cipher, &nonce, &file_contents);
    write_to_file("ciphertext.txt", &ciphertext)?;

    let encrypted_file_contents = read_from_file("ciphertext.txt")?;
    let decrypted_text = decrypt_content(cipher_clone, &nonce, &encrypted_file_contents);
    write_to_file("decrypted.txt", &decrypted_text)?;

    Ok(())
}
