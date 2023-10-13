use std::path::PathBuf;

use crate::search_file;

const WORKSPACE_FILE_NAME: &str = ".atworkspace.json";

#[derive(Debug, serde::Deserialize)]
pub struct WorkspaceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

#[derive(Debug)]
pub struct Workspace {
    pub config: WorkspaceConfig,
    pub directory_path: PathBuf,
}

pub fn load_workspace() -> Option<Workspace> {
    let working_directory = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => PathBuf::new(),
    };
    if let Some(searct_result) = search_file(working_directory, WORKSPACE_FILE_NAME) {
        if let Ok(file) = std::fs::OpenOptions::new()
            .read(true)
            .open(searct_result.file_path)
        {
            let config: WorkspaceConfig = serde_json::from_reader(file).unwrap();
            return Some(Workspace {
                config,
                directory_path: searct_result.directory_path,
            });
        }
    }
    None
}
