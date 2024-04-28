//! Standard LFA error types

use std::env;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub details: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IO,
    ExecError,
    OpenError,
    ReadError,
    WriteError,
    ExtractError,
    DatabaseError,
    PermissionDenied,
    DeserializationError,
    SerializationError,

    Other,
}

impl Error {
    pub fn new<M: ToString>(msg: M) -> Self {
        Self {
            details: msg.to_string(),
            kind: ErrorKind::Other,
        }
    }

    pub fn set_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arg0 = env::args().next();
        if let Some(arg) = arg0 {
            write!(f, "{}: {}", arg, &self.details)
        } else {
            write!(f, "{}", &self.details)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}
