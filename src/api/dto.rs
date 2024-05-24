use crate::{domain::Status, Error, Result};
use serde::Deserialize;
use std::fmt::Debug;
use std::str::FromStr;

/// Limit name size in http request body.
const MAX_NAME_LEN: usize = 100;

/// The request body for creating or updating stories
#[derive(Debug, Deserialize)]
pub struct StoryBody {
    name: String,
}

impl StoryBody {
    /// Sanitize and validate story name from request body
    pub fn validate(&self) -> Result<String> {
        let name = self.name.trim().to_string();
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            return Err(Error::invalid_args("name: invalid length"));
        }
        Ok(name)
    }
}

/// The POST body for creating tasks
#[derive(Debug, Deserialize)]
pub struct CreateTaskBody {
    pub name: String,
    pub story_id: i32,
}

impl CreateTaskBody {
    /// Sanitize and validate task name and story_id from request body
    pub fn validate(&self) -> Result<(i32, String)> {
        // Collects error messages
        let mut messages = Vec::new();

        // Validate body params
        let story_id = self.story_id;
        if story_id <= 0 {
            messages.push("story_id: must be > 0".into());
        }
        let name = self.name.trim().to_string();
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            messages.push("name: invalid length".into());
        }

        // Return params or errors
        if messages.is_empty() {
            Ok((story_id, name))
        } else {
            Err(Error::InvalidArgs { messages })
        }
    }
}

/// The PATCH body for updating tasks
#[derive(Debug, Deserialize)]
pub struct PatchTaskBody {
    pub name: Option<String>,
    pub status: Option<String>,
}

impl PatchTaskBody {
    /// Helper to validate fields to update for a task.
    pub fn validate(&self) -> Result<(Option<String>, Option<Status>)> {
        // Make sure at least one field is provided
        if self.name.is_none() && self.status.is_none() {
            return Err(Error::invalid_args("name and/or status must be provided"));
        }

        // Defaults for return values
        let mut messages = Vec::new();
        let mut name: Option<String> = None;
        let mut status: Option<Status> = None;

        // Validate
        if let Some(n) = &self.name {
            let n = n.trim();
            if n.is_empty() || n.len() > MAX_NAME_LEN {
                messages.push("name: invalid length".into());
            } else {
                name = Some(n.to_string());
            }
        }
        if let Some(s) = &self.status {
            if let Ok(parsed) = Status::from_str(s) {
                status = Some(parsed)
            } else {
                messages.push("status: invalid enum variant".into());
            }
        }

        // Determine result of validation
        if !messages.is_empty() {
            return Err(Error::InvalidArgs { messages });
        }

        Ok((name, status))
    }
}
