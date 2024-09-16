use crate::{driver::storage::Storage, repo::Repo};
use std::sync::Arc;
use uuid::Uuid;

/// Context contains pointers to keepers and drivers for use in API routes.
#[derive(Clone)]
pub struct Ctx {
    /// Binary object storage
    pub storage: Arc<Box<dyn Storage<Uuid>>>,

    /// Persistence API
    pub repo: Arc<Repo>,
}

impl Ctx {
    /// Create a new API context
    pub fn new(storage: Arc<Box<dyn Storage<Uuid>>>, repo: Arc<Repo>) -> Self {
        Self { storage, repo }
    }
}
