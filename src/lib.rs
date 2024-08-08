pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod repo;

/// Expose error at the top level
pub use error::Error;

/// Project level result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
