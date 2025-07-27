use std::error::Error;
use std::fmt;
use std::io;

#[derive(fmt::Debug)]
pub struct TapestryError {
    reason: Option<String>,
}

impl TapestryError {
    pub fn new(reason: String) -> Self {
        TapestryError {
            reason: Some(reason),
        }
    }
}

impl fmt::Display for TapestryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.reason {
            Some(msg) => write!(f, "{}", msg),
            None => write!(f, "unknown error"),
        }
    }
}

impl Error for TapestryError {}

impl From<io::Error> for TapestryError {
    fn from(e: io::Error) -> Self {
        TapestryError::new(e.to_string())
    }
}

impl From<fmt::Error> for TapestryError {
    fn from(e: fmt::Error) -> Self {
        TapestryError::new(e.to_string())
    }
}
