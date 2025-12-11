
use tokio::sync::Mutex;
use tokio_serial::{SerialStream, SerialPortBuilderExt, DataBits, Parity, 
    StopBits, FlowControl}; //SerialStream
use tokio::io::AsyncWriteExt;

/// Represents a handle to an MM2T device capable of sending "Boom" trigger packets
/// over a serial connection.
///
/// This struct manages a serial port connection internally via a `Mutex` for
/// safe asynchronous access. It provides methods to initialize the device and
/// send a predefined packet.
///
/// # Example
/// ```rust,no_run
/// use mm2t::MM2TBoomHandle;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Initialize the MM2T device
///     let boom_handle = MM2TBoomHandle::start().await?;
///
///     // Send a trigger packet
///     boom_handle.send_trigger_packet().await?;
///
///     Ok(())
/// }
/// ```
pub struct MM2TBoomHandle {
    port: Mutex<SerialStream>
}

impl MM2TBoomHandle {

    // packet contents
    const SOP: u8 = 0xAA; // start of packet
    const PACKET_TYPE: u8 = 0x42; //character 'B'... as in Boom
    const PAYLOAD: [u8; 1] = [0x01];
    const LENGTH_PAYLOAD: u8 = Self::PAYLOAD.len() as u8;
    // precalculated checksum
    // NOTE: including logic for calculation for reverse engineering
    //      on receiving side.
    const CHECKSUM: u8 = Self::calc_checksum(Self::PACKET_TYPE, 
        Self::LENGTH_PAYLOAD, &Self::PAYLOAD);//0x42;
    // constructed packet
    const PACKET: [u8; 5] = [
        Self::SOP,
        Self::PACKET_TYPE,
        Self::LENGTH_PAYLOAD,
        Self::PAYLOAD[0],
        Self::CHECKSUM,
    ];

    /// opens a serial connection to the MM2T device
    pub async fn start() -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new("/dev/ttyUSB0", 38_400)
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

    /// sends the predefined "Boom" packet
    pub async fn send_trigger_packet(&self) -> anyhow::Result<()> {
        
        let mut port = self.port.lock().await;
        port.write_all(&Self::PACKET).await?;
        port.flush().await?;
        println!("mm2t: sent packet {:?}", &Self::PACKET);
        Ok(())
    }

    /// internal helper for for checksum calculation
    const fn calc_checksum(p_t: u8, l: u8, p: &[u8]) -> u8 {
        let mut checksum = p_t ^ l;

        let mut i = 0;
        while i < p.len() {
            checksum ^= p[i];
            i += 1;
        }
        checksum
    }
}