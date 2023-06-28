//!
//! Custom Error
//!
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database is unhealthy")]
    SurrealdbUnHealthy(surrealdb::Error),
    #[error("tonic error")]
    ConnectError(tonic::transport::Error),
    #[error("data store disconnected")]
    Disconnect(#[from] tokio::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("standard application error")]
    StdError(Box<dyn std::error::Error + Send + Sync>),
    #[error("unknown data store error")]
    Unknown,
}

/// A specialized `Result` type for general operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, AppError>;
