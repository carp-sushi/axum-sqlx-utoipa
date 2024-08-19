use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
pub struct StoryFile {
    pub id: Uuid,
    pub story_id: Uuid,
    #[serde(skip_serializing)]
    pub storage_id: Uuid,
    pub name: String,
    pub size: i64,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
