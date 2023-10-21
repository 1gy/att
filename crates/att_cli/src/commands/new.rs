use crate::{
    adapters::{create_service, service::ServiceError},
    contest::{init_contest, ContestError},
};

use super::AttContext;

#[derive(thiserror::Error, Debug)]
pub enum NewCommandError {
    #[error("service not implemented")]
    ServiceNotImplemented,
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
    #[error("contest error: {0}")]
    ContestError(#[from] ContestError),
}

pub async fn execute(_context: &AttContext, url: &str) -> Result<(), NewCommandError> {
    let service = create_service(url).ok_or(NewCommandError::ServiceNotImplemented)?;
    println!("Service: {}", service.get_name());
    let info = init_contest(service.fetch_contest_info(url).await?)?;
    println!("Initialized contest: {}", info.file_path);
    println!("{}", info.text);
    Ok(())
}
