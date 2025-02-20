use clap::{arg, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(name = "srm", version = "1.0")]
#[command(author = "larpi")]
#[command(
    about = "Securely remove files or directories",
    long_about = "This command removes files or directories by overwriting them with random data."
)]
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
        /// Restore all files
        #[arg(short, long)]
        all: bool,

        /// Files to restore (specify file names as listed in storage)
        #[arg(value_name = "FILE")]
        files: Vec<String>,
    },

    /// List all files stored in the safe storage
    List,

    /// Clean the safe storage by removing expired files
    Clean {
        /// Force clean without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// View the contents of files
    View {
        /// File to view
        #[arg(required = true, value_name = "FILE")]
        files: Vec<String>,
    },

    /// Configure the application
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Start the daemon to monitor trash files
    Daemon {
        /// Check interval in seconds
        #[arg(short, long, default_value = "300")]
        interval: u64,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Set a configuration key to a specific value
    Set {
        /// Configuration key to set (e.g., "default_duration")
        #[arg(value_name = "KEY")]
        key: String,

        /// Value to assign to the key (e.g., "7d")
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Get the value of a configuration key
    Get {
        /// Configuration key to retrieve
        #[arg(value_name = "KEY")]
        key: String,
    },
}
