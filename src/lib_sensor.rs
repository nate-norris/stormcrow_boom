pub mod models;
pub mod sound_sensor;
pub mod sound_sensor_mock;
pub mod trait_sensor;

pub use trait_sensor::SoundSensorT;
pub use models::{EventTx, EdgeDetection};
pub use sound_sensor::SoundSensor;
pub use sound_sensor_mock::SoundSensorMock;