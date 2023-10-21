use std::path::PathBuf;

use crate::config::search_file;

const CONSTEST_FILE_NAME: &str = ".atcontest.json";

#[derive(Debug, serde::Deserialize)]
pub struct ContestInfo {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ContestConfig {
    pub contest: ContestInfo,
}

#[derive(Debug)]
pub struct Contest {
    pub directory_path: PathBuf,
    pub contest: ContestConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum ContestError {
    #[error("contest not found")]
    NotFound,
    #[error(transparent)]
    Io(std::io::Error),
    #[error(transparent)]
    Json(serde_json::Error),
}

pub fn load_contest() -> Result<Contest, ContestError> {
    let working_directory = std::env::current_dir().map_err(|err| ContestError::Io(err))?;
    if let Some(searct_result) = search_file(working_directory, CONSTEST_FILE_NAME) {
        if let Ok(file) = std::fs::OpenOptions::new()
            .read(true)
            .open(searct_result.file_path)
        {
            let contest: ContestConfig =
                serde_json::from_reader(file).map_err(|err| ContestError::Json(err))?;
            return Ok(Contest {
                directory_path: searct_result.directory_path,
                contest,
            });
        }
    }
    Err(ContestError::NotFound)
}
