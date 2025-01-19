use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Structure to represent a safely removed file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafeFile {
    pub original_path: PathBuf,
    pub moved_path: PathBuf,
    pub deleted_at: DateTime<Utc>,
}

/// StorageManager handles all operations related to safe storage
pub struct StorageManager {
    pub safe_dir: PathBuf,
    pub metadata_file: PathBuf,
    pub safe_files: Vec<SafeFile>,
}

impl StorageManager {
    pub fn new() -> Result<Self, String> {
        let proj_dirs = directories::ProjectDirs::from("com", "larpi", "srm")
            .ok_or_else(|| "Cannot determine project directories".to_string())?;
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir)
            .map_err(|e| format!("Failed to create safe directory: {}", e))?;

        let metadata_file = data_dir.join("metadata.yaml");
        let safe_files: Vec<SafeFile> = if metadata_file.exists() {
            let contents =
                fs::read_to_string(&metadata_file).expect("Failed to read metadata file");
            serde_yaml::from_str(&contents).expect("Failed to parse metadata file")
        } else {
            Vec::new()
        };

        Ok(StorageManager {
            safe_dir: data_dir.to_path_buf(),
            metadata_file,
            safe_files,
        })
    }

    pub fn save_metadata(&self) -> Result<(), String> {
        let serialized = serde_yaml::to_string(&self.safe_files)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
        fs::write(&self.metadata_file, serialized)
            .map_err(|e| format!("Failed to write metadata: {}", e))
    }

    pub fn add_file(&mut self, safe_file: SafeFile) {
        self.safe_files.push(safe_file);
    }

    /// Remove a file from the safe storage metadata
    pub fn remove_file(&mut self, moved_path: &Path) {
        self.safe_files.retain(|f| f.moved_path != moved_path);
    }

    pub fn find_safe_file(&self, file_name: &str) -> Option<&SafeFile> {
        self.safe_files.iter().find(|f| {
            f.moved_path
                .file_name()
                .map(|n| n.to_string_lossy() == file_name)
                .unwrap_or(false)
        })
    }

    pub fn cleanup(&mut self) -> Result<(), String> {
        let now = Utc::now();
        let mut updated = Vec::new();

        for file in self.safe_files.iter() {
            if now >= file.deleted_at {
                if let Err(e) = fs::remove_file(&file.moved_path) {
                    eprintln!("Failed to deleted {:?}: {}", file.moved_path, e);
                    updated.push(file.clone());
                } else {
                    println!(
                        "Deleted '{}' from safe storage",
                        file.moved_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                    );
                }
            } else {
                updated.push(file.clone());
            }
        }

        self.safe_files = updated;
        self.save_metadata()
    }

    pub fn get_safe_files(&self) -> Vec<&SafeFile> {
        self.safe_files.iter().collect()
    }

    pub fn list_files(&self) {
        if self.safe_files.is_empty() {
            println!("No files stored in the safe storage");
            return;
        }

        println!(
            "{:<30} {:<50} {:<25}",
            "File Name", "Original Path", "Deleted At"
        );
        println!("{}", "-".repeat(110));

        for file in &self.safe_files {
            let file_name = file
                .moved_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            let original_path = file.original_path.to_string_lossy();
            let deleted_at = file.deleted_at.to_rfc3339();

            println!("{:<30} {:<50} {:<25}", file_name, original_path, deleted_at);
        }
    }
}
