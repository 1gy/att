use std::{fs::create_dir, path::PathBuf};

use crate::config::search_file;

const CONSTEST_FILE_NAME: &str = ".atcontest.json";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ContestInfo {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("contest already exists")]
    ContestAlreadyExists,
}

pub fn load_contest() -> Result<Contest, ContestError> {
    let working_directory = std::env::current_dir()?;
    if let Some(searct_result) = search_file(working_directory, CONSTEST_FILE_NAME, true) {
        if let Ok(file) = std::fs::OpenOptions::new()
            .read(true)
            .open(searct_result.file_path)
        {
            let contest: ContestConfig = serde_json::from_reader(file)?;
            return Ok(Contest {
                directory_path: searct_result.directory_path,
                contest,
            });
        }
    }
    Err(ContestError::NotFound)
}

pub struct InitializedContestInfo {
    pub text: String,
    pub file_path: String,
}

pub fn init_contest(contest_info: ContestInfo) -> Result<InitializedContestInfo, ContestError> {
    let working_directory = std::env::current_dir()?;
    let contest_directory = working_directory.join(&contest_info.id);
    if contest_directory.exists() {
        return Err(ContestError::ContestAlreadyExists);
    }
    create_dir(&contest_directory)?;
    let contest_config = ContestConfig {
        contest: contest_info,
    };
    let config_file_path = contest_directory.join(CONSTEST_FILE_NAME);
    let config_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&config_file_path)?;
    serde_json::to_writer_pretty(config_file, &contest_config)?;
    let contest_config_text = serde_json::to_string_pretty(&contest_config)?;
    Ok(InitializedContestInfo {
        text: contest_config_text,
        file_path: config_file_path.to_string_lossy().into_owned(),
    })
}
