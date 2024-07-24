use super::UseCase;
use crate::{domain::Story, repo::Repo, Result};
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
pub struct StoryService {
    create_story: CreateStory,
    delete_story: DeleteStory,
    get_story: GetStory,
    get_stories: GetStories,
    update_story: UpdateStory,
}

impl StoryService {
    /// Create a new story service
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            create_story: CreateStory::new(repo.clone()),
            delete_story: DeleteStory::new(repo.clone()),
            get_story: GetStory::new(repo.clone()),
            get_stories: GetStories::new(repo.clone()),
            update_story: UpdateStory::new(repo),
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
    pub async fn list(&self, page_id: i32, page_size: i32) -> Result<(i32, Vec<Story>)> {
        let args = (page_id, page_size);
        self.get_stories.execute(args).await
    }

    /// Update a story
    pub async fn update(&self, id: i32, name: String) -> Result<Story> {
        let args = (id, name);
        self.update_story.execute(args).await
    }
}
