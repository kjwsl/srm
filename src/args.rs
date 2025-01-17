use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};

// TODO: Do I want to allow 2+ time conditions?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TimeCondition {
    Duration(u32),
    Date(DateTime<Utc>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RemoveCond {
    pub time_cond: Option<TimeCondition>,
    pub size_limit: Option<u32>,
}

impl RemoveCond {
    pub fn check(&self, file: &std::fs::Metadata) -> bool {
        if let Some(size_limit) = self.size_limit {
            if file.len() > size_limit as u64 {
                return true;
            }
        }

        if let Some(time_cond) = &self.time_cond {
            match time_cond {
                TimeCondition::Duration(duration) => {
                    let now = Utc::now();
                    let created_at: DateTime<Utc> = file.created().unwrap().into();
                    let duration = chrono::Duration::hours(*duration as i64);
                    if now - created_at > duration {
                        return true;
                    }
                }
                TimeCondition::Date(date) => {
                    let created_at: DateTime<Utc> = file.created().unwrap().into();
                    if created_at < *date {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn set_time_condition(&mut self, time_cond: TimeCondition) {
        self.time_cond = Some(time_cond);
    }

    pub fn set_size_limit(&mut self, size_limit: u32) {
        self.size_limit = Some(size_limit);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoveCondBuilder {
    time_cond: Option<TimeCondition>,
    size_limit: Option<u32>,
}

impl Default for RemoveCondBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoveCondBuilder {
    pub fn new() -> Self {
        Self {
            time_cond: None,
            size_limit: None,
        }
    }

    pub fn set_time_condition(mut self, time_cond: TimeCondition) -> Self {
        self.time_cond = Some(time_cond);
        self
    }

    pub fn set_size_limit(mut self, size_limit: u32) -> Self {
        self.size_limit = Some(size_limit);
        self
    }

    pub fn build(&self) -> RemoveCond {
        RemoveCond {
            time_cond: self.time_cond.clone(),
            size_limit: self.size_limit,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub recursive: bool,
    pub store: bool,
    pub life_duration: u32,
    pub size_limit: u32,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            recursive: false,
            store: true,
            life_duration: 24,
            size_limit: 1024 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParsedArgs {
    pub files: Vec<PathBuf>,
    pub flags: Flags,
}

#[derive(Debug, Clone, Default)]
pub struct ArgParser;

impl ArgParser {
    pub fn parse(args: &[String]) -> Result<ParsedArgs, Error> {
        let mut parsed_args = ParsedArgs::default();
        let mut i = 1;
        let mut found_file_str = false;
        while i < args.len() {
            match args[i].as_str() {
                "--recursive" | "-r" => {
                    parsed_args.flags.recursive = true;
                }
                "--life-duration" => {
                    i += 1;
                    parsed_args.flags.life_duration = Self::parse_life_duration(&args[i])?;
                }
                "--size-limit" => {
                    i += 1;
                    parsed_args.flags.size_limit = Self::parse_file_size(&args[i])?;
                }
                "--no-store" | "-n" => {
                    parsed_args.flags.store = false;
                }

                arg if !arg.starts_with("-") && !arg.is_empty() => {
                    let file_str = Self::parse_file_str(arg)?;
                    parsed_args.files = file_str;
                    found_file_str = true;
                }

                _ => {
                    return Err(Error::new(ErrorKind::InvalidArg(format!(
                        "Invalid argument: {}",
                        args[i]
                    ))));
                }
            }
            i += 1;
        }

        if !found_file_str {
            return Err(Error::new(ErrorKind::InvalidArg(
                "No file string found".to_string(),
            )));
        }

        Ok(parsed_args)
    }

    // TODO: Implement it to support a mix of units.
    fn parse_life_duration(duration: &str) -> Result<u32, Error> {
        let (value, unit) = duration.split_at(duration.len() - 1);
        let value = value.parse::<u32>().map_err(|_| {
            Error::new(ErrorKind::ParseError(format!(
                "Failed to parse an integer: {}",
                value
            )))
        })?;
        match unit.to_lowercase().as_str() {
            "h" => Ok(value),
            "d" => Ok(value * 24),
            "w" => Ok(value * 24 * 7),
            _ => Err(Error::new(ErrorKind::ParseError(
                "Failed to parse duration unit".to_string(),
            ))),
        }
    }

    fn parse_file_size(size_str: &str) -> Result<u32, Error> {
        if size_str.is_empty() {
            return Err(Error::new(ErrorKind::InvalidArg(
                "String is empty".to_string(),
            )));
        }
        let (value, unit) = size_str.split_at(size_str.len() - 1);
        let value = value
            .trim()
            .parse::<u32>()
            .map_err(|e| Error::new(ErrorKind::ParseError(e.to_string())))?;
        match unit.trim().to_lowercase().as_str() {
            "k" | "kb" => Ok(value * 1024),
            "m" | "mb" => Ok(value * 1024 * 1024),
            "g" | "gb" => Ok(value * 1024 * 1024 * 1024),
            _ => Err(Error::new(ErrorKind::ParseError(
                "Failed to parse size unit".to_string(),
            ))),
        }
    }

    fn parse_file_str(file_str: &str) -> Result<Vec<PathBuf>, Error> {
        if file_str.is_empty() {
            return Err(Error::new(ErrorKind::ParseError(
                "Empty file string".to_string(),
            )));
        }

        std::fs::canonicalize(file_str)
            .map(|path| vec![path])
            .map_err(|e| {
                Error::new(ErrorKind::ParseError(format!(
                    "Failed to parse file: {}",
                    e
                )))
            })
    }
}
