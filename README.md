# 🔐 LockNote

LockNote is a **secure local note manager** written in Rust.  
It uses **Argon2 password hashing** and **AES-GCM encryption** to safely store your private notes, only you can read them.

---

## 🚀 Features

- 🧠 Password-based encryption using Argon2 + AES-GCM 
- 🔒 AES-256 encryption for every note  
- 📝 Create, list, view, and delete encrypted notes  
- 🔍 Full-text search across notes (decrypted in-memory only)  
- ⚡ Asynchronous file operations via Tokio  
- 💾 Stored in a simple JSON lockfile and locally in your home directory (`~/.locknote.json`) 

---

## Built With
- [Rust](https://www.rust-lang.org/)
- [AES-GCM](https://crates.io/crates/aes-gcm)
- [Argon2](https://crates.io/crates/argon2)
- [Serde](https://crates.io/crates/serde)
- [assert_cmd](https://crates.io/crates/assert_cmd)

---

## 🛠️ Installation

```bash
git clone https://github.com/dicethedev/LockNote.git
cd LockNote
cargo build --release
```

## How to Use it

```bash
locknote init
```
Creates a new encrypted lockfile (locknote.json).

### Add a Note

```bash
locknote add
```
You’ll be prompted for a title and content.

### List Notes

```bash
locknote list
```
Shows IDs and titles of all notes.

### View a Note

```bash
locknote view <id>
```
Decrypts and prints a specific note.

### Delete a Note

```bash
locknote delete <id>
```
Removes a note from your lockfile

### Search Notes

```bash
locknote search <keyword>
```
Finds all notes containing the keyword in decrypted text.

## 🔒 Security Model

- Each lockfile is encrypted using `AES-GCM` with a key derived from your password via `Argon2`.
- Notes are decrypted only in memory during viewing/searching.
- `JSON` structure makes backups and portability simple.

## Running Tests

```bash
cargo test
```
You can write CLI tests in tests/cli_tests.rs using assert_cmd.

## Workflow EXample

```bash
locknote init
locknote add
locknote list
locknote view 123e4567-e89b-12d3-a456-426614174000
locknote delete 123e4567-e89b-12d3-a456-426614174000
locknote search password
```
or 

```bash
locknote init          # Initialize a secure note store
locknote add "My note" # Add a note
locknote list          # List all notes
locknote view 1        # View a note
locknote delete 1      # Delete a note
```

## Distribution

You can copy the built binary anywhere and use it standalone

```bash
cp target/release/locknote/usr/local/bin
```

## Future Improvements

- Password-based encryption for each session
- Configurable note directories
- Cross-platform desktop version


### Author

Author

Built with 🦀 by `Blessing Samuel`
