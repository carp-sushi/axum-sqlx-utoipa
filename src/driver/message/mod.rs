use crate::Result;

pub mod email;
pub mod sms;

/// Sends communication messages
#[async_trait::async_trait]
pub trait Messenger: Send + Sync {
    /// Send a message
    async fn send(&self, to: &str, msg: &str) -> Result<()>;
}
