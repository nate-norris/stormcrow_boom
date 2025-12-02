use tokio::sync::mpsc;

pub enum EdgeDetection {
    Idle,
    Triggered,
    Error(String),
}

pub type EventTx = mpsc::Sender<EdgeDetection>;