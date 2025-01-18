use std::fs;

use directories::ProjectDirs;
use serde::Deserialize;

/// Configuration structure
#[derive(Debug, Deserialize, Default)]
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
            toml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))
        } else {
            Ok(Config {
                default_duration: None,
            })
        }
    }
}
