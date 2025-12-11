//! # lib_sensor
//!
//! `lib_sensor` provides an abstraction layer for interacting with sound sensor
//! hardware or mocks in a Tokio async context. It includes:
//!
//! This library exposes:
//! - [`SoundSensorT`]: the trait defining sound sensor behavior.
//! - [`SoundSensor`]: a real hardware implementation using a serial port.
//! - [`SoundSensorMock`]: a mock implementation for testing and simulation.
//! - [`EventTx`] and [`EdgeDetection`]: channels and event types for trigger
//!
//! ## Overview
//!
//! This crate is designed to work with async event streams
//! and provides both real and mock sound sensor implementations for testing
//! and development. The SoundSensor sends tx channel event triggers when 
//! bytes are received from hardware.
//!
//! ## Usage
//!
//! ```no_run
//! use tokio::sync::mpsc;
//! use lib_mic::{mic_consume_task, MicNotification};
//!
//! #[tokio::main]
//! async fn main() {
//!     let (tx, rx) = mpsc::channel(32);
//!
//!     tokio::spawn(async move {
//!         mic_consume_task(rx).await;
//!     });
//!
//!     tx.send(MicNotification::Boom).await.unwrap();
//!     tx.send(MicNotification::SoundSensorError).await.unwrap();
//! }
//! ```

pub mod models;
pub mod sound_sensor;
pub mod sound_sensor_mock;
pub mod trait_sensor;

pub use trait_sensor::SoundSensorT;
pub use models::{EventTx, EdgeDetection};
pub use sound_sensor_mock::SoundSensorMock;
pub use sound_sensor::SoundSensor;