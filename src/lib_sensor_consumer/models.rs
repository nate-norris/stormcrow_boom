use tokio::sync::mpsc;

use crate::lib_sensor::EdgeDetection;

// mpsc channel for EdgeDetection type
// used by consumer to process edge events
pub type EventRx = mpsc::Receiver<EdgeDetection>;