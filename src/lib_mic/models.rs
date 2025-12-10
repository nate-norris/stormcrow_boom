use tokio::sync::mpsc;

#[derive(Debug)]
pub enum MicNotification {
    Boom,
    SoundSensorError,
    RadioError,
}

// mpsc channels for MicNotification types
pub type MicTx = mpsc::Sender<MicNotification>;
pub type MicRx = mpsc::Receiver<MicNotification>;

pub(crate) static ERROR_START: std::sync::Once = std::sync::Once::new();