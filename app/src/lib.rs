mod settings;
pub mod cmd;
pub mod clients;
pub mod server;
pub mod echo;
pub mod errors;

pub use errors::*;
pub use settings::{Settings, GLOBAL_SETTINGS};

/// Default port that a server listens on.
///
/// Used if no port is specified.
pub const DEFAULT_PORT: u16 = 50051;

pub mod protobuffer {
    include!("./echo.rs");
}
