use tokio::sync::mpsc;

use crate::lib_sensor::EdgeDetection;

pub type EventRx = mpsc::Receiver<EdgeDetection>;