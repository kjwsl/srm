use std::process;

use clap::Parser;
use colored::Colorize;
use safe_remove::cli::Commands;
use safe_remove::commands::{
    clean_command, list_command, remove_command, restore_command, view_command,
};
use safe_remove::{cli::Cli, config::Config};
use safe_remove::daemon::start_daemon;

#[tokio::main]
async fn main() -> Result<(), String> {
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
        Commands::Restore { files, all } => restore_command(files, all).await,
        Commands::List => list_command().await,
        Commands::Clean { force } => clean_command(force).await,
        Commands::View { files } => view_command(files).await,
        Commands::Config { action } => match action {
            safe_remove::cli::ConfigAction::Set { key, value } => {
                config.set(key, value).map(|_| ())
            }
            safe_remove::cli::ConfigAction::Get { key } => config.get(key),
        },
        Commands::Daemon { interval } => start_daemon(interval).await,
    };

    if let Err(e) = result {
        eprintln!("{}: {}", "Error".red(), e);
        process::exit(1);
    }
    Ok(())
}
