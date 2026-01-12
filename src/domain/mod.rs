use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

mod file;
mod status;
mod story;
mod task;

pub use file::{StoryFile, StoryFileId};
pub use status::Status;
pub use story::{Story, StoryId};
pub use task::{Task, TaskId};

/// The newtype storage id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
pub struct StorageId(pub Uuid);

// Display the inner uuid.
impl std::fmt::Display for StorageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
