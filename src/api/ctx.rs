use crate::{
    config::Config,
    domain::{Status, Story, Task},
    repo::{StoryRepo, TaskRepo},
    service::{
        story::{CreateStory, DeleteStory, GetStories, GetStory, UpdateStory},
        task::{CreateTask, DeleteTask, GetTask, GetTasks, UpdateTask},
        Service,
    },
    Result,
};
use sqlx::postgres::PgPool;
use std::sync::Arc;

/// API context
#[derive(Clone)]
pub struct Ctx {
    /// Config
    pub config: Arc<Config>,
    /// Story service api
    pub story: Arc<StoryApi>,
    /// Task service api
    pub task: Arc<TaskApi>,
}

impl Ctx {
    /// Create a new api context.
    pub fn new(config: Arc<Config>, db: Arc<PgPool>) -> Self {
        // Repos
        let story_repo = Arc::new(StoryRepo::new(db.clone()));
        let task_repo = Arc::new(TaskRepo::new(db));

        // Story service
        let story_service = StoryApi {
            create_story: CreateStory::new(story_repo.clone()),
            delete_story: DeleteStory::new(story_repo.clone()),
            get_story: GetStory::new(story_repo.clone()),
            get_stories: GetStories::new(story_repo.clone()),
            update_story: UpdateStory::new(story_repo),
        };

        // Task service
        let task_service = TaskApi {
            create_task: CreateTask::new(task_repo.clone()),
            delete_task: DeleteTask::new(task_repo.clone()),
            get_task: GetTask::new(task_repo.clone()),
            get_tasks: GetTasks::new(task_repo.clone()),
            update_task: UpdateTask::new(task_repo),
        };

        // Ctx
        Self {
            config,
            story: Arc::new(story_service),
            task: Arc::new(task_service),
        }
    }
}

/// Container for story services.
pub struct StoryApi {
    create_story: CreateStory,
    delete_story: DeleteStory,
    get_story: GetStory,
    get_stories: GetStories,
    update_story: UpdateStory,
}

impl StoryApi {
    /// Create a story
    pub async fn create(&self, name: String) -> Result<Story> {
        self.create_story.call(name).await
    }

    /// Delete a story
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.delete_story.call(id).await
    }

    /// Get a story
    pub async fn get(&self, id: i32) -> Result<Story> {
        self.get_story.call(id).await
    }

    /// Get recent stories
    pub async fn list(&self) -> Result<Vec<Story>> {
        self.get_stories.call(()).await
    }

    /// Update a story
    pub async fn update(&self, id: i32, name: String) -> Result<Story> {
        self.update_story.call((id, name)).await
    }
}

/// Container for task services.
pub struct TaskApi {
    create_task: CreateTask,
    delete_task: DeleteTask,
    get_task: GetTask,
    get_tasks: GetTasks,
    update_task: UpdateTask,
}

impl TaskApi {
    /// Create a task
    pub async fn create(&self, story_id: i32, name: String) -> Result<Task> {
        self.create_task.call((story_id, name)).await
    }

    /// Delete a task
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.delete_task.call(id).await
    }

    /// Get a task
    pub async fn get(&self, id: i32) -> Result<Task> {
        self.get_task.call(id).await
    }

    /// Get tasks for a story
    pub async fn list(&self, story_id: i32) -> Result<Vec<Task>> {
        self.get_tasks.call(story_id).await
    }

    /// Update a task
    pub async fn update(
        &self,
        id: i32,
        name: Option<String>,
        status: Option<Status>,
    ) -> Result<Task> {
        self.update_task.call((id, name, status)).await
    }
}
