use std::process;

use clap::Parser;
use colored::Colorize;
use srm::cli::Commands;
use srm::commands::{list_command, remove_command, restore_command};
use srm::{cli::Cli, config::Config};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
            process::exit(1);
        }
    };

    let result = match cli.command {
        Commands::Remove { duration, files } => remove_command(&config, duration, files).await,
        Commands::Restore { files } => restore_command(files).await,
        Commands::List => list_command().await,
    };

    if let Err(e) = result {
        eprintln!("{}: {}", "Error".red(), e);
        process::exit(1);
    }
}
