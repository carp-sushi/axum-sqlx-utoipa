use crate::{domain::StorageId, driver::storage::Storage, repo::Repo};
use std::sync::Arc;

/// Context contains repo and driver pointers for use in API routes.
#[derive(Clone)]
pub struct Ctx {
    /// Binary object storage
    pub storage: Arc<Box<dyn Storage<StorageId>>>,

    /// Database storage
    pub repo: Arc<Repo>,
}

impl Ctx {
    /// Create a new API context
    pub fn new(storage: Arc<Box<dyn Storage<StorageId>>>, repo: Arc<Repo>) -> Self {
        Self { storage, repo }
    }
}
