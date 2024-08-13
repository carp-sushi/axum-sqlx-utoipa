use super::Messenger;
use crate::Result;

/// Sends text messages
pub struct SmsMessenger;

#[async_trait::async_trait]
impl Messenger for SmsMessenger {
    /// Send a SMS text message
    async fn send(&self, to: &str, msg: &str) -> Result<()> {
        tracing::info!("sending sms: to={}, msg={}", to, msg);
        Ok(())
    }
}
