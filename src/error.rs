use std::fmt::{Display, Formatter};

/// Wrapper for all errors that can occur in `crossterm`.
#[derive(Debug)]
pub enum ErrorKind {
    SerializationError(String),
}

impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::SerializationError(e) => {
                write!(fmt, "Serialization error occurred: {:?}", e)
            }
        }
    }
}
