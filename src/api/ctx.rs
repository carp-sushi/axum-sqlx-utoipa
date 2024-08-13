use crate::{driver::message::Messenger, repo::Repo};
use std::{ops::Deref, sync::Arc};

/// API context
#[derive(Clone)]
pub struct Ctx {
    repo: Arc<Repo>,
    pub messenger: Arc<Box<dyn Messenger>>,
}

impl Ctx {
    /// Create a new api context.
    pub fn new(repo: Arc<Repo>, messenger: Arc<Box<dyn Messenger>>) -> Self {
        Self { repo, messenger }
    }
}

// Allow access to calls on the inner repo.
impl Deref for Ctx {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}
