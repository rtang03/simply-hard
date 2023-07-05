//!
//! App Library
//!
// #![warn(missing_docs)]

pub mod clients;
mod cmd;
pub mod errors;
pub mod models;
pub mod server;
mod settings;

mod connection;
mod setup_logging;
pub use setup_logging::{set_up_logging, shutdown_tracer_provider};

pub use connection::*;

pub use errors::*;
pub use settings::{Settings, GLOBAL_SETTINGS};

/// Default port that a server listens on.
///
/// Used if no port is specified.
pub const DEFAULT_PORT: u16 = 50051;

///
/// Generated code from protoc
///
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod protobuffer {
    include!("./echo.rs");
}
