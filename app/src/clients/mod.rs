//!
//! Client
//!
//!

#[cfg(feature = "cli")]
mod client;

#[cfg(feature = "cli")]
pub use client::Client;

mod setup_logging;
pub use setup_logging::{set_up_logging, shutdown_tracer_provider};
