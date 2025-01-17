pub struct Error {
    pub kind: ErrorKind,
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
