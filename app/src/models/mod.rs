//!
//! Surreal database
//!

#[cfg(feature = "server")]
mod repository;

pub use repository::*;

pub mod model;
