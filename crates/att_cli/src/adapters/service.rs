use crate::contest::ContestInfo;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    //
}

pub trait Service {
    fn get_name(&self) -> String;

    fn fetch_contest_info(&self, url: &str) -> Result<ContestInfo, ServiceError>;
}
