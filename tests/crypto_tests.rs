use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    self,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::RngCore;
use std::error::Error;

/// Generates a new random 32-byte key for AES-256-GCM.
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Encrypts plaintext using AES-256-GCM.
/// Returns a tuple of (nonce, ciphertext).
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    // Generate random 12-byte nonce (unique per encryption)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failure!");
    (nonce_bytes.to_vec(), ciphertext)
}

/// Decrypts ciphertext using AES-256-GCM.
pub fn decrypt(ciphertext: &[u8], nonce_bytes: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| format!("DEcryption failed: {e}"))?;
    Ok(plaintext)
}

/// Hashes a password using Argon2 for secure authentication.
pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Password hashing failed: {e}"))?  //convert to String
        .to_string();
    Ok(hash)
}

/// Verifies a password against its Argon2 hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| format!("Invalid hash format: {e}"))?; //convert to String
    let argon2 = Argon2::default();

    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
