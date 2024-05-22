use async_trait::async_trait;

/// Services for stories.
pub mod story;

/// Services for tasks.
pub mod task;

/// Defines a single unit of business logic.
/// Inspired by Finagle's Service type: `trait Service[Req, Rep] extends (Req => Future[Rep])`
/// TODO: Look at using `Fn` (requires unstable feature "fn_traits").
#[async_trait]
pub trait Service: Sized + Send + Sync + 'static {
    /// Input arguments
    type Req: Send + 'static;

    /// Output results
    type Rep: Send + 'static;

    /// Business logic
    async fn call(&self, req: Self::Req) -> Self::Rep;
}
