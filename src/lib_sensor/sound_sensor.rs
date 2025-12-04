
use std::future::Future;
use std::pin::Pin;
use tokio_serial::SerialPortBuilderExt;
use tokio::io::AsyncReadExt;

use super::models::{EdgeDetection, EventTx};
use super::trait_sensor::SoundSensorT;

/*
baud rate checks
9600 (classic for many modules) x
19200
38400
57600
115200 (very common for high-speed microcontrollers) x
230400
460800
921600 (rare, mostly for high-throughput devices)
*/
pub struct SoundSensor;
impl SoundSensorT for SoundSensor {
    fn detect_edge_task(&self, tx: EventTx) -> 
    Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>  {
        Box::pin(async move {
            let mut port = tokio_serial::new("/dev/ttyUSB0", 
                115_200).open_native_async()?;
            let mut buf = [0u8; 1]; // read one byte

            loop {
                match port.read(&mut buf).await {
                    Ok(1) if buf[0] == 0x01 => {tx.send(EdgeDetection::Triggered).await?; },
                    Ok(_) => {/* ignore wildcard */},
                    Err(e) => {
                        let _ = tx.send(EdgeDetection::Error(format!("failed sound sensor"))).await;
                        return Err(e.into()); 
                    }
                }
            }
        })
    }
}