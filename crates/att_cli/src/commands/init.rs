use crate::workspace::{init_workspace, WorkspaceError};

use super::AttContext;

#[derive(thiserror::Error, Debug)]
pub enum InitCommandError {
    #[error("workspace error: {0}")]
    WorkspaceError(#[from] WorkspaceError),
}

pub fn execute(_context: &AttContext) -> Result<(), InitCommandError> {
    let info = init_workspace()?;
    println!("Initialized workspace: {}", info.file_path);
    println!("{}", info.text);
    Ok(())
}
