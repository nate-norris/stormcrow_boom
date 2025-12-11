use tokio::sync::mpsc;

/// Notification events emitted by microphone hardware or serial interrupts
#[derive(Debug)]
pub enum MicNotification {
    Boom, // sound sensor has detected an event
    // both SoundSensorError and Radio error trigger program failure
    SoundSensorError, // indicates physical sound sensor has an error
    RadioError, // indicates mm25 radio has an error
}

// mpsc channels for MicNotification types
// used by producers to send edge events
pub type MicTx = mpsc::Sender<MicNotification>;
// used by consuemrs to receive edge events
pub type MicRx = mpsc::Receiver<MicNotification>;  

// global one-time initialization flag for triggering
//      microphone error pattern only once
//
// Implementations wrap calls with ERRO_START.call_once()
pub(crate) static ERROR_START: std::sync::Once = std::sync::Once::new();