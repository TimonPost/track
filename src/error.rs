use std::fmt::{Display, Formatter};

/// Wrapper for all errors that can occur in `crossterm`.
#[derive(Debug)]
pub enum ErrorKind {
    SerialisationError(String),
}

impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::SerialisationError(e) => {
                write!(fmt, "Serialisation error occurred: {:?}", e)
            }
        }
    }
}
