use std::time::Duration;
use bme280_rs::{Oversampling, SensorMode};

/// Sensor Mode used for measuring. Normal is measuring at regular times,
/// Forced only once and then enters Sleep.
pub static SENSOR_MODE: SensorMode = SensorMode::Normal;

// Oversampling mode, whereas Oversample1 is collecting a single sample, etc.
pub static GLOBAL_OVERSAMPLING: Oversampling = Oversampling::Oversample1;

// Delay between measurements.
pub static MEASUREMENT_DELAY: Duration =
    Duration::new(MEASUREMENT_DELAY_SECONDS, MEASUREMENT_DELAY_MILLIS);

/// Enable Temperature measurement.
pub static TEMPERATURE_MEASURING_ENABLED: bool = true;

/// Enable Humidity measurement.
pub static HUMIDITY_MEASURING_ENABLED: bool = true;

/// Enable Pressure measurement.
pub static PRESSURE_MEASURING_ENABLED: bool = true;

static MEASUREMENT_DELAY_SECONDS: u64 = 5; // TODO: Delay Duration
static MEASUREMENT_DELAY_MILLIS: u32 = 0;
