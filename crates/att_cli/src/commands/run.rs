use std::process;

use crate::workspace::{load_workspace, RunConfig, WorkspaceError};

use super::AttContext;

#[derive(thiserror::Error, Debug)]
pub enum RunCommandError {
    #[error("workspace error: {0}")]
    WorkspaceError(#[from] WorkspaceError),
    #[error("failed to execute process: {0}")]
    FailedToExecuteProcess(RunConfig),
}

pub fn execute(_context: &AttContext) -> Result<(), RunCommandError> {
    let workspace = load_workspace()?;

    let run_config = &workspace.config.run;
    process::Command::new(&run_config.program)
        .args(&run_config.args)
        .spawn()
        .map_err(|_| RunCommandError::FailedToExecuteProcess(run_config.clone()))?
        .wait()
        .map_err(|_| RunCommandError::FailedToExecuteProcess(run_config.clone()))?;
    Ok(())
}
