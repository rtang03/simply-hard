//!
//! Surreal database
//!

#[cfg(feature = "server")]
mod repository;
pub use repository::*;

#[cfg(feature = "default")]
pub mod model;
pub use model::*;
