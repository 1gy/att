use thiserror::Error;

use crate::commands::{
    init::InitCommandError, new::NewCommandError, run::RunCommandError, status::StatusCommandError,
};

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

    #[error(transparent)]
    New(#[from] NewCommandError),
}
