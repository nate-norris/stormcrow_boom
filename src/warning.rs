use tokio::sync::mpsc;

#[derive(Debug)]
pub enum SerialError {
    SoundSensor,
    Radio,
}

pub type ErrorTx = mpsc::Sender<SerialError>;
pub type ErrorRx = mpsc::Receiver<SerialError>;

pub async fn notify_serial_error(mut rx: ErrorRx) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            Some(err) = rx.recv() => {
                match err {
                    SerialError::SoundSensor => turn_on_led().await,
                    SerialError::Radio => turn_on_led().await,
                }
            }
        }
    }
}

async fn turn_on_led() {
    loop {
        println!("Failed LED is on");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}