use super::models::{MicRx, MicNotification};
use super::mic_mock::MicrophoneMock;
use super::trait_mic::MicrophoneT;

/// Asynchronous microphone consumer task.
///
/// This task listens on an `mpsc` channel (`MicRx`) for microphone-related
/// notifications MicNotification. It is typically spawned using `tokio::spawn` and runs until
/// the sending side of the channel is dropped.
///
/// # Behavior
/// - **`MicNotification::Boom`**  
///   Triggers a one-time *boom* beep pattern using `boom_pattern()`,  
///   **but only if no error has been triggered yet**.
///
/// - **`MicNotification::SoundSensorError` or `MicNotification::RadioError`**  
///   Marks the system as being in an error state and calls
///   `spawn_error_pattern()`, which begins a *repeating* error beep loop.
/// Once an error state has been triggered, subsequent `Boom` events are ignored.
///
/// # Arguments
/// * `rx` – The receiving half of an `mpsc` channel through which
///   `MicNotification` events are delivered.
///
/// # Example
/// ```no_run
/// use tokio::sync::mpsc;
/// use mycrate::lib_mic::consume::mic_consume_task;
/// use mycrate::lib_mic::models::MicNotification;
///
/// #[tokio::main]
/// async fn main() {
///     let (tx, rx) = mpsc::channel(32);
///
///     // Spawn the microphone task
///     tokio::spawn(async move {
///         mic_consume_task(rx).await;
///     });
///
///     // Send events
///     tx.send(MicNotification::Boom).await.unwrap();
///     tx.send(MicNotification::SoundSensorError).await.unwrap();
/// }
/// ```
///
/// # Notes
/// - `MicrophoneMock` is used internally; replace with a real microphone
///   implementation by updating the constructor.
/// - Task terminates when `rx` is closed and all senders are dropped.
///
/// # Errors
/// This function does not return errors; microphone failures should be handled
/// inside the `MicrophoneT` implementation.
pub async fn mic_consume_task(mut rx: MicRx) {
    let mut error_triggered = false;
    // microphone for sound patterns
    let microphone = MicrophoneMock::new().unwrap();

    // wait for incoming MicRx
    while let Some(event) = rx.recv().await {
        match event {
            // when there is a boom with no errors create the boom pattern
            MicNotification::Boom => {
                if !error_triggered {
                    let _ = microphone.boom_pattern().await;
                }
            }
            // if errors occur turn on the error pattern
            //      lockout other booms from occuring
            MicNotification::SoundSensorError | MicNotification::RadioError => {
                error_triggered = true;
                microphone.spawn_error_pattern();
            }
        }
    }

}