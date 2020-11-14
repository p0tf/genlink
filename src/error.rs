extern crate thiserror;

/// The General Result Type
pub type LinkResult<T> = Result<T, LinkError>;

use thiserror::Error;
use std::io;
use std::process::Output;

/// The Linking Error
#[derive(Debug, Error)]
pub enum LinkError {
    #[error("Command exited with the failure: {0:?}")]
    Cmd(Output),
    #[error("IO Error: {0}")]
    IO(#[from] io::Error),
}

impl From<Output> for LinkError {
    fn from(o: Output) -> Self {
        Self::Cmd(o)
    }
}
