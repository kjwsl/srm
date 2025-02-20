use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new(log_path: PathBuf) -> Result<Self, String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .map_err(|e| format!("Failed to open log file: {}", e))?;

        Ok(Logger { file })
    }

    pub fn log(&mut self, message: &str) -> Result<(), String> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(self.file, "[{}] {}", timestamp, message)
            .map_err(|e| format!("Failed to write to log: {}", e))?;
        Ok(())
    }
}
