use axum::extract::multipart::MultipartError;

// Http support for errors
mod http;

pub use http::Errors;

/// Project level error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid arguments: {messages:?}")]
    InvalidArgs { messages: Vec<String> },
    #[error("internal error: {message}")]
    Internal { message: String },
    #[error("not found error: {message}")]
    NotFound { message: String },
}

// Error helpers
impl Error {
    pub fn internal(s: impl Into<String>) -> Self {
        Error::Internal { message: s.into() }
    }

    pub fn not_found(s: impl Into<String>) -> Self {
        Error::NotFound { message: s.into() }
    }

    pub fn invalid_args(s: impl Into<String>) -> Self {
        Error::InvalidArgs {
            messages: vec![s.into()],
        }
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::internal(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::internal(err.to_string())
    }
}

impl From<MultipartError> for Error {
    fn from(err: MultipartError) -> Self {
        Error::invalid_args(err.to_string())
    }
}
