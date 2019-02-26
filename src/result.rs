//! Success and error values.

use std::future::Future;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::pin::Pin;

/// A type indicating a successful operation  that has no meaningful return value.
///
/// Returned by callbacks and some chat service operations.
pub type Success = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

/// An error generated while the robot is running.
#[derive(Clone, Debug)]
pub struct Error {
    inner: ErrorKind,
}

impl Error {
    /// Create a custom error from a message.
    pub fn custom<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Error {
            inner: ErrorKind::Custom(message.into()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.inner)
    }
}

impl StdError for Error {}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::custom(error)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Self {
        Error::custom(error)
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error {
            inner: ErrorKind::Regex(error),
        }
    }
}

#[derive(Clone, Debug)]
enum ErrorKind {
    Custom(String),
    // Io(std::io::Error),
    Regex(regex::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}
