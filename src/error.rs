use std::fmt::{Display, Formatter, Result as FmtResult};

/// An error generated while the robot is running.
#[derive(Clone, Copy, Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "unknown error")
    }
}
