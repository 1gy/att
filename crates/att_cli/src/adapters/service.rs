use async_trait::async_trait;

use crate::contest::ContestInfo;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    //
}

#[async_trait]
pub trait Service {
    fn get_name(&self) -> String;

    async fn fetch_contest_info(&self, url: &str) -> Result<ContestInfo, ServiceError>;
}
