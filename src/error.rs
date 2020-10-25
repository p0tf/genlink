/// The General Result Type
pub type LinkResult<T> = Result<T, LinkError>;

use std::error::Error;
use std::fmt;
use std::io;
use std::process::Output;

/// The Linking Error
#[derive(Debug)]
pub enum LinkError {
    Cmd(Output),
    IO(io::Error),
}

impl From<Output> for LinkError {
    fn from(o: Output) -> Self {
        LinkError::Cmd(o)
    }
}

impl From<io::Error> for LinkError {
    fn from(e: io::Error) -> Self {
        LinkError::IO(e)
    }
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(e) => write!(f, "{}", e),
            Self::Cmd(o) => write!(f, "command exied with code \"{}\"", o.status),
        }
    }
}

impl Error for LinkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IO(e) => Some(e),
            Self::Cmd(_) => None,
        }
    }
}
