use std::pin::Pin;
use std::future::Future;
use tokio::time::{sleep, Duration, Sleep};

use super::models::EventRx;
use crate::lib_sensor::EdgeDetection;
use crate::logger;

/// Asynchronous sound sensor consumer task.
///
/// This task listens on an `mpsc` channel (`EventRx`) for sound trigger
/// notifications EdgeDetection. It is typically spawned using `tokio::spawn` and runs until
/// the sending side of the channel is dropped.
///
/// # Type Parameters
/// - `F` — A closure type that is called when a trigger is finalized.
/// - `Fut` — The future returned by the closure `F`. Must be `Send` and `'static`.
///
/// # Arguments
/// * `rx` — The receiving end of an event channel providing [`EdgeDetection`] events.
/// * `on_trigger` — A closure returning a future that is executed after the trigger timer.
///
/// # Behavior
/// 1. Waits for sensor events on `rx`.
/// 2. On `EdgeDetection::Triggered`, starts a 1-second timer (resetting if multiple triggers occur).
/// 3. After the timer completes, executes `on_trigger().await` and resets the timer.
/// 4. On `EdgeDetection::Error`, logs the error to stderr but continues listening.
///
///
/// # Example
/// ```rust
/// use lib_sensor::{EdgeDetection, EventTx, SoundSensorMock, SoundSensorT};
/// use lib_sensor_consumer::sensor_consume_task;
/// use tokio::sync::mpsc;
///
/// #[tokio::main]
/// async fn main() {
///     let (tx, rx) = mpsc::channel(32);
///     let sensor = SoundSensorMock;
///
///     // Spawn the sensor simulation
///     tokio::spawn(sensor.detect_edge_task(tx));
///
///     // Spawn the consumer task
///     tokio::spawn(sensor_consume_task(rx, || async {
///         println!("Trigger action executed!");
///     }));
/// }
/// ```
pub async fn sensor_consume_task<F, Fut>(mut rx: EventRx, mut on_trigger: F) 
    where F: FnMut() -> Fut,
    Fut: Future<Output = ()> + Send + 'static {

    let mut timer: Option<Pin<Box<Sleep>>> = None; // start timer delay
    // 1 sec duration to trigger on last received
    let wait = Duration::from_secs(1); 

    loop {
        tokio::select! {
            // perform match for EdgeDetection types
            Some(event) = rx.recv() => {
                match event {
                    // assign the timer future on heap
                    EdgeDetection::Triggered => {
                        logger::info("Trigger sensor consumer received");
                        timer = Some(Box::pin(sleep(wait)));
                    }
                    // log any errors
                    EdgeDetection::Error(msg) => {
                        logger::error("Trigger sensor error", Some(msg));
                    }
                }
            }

            // verify timer completes a run then complete trigger action
            _ = async {
                match &mut timer {
                    // poll sleep future
                    Some(t) => {
                        t.as_mut().await;
                    },
                    // return a matched future that never resolves
                    None => std::future::pending::<()>().await,
                }
            } => {
                // after completion drop timer and execute on_trigger
                on_trigger().await;
                timer = None;
            }
        }
    }
}