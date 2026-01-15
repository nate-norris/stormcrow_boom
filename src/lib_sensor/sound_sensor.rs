
use std::future::Future;
use std::pin::Pin;
use tokio_serial::SerialPortBuilderExt;
use tokio::io::AsyncReadExt;

use super::models::{EdgeDetection, EventTx};
use super::trait_sensor::SoundSensorT;

/// A real implementation of [`SoundSensorT`] used for
/// connecting to physical hardware.
pub struct SoundSensor;

impl SoundSensorT for SoundSensor {
    /// Creates a future that continuously detects edge transitions and
    /// publishes resulting events to the provided event channel.
    ///
    /// # Arguments
    /// * `tx` — The channel used to send EdgeDetection event notifications to 
    /// the consumer.
    ///
    /// # Returns
    /// A boxed, pinned, sendable future that resolves to:
    /// * `Err(anyhow::Error)` if sensor reading, parsing, or communication fails.
    /// 
    /// # Behavior
    /// 1. Opens the port and continues reading for 0x01
    /// 2. Sends EdgeDetection::Triggered upon receiving a proper response
    /// 3. Sends EdgeDetection::Error on failures and returns
    fn detect_edge_task(&self, tx: EventTx) -> 
        Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>  {
        Box::pin(async move {
            /* TODO determine buad rate
            9600 (classic for many modules) x
            19200
            38400
            57600
            115200 (very common for high-speed microcontrollers) x
            230400
            460800
            921600 (rare, mostly for high-throughput devices)
            */
            // open the port
            let mut port = tokio_serial::new("/dev/ttyUSB1", 
                115_200).open_native_async()?;
            let mut buf = [0u8; 1]; // read one byte

            loop {
                match port.read(&mut buf).await {
                    // transmit if reading 0x01 byte
                    Ok(1) if buf[0] == 0x01 => {tx.send(EdgeDetection::Triggered).await?; },
                    Ok(_) => {/* ignore wildcard */},
                    // notify and return if any serial errors
                    Err(e) => {
                        let _ = tx.send(EdgeDetection::Error(format!("failed sound sensor"))).await;
                        return Err(e.into()); 
                    }
                }
            }
        })
    }
}