use serde::Serialize;

// Http support for errors
mod http;

/// Project level error type
#[derive(thiserror::Error, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
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
    pub fn internal(message: String) -> Self {
        Error::Internal { message }
    }

    pub fn not_found(message: String) -> Self {
        Error::NotFound { message }
    }

    pub fn invalid_args(message: &str) -> Self {
        Error::InvalidArgs {
            messages: vec![message.into()],
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
