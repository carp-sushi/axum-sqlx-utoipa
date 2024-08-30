use crate::{domain::Story, Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::ToSchema;

/// Limit name size in http request body.
const MAX_NAME_LEN: usize = 100;

/// The request body for creating or updating stories
#[derive(Debug, Deserialize, ToSchema)]
pub struct StoryRequest {
    name: String,
}

impl StoryRequest {
    /// Validate a story create/update request.
    pub fn validate(&self) -> Result<String> {
        let name = self.name.trim().to_string();
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            return Err(Error::invalid_args("name: invalid length"));
        }
        Ok(name)
    }
}

/// A page of stories
#[derive(Debug, Serialize, ToSchema)]
pub struct Stories {
    #[serde(skip_serializing_if = "Option::is_none")]
    next_page: Option<String>,
    stories: Vec<Story>,
}

impl Stories {
    // Create a new page of stories
    pub fn new(next_page: Option<String>, stories: Vec<Story>) -> Self {
        Self { next_page, stories }
    }
}
