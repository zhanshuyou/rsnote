use clap::Parser;
use crate::config::Config;
use crate::{Cli, Commands};
use crate::note::NoteApp;


pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let app = NoteApp::new()?;  // 现在返回 Result

    match cli.command {
        Commands::New { title, content } => {
            app.create_note(&title, content)?;
        }
        Commands::List => {
            let notes = app.list_notes()?;
            if notes.is_empty() {
                println!("No notes found.");
            } else {
                println!("{:<4} | {:<30} | {:<19} | {:<19}", "ID", "Title", "Created", "Last Updated");
                println!("{}", "-".repeat(80));
                for note in notes {
                    println!("{:<4} | {:<30} | {:<19} | {:<19}",
                        note.id,
                        note.title.chars().take(30).collect::<String>(),
                        note.created,
                        note.last_updated
                    );
                }
            }
        }
        Commands::Show { identifier } => {
            let content = app.show_note(&identifier)?;
            println!("{}", content);
        }
        Commands::Delete { identifier } => {
            app.delete_note(&identifier)?;
        }
        Commands::Update { identifier, content } => {
            app.update_note(&identifier, content)?;
        }
        Commands::Search { keyword } => {
            let results = app.search_notes(&keyword)?;
            if results.is_empty() {
                println!("No notes found.");
            } else {
                println!("{:<4} | {:<30} | {:<19}", "ID", "Title", "Last Updated");
                println!("{}", "-".repeat(80));
                for result in results {
                    println!("{:<4} | {:<30} | {:<19}", result.note.id, result.note.title, result.note.last_updated)
                }
            }
        }
        Commands::ClearConfig => {
            Config::clear_config()?;
        }
    }

    Ok(())
}