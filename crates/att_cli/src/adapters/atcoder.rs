use crate::contest::ContestInfo;

use super::service::{Service, ServiceError};

pub struct AtCoderService {}

fn get_contest_id(url: &str) -> &str {
    if url.starts_with("https://atcoder.jp/contests/") {
        let parts: Vec<&str> = url.split('/').collect();
        parts[4]
    } else {
        url
    }
}

fn get_contest_page_url(contest_id: &str) -> String {
    format!("https://atcoder.jp/contests/{}", contest_id)
}

fn fetch_contest_info(url: &str) -> Result<ContestInfo, ServiceError> {
    let contest_id = get_contest_id(url);
    let contest_page_url = get_contest_page_url(contest_id);

    let title = "TODO".to_string();

    Ok(ContestInfo {
        id: contest_id.to_string(),
        url: contest_page_url,
        title,
    })
}

impl AtCoderService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Service for AtCoderService {
    fn get_name(&self) -> String {
        "AtCoder".to_string()
    }

    fn fetch_contest_info(&self, url: &str) -> Result<ContestInfo, ServiceError> {
        let contest_id = get_contest_id(url);
        fetch_contest_info(contest_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_contest_page_url() {
        assert_eq!(
            get_contest_page_url("abc123"),
            "https://atcoder.jp/contests/abc123"
        );
        assert_eq!(
            get_contest_page_url("https://atcoder.jp/contests/abc123"),
            "https://atcoder.jp/contests/abc123"
        );
        assert_eq!(
            get_contest_page_url("https://atcoder.jp/contests/abc123/tasks"),
            "https://atcoder.jp/contests/abc123"
        );
        assert_eq!(
            get_contest_page_url("https://atcoder.jp/contests/abc123/standings/virtual"),
            "https://atcoder.jp/contests/abc123"
        );
    }
}
