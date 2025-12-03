use tokio::sync::mpsc;

mod lib_sensor;
mod lib_sensor_consumer;
mod mm2t;
mod mic;
use lib_sensor::{EventTx, SoundSensor, SoundSensorMock, SoundSensorT};
use lib_sensor_consumer::{EventRx, sensor_consume_task};
use mm2t::send_radio_packet;
use mic::{MicTx, MicRx, MicNotification, mic_consume_task};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // sound sensor: tx and rx for EdgeDetection events
    let (tx, rx): (EventTx, EventRx) 
        = mpsc::channel(32);

    // handle mic transmit and receive
    //      notifications include sound sensor trigger, sound sensor error, 
    //      and radio error
    let (mic_tx, mic_rx): 
        (MicTx, MicRx) = mpsc::channel(32);
        // consume microphone receive
    mic_consume_task(mic_rx).await;

    // swap which sensor will detect edge
    let _sensor = SoundSensor; //hardware sound sensor
    let sensor = SoundSensorMock; //mock sensor for testing

    // spawn sound sensor thread for triggering edge detects
    let mic_tx2 = mic_tx.clone(); // clone for mic tx
    tokio::spawn(async move {
        // await sound sensor edge detect
        //      handle sound sensor error if occurs
        if let Err(_e) = sensor.detect_edge_task(tx.clone()).await {
            let _ = mic_tx2.send(MicNotification::SoundSensorError).await;
        }
    });

    // consume sound sensor receive
    let mic_tx2 = mic_tx.clone(); // clone for mic tx
    sensor_consume_task(rx, move || {
        // let error_tx = error_tx_clone.clone();   // clone here → fresh Sender each run
        let mic_tx3 = mic_tx2.clone();
        async move {
            //notify mic of shot success
            let _ = mic_tx3.send(MicNotification::Boom).await;

            // send radio packet and handle 
            if let Err(e) = send_radio_packet().await {
                // let _ = error_tx.send(SerialError::Radio).await;
                let _ = mic_tx3.send(MicNotification::RadioError).await;
            }
        }
    })
    .await;

    Ok(())
}