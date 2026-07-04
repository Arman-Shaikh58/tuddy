use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tuddy")]
#[command(version, about = "A buddy always ready to listen to you 🐻")]
pub struct MyArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure the database connection URL (stored securely in system keyring)
    Set {
        /// The PostgreSQL connection URL (e.g. postgres://user:pass@localhost/tuddy)
        database_url: Option<String>,
    },

    /// Write a new journal entry — tuddy is listening!
    Listen {
        /// Title for the journal entry
        title: Option<String>,
    },

    /// Recall entries from a specific date
    Tell {
        /// Date to look up entries for (YYYY-MM-DD format). Omit to see all entries.
        date: Option<String>,
    },

    /// Delete a journal entry by its ID
    Forget {
        /// The entry ID to delete
        id: i32,
    },
}
