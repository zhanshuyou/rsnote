pub mod cli;
pub mod config;
pub mod note;

use clap::{Parser, Subcommand};

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

    /// Search for notes
    Search {
        /// Search query
        keyword: String,
    },
    Update {
        identifier: String,
        content: Option<String>,
    },
    ClearConfig,
}
