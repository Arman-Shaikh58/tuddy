mod command_line;
mod config;
mod database;
mod error_handler;
mod utils;

use chrono::Local;
use clap::Parser;
use command_line::MyArgs;

use crate::{
    command_line::Commands,
    config::{load_config, set_database_url},
    database::{delete_entry, get_all_entries, get_entries_by_date, init_db, insert_entry},
    error_handler::AppError,
    utils::{is_valid_date, parse_date, prompt, prompt_multiline},
};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), AppError> {
    let cli = MyArgs::parse();

    match cli.command {
        Commands::Set { database_url } => {
            set_database_url(database_url).await?;
        }

        Commands::Listen { title } => {
            ensure_configured().await?;

            let title = match title {
                Some(val) if !val.is_empty() => val,
                _ => prompt("What's on your mind? (title): "),
            };

            if title.is_empty() {
                return Err(AppError::Utility("Title cannot be empty".to_string()));
            }

            let body = prompt_multiline("Tell me more... (press Enter twice when done):");

            if body.is_empty() {
                return Err(AppError::Utility("Entry body cannot be empty".to_string()));
            }

            insert_entry(&title, &body).await?;

            let today = Local::now().format("%Y-%m-%d");
            println!();
            println!("Got it! I'll remember this for {}.", today);
            println!(" \"{}\"", title);
        }

        Commands::Tell { date } => {
            ensure_configured().await?;

            match date {
                Some(mut date_str) => {
                    // validate_date
                    while !is_valid_date(&date_str) {
                        eprintln!(" Invalid date format. Use YYYY-MM-DD (e.g. 2025-07-04)");
                        date_str = prompt("Enter date: ");
                    }
                    let date = parse_date(&date_str)?;
                    let entries = get_entries_by_date(&date).await?;

                    if entries.is_empty() {
                        println!("No entries found for {}.", date_str);
                    } else {
                        println!("Entries for {}:", date_str);
                        println!("{}", "─".repeat(50));
                        for entry in &entries {
                            print_entry(entry);
                        }
                        println!("{}", "─".repeat(50));
                        println!("   {} entry(ies) found.", entries.len());
                    }
                }
                None => {
                    let entries = get_all_entries().await?;

                    if entries.is_empty() {
                        println!("No entries yet. Use `tuddy listen` to write your first one!");
                    } else {
                        println!("All entries:");
                        println!("{}", "─".repeat(50));
                        for entry in &entries {
                            print_entry(entry);
                        }
                        println!("{}", "─".repeat(50));
                        println!("   {} entry(ies) total.", entries.len());
                    }
                }
            }
        }

        Commands::Forget { id } => {
            ensure_configured().await?;

            let deleted = delete_entry(id).await?;
            if deleted {
                println!("Entry #{} has been forgotten.", id);
            } else {
                println!("No entry found with ID #{}.", id);
            }
        }
    }

    Ok(())
}

async fn ensure_configured() -> Result<(), AppError> {
    let loaded = load_config()?;
    if !loaded {
        return Err(AppError::Config(
            "Database not configured. Run `tuddy set <database_url>` first.".to_string(),
        ));
    }
    init_db().await?;
    Ok(())
}

fn print_entry(entry: &database::JournalEntry) {
    println!();
    println!("  #{} │ {} │ {}", entry.id, entry.created_at, entry.title);
    for line in entry.body.lines() {
        println!("       │ {}", line);
    }
}
