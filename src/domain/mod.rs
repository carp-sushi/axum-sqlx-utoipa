use serde::Serialize;
use uuid::Uuid;

mod status;
mod story;
mod story_file;
mod task;

pub use status::Status;
pub use story::Story;
pub use story_file::StoryFile;
pub use task::Task;

/// A domain object used for storage keys.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct StorageId(pub Uuid);

impl StorageId {
    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }
}
