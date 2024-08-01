use crate::{
    repo::Repo,
    service::{story::StoryService, task::TaskService},
};
use sqlx::postgres::PgPool;
use std::sync::Arc;

/// API context
#[derive(Clone)]
pub struct Ctx {
    /// Story service
    pub stories: Arc<StoryService>,
    /// Task service
    pub tasks: Arc<TaskService>,
}

impl Ctx {
    /// Create a new api context.
    pub fn new(db: Arc<PgPool>) -> Self {
        // Repo
        let repo = Arc::new(Repo::new(db.clone()));

        // Services (organize/group functionality by domain).
        let task_service = TaskService::new(repo.clone());
        let story_service = StoryService::new(repo);

        // Ctx
        Self {
            stories: Arc::new(story_service),
            tasks: Arc::new(task_service),
        }
    }
}
