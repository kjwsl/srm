use clap::{arg, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "srm", version = "1.0")]
#[command(author = "larpi")]
#[command(about = "Securely remove files or directories", long_about = "This command removes files or directories by overwriting them with random data.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Safely remove files by moving them to a safe storage
    Remove {
        /// Retention duration for the removed files (e.g., "7d", "12h", "30m")
        #[arg(short, long, value_name = "DURATION")]
        duration: Option<String>,

        /// Files to remove
        #[arg(required = true, value_name = "FILE")]
        files: Vec<String>,
    },

    /// Restore previously removed files to their original locations
    Restore {
        /// Files to restore (specify file names as listed in storage)
        #[arg(required = true, value_name = "FILE")]
        files: Vec<String>,
    },

    /// List all files stored in the safe storage
    List,
}
