use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Service query failed: {0}")]
    QueryFailed(String),
}
