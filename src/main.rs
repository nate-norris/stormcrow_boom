use tokio::sync::mpsc;
use std::sync::Arc;

mod lib_sensor;
mod lib_sensor_consumer;
mod mm2t;
mod mic;
use lib_sensor::{EventTx, SoundSensorMock, SoundSensorT};
use lib_sensor_consumer::{EventRx, sensor_consume_task};
use mm2t::MM2THandle;
use mic::{MicTx, MicRx, MicNotification, mic_consume_task};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // sound sensor channels for EdgeDetection events
    let (tx, rx): (EventTx, EventRx) 
        = mpsc::channel(32);
    // mic channels for MicNotification events
    let (mic_tx, mic_rx): 
        (MicTx, MicRx) = mpsc::channel(32);

    // await for MicNotification
    // NOTE: mic is initialized before tasks so that it is ready to receive
    tokio::spawn(mic_consume_task(mic_rx));

    // sensor for sound trigger
    let sensor = SoundSensorMock; //mock sensor for testing

    // initialize mm2t radio
    //      sends mic notification error if failed to start
    let radio = match MM2THandle::start().await {
        Ok(r) => Arc::new(r), // assign to radio
        Err(_e) => {
            let _ = mic_tx.send(MicNotification::RadioError).await;
            return Ok(());
        }
    };

    // spawn sound sensor thread for triggering edge detects
    let mic_tx_edge = mic_tx.clone();
    tokio::spawn(async move {
        // await sound sensor edge detect
        //      handle sound sensor error if occurs
        if let Err(_e) = sensor.detect_edge_task(tx.clone()).await {
            let _ = mic_tx_edge.send(MicNotification::SoundSensorError).await;
        }
    });

    // consume sound sensor receive
    let mic_tx_consume = mic_tx.clone(); // clone for mic tx
    sensor_consume_task(rx, move || {
        let mic_tx = mic_tx_consume.clone();
        let radio = Arc::clone(&radio);
        async move {
            //notify mic of shot success
            let _ = mic_tx.send(MicNotification::Boom).await;

            // send radio packet and handle 
            if let Err(_e) = radio.send_trigger_packet().await {
                let _ = mic_tx.send(MicNotification::RadioError).await;
            }
        }
    })
    .await;

    Ok(())
}