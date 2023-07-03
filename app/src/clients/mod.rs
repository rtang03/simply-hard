//!
//! Client
//! 
//! 

#[cfg(feature = "cli")]
mod client;

#[cfg(feature = "cli")]
pub use client::Client;