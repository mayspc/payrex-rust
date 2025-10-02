//! Unofficial Rust SDK for PayRex payment platform.
//!
//! This is a work in progress SDK with foundation complete but API implementations pending.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

// Core modules
mod client;
mod config;
mod error;
mod http;

// Type modules
pub mod types;

// Resource modules
pub mod resources;

// Re-exports
pub use client::Client;
pub use config::{Config, ConfigBuilder};
pub use error::{Error, ErrorKind, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const API_BASE_URL: &str = "https://api.payrexhq.com/v1";
pub const API_BASE_URL_TEST: &str = "https://api.payrexhq.com/v1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_api_urls_valid() {
        assert!(API_BASE_URL.starts_with("https://"));
        assert!(API_BASE_URL_TEST.starts_with("https://"));
    }
}
