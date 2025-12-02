use tokio::sync::mpsc;

mod lib_sensor;
mod lib_sensor_consumer;
use lib_sensor::{EventTx, SoundSensor, SoundSensorMock, SoundSensorT};
use lib_sensor_consumer::EventRx;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tx and rx with buffer of 32 messages
    let (tx, _rx): (EventTx, EventRx) = mpsc::channel(32);

    let sensor = SoundSensor;
    let _sensor = SoundSensorMock;
    tokio::spawn(async move {
        if let Err(e) = sensor.detect_edge_task(tx.clone()).await {
            eprintln!("Sensor failed: {:?}", e);
        }
    });

    // consume_sensor_events(rx).await;
    Ok(())
}