use locknote::storage::{save_lockfile, load_lockfile, add_note, remove_note};
use locknote::types::{LockFile, StoredNote};
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_save_and_load_lockfile() {
    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_str().unwrap();

    let lockfile = LockFile { notes: vec![], salt: "test".to_string() };
    save_lockfile(path, &lockfile).await.unwrap();

    let loaded = load_lockfile(path).await.unwrap();
    assert_eq!(loaded.salt, "test");
}

#[test]
fn test_add_and_remove_note() {
    let mut lockfile = LockFile { notes: vec![], salt: "abc".to_string() };

    let note = StoredNote {
        id: "1".to_string(),
        nonce: "randomnonce".to_string(), 
        ciphertext: "xyz".to_string(),
    };

    add_note(&mut lockfile, note.clone());
    assert_eq!(lockfile.notes.len(), 1);

    let removed = remove_note(&mut lockfile, "1");
    assert!(removed);
    assert!(lockfile.notes.is_empty());
}