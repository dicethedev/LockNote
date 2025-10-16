/*!
# LockNote Storage
Handles reading/writing JSON lockfile using async tokio file operations.
*/

use crate::types::{LockFile, StoredNote};
use serde_json::json;
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::path::Path;

/// Save lockfile asynchronously
pub async fn save_lockfile(path: &str, lockfile: &LockFile) -> io::Result<()> {
    let data = serde_json::to_string_pretty(lockfile).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path).await?;
    file.write_all(data.as_bytes()).await?;
    Ok(())
}

/// Load lockfile asynchronously
pub async fn load_lockfile(path: &str) -> io::Result<LockFile> {
    let mut content = String::new();
    let mut file = File::open(path).await?;
    file.read_to_string(&mut content).await?;
    let obj: LockFile = serde_json::from_str(&content)?;
    Ok(obj)
}

/// Check if lockfile exists
pub fn lockfile_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Add a note
pub fn add_note(lockfile: &mut LockFile, note: StoredNote) {
    lockfile.notes.push(note);
}

/// Remove a note by id
pub fn remove_note(lockfile: &mut LockFile, id: &str) -> bool {
    let before = lockfile.notes.len();
    lockfile.notes.retain(|n| n.id != id);
    lockfile.notes.len() < before
}

/// Search notes by keyword in ciphertext (needs decryption externally)
pub fn search_notes(lockfile: &LockFile, keyword: &str, decrypt_fn: &dyn Fn(&str) -> Option<String>) -> Vec<String> {
    lockfile.notes.iter()
        .filter_map(|note| {
            if let Some(content) = decrypt_fn(&note.ciphertext) {
                if content.contains(keyword) { Some(note.id.clone()) } else { None }
            } else { None }
        })
        .collect()
}


