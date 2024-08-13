use super::Messenger;
use crate::Result;

/// Sends emails
pub struct EmailMessenger;

#[async_trait::async_trait]
impl Messenger for EmailMessenger {
    /// Send an email message
    async fn send(&self, to: &str, msg: &str) -> Result<()> {
        tracing::info!("sending email: to={}, msg={}", to, msg);
        Ok(())
    }
}
