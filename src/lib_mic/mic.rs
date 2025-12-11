
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio_serial::{SerialStream, SerialPortBuilderExt};
use tokio::time::sleep;

use super::trait_mic::MicrophoneT;
use super::models::ERROR_START;

/// A real microphone implementation using a serial port.
///
/// This struct communicates with a physical microphone device connected  
/// The device expects byte sequences to trigger boom and error patterns.
#[allow(dead_code)]
pub(crate) struct Microphone {
    port: Arc<Mutex<SerialStream>>,
}
impl MicrophoneT for Microphone {
    /// initialized, or configured properly.
    /// 
    /// Initializes a new microphone instance.
    ///
    /// # Returns
    /// - `Ok(Self)` if initialization succeeds.
    /// - `Err(...)` if initialization fails.
    ///
    /// # Errors
    /// Returns an error if the microphone port cannot be created,
    /// initialized, or configured properly.
    #[allow(dead_code)]
    fn new() -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new(
            "/dev/ttyUSB2", 115_200);
        let port = port_builder.open_native_async()?;
        Ok(Self { 
            port: Arc::new(Mutex::new(port)),
        })
    }

    /// Plays a one-time “boom” beep pattern.
    ///
    /// # Behavior
    /// - Activated upon sound sensor edge detects.
    /// - It is only triggered when no error state has occurred.
    ///
    /// Returns an error if the tone or pattern playback fails.
    /// - `Ok(())` if the boom succeeds.
    /// - `Err(...)` if boom serial fails.
    /// 
    /// # Errors
    ///  Returns an error if the boom fails.
    #[allow(dead_code)]
    async fn boom_pattern(&self) -> anyhow::Result<()> {
        let mut port = self.port.lock().await;

        port.write_all(b"1").await?;
        sleep(std::time::Duration::from_millis(500)).await;
        port.write_all(b"1").await?;
        Ok(())
    }

    /// Starts a repeating error notification pattern.
    ///
    /// # Behavior
    /// - This function must immediately return.
    /// - The pattern should continue repeating in the background.
    /// - Implementations typically use `tokio::spawn` or similar.
    ///
    /// # Notes
    /// This function is intentionally non-async because it should launch
    /// its own async task rather than block the caller.
    #[allow(dead_code)]
    fn spawn_error_pattern(&self) {
        let port = self.port.clone();
        // call spawn for error detection pattern one time only
        ERROR_START.call_once(|| {
            tokio::spawn(async move {
                loop {
                    let mut p = port.lock().await;
                    // send bytes for beep command
                    //      if error occurs simply log
                    if let Err(e) = p.write_all(b"1").await {
                        eprintln!("failed {:?}", e);
                    }
                    drop(p); // release lock before sleeping
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });
    }
}