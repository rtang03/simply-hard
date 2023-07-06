//!
//! Custom Error
//!
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    /// Surrealdb: Fail to connect db
    #[error("database is unhealthy")]
    DbConnectError(surrealdb::Error),

    /// Surrealdb: Unhealthy
    #[error("database is unhealthy")]
    SurrealdbUnHealthy(surrealdb::Error),

    /// Surrealdb: set_value error
    #[error("set_value error")]
    SurrealdbSetError(surrealdb::Error),

    /// Surrealdb: get_value error
    #[error("get_value error")]
    SurrealdbGetError(surrealdb::Error),

    /// grpc: Fail to connect server
    #[error("tonic error")]
    TonicError(tonic::transport::Error),

    /// Tracing
    #[error("tracing-subscriber init error")]
    TracingSubscriberInitError(tracing_subscriber::util::TryInitError),

    #[error("tracing setGlobalDefaultError")]
    TracingSetGlobalDefaultError(tracing::subscriber::SetGlobalDefaultError),

    // TODO: clean up below errors
    #[error("data store disconnected")]
    Disconnect(#[from] tokio::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("standard application error")]
    StdError(Box<dyn std::error::Error + Send + Sync>),
    /// Unknown error
    #[error("unknown error")]
    Unknown,
}

/// A specialized `Result` type for general operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, AppError>;
