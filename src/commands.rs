use std::{fs, path::PathBuf, time::Duration};

use chrono::Utc;
use colored::Colorize;

use crate::{
    config::Config,
    storage::{SafeFile, StorageManager},
    utils::parse_duration,
};

pub async fn remove_command(
    config: &Config,
    duration_str: Option<String>,
    files: Vec<String>,
) -> Result<(), String> {
    let mut storage = StorageManager::new()?;

    let duration_str = duration_str
        .as_deref()
        .or(config.default_duration.as_deref());
    let duration = match duration_str {
        Some(s) => match parse_duration(s) {
            Ok(d) => d,
            Err(e) => return Err(format!("Error parsing duration '{}': {}", s, e)),
        },
        None => Duration::from_secs(7 * 24 * 60 * 60), // 7 days
    };

    storage.cleanup()?;

    for file in files {
        let path = PathBuf::from(&file);
        if !path.exists() {
            eprintln!("{}: File does not exist: {}", "Error".red(), file);
            continue;
        }

        let file_name = match path.file_name() {
            Some(name) => name,
            None => {
                eprintln!("{}: Invalid file name:  {}", "Error".red(), file);
                continue;
            }
        };
        let mut moved_path = storage.safe_dir.join(file_name);

        if moved_path.exists() {
            let timestamp = Utc::now().format("%Y%m%d%H%M%S");
            let stem = path.file_stem().unwrap().to_string_lossy();
            let ext = path
                .extension()
                .map_or_else(|| "".to_string(), |e| format!(".{}", e.to_string_lossy()));
            let new_name = if ext.is_empty() {
                format!("{}_{}", stem, timestamp)
            } else {
                format!("{}_{}.{}", stem, timestamp, ext)
            };
            moved_path = storage.safe_dir.join(new_name);
        }

        if let Err(e) = fs::rename(&path, &moved_path) {
            eprintln!(
                "{}: Failed to move '{}' to safe storage: {}",
                "Error".red(),
                file,
                e
            );
            continue;
        }

        let deleted_at =
            Utc::now() + chrono::Duration::from_std(duration).map_err(|e| e.to_string())?;

        let safe_file = SafeFile {
            original_path: path.clone(),
            moved_path: moved_path.clone(),
            deleted_at,
        };
        storage.add_file(safe_file);

        println!(
            "{} '{}' {}",
            "Moved".green(),
            file,
            format!("to safe storage. It will be deleted at {}", deleted_at).blue()
        );
    }

    storage.save_metadata()?;

    Ok(())
}

pub async fn restore_command(files: Vec<String>, restore_all: bool) -> Result<(), String> {
    let mut storage = StorageManager::new()?;

    // Perform cleanup before restoring
    storage.cleanup()?;

    let files_to_remove = if restore_all {
        storage.get_safe_files().iter().map(|f| f.moved_path.file_name().unwrap().to_string_lossy().to_string()).collect()
    } else {
        files
    };


    // Process each file before restoring
    for file_name in files_to_remove.iter() {
        let safe_file = match storage.find_safe_file(file_name) {
            Some(f) => f.clone(),
            None => {
                eprintln!(
                    "{}: File not found in safe storage: {}",
                    "Error".red(),
                    file_name
                );
                continue;
            }
        };

        // Ensure parent directory exists
        if let Some(parent) = safe_file.original_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!(
                        "{}: Failed to create parent directory for '{}': {}",
                        "Error".red(),
                        file_name,
                        e
                    );
                    continue;
                }
            }
        }

        // Restore the file
        if let Err(e) = fs::rename(&safe_file.moved_path, &safe_file.original_path) {
            eprintln!(
                "{}: Failed to restore '{}' from safe storage: {}",
                "Error".red(),
                file_name,
                e
            );
            continue;
        }

        println!("{} '{}'", "Restored".green(), file_name);

        // Remove the file from the safe storage metadata
        storage.remove_file(&safe_file.moved_path);
    }

    storage.save_metadata()?;

    Ok(())
}

pub async fn list_command() -> Result<(), String> {
    let mut storage = StorageManager::new()?;

    // Perform cleanup before listing
    storage.cleanup()?;

    storage.list_files();

    Ok(())
}
