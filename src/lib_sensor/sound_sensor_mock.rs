use std::future::Future;
use std::pin::Pin;
use tokio::time::{sleep, Duration};
use fastrand;

use super::models::{EdgeDetection, EventTx};
use super::trait_sensor::SoundSensorT;

pub struct SoundSensorMock;
impl SoundSensorT for SoundSensorMock {
    fn detect_edge_task(&self, tx: EventTx) -> 
        Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        Box::pin(async move {
            loop {
                // Simulate multiple triggers
                for _ in 0..fastrand::u64(1..=5) {
                    tx.send(EdgeDetection::Triggered).await.unwrap();
                    sleep(Duration::from_millis(50)).await;
                }
                
                // waiting period until next trigger occurs
                sleep(Duration::from_secs(fastrand::u64(10..=60))).await;
            }
        })
    }
}