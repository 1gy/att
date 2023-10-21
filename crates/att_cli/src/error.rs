use thiserror::Error;

use crate::commands::{run::RunCommandError, status::StatusCommandError};

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("command error")]
    Command(#[from] CommandError),
}

// command error
#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    Status(#[from] StatusCommandError),

    #[error(transparent)]
    Run(#[from] RunCommandError),
}
