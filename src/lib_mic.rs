//! # lib_mic
//!
//! `lib_mic` provides an abstraction layer for interacting with microphone
//! hardware or mocks in a Tokio async context. It includes:
//!
//! - **Event types and channels** (`MicNotification`, `MicTx`, `MicRx`)
//! - **Microphone consumer task** (`mic_consume_task`) for handling events
//! - **Trait for microphone implementations** (`MicrophoneT`)
//! - **Concrete implementations** (`Microphone` and `MicrophoneMock`)
//!
//! ## Overview
//!
//! This crate is designed to work with async event streams (`tokio::sync::mpsc`)
//! and provides both real and mock microphone implementations for testing
//! and development. The consumer task listens for events and triggers
//! one-shot "boom" patterns or repeating error patterns as appropriate.
//!
//! ## Example
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

mod trait_mic;

pub mod models;
pub mod consume;
pub mod mic_mock;
pub mod mic;

// re-export commmon types and functions
pub use models::{MicTx, MicRx, MicNotification};
pub use consume::mic_consume_task;