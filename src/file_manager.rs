use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::cli::RemoveCond;

// Relative to home directory
const PROG_USER_DATA_DIR: &str = ".local/share/srm";
const METADATA_FILE: &str = "metadata.json";

#[derive(Serialize, Deserialize)]
struct FileMetadata {
    file_name: String,
    size: u64,
    created_at: DateTime<Utc>,
    safe_deleted_at: DateTime<Utc>,
    checksum: String,
    remove_condition: RemoveCond,
}

pub struct FileManager {
    home_dir: PathBuf,
}

impl FileManager {
    pub fn new() -> Result<Self, std::io::Error> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to get home directory")
        })?;

        Ok(Self { home_dir })
    }

    pub fn create_metadata(&self) -> Result<File, std::io::Error> {
        let path = self.get_metadata_file_path();
        if path.exists() {
            return File::open(path);
        }

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        File::create(path)
    }

    pub fn safe_remove(
        &self,
        file: &PathBuf,
        remove_cond: &RemoveCond,
    ) -> Result<(), std::io::Error> {
        if !file.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File does not exist",
            ));
        }

        self.add_to_metadata(file, remove_cond)?;

        // Ensure trash directory exists
        self.ensure_trash_dir()?;

        // Move file to trash
        let to_file = self.get_trash_dir().join(file.file_name().unwrap());
        std::fs::rename(file, to_file)?;

        Ok(())
    }

    fn ensure_trash_dir(&self) -> Result<(), std::io::Error> {
        let trash_dir = self.get_trash_dir();
        if !trash_dir.exists() {
            println!(
                "Creating trash directory: {}",
                trash_dir.as_path().display()
            );
            std::fs::create_dir_all(&trash_dir)?;
        }

        Ok(())
    }

    fn add_to_metadata(
        &self,
        file: &PathBuf,
        remove_cond: &RemoveCond,
    ) -> Result<(), std::io::Error> {
        if !file.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File does not exist",
            ));
        }

        // Load metadata and add new file to it
        let metadata = self.load_metadata()?;

        // Create new metadata entry
        let checksum = Self::compute_checksum(file)?;
        let file_metadata = FileMetadata {
            file_name: file.to_string_lossy().to_string(),
            size: file.metadata()?.len(),
            created_at: file.metadata()?.created()?.into(),
            safe_deleted_at: Utc::now(),
            checksum,
            remove_condition: remove_cond.clone(),
        };

        // Turn metadata into mutable vector and add new file
        let mut metadata = metadata;
        metadata.push(file_metadata);

        // Write metadata back to file
        let metadata_file = self.get_metadata_file()?;

        match serde_json::to_writer(metadata_file, &metadata) {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to write metadata to file: {}", e),
            )),
        }
    }

    pub fn clean_up_files(&self) -> Result<(), std::io::Error> {
        let files_to_remove = self.find_files_to_remove()?;
        for file in files_to_remove {
            std::fs::remove_file(file)?;
        }

        Ok(())
    }

    fn get_metadata_file(&self) -> Result<File, std::io::Error> {
        let path = self.get_metadata_file_path();
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Metadata file not found",
            ));
        }
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)
    }

    fn load_metadata(&self) -> Result<Vec<FileMetadata>, std::io::Error> {
        let path = self.get_metadata_file_path();
        if !path.exists() {
            self.create_metadata()?;
        }

        let metadata_file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Failed to open metadata file",
                ))
            }
        };

        // If metadata file is empty, return empty vector
        // We'll get an error when trying to parse an empty file
        if metadata_file.metadata()?.len() == 0 {
            return Ok(Vec::new());
        }
        let metadata: Vec<FileMetadata> = match serde_json::from_reader(metadata_file) {
            Ok(metadata) => metadata,
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse metadata file",
            ))?,
        };

        Ok(metadata)
    }

    fn find_files_to_remove(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let metadata = self.load_metadata()?;
        let mut files_to_remove = Vec::new();

        for file_meta in metadata {
            let file_path = self.home_dir.join(&file_meta.file_name);
            let file = File::open(&file_path)?;
            if file_meta.remove_condition.check(&file.metadata()?) {
                files_to_remove.push(file_path);
            }
        }

        Ok(files_to_remove)
    }

    fn compute_checksum(file: &PathBuf) -> Result<String, std::io::Error> {
        sha256::try_digest(file)
    }

    fn get_trash_dir(&self) -> PathBuf {
        self.home_dir.join(PROG_USER_DATA_DIR).join("trash")
    }

    fn get_metadata_file_path(&self) -> PathBuf {
        // ~/.local/share/srm/metadata.json
        self.home_dir.join(PROG_USER_DATA_DIR).join(METADATA_FILE)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::{RemoveCondBuilder, TimeCondition};

    use super::*;

    #[test]
    fn test_create_metadata() {
        println!("Running test_create_metadata");

        let manager = FileManager::new();
        assert!(manager.is_ok(), "Failed to create FileManager");
        let manager = manager.unwrap();

        let res = manager.create_metadata();
        assert!(res.is_ok(), "Failed to create metadata file");
    }

    #[test]
    fn test_add_to_metadata() {
        println!("Running test_add_to_metadata");

        let manager = FileManager::new();
        assert!(manager.is_ok(), "Failed to create FileManager");
        let manager = manager.unwrap();

        let file = PathBuf::from("Cargo.toml");
        let res = manager.add_to_metadata(
            &file,
            &RemoveCondBuilder::new()
                .set_size_limit(30 * 1024)
                .set_time_condition(TimeCondition::Duration(24))
                .build(),
        );
        match res {
            Ok(_) => println!("Successfully added to metadata"),
            Err(e) => panic!("Failed to add to metadata: {}", e),
        }
    }
}
