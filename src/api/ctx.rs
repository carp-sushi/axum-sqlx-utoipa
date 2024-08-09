use crate::repo::Repo;
use sqlx::postgres::PgPool;
use std::{ops::Deref, sync::Arc};

/// API context
#[derive(Clone)]
pub struct Ctx {
    repo: Arc<Repo>,
}

impl Ctx {
    /// Create a new api context.
    pub fn new(db: Arc<PgPool>) -> Self {
        let repo = Arc::new(Repo::new(db.clone()));
        Self { repo }
    }
}

// Allow access to calls on the inner repo.
impl Deref for Ctx {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}
