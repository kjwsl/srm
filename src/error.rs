use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Failed to parse arguments: {0}")]
    ParseError(String),

    #[error("Invalid arg: {0}")]
    InvalidArg(String),
}
