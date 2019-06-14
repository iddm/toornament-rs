//! This module gives the build and version information about the library.
//!
//! # Usage
//!
//! ```rust,no_run
//! use toornament::info::*;
//!
//! println!("Crate info:\n\tVersion: {}\n\tAuthors: {}\n\tName: {}\n\tHome page:
//! {}\n\tDescription: {}",
//!     CRATE_VERSION,
//!     CRATE_AUTHORS,
//!     CRATE_NAME,
//!     CRATE_HOMEPAGE,
//!     CRATE_DESCRIPTION);
//!

/// Crate `version` field from library's `Cargo.toml`
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Crate `authors` field from library's `Cargo.toml`
pub const CRATE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
/// Crate `package` field from library's `Cargo.toml`
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
/// Crate `homepage` field from library's `Cargo.toml`
pub const CRATE_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
/// Crate `description` field from library's `Cargo.toml`
pub const CRATE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
