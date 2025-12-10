use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio_serial::{SerialStream, SerialPortBuilderExt};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub enum MicNotification {
    Boom,
    SoundSensorError,
    RadioError,
}
pub type MicTx = mpsc::Sender<MicNotification>;
pub type MicRx = mpsc::Receiver<MicNotification>;
static ERROR_START: std::sync::Once = std::sync::Once::new();

pub async fn mic_consume_task(mut rx: MicRx) {
    let mut error_triggered: bool = false;
    let microphone = MicrophoneMoc::new().unwrap();
    let _microphone = Microphone::new().unwrap();

    while let Some(event) = rx.recv().await {
        match event {
            MicNotification::Boom => {
                if !error_triggered {
                    let _ = microphone.boom_pattern().await;
                }
            }
            MicNotification::SoundSensorError | MicNotification::RadioError => {
                error_triggered = true;
                microphone.spawn_error_pattern();
            }
        }
    }

}

#[allow(dead_code)]
struct Microphone {
    port: Arc<Mutex<SerialStream>>,
}
impl Microphone {
    fn new() -> anyhow::Result<Self> {
        //define parameters for opening serial port
        let port_builder = tokio_serial::new(
            "/dev/ttyUSB2", 115_200);
        let port = port_builder.open_native_async()?;
        Ok(Self { 
            port: Arc::new(Mutex::new(port)),
        })
    }

    #[allow(dead_code)]
    async fn boom_pattern(&self) -> anyhow::Result<()> {
        let mut port = self.port.lock().await;

        port.write_all(b"1").await?;
        sleep(Duration::from_millis(500)).await;
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

#[allow(dead_code)]
struct MicrophoneMoc {}

impl MicrophoneMoc {
    fn new() -> anyhow::Result<Self> {
        println!("mic: new");
        Ok(Self {})
    }

    #[allow(dead_code)]
    async fn boom_pattern(&self) -> anyhow::Result<()> {
        println!("mic: boom pattern");
        Ok(())
    }

    #[allow(dead_code)]
    fn spawn_error_pattern(&self) {
        ERROR_START.call_once(|| {
            tokio::spawn(async move {
                loop {
                    println!("mic: error pattern output");
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });


    }
}