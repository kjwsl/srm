use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::{
    cli::{Flags, ParsedArgs, RemoveCondBuilder, TimeCondition},
    file_manager::FileManager,
};

pub struct Srm;

impl Default for Srm {
    fn default() -> Self {
        Srm::new()
    }
}

impl Srm {
    pub fn new() -> Self {
        Srm
    }
    pub fn run(&self, parsed_args: &ParsedArgs) -> Result<(), std::io::Error> {
        if parsed_args.files.is_empty() {
            return Err(Error::new(ErrorKind::NotFound, "No files to remove"));
        }

        for file in parsed_args.files.iter() {
            if parsed_args.flags.store {
                let manager = FileManager::new()?;
                let cond = RemoveCondBuilder::new()
                    .set_size_limit(parsed_args.flags.size_limit)
                    .set_time_condition(TimeCondition::Duration(parsed_args.flags.life_duration))
                    .build();

                manager.safe_remove(file, &cond)?;
            } else if file.is_dir() {
                if parsed_args.flags.recursive {
                    std::fs::remove_dir_all(file).unwrap();
                } else {
                    return Err(Error::new(
                        ErrorKind::IsADirectory,
                        "Cannot remove directory without recursive flag",
                    ));
                }
            } else {
                std::fs::remove_file(file).unwrap();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_execute() {
        let srm = Srm;
        let file_path = PathBuf::from("test.txt");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "test").unwrap();

        let files = vec![file_path];
        let flags = Flags {
            store: false,
            recursive: false,
            ..Default::default()
        };
        let parsed_args = ParsedArgs { files, flags };

        srm.run(&parsed_args).unwrap();

        assert!(parsed_args.files.iter().all(|f| !f.exists()));
    }
}
