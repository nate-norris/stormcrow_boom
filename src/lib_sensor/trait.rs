use std::pin::Pin;
use std::future::Future;
 
use super::models::EventTx; // use crate::models::EventTx;

pub trait SoundSensorT {
    fn detect_edge_task(&self, tx: EventTx) -> 
    Pin<Box<dyn Future<Output= anyhow::Result<()>> + Send>>;
}