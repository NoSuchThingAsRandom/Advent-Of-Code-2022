//! Src: https://github.com/m-rutter/advent-of-code/blob/master/src/error.rs
//! AoC error module
#![allow(dead_code)]

use std::error::Error as StdError;
use std::fmt;

use serde_plain::Error;

// Convenience Result type
pub type AoCResult<T> = std::result::Result<T, AoCError>;

/// An error type for the Advent of Code crate
#[derive(Debug)]
pub struct AoCError {
    kind: ErrorKind,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl AoCError {
    pub fn new(msg: String) -> AoCError {
        Self {
            kind: ErrorKind::Msg(msg),
            source: None,
        }
    }

    #[allow(dead_code)]
    ///Creates generic error with a message and a cause
    pub(crate) fn chain(
        value: &impl ToString,
        cause: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: Some(cause.into()),
        }
    }
    // TODO Add context
    pub fn from_option<T>(option: Option<T>, _context: Option<String>) -> AoCResult<T> {
        if let Some(val) = option {
            Ok(val)
        } else {
            Err(AoCError::from(ErrorKind::Msg(String::from(
                "Failed to unwrap option",
            ))))
        }
    }
}

impl StdError for AoCError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|c| c.as_ref() as &(dyn StdError + 'static))
    }

    fn cause(&self) -> Option<&(dyn StdError)> {
        self.source().as_ref().map(|c| &**c)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Generic error message
    Msg(String),
    /// Error when parsing provided input
    InputParse,
    /// Error when the day is not supported or does not exist
    UnsupportedDay { year: u16, day: u8 },
}

impl From<ErrorKind> for AoCError {
    fn from(error: ErrorKind) -> Self {
        Self {
            kind: error,
            source: None,
        }
    }
}

impl From<std::num::ParseIntError> for AoCError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self {
            kind: ErrorKind::InputParse,
            source: Some(error.into()),
        }
    }
}

impl From<std::io::Error> for AoCError {
    fn from(error: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::InputParse,
            source: Some(error.into()),
        }
    }
}

impl From<serde_plain::Error> for AoCError {
    fn from(error: Error) -> Self {
        Self {
            kind: ErrorKind::InputParse,
            source: Some(error.into()),
        }
    }
}
/*impl From<regex::Error> for AoCError {
    fn from(error: regex::Error) -> Self {
        Self {
            kind: ErrorKind::InputParse,
            source: Some(error.into()),
        }
    }
}*/

impl fmt::Display for AoCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::Msg(message) => write!(f, "{}", message),
            ErrorKind::InputParse => write!(f, "Error parsing input"),
            ErrorKind::UnsupportedDay { day, year } => write!(
                f,
                "Day {} for year {} either does not exist or is unsupported",
                day, year
            ),
        }
    }
}
