use tokio::sync::mpsc;

mod lib_sensor;
mod lib_sensor_consumer;
mod mm2t;
use lib_sensor::{EventTx, SoundSensor, SoundSensorMock, SoundSensorT};
use lib_sensor_consumer::{EventRx, sensor_consume_task};
use mm2t::send_radio_packet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tx and rx with buffer of 32 messages
    let (tx, rx): (EventTx, EventRx) = mpsc::channel(32);

    let sensor = SoundSensor;
    let _sensor = SoundSensorMock;
    tokio::spawn(async move {
        if let Err(e) = sensor.detect_edge_task(tx.clone()).await {
            eprintln!("Sensor failed: {:?}", e);
        }
    });

    sensor_consume_task(rx, move || async {
        let _ = send_radio_packet().await.unwrap();
    }).await;

    Ok(())
}