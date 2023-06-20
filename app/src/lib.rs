pub mod settings;
pub mod cmd;
mod shutdown;
pub mod clients;

pub use settings::Settings;

use thiserror::Error;

/// Default port that a server listens on.
///
/// Used if no port is specified.
pub const DEFAULT_PORT: u16 = 6379;

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