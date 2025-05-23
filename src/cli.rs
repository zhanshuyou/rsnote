use clap::Parser;
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
            app.show_note(&identifier)?;
        }
        Commands::Delete { identifier } => {
            app.delete_note(&identifier)?;
        }
        Commands::Search { keyword } => {
            app.search_notes(&keyword)?;
        }
    }

    Ok(())
}