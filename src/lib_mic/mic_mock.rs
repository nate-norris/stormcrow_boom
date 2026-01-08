use tokio::time::sleep;

use super::trait_mic::MicrophoneT;
use super::models::ERROR_START;

/// A mock microphone implementation used for testing and development.
///
/// `MicrophoneMock` does not interact with real hardware. Instead, it prints
/// messages to stdout to simulate:
#[allow(dead_code)]
pub(crate) struct MicrophoneMock {}

impl MicrophoneT for MicrophoneMock {
    /// Initializes a new microphone instance.
    ///
    /// # Returns
    /// - `Ok(Self)`
    fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }


    /// Plays a one-time “boom” beep pattern.
    ///
    /// # Behavior
    /// - Activated upon sound sensor edge detects.
    /// - It is only triggered when no error state has occurred.
    ///
    /// Returns an error if the tone or pattern playback fails.
    /// - `Ok(())``
    #[allow(dead_code)]
    async fn boom_pattern(&self) -> anyhow::Result<()> {
        println!("mic mock: boom pattern");
        Ok(())
    }

    /// Starts a repeating error notification pattern.
    ///
    /// # Behavior
    /// - Uses `ERROR_START.call_once` to ensure the loop is only launched once.
    /// - Spawns the loop via `tokio::spawn` so the function returns immediately.
    ///
    /// # Notes
    /// This function is intentionally non-async because it should launch
    /// its own async task rather than block the caller.
    #[allow(dead_code)]
    fn spawn_error_pattern(&self) {
        // call spawn for error detection pattern one time only
        ERROR_START.call_once(|| {
            tokio::spawn(async move {
                loop {
                    println!("mic mock: error pattern output");
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });
    }
}