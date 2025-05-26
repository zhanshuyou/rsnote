use rsnote_cli::config::Config;
use rsnote_cli::note::{NoteApp, NoteError, NoteMetadata};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

fn create_test_note_app(notes_dir: &PathBuf) -> NoteApp {
    // Save a test config pointing to the temp dir
    let config = Config {
        notes_dir: notes_dir.clone(),
    };
    config.save().unwrap();
    NoteApp {
        notes_dir: notes_dir.clone(),
    }
}

#[test]
fn test_create_and_list_note() {
    let dir = tempdir().unwrap();
    let notes_dir = dir.path().join("notes");
    fs::create_dir_all(&notes_dir).unwrap();
    let app = create_test_note_app(&notes_dir);

    // Create a note
    let title = "TestNote";
    let content = Some("This is a test note.".to_string());
    app.create_note(title, content.clone()).unwrap();

    // List notes
    let notes = app.list_notes().unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].title, title);

    // Show note content
    let shown = app.show_note(title).unwrap();
    assert!(shown.contains("This is a test note."));
}
