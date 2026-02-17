//! Boom Detection Application Entry Point
//!
//! This binary initializes the sound sensor, speaker notification handler,
//! and the MM2T interface. 
//! 
//! Tasks are spawned for:
//! - reading sound sensor edges,
//! - processing sensor events,
//! - sending mm2t trigger packets,
//! - notifying a speaker device of system events.
//!
//! The application is meant to ran in headless mode on the host power-up but 
//! can be ran manually and resolved when the user presses Ctrl+C.

use tokio::sync::mpsc;
use std::sync::Arc;

mod lib_sensor;
mod lib_sensor_consumer;
mod packet_boom;
use lib_sensor::{EventTx, SoundSensor, SoundSensorMock, SoundSensorT};
use lib_sensor_consumer::{EventRx, sensor_consume_task};
use packet_boom::BoomPacket;
// utility functionality
use utils::mm2t::{MM2TTransport, PacketT};
use utils::logger;
use utils::speaker::{SpeakerTx, SpeakerRx, SpeakerNotification, speaker_consume_task};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate logging
    logger::init_logger(None);
    logger::info("Boom started");

    // sound sensor channels for EdgeDetection events
    let (tx, rx): (EventTx, EventRx) 
        = mpsc::channel(32);
    
    // speaker for SpeakerNotifications to listen for SpeakerTx
    // NOTE initialize before any tx can transmit
    let speaker_tx: SpeakerTx = init_speaker();

    // serial radio packets
    //  NOTE: failed init here is a failed program and will 
    //  notify through SpeakerNotification
    let mm2t: Option<Arc<MM2TTransport>> = init_mm2t(&speaker_tx).await;

    // consume rx of sound sensor edge detection
    //      sends radio packet
    //      handles SpeakerNotifications for errors and triggers
    if let Some(m) = mm2t {
        // initiate EventTx for sound sensor
        spawn_edge_detector(tx.clone(), speaker_tx.clone());

        // initiate listening for EventRx for sound sensor
        //      sends radio packet
        //      handles SpeakerNotifications
        spawn_sensor_consumer(rx, m, speaker_tx.clone());
    }

    // await Ctrl+C from user to end program
    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}

// Initialize the speaker channels and notification task.
// Begins consuming Rx channel and returns Tx channel
fn init_speaker() -> SpeakerTx {

    // speaker channels for SpeakerNotification events
    let (speaker_tx, speaker_rx): 
        (SpeakerTx, SpeakerRx) = mpsc::channel(32);

    // await for SpeakerNotification
    // NOTE: speaker is initialized before tasks so that it is ready to receive
    tokio::spawn(speaker_consume_task(speaker_rx));

    speaker_tx
}

// Initializes MM2T radio
// On failure begins a SpeakerNotification::RadioError
async fn init_mm2t(speaker_tx: &SpeakerTx) -> Option<Arc<MM2TTransport>> {
    match MM2TTransport::start("/dev/ttyUSB0").await {
        Ok(r) => Some(Arc::new(r)),
        Err(e) => {
            logger::error_with("Failed mm2t init", e);
            let _ = speaker_tx.send(SpeakerNotification::RadioError).await;
            None
        }
    }
}

// Spawn background task reading sound sensor edges
fn spawn_edge_detector(tx: EventTx, speaker_tx: SpeakerTx) {
    // sensor for sound trigger
    let sensor = SoundSensorMock;
    let _sensor = SoundSensor;

    // spawn sound sensor thread for triggering edge detects
    tokio::spawn(async move {
        // await sound sensor edge detect
        //      handle sound sensor error if occurs
        if let Err(e) = sensor.detect_edge_task(tx).await {
            logger::error_with(
                "Failed sound sensor edge detect init", e);
            let _ = speaker_tx.send(SpeakerNotification::SoundSensorError).await;
        }
    });
}

// spawn background task that consumes EventRx events
// Sends speaker notifications and radio packets
fn spawn_sensor_consumer(rx: EventRx, radio: Arc<MM2TTransport>, speaker_tx: SpeakerTx) {
    // Spawn background task for consuming sensor events
    tokio::spawn(async move {
        sensor_consume_task(rx, move || {
            let speaker_tx = speaker_tx.clone();       // clone per callback invocation
            let radio: Arc<MM2TTransport> = Arc::clone(&radio);    // clone Arc per callback invocation
            async move {
                // Notify speaker of shot success
                let _ = speaker_tx.send(SpeakerNotification::Boom).await;
                // TODO only make notification if mm2t has sent the packet
                

                // Send radio packet and handle errors
                let packet = BoomPacket;
                if let Err(e) = radio.send(&packet.to_bytes()).await {
                    logger::error_with("Failed to send trigger pacaket", e);
                    let _ = speaker_tx.send(SpeakerNotification::RadioError).await;
                }
            }
        })
        .await; // Await the sensor_consume_task future inside the spawned task
    });
}