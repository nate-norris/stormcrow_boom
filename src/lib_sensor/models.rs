use tokio::sync::mpsc;

// Notification events emitted by sound sensor hardware
pub enum EdgeDetection {
    Triggered, // sound sensor has detected an event
    Error(String), // error in physical sound sensor
}

// mpsc channels for EdgeDetection types
// used by producers to send edge events
pub type EventTx = mpsc::Sender<EdgeDetection>;