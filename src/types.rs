use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredNote {
    pub id: String,
    pub nonce: String, // base64
    pub ciphertext: String, //base64 (It contain both tittle & body)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFile {
    pub salt: String,  // base64 salt for PBKDF2 (per-file)
    pub notes: Vec<StoredNote>
}

impl LockFile {
    pub fn new(salt_b64: String) -> Self {
        Self {
            salt: salt_b64,
            notes: Vec::new(),
        }
    }
}