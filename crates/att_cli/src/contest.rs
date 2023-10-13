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

pub fn load_contest() -> Option<Contest> {
    let working_directory = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => PathBuf::new(),
    };
    if let Some(searct_result) = search_file(working_directory, CONSTEST_FILE_NAME) {
        if let Ok(file) = std::fs::OpenOptions::new()
            .read(true)
            .open(searct_result.file_path)
        {
            let contest: ContestConfig = serde_json::from_reader(file).unwrap();
            return Some(Contest {
                directory_path: searct_result.directory_path,
                contest,
            });
        }
    }
    None
}
