
use tokio::sync::Mutex;
use tokio_serial::{SerialStream, SerialPortBuilderExt, DataBits, Parity, 
    StopBits, FlowControl}; //SerialStream
use tokio::io::AsyncWriteExt;

pub struct MM2THandle {
    port: Mutex<SerialStream>
}
impl MM2THandle {
    const PACKET: [u8; 3] = [0xAA, 0xBB, 0xCC];

    pub async fn start() -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new("/dev/ttyUSB1", 38_400)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .timeout(std::time::Duration::from_secs(3));

        let stream = port_builder
            .open_native_async()?;

        Ok(Self {
            port: Mutex::new(stream)
        })
    }

    pub async fn send_trigger_packet(&self) -> anyhow::Result<()> {
        
        let mut port = self.port.lock().await;
        port.write_all(&Self::PACKET).await?;
        port.flush().await?;
        Ok(())
    }
}