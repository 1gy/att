use crate::adapters::{create_service, service::ServiceError};

use super::AttContext;

#[derive(thiserror::Error, Debug)]
pub enum NewCommandError {
    #[error("service not implemented")]
    ServiceNotImplemented,
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
}

pub async fn execute(_context: &AttContext, url: &str) -> Result<(), NewCommandError> {
    let service = create_service(url).ok_or(NewCommandError::ServiceNotImplemented)?;
    println!("Service: {}", service.get_name());
    let info = service.fetch_contest_info(url).await?;
    println!("info: {0:?}", info);
    Ok(())
}
