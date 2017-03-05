use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "unknown error")
    }
}
