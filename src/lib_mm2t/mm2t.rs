
use tokio::sync::Mutex;
use tokio_serial::{SerialStream, SerialPortBuilderExt, DataBits, Parity, 
    StopBits, FlowControl}; //SerialStream
use tokio::io::AsyncWriteExt;

/// Represents a handle to an MM2T device over a serial connection.
///
/// This struct manages a serial port connection internally via a `Mutex` for
/// safe asynchronous access. It provides methods to initialize the device and
/// send a bytes.
///
/// # Example
/// ```rust,no_run
/// use mm2t::MM2TTransport;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Initialize the MM2T device
///     let mm2t = MM2TTransport::start("/dev/ttyUSB0").await?;
///
///     // Send a trigger packet
///     mm2t.send(&[0x01]).await?;
///
///     Ok(())
/// }
/// ```
pub struct MM2TTransport {
    port: Mutex<SerialStream>
}

impl MM2TTransport {

    /// opens a serial connection to the MM2T device
    pub async fn start(port_name: &str) -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new(port_name, 38_400)
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

    /// send bytes over the port
    pub async fn send(&self, bytes: &[u8]) -> anyhow::Result<()> {
        let mut port = self.port.lock().await;
        port.write_all(bytes).await?;
        port.flush().await?;

        Ok(())
    }
}