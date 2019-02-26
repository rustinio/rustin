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
#[derive(Clone, Copy, Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "unknown error")
    }
}

impl StdError for Error {}
