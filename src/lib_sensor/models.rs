use tokio::sync::mpsc;

pub enum EdgeDetection {
    Triggered,
    Error(String),
}

pub type EventTx = mpsc::Sender<EdgeDetection>;