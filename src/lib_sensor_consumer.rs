// pull EdgeDetection from external library
//TODO see if needed, showing unused but was required previously
// pub use crate::lib_sensor::EdgeDetection;

pub mod models;
pub mod consume;

pub use models::EventRx;
pub use consume::sensor_consume_task;