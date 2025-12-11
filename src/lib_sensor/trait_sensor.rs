use std::pin::Pin;
use std::future::Future;
 
use super::models::EventTx; // use crate::models::EventTx;

/// Trait defining sound sensor behavior for real or mocked implementations
/// 
/// Implementations of this trait produce an asynchronous task that monitors
/// the underlying sensor for an “edge” event (detected sound spike)
///
/// When an edge is detected, the task should send an appropriate notification
/// through the provided [`EventTx`] channel.
pub trait SoundSensorT {
    /// Creates a future that continuously detects edge transitions and
    /// publishes resulting events to the provided event channel.
    ///
    /// # Arguments
    /// * `tx` — The channel used to send event notifications to the consumer.
    ///
    /// # Returns
    /// A boxed, pinned, sendable future that resolves to:
    /// * `Ok(())` when the task ends gracefully.
    /// * `Err(anyhow::Error)` if sensor reading, parsing, or communication fails.
    ///
    /// The future typically runs in a loop and only resolves when the task
    /// is cancelled or a fatal error occurs.
    fn detect_edge_task(&self, tx: EventTx) -> 
    Pin<Box<dyn Future<Output= anyhow::Result<()>> + Send>>;
}