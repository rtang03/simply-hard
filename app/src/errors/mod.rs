use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("data store disconnected")]
    Disconnect(#[from] tokio::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

pub type CommonError = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for general operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, CommonError>;