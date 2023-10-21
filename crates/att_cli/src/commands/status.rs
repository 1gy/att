use crate::{
    contest::{load_contest, ContestError},
    workspace::{load_workspace, WorkspaceError},
};

use super::AttContext;

#[derive(thiserror::Error, Debug)]
pub enum StatusCommandError {
    #[error("workspace error: {0}")]
    WorkspaceError(#[from] WorkspaceError),
    #[error("contest error: {0}")]
    ContestError(#[from] ContestError),
}

pub fn execute(_context: &AttContext) -> Result<(), StatusCommandError> {
    match load_workspace() {
        Ok(workspace) => {
            println!("{:?}", workspace);
        }
        Err(WorkspaceError::NotFound) => {
            println!("workspace not found");
        }
        Err(err) => return Err(err.into()),
    };

    match load_contest() {
        Ok(contest) => {
            println!("{:?}", contest);
        }
        Err(ContestError::NotFound) => {
            println!("contest not found");
        }
        Err(err) => return Err(err.into()),
    };

    Ok(())
}
