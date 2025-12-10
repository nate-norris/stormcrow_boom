
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio_serial::{SerialStream, SerialPortBuilderExt};
use tokio::time::sleep;

use super::trait_mic::MicrophoneT;
use super::models::ERROR_START;

#[allow(dead_code)]
pub(crate) struct Microphone {
    port: Arc<Mutex<SerialStream>>,
}
impl MicrophoneT for Microphone {
    #[allow(dead_code)]
    fn new() -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new(
            "/dev/ttyUSB1", 115_200);
        let port = port_builder.open_native_async()?;
        Ok(Self { 
            port: Arc::new(Mutex::new(port)),
        })
    }

    #[allow(dead_code)]
    async fn boom_pattern(&self) -> anyhow::Result<()> {
        let mut port = self.port.lock().await;

        port.write_all(b"1").await?;
        sleep(std::time::Duration::from_millis(500)).await;
        port.write_all(b"1").await?;
        Ok(())
    }

    #[allow(dead_code)]
    fn spawn_error_pattern(&self) {
        let port = self.port.clone();

        ERROR_START.call_once(|| {
            tokio::spawn(async move {
                loop {
                    let mut p = port.lock().await;
                    if let Err(e) = p.write_all(b"1").await {
                        eprintln!("failed {:?}", e);
                    }
                    // p.write_all(b"1").await;
                    drop(p); // release lock before sleeping
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });
    }
}