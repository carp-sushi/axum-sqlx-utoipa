use crate::{
    driver::storage::Storage,
    keeper::{FileKeeper, StoryKeeper, TaskKeeper},
};
use std::sync::Arc;
use uuid::Uuid;

/// Context contains pointers to keepers and drivers for use in API routes.
#[derive(Clone)]
pub struct Ctx {
    /// Binary object storage
    pub storage: Arc<Box<dyn Storage<Uuid>>>,

    /// Story persistence API
    pub story_keeper: Arc<Box<dyn StoryKeeper>>,

    /// Task persistence API
    pub task_keeper: Arc<Box<dyn TaskKeeper>>,

    /// File metadata persistence API
    pub file_keeper: Arc<Box<dyn FileKeeper>>,
}

impl Ctx {
    /// Create a new API context
    pub fn new(
        storage: Arc<Box<dyn Storage<Uuid>>>,
        story_keeper: Arc<Box<dyn StoryKeeper>>,
        task_keeper: Arc<Box<dyn TaskKeeper>>,
        file_keeper: Arc<Box<dyn FileKeeper>>,
    ) -> Self {
        Self {
            storage,
            story_keeper,
            task_keeper,
            file_keeper,
        }
    }
}
