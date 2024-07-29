use crate::{Error, Result};
use serde::Deserialize;
use std::fmt::Debug;

/// Limit name size in http request body.
const MAX_NAME_LEN: usize = 100;

/// The request body for creating or updating stories
#[derive(Debug, Deserialize)]
pub struct StoryBody {
    name: String,
}

impl StoryBody {
    /// Validate a story create/update request.
    pub fn validate(&self) -> Result<String> {
        let name = self.name.trim().to_string();
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            return Err(Error::invalid_args("name: invalid length"));
        }
        Ok(name)
    }
}
