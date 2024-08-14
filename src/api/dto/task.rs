use crate::{domain::Status, Error, Result};
use serde::Deserialize;
use std::fmt::Debug;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

/// Limit name size in http request body.
const MAX_NAME_LEN: usize = 100;

/// The POST body for creating tasks
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskBody {
    pub name: String,
    pub story_id: Uuid,
    pub status: Option<String>,
}

impl CreateTaskBody {
    /// Validate a task create request.
    pub fn validate(&self) -> Result<(Uuid, String, Status)> {
        // Collects error messages
        let mut messages = Vec::new();

        // Validate body params
        let name = self.name.trim().to_string();
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            messages.push("name: invalid length".into());
        }
        let mut status: Option<Status> = None;
        if let Some(s) = &self.status {
            match Status::from_str(s) {
                Ok(parsed) => status = Some(parsed),
                Err(err) => messages.push(format!("status: {}", err)),
            }
        }

        // Check for validation failures and return an error if found
        if !messages.is_empty() {
            return Err(Error::InvalidArgs { messages });
        }

        Ok((self.story_id, name, status.unwrap_or_default()))
    }
}

/// The PATCH body for updating tasks
#[derive(Debug, Deserialize, ToSchema)]
pub struct PatchTaskBody {
    pub name: Option<String>,
    pub status: Option<String>,
}

impl PatchTaskBody {
    /// Validate a task update request.
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
            match Status::from_str(s) {
                Ok(parsed) => status = Some(parsed),
                Err(err) => messages.push(format!("status: {}", err)),
            }
        }

        // Check for validation failures and return an error if found
        if !messages.is_empty() {
            return Err(Error::InvalidArgs { messages });
        }

        Ok((name, status))
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct TaskParams {
    pub status: Option<String>,
}

impl TaskParams {
    /// Parse status enum from param string
    pub fn status(&self) -> Option<Status> {
        if let Some(s) = self.status.clone() {
            if let Ok(status) = Status::from_str(&s) {
                return Some(status);
            }
        }
        None
    }
}
