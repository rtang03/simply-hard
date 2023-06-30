//!
//! Surreal database
//!

#[cfg(feature = "default")]
mod repository;
pub use repository::*;

#[cfg(feature = "default")]
pub mod model;
pub use model::*;
