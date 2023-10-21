use thiserror::Error;

use crate::commands::{init::InitCommandError, run::RunCommandError, status::StatusCommandError};

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

    #[error(transparent)]
    Init(#[from] InitCommandError),
}
