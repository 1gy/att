use std::{fmt, path::PathBuf};

use crate::config::search_file;

const WORKSPACE_FILE_NAME: &str = ".atworkspace.json";

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RunConfig {
    pub program: String,
    pub args: Vec<String>,
}

impl fmt::Display for RunConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.program, self.args.join(" "))
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("workspace already exists")]
    WorkspaceAlreadyExists,
}

pub fn load_workspace() -> Result<Workspace, WorkspaceError> {
    let working_directory = std::env::current_dir()?;
    if let Some(searct_result) = search_file(working_directory, WORKSPACE_FILE_NAME, true) {
        if let Ok(file) = std::fs::OpenOptions::new()
            .read(true)
            .open(searct_result.file_path)
        {
            let config: WorkspaceConfig = serde_json::from_reader(file)?;
            return Ok(Workspace {
                config,
                directory_path: searct_result.directory_path,
            });
        }
    }
    Err(WorkspaceError::NotFound)
}

pub struct InitializedWorkspaceInfo {
    pub text: String,
    pub file_path: String,
}

pub fn init_workspace() -> Result<InitializedWorkspaceInfo, WorkspaceError> {
    let working_directory = std::env::current_dir()?;
    if let Some(_) = search_file(working_directory.clone(), WORKSPACE_FILE_NAME, false) {
        return Err(WorkspaceError::WorkspaceAlreadyExists);
    }
    let config = WorkspaceConfig {
        run: RunConfig {
            program: "python3".to_string(),
            args: vec!["main.py".to_string()],
        },
    };
    let file_path = working_directory.join(WORKSPACE_FILE_NAME);
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file_path)?;
    serde_json::to_writer_pretty(file, &config)?;
    let text = serde_json::to_string_pretty(&config)?;
    Ok(InitializedWorkspaceInfo {
        text,
        file_path: file_path.to_string_lossy().into_owned(),
    })
}
