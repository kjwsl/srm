use std::fs;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Configuration structure
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_duration: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let proj_dirs = ProjectDirs::from("com", "larpi", "srm")
            .ok_or_else(|| "Cannot determine project directories".to_string())?;
        let config_path = proj_dirs.config_dir();
        let config_file = config_path.join("config.yaml");

        if config_file.exists() {
            let contents = fs::read_to_string(config_file)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            serde_yaml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))
        } else {
            Ok(Config {
                default_duration: None,
            })
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<(), String> {
        let proj_dirs = ProjectDirs::from("com", "larpi", "srm")
            .ok_or_else(|| "Cannot determine project directories".to_string())?;
        let config_path = proj_dirs.config_dir();
        let config_file = config_path.join("config.yaml");

        let mut config = if config_file.exists() {
            let contents = fs::read_to_string(&config_file)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            serde_yaml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))?
        } else {
            Config {
                default_duration: None,
            }
        };

        match key.as_str() {
            "default_duration" => config.default_duration = Some(value),
            _ => return Err(format!("Unknown key: {}", key)),
        }

        let contents = serde_yaml::to_string(&config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        // ensure the parent directory exists
        fs::create_dir_all(config_path)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;

        fs::write(&config_file, contents)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    pub fn get(&self, key: String) -> Result<(), String> {
        match key.as_str() {
            "default_duration" => {
                if let Some(duration) = &self.default_duration {
                    println!("{}", duration);
                } else {
                    println!("No default duration set");
                }
            }
            _ => return Err(format!("Unknown key: {}", key)),
        }

        Ok(())
    }
}
