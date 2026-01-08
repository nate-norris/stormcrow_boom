//! Boom Detection Application Entry Point
//!
//! This binary initializes the sound sensor, microphone notification handler,
//! and the MM2T radio interface. Tasks are spawned for:
//! - reading sound sensor edges,
//! - processing sensor events,
//! - sending radio trigger packets,
//! - notifying a microphone device of system events.
//!
//! The application runs until the user presses Ctrl+C.

use tokio::sync::mpsc;
use std::sync::Arc;

mod lib_sensor;
mod lib_sensor_consumer;
mod lib_mic;
mod mm2t;
mod logger;
use lib_sensor::{EventTx, SoundSensor, SoundSensorMock, SoundSensorT};
use lib_sensor_consumer::{EventRx, sensor_consume_task};
use lib_mic::{MicTx, MicRx, MicNotification, mic_consume_task};
use mm2t::MM2TBoomHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate logging
    logger::init_logger();
    logger::info("Boom started");
    // sound sensor channels for EdgeDetection events
    let (tx, rx): (EventTx, EventRx) 
        = mpsc::channel(32);
    
    // mic for MicNotifications to listen for MicTx
    // NOTE initialize before any tx can transmit
    let mic_tx: mpsc::Sender<MicNotification> = init_mic();

    // serial radio packets
    //  NOTE: failed init here is a failed program and will 
    //  notify through MicNotification
    let radio = match init_radio(&mic_tx).await {
        Ok(r) => Some(r),
        Err(_e) => {
            let mic_tx_init_radio = mic_tx.clone();
            tokio::spawn(async move {
                let _ = mic_tx_init_radio.send(MicNotification::RadioError).await;
            });
            logger::error("Failed to init mm2t");
            None
        }
    };

    // consume rx of sound sensor edge detection
    //      sends radio packet
    //      handles MicNotifications for errors and triggers
    if let Some(r) = radio {
        // initiate EventTx for sound sensor
        spawn_edge_detector(tx.clone(), mic_tx.clone());

        // initiate listening for EventRx for sound sensor
        //      sends radio packet
        //      handles MicNotifications
        spawn_sensor_consumer(rx, r, mic_tx.clone());
    }

    // await Ctrl+C from user to end program
    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}

// Initialize the mic channels and notification task.
// Begins consuming Rx channel and returns Tx channel
fn init_mic() -> MicTx {

    // mic channels for MicNotification events
    let (mic_tx, mic_rx): 
        (MicTx, MicRx) = mpsc::channel(32);

    // await for MicNotification
    // NOTE: mic is initialized before tasks so that it is ready to receive
    tokio::spawn(mic_consume_task(mic_rx));

    mic_tx
}

// Initializes MM2T radio
// On failure begins a MicNotification::RadioError
async fn init_radio(mic_tx: &MicTx) ->anyhow::Result< Arc<MM2TBoomHandle>> {
    // initialize mm2t radio
    //      sends mic notification error if failed to start
    match MM2TBoomHandle::start().await {
        Ok(r) => Ok(Arc::new(r)), // assign to radio
        Err(e) => {
            logger::error("mm2t init");
            let _ = mic_tx.send(MicNotification::RadioError).await;
            Err(e.into())
        }
    }
}

// Spawn background task reading sound sensor edges
fn spawn_edge_detector(tx: EventTx, mic_tx: MicTx) {
    // sensor for sound trigger
    let sensor = SoundSensorMock;
    let _sensor = SoundSensor;

    // spawn sound sensor thread for triggering edge detects
    tokio::spawn(async move {
        // await sound sensor edge detect
        //      handle sound sensor error if occurs
        if let Err(_e) = sensor.detect_edge_task(tx.clone()).await {
            logger::error("sound sensor failed init");
            let _ = mic_tx.send(MicNotification::SoundSensorError).await;
        }
    });
}

// spawn background task that consumes EventRx events
// Sends mic notifications and radio packets
fn spawn_sensor_consumer(rx: EventRx, radio: Arc<MM2TBoomHandle>, mic_tx: MicTx) {
    // Spawn background task for consuming sensor events
    tokio::spawn(async move {
        sensor_consume_task(rx, move || {
            let mic_tx = mic_tx.clone();       // clone per callback invocation
            let radio: Arc<MM2TBoomHandle> = Arc::clone(&radio);    // clone Arc per callback invocation
            async move {
                // Notify mic of shot success
                let _ = mic_tx.send(MicNotification::Boom).await;

                // Send radio packet and handle errors
                if let Err(_e) = radio.send_trigger_packet().await {
                    logger::error("failed on trigger packet send");
                    let _ = mic_tx.send(MicNotification::RadioError).await;
                }
            }
        })
        .await; // Await the sensor_consume_task future inside the spawned task
    });
}