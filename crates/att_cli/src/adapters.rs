use self::{atcoder::AtCoderService, service::Service};

pub mod service;

pub mod atcoder;

pub fn create_service(url: &str) -> Option<Box<dyn Service>> {
    if url.starts_with("http") {
        if url.starts_with("https://atcoder.jp/contests/") {
            Some(Box::new(AtCoderService::new()))
        } else {
            None
        }
    } else {
        // fallback
        Some(Box::new(AtCoderService::new()))
    }
}
