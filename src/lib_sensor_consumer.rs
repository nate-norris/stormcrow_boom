//! # lib_sensor_consumer
//!
//! `lib_sensor_consumer` handles events triggered through mpsc channel EventTx
//! that are passed to EventRx. Upon receiving an EventRx the provided function
//! is executed. EdgeDetection events are pulled from `lib_sensor`.
//!
//! - **Event channel** `EventRx``
//! - **Sound sensor consumer task** (`sensor_consume_task`) for handling events
//! 
//! ## Usage
//!
//! ```no_run
//! use lib_sensor::{EdgeDetection, EventTx, SoundSensorMock, SoundSensorT};
//! use lib_sensor_consumer::{sensor_consume_task, EventRx};
//! use tokio::sync::mpsc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let (tx, rx) = mpsc::channel(32);
//!     let sensor = SoundSensorMock;
//!
//!     // Spawn the sensor simulation
//!     tokio::spawn(sensor.detect_edge_task(tx));
//!
//!     // Spawn the consumer task
//!     tokio::spawn(sensor_consume_task(rx, || async {
//!         println!("Trigger action executed!");
//!     }));
//! }
//! ```

pub mod models;
pub mod consume;

pub use models::EventRx;
pub use consume::sensor_consume_task;