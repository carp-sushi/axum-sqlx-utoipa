use super::{StorageId, StoryId};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

/// The newtype story file relation id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
pub struct StoryFileId(pub Uuid);

// Display the inner uuid.
impl std::fmt::Display for StoryFileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
pub struct StoryFile {
    pub id: StoryFileId,
    pub story_id: StoryId,
    pub storage_id: StorageId,
    pub name: String,
    pub size: i64,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
