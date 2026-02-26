use std::future::Future;
use std::pin::Pin;
use tokio::time::{sleep, Duration};
use fastrand;

use super::models::{EdgeDetection, EventTx};
use super::trait_sensor::SoundSensorT;

/// A mock implementation of [`SoundSensorT`] used for testing or development
/// environments where no physical hardware is present.
///
/// This mock repeatedly generates randomized edge-detection events. It sends
/// between 1 and 5 rapid triggers, waits a random period, and repeats
/// indefinitely.
///
/// # Behavior
/// - The returned future **never resolves**; and has *diverging* type
///   (`!`) return which coerces into `anyhow::Result<()>`.
pub struct SoundSensorMock;

impl SoundSensorT for SoundSensorMock {
    /// Creates a future that continuously detects edge transitions and
    /// publishes resulting events to the provided event channel.
    ///
    /// # Arguments
    /// * `tx` — The channel used to send EdgeDetection event notifications to 
    /// the consumer.
    ///
    /// # Returns
    /// `!` which coerces to anyhow::Result<()>
    /// 
    /// # Behavior
    /// 1. Simulates 1-5 EdgeDetection::Triggered events 
    /// 2. Waits a period, 
    /// 3. Repeats.
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
                let wait = Duration::from_secs(fastrand::u64(5..=10));
                sleep(wait).await;
            }
        })
    }
}