use tokio::sync::mpsc;
use std::sync::Arc;

mod lib_sensor;
mod lib_sensor_consumer;
mod mm2t;
mod mic;
use lib_sensor::{EventTx, SoundSensor, SoundSensorMock, SoundSensorT};
use lib_sensor_consumer::{EventRx, sensor_consume_task};
use mm2t::MM2THandle;
use mic::{MicTx, MicRx, MicNotification, mic_consume_task};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // define channels
    // sound sensor: tx and rx for EdgeDetection events
    let (tx, rx): (EventTx, EventRx) 
        = mpsc::channel(32);
    // handle mic transmit and receive
    //      notifications include sound sensor trigger, sound sensor error, 
    //      and radio error
    let (mic_tx, mic_rx): 
        (MicTx, MicRx) = mpsc::channel(32);

    // await for MicNotification
    // NOTE: mic is initialized before tasks so that it is ready to receive
    mic_consume_task(mic_rx).await;
    // swap which sensor will detect edge
    let _sensor = SoundSensor; //hardware sound sensor
    let sensor = SoundSensorMock; //mock sensor for testing

    // initialize mm2t radio
    //      sends mic notification error if failed to start
    let mic_tx = mic_tx.clone(); // shadow tx
    let radio = match MM2THandle::start().await {
        Ok(r) => r, // assign to radio
        Err(_e) => {
            let _ = mic_tx.send(MicNotification::RadioError).await;
            return Ok(());
        }
    };
    let radio = Arc::new(radio); //

    // spawn sound sensor thread for triggering edge detects
    let mic_tx_spawn = mic_tx.clone();
    tokio::spawn(async move {
        // await sound sensor edge detect
        //      handle sound sensor error if occurs
        if let Err(_e) = sensor.detect_edge_task(tx.clone()).await {
            let _ = mic_tx_spawn.send(MicNotification::SoundSensorError).await;
        }
    });

    // consume sound sensor receive
    let mic_tx_spawn = mic_tx.clone(); // clone for mic tx
    sensor_consume_task(rx, move || {
        let mic_tx_spawn2 = mic_tx_spawn.clone();
        let radio_spawn = Arc::clone(&radio);
        async move {
            //notify mic of shot success
            let _ = mic_tx_spawn2.send(MicNotification::Boom).await;

            // send radio packet and handle 
            if let Err(_e) = radio_spawn.send_trigger_packet().await {
                let _ = mic_tx_spawn2.send(MicNotification::RadioError).await;
            }
        }
    })
    .await;

    Ok(())
}