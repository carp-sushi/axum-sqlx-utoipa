use uuid::Uuid;

use crate::{driver::storage::Storage, repo::Repo};
use std::sync::Arc;

/// API context
#[derive(Clone)]
pub struct Ctx {
    pub repo: Arc<Repo>,
    pub storage: Arc<Box<dyn Storage<Uuid>>>,
}

impl Ctx {
    /// Create a new api context.
    pub fn new(repo: Arc<Repo>, storage: Arc<Box<dyn Storage<Uuid>>>) -> Self {
        Self { repo, storage }
    }
}
