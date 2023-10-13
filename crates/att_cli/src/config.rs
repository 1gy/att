use std::{fs::OpenOptions, path::PathBuf};

#[derive(Debug)]
pub struct SearctResult {
    pub directory_path: PathBuf,
    pub file_path: PathBuf,
}

pub fn search_file(mut file_path: PathBuf, file_name: &str) -> Option<SearctResult> {
    let mut current = file_path.join(file_name);
    loop {
        let file = OpenOptions::new().read(true).open(&current);
        return match file {
            Ok(_) => Some(SearctResult {
                directory_path: file_path,
                file_path: current,
            }),
            Err(_) => {
                if !file_path.pop() {
                    return None;
                }
                current = file_path.join(file_name);
                continue;
            }
        };
    }
}
