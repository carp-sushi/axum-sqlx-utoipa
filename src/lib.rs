#![deny(
    clippy::exit,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::unwrap_used,
    clippy::wildcard_imports
)]

/// Restful API (http/json)
pub mod api;

/// Twelve-factor configuration
pub mod config;

/// Dependency management
pub mod container;

/// Core data types
pub mod domain;

/// External system integrations
pub mod driver;

/// Project errors
pub mod error;

/// Persistence API (abstraction over repo layer)
pub mod keeper;

/// Postgres database logic
pub mod repo;

/// Project level error type
pub use error::Error;

/// Project level result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
