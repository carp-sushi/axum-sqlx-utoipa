use crate::domain::Status;
use serde::Deserialize;
use std::{fmt::Debug, str::FromStr};
use validator::{Validate, ValidationError};

// Min string length bytes
const MIN_LEN: u64 = 1;

// Max string length bytes
const MAX_LEN: u64 = 100;

/// The POST body for creating and updating stories
#[derive(Debug, Deserialize, Default, Validate)]
pub struct StoryBody {
    #[validate(length(min = "MIN_LEN", max = "MAX_LEN", message = "invalid length"))]
    pub name: String,
}

/// The POST body for creating tasks
#[derive(Debug, Deserialize, Default, Validate)]
pub struct CreateTaskBody {
    #[validate(length(min = "MIN_LEN", max = "MAX_LEN", message = "invalid length"))]
    pub name: String,
    #[validate(range(min = 1, message = "must be > 0"))]
    pub story_id: i32,
}

/// The PATCH body for updating tasks
#[derive(Debug, Deserialize, Default, Validate)]
pub struct PatchTaskBody {
    #[validate(length(min = "MIN_LEN", max = "MAX_LEN", message = "invalid length"))]
    pub name: Option<String>,
    #[validate(custom(function = "validate_status", message = "unmatched enum variant"))]
    pub status: Option<String>,
}

impl PatchTaskBody {
    /// Helper to unwrap status if provided.
    pub fn get_status(&self) -> Option<Status> {
        if let Some(s) = &self.status {
            if let Ok(status) = Status::from_str(s) {
                return Some(status);
            }
        }
        None
    }
}

/// Custom status validation function
fn validate_status(status_opt: &Option<String>) -> Result<(), ValidationError> {
    match status_opt {
        None => Ok(()),
        Some(status) => match Status::from_str(status) {
            Err(_) => Err(ValidationError::new("invalid_status")),
            Ok(_) => Ok(()),
        },
    }
}
