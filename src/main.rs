use std::fs;

use rsnote::{NOTES_DIR};

fn main() {
    // Ensure notes directory exists
    if let Err(e) = fs::create_dir_all(NOTES_DIR) {
        eprintln!("Failed to create notes directory: {}", e);
        return;
    }
}
