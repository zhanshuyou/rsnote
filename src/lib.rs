mod config;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use clap::{Parser, Subcommand};

pub const NOTES_DIR: &str = "rsnotes";

#[derive(Parser)]
#[command(name = "Note App")]
#[command(version = "1.0")]
#[command(about = "A simple command-line note-taking application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new note
    New {
        /// Note title
        title: String,
        /// Note content (optional, can be entered interactively)
        content: Option<String>,
    },
    /// List all notes
    List,
    /// Show a specific note
    Show {
        /// Note title or ID
        identifier: String,
    },
    /// Delete a note
    Delete {
        /// Note title or ID
        identifier: String,
    },
    /// Search notes by keyword
    Search {
        /// Keyword to search for
        keyword: String,
    },
}