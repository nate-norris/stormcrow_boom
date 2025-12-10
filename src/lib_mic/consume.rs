use super::models::{MicRx, MicNotification};
use super::mic_mock::MicrophoneMock;
use super::trait_mic::MicrophoneT;

pub async fn mic_consume_task(mut rx: MicRx) {
    let mut error_triggered: bool = false;
    let microphone = MicrophoneMock::new().unwrap();

    while let Some(event) = rx.recv().await {
        match event {
            MicNotification::Boom => {
                if !error_triggered {
                    let _ = microphone.boom_pattern().await;
                }
            }
            MicNotification::SoundSensorError | MicNotification::RadioError => {
                error_triggered = true;
                microphone.spawn_error_pattern();
            }
        }
    }

}