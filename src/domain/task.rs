use super::Status;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub story_id: Uuid,
    pub name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn status(&self) -> Status {
        Status::from_str(&self.status).unwrap_or_default()
    }
}
