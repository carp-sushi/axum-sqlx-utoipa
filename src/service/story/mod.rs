use super::UseCase;
use crate::{domain::Story, repo::StoryRepo, Result};
use std::sync::Arc;

// Use case mods
mod create_story;
mod delete_story;
mod get_stories;
mod get_story;
mod update_story;

// Use cases
use create_story::CreateStory;
use delete_story::DeleteStory;
use get_stories::GetStories;
use get_story::GetStory;
use update_story::UpdateStory;

/// A high-level API for managaing stories.
/// This service is composed of (comprises) use cases.
pub struct StoryService {
    create_story: CreateStory,
    delete_story: DeleteStory,
    get_story: GetStory,
    get_stories: GetStories,
    update_story: UpdateStory,
}

impl StoryService {
    /// Create a new story service
    pub fn new(story_repo: Arc<StoryRepo>) -> Self {
        Self {
            create_story: CreateStory::new(story_repo.clone()),
            delete_story: DeleteStory::new(story_repo.clone()),
            get_story: GetStory::new(story_repo.clone()),
            get_stories: GetStories::new(story_repo.clone()),
            update_story: UpdateStory::new(story_repo),
        }
    }

    /// Create a story
    pub async fn create(&self, name: String) -> Result<Story> {
        self.create_story.execute(name).await
    }

    /// Delete a story
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.delete_story.execute(id).await
    }

    /// Get a story
    pub async fn get(&self, id: i32) -> Result<Story> {
        self.get_story.execute(id).await
    }

    /// Get a page of stories
    pub async fn list(&self, page_id: i32) -> Result<(i32, Vec<Story>)> {
        self.get_stories.execute(page_id).await
    }

    /// Update a story
    pub async fn update(&self, id: i32, name: String) -> Result<Story> {
        self.update_story.execute((id, name)).await
    }
}
