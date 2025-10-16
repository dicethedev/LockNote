/*!
# LockNote Crypto Module

Handles all cryptographic operations for LockNote.

## Features
- Password hashing & verification using **Argon2**
- Encryption/Decryption of notes using **AES-256-GCM**
- Safe **Base64** encoding/decoding helpers
- Secure zeroization of key material to prevent leaks

Each note is encrypted with:
- A unique AES-GCM nonce
- A random 32-byte key derived from user input or generated securely
*/

use aes_gcm::{aead::Aead, Aes256Gcm, Key, Nonce, KeyInit};
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher, PasswordVerifier}, Algorithm, Params, Version};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use zeroize::Zeroize;

pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 12;

/// Derive a 32-byte AES key from password and salt using Argon2id
pub fn derive_key(password: &str, salt: &str) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());
    argon2
        .hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key)
        .expect("Key derivation failed");
    key
}

/// Hash a password using Argon2, returning (salt, hash)
pub fn hash_password(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();
    (salt.as_str().to_string(), hash)
}

/// Verify a password against the stored Argon2 hash
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = argon2::password_hash::PasswordHash::new(hash).expect("Invalid stored hash");
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

/// Generate a random AES-GCM key (32 bytes)
pub fn generate_key() -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

/// Encrypt a plaintext note using AES-256-GCM
pub fn encrypt(plaintext: &[u8], key_bytes: &[u8; KEY_LEN]) -> (Vec<u8>, Vec<u8>) {
    let mut nonce = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce);

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .expect("AES-GCM encryption failed");

    (nonce.to_vec(), ciphertext)
}

/// Decrypt AES-GCM ciphertext
// pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key_bytes: &[u8; KEY_LEN]) -> Option<Vec<u8>> {
//     let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));
//     cipher.decrypt(Nonce::from_slice(nonce), ciphertext).ok()
// }
pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key_bytes: &[u8; KEY_LEN]) -> Option<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));
    cipher.decrypt(Nonce::from_slice(nonce), ciphertext).ok()
}


/// Base64 encode bytes
pub fn b64_encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

/// Base64 decode string
pub fn b64_decode(s: &str) -> Option<Vec<u8>> {
    general_purpose::STANDARD.decode(s).ok()
}

/// Securely zeroize key material from memory
pub fn zeroize_key(mut key: [u8; KEY_LEN]) {
    key.zeroize();
}
