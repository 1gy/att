use std::{fmt, path::PathBuf};

use crate::config::search_file;

const WORKSPACE_FILE_NAME: &str = ".atworkspace.json";

#[derive(Debug, serde::Deserialize, Clone)]
pub struct RunConfig {
    pub program: String,
    pub args: Vec<String>,
}

impl fmt::Display for RunConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.program, self.args.join(" "))
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct WorkspaceConfig {
    pub run: RunConfig,
}

#[derive(Debug)]
pub struct Workspace {
    pub config: WorkspaceConfig,
    pub directory_path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceError {
    #[error("workspace not found")]
    NotFound,
    #[error(transparent)]
    Io(std::io::Error),
    #[error(transparent)]
    Json(serde_json::Error),
}

pub fn load_workspace() -> Result<Workspace, WorkspaceError> {
    let working_directory = std::env::current_dir().map_err(|err| WorkspaceError::Io(err))?;
    if let Some(searct_result) = search_file(working_directory, WORKSPACE_FILE_NAME) {
        if let Ok(file) = std::fs::OpenOptions::new()
            .read(true)
            .open(searct_result.file_path)
        {
            let config: WorkspaceConfig =
                serde_json::from_reader(file).map_err(|err| WorkspaceError::Json(err))?;
            return Ok(Workspace {
                config,
                directory_path: searct_result.directory_path,
            });
        }
    }
    Err(WorkspaceError::NotFound)
}
