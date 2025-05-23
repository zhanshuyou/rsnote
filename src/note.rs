use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use thiserror::Error;
use crate::config::{Config, ConfigError};

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Note already exists")]
    NoteExists,
    #[error("Note not found")]
    NoteNotFound,
    #[error("Invalid note ID")]
    InvalidId,
}

pub struct NoteApp {
  notes_dir: PathBuf,
}

impl NoteApp {
  /// Create a new NoteApp instance with loaded configuration
  pub fn new() -> Result<Self, ConfigError> {
      let config = Config::load()?;
      Ok(Self {
          notes_dir: config.notes_dir,
      })
  }

  /// Create a new note with given title and optional content
  pub fn create_note(&self, title: &str, content: Option<String>) -> Result<(), NoteError> {
      let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      let path = self.get_note_path(title)?;

      if path.exists() {
          return Err(NoteError::NoteExists);
      }

      let mut file = File::create(&path).map_err(NoteError::Io)?;

      // Write metadata
      writeln!(file, "title: {}", title).map_err(NoteError::Io)?;
      writeln!(file, "created: {}", timestamp).map_err(NoteError::Io)?;
      writeln!(file, "last_updated: {}", timestamp).map_err(NoteError::Io)?;
      writeln!(file, "---").map_err(NoteError::Io)?;

      // Write content
      let content = match content {
          Some(c) => c,
          None => {
              println!("Enter your note content (press Ctrl+D when finished):");
              let mut input = String::new();
              io::stdin().read_to_string(&mut input).map_err(NoteError::Io)?;
              input
          }
      };

      writeln!(file, "{}", content).map_err(NoteError::Io)?;

      println!("Note '{}' created successfully at {}", title, path.display());
      Ok(())
  }

  /// List all notes with their metadata
  pub fn list_notes(&self) -> Result<Vec<NoteMetadata>, NoteError> {
      let entries = fs::read_dir(&self.notes_dir).map_err(NoteError::Io)?;
      let mut notes = Vec::new();

      for (i, entry) in entries.enumerate() {
          let entry = entry.map_err(NoteError::Io)?;
          let path = entry.path();

          if let Some(metadata) = self.get_note_metadata(&path)? {
              notes.push(NoteMetadata {
                  id: i + 1,
                  title: metadata.title,
                  created: metadata.created,
                  last_updated: metadata.last_updated,
                  path,
              });
          }
      }
      
      Ok(notes)
  }

  /// Show the content of a specific note
  pub fn show_note(&self, identifier: &str) -> Result<String, NoteError> {
      let path = self.find_note_path(identifier)?;
      let file = File::open(&path).map_err(NoteError::Io)?;
      let reader = BufReader::new(file);

      let mut content = String::new();
      let mut in_content = false;

      for line in reader.lines() {
          let line = line.map_err(NoteError::Io)?;
          if in_content {
              content.push_str(&line);
              content.push('\n');
          } else if line == "---" {
              in_content = true;
          }
      }

      Ok(content)
  }

  /// Delete a note
  pub fn delete_note(&self, identifier: &str) -> Result<(), NoteError> {
      let path = self.find_note_path(identifier)?;
      fs::remove_file(&path).map_err(NoteError::Io)?;
      Ok(())
  }

  /// Search notes by keyword in title or content
  pub fn search_notes(&self, keyword: &str) -> Result<Vec<NoteSearchResult>, NoteError> {
      let notes = self.list_notes()?;
      let mut results = Vec::new();
      let keyword_lower = keyword.to_lowercase();

      for note in notes {
          // Search in title
          if note.title.to_lowercase().contains(&keyword_lower) {
              results.push(NoteSearchResult {
                  note,
                  match_type: MatchType::Title,
                  preview: None,
              });
              continue;
          }

          // Search in content
          if let Ok(content) = self.show_note(&note.title) {
              if content.to_lowercase().contains(&keyword_lower) {
                  let preview = self.extract_preview(&content, &keyword_lower);
                  results.push(NoteSearchResult {
                      note,
                      match_type: MatchType::Content,
                      preview: Some(preview),
                  });
              }
          }
      }

      Ok(results)
  }

  /// Update an existing note
  pub fn update_note(&self, identifier: &str, new_content: &str) -> Result<(), NoteError> {
      let path = self.find_note_path(identifier)?;
      let metadata = self.get_note_metadata(&path)?;

      let mut file = File::create(&path).map_err(NoteError::Io)?;

      // Write original metadata with updated timestamp
      writeln!(file, "title: {}", metadata.as_ref().unwrap().title).map_err(NoteError::Io)?;
      writeln!(file, "created: {}", metadata.as_ref().unwrap().created).map_err(NoteError::Io)?;
      writeln!(file, "last_updated: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")).map_err(NoteError::Io)?;
      writeln!(file, "---").map_err(NoteError::Io)?;

      // Write new content
      writeln!(file, "{}", new_content).map_err(NoteError::Io)?;

      Ok(())
  }

  // ===== Helper methods =====

  fn get_note_path(&self, title: &str) -> Result<PathBuf, NoteError> {
      let filename = self.sanitize_filename(title);
      Ok(self.notes_dir.join(filename))
  }

  fn find_note_path(&self, identifier: &str) -> Result<PathBuf, NoteError> {
      // Check if identifier is a number (ID)
      if let Ok(id) = identifier.parse::<usize>() {
          let notes = self.list_notes()?;

          if id == 0 || id > notes.len() {
              return Err(NoteError::InvalidId);
          }

          return Ok(notes[id - 1].path.clone());
      }

      // Otherwise treat as title
      let path = self.get_note_path(identifier)?;

      if path.exists() {
          Ok(path)
      } else {
          Err(NoteError::NoteNotFound)
      }
  }

  fn get_note_metadata(&self, path: &PathBuf) -> Result<Option<NoteBasicMetadata>, NoteError> {
      if !path.is_file() {
          return Ok(None);
      }

      let file = File::open(path).map_err(NoteError::Io)?;
      let reader = BufReader::new(file);

      let mut metadata = NoteBasicMetadata {
          title: String::new(),
          created: String::new(),
          last_updated: String::new(),
      };
      let mut found_title = false;
      let mut found_created = false;

      for line in reader.lines() {
          let line = line.map_err(NoteError::Io)?;

          if line.starts_with("title: ") {
              metadata.title = line[7..].to_string();
              found_title = true;
          } else if line.starts_with("created: ") {
              metadata.created = line[9..].to_string();
              found_created = true;
          } else if line.starts_with("last_updated: ") {
              metadata.last_updated = line[14..].to_string();
          } else if line == "---" {
              break;
          }
      }

      if found_title && found_created {
          Ok(Some(metadata))
      } else {
          Ok(None)
      }
  }

  fn sanitize_filename(&self, filename: &str) -> String {
      filename
          .chars()
          .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
          .collect()
  }

  fn extract_preview(&self, content: &str, keyword: &str) -> String {
      if let Some(pos) = content.to_lowercase().find(keyword) {
          let start = pos.saturating_sub(20);
          let end = (pos + keyword.len() + 20).min(content.len());
          let mut preview = content[start..end].to_string();

          if start > 0 {
              preview.insert_str(0, "...");
          }
          if end < content.len() {
              preview.push_str("...");
          }

          preview.replace('\n', " ")
      } else {
          String::new()
      }
  }
}

#[derive(Debug)]
pub struct NoteMetadata {
  pub id: usize,
  pub title: String,
  pub created: String,
  pub path: PathBuf,
  pub last_updated: String,
}

#[derive(Debug)]
struct NoteBasicMetadata {
    title: String,
    created: String,
    last_updated: String,
}

#[derive(Debug)]
pub struct NoteSearchResult {
    pub note: NoteMetadata,
    pub match_type: MatchType,
    pub preview: Option<String>,
}

#[derive(Debug)]
pub enum MatchType {
    Title,
    Content,
}
