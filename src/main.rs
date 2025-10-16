/*!
# LockNote Main
Glue together modules:
- Async storage
- Crypto for Argon2 + AES-GCM
- CLI commands
- Password prompts
- Search & delete
*/

mod cli;
mod crypto;
mod storage;
mod types;

use crate::cli::{Cli, Commands};
use crate::crypto::*;
use crate::storage::*;
use crate::types::{LockFile, StoredNote};
use clap::Parser;
use hex;
use rpassword::prompt_password;
use std::io::{self, Read, Write};
use uuid::Uuid;

const DEFAULT_FILE: &str = "locknote.json";

fn ask_password(prompt: &str) -> String {
    prompt_password(prompt).unwrap_or_else(|_| "".to_string())
}

fn ask_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string()
}

#[tokio::main]
async fn main() {
    let cli = Cli::try_parse().unwrap();

    match &cli.command {
        Commands::Init { file } => {
            let path = file.as_deref().unwrap_or(DEFAULT_FILE);
            if lockfile_exists(path) {
                eprintln!("Lock file already exists at {path}");
                return;
            }

            let pass = ask_password("Set master password: ");
            let (salt, _) = hash_password(&pass);
            let lockfile = LockFile::new(salt.clone());

            save_lockfile(path, &lockfile)
                .await
                .expect("Failed to save lockfile");
            println!("Initialized lockfile at {path}");
        }

        Commands::Add { file } => {
            let path = file.as_deref().unwrap_or(DEFAULT_FILE);
            if !lockfile_exists(path) {
                eprintln!("Lock file not found. Run `locknote init` first.");
                return;
            }

            let mut lockfile = load_lockfile(path).await.expect("Failed to load lockfile");
            let pass = ask_password("Enter master password: ");
            let key = derive_key(&pass, &lockfile.salt);

            let title = ask_input("Title: ");
            println!("Enter note content (end with Ctrl+D):");

            let mut content = String::new();
            io::stdin().read_to_string(&mut content).unwrap();

            // Combine title and body before encrypting
            let combined = format!("{}\n{}", title, content.trim());
            let (nonce, ciphertext) = encrypt(combined.as_bytes(), &key);

            let note = StoredNote {
                id: Uuid::new_v4().to_string(),
                nonce: b64_encode(&nonce),
                ciphertext: b64_encode(&ciphertext),
            };

            add_note(&mut lockfile, note);
            save_lockfile(path, &lockfile)
                .await
                .expect("Failed to save note");
            println!("Note added!");
        }

        Commands::List { file } => {
            let path = file.as_deref().unwrap_or(DEFAULT_FILE);
            let lockfile = load_lockfile(path).await.expect("Failed to load lockfile");

            println!("Stored Notes:");
            for note in &lockfile.notes {
                println!("- {}", note.id);
            }
        }

        Commands::View { id, file } => {
            let path = file.as_deref().unwrap_or(DEFAULT_FILE);
            let lockfile = load_lockfile(path).await.expect("Failed to load lockfile");
            let pass = ask_password("Enter master password: ");
            let key = derive_key(&pass, &lockfile.salt);
            let key_array: [u8; 32] = key.clone().try_into().expect("Key must be 32 bytes");

            if let Some(note) = lockfile.notes.iter().find(|n| &n.id == id) {
                let nonce_bytes = hex::decode(&note.nonce).expect("Invalid nonce encoding");
                let ciphertext_bytes =
                    hex::decode(&note.ciphertext).expect("Invalid ciphertext encoding");

                let plaintext = decrypt(&ciphertext_bytes[..], &nonce_bytes[..], &key)
                    .expect("Decryption failed");

                let text = String::from_utf8_lossy(&plaintext);
                println!("Note ID: {}\n\n{}", note.id, text);
            } else {
                println!("Note not found");
            }
        }

        Commands::Delete { id, file } => {
            let path = file.as_deref().unwrap_or(DEFAULT_FILE);
            let mut lockfile = load_lockfile(path).await.expect("Failed to load lockfile");

            if remove_note(&mut lockfile, id) {
                save_lockfile(path, &lockfile)
                    .await
                    .expect("Failed to save lockfile");
                println!("Note deleted.");
            } else {
                println!("Note ID not found.");
            }
        }

        Commands::Search { keyword, file } => {
            let path = file.as_deref().unwrap_or(DEFAULT_FILE);
            let lockfile = load_lockfile(path).await.expect("Failed to load lockfile");
            let pass = ask_password("Enter master password: ");
            let key = derive_key(&pass, &lockfile.salt);

            let matches: Vec<String> = lockfile
                .notes
                .iter()
                .filter_map(|n| {
                    let nonce_bytes = hex::decode(&n.nonce).ok()?;
                    let ciphertext_bytes = hex::decode(&n.ciphertext).ok()?;
                    let plaintext = decrypt(&ciphertext_bytes[..], &nonce_bytes[..], &key)
                        .expect("Decryption failed");
                    let content = String::from_utf8_lossy(&plaintext);
                    if content.contains(keyword) {
                        Some(n.id.clone())
                    } else {
                        None
                    }
                })
                .collect();

            if matches.is_empty() {
                println!("No matches found for '{keyword}'");
            } else {
                println!("Found matches in IDs:");
                for id in matches {
                    println!("- {id}");
                }
            }
        }
    }
}
