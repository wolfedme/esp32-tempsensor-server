use bme280::Oversampling;
use bme280::SensorMode;
use std::time::Duration;

// TODO: Remove unused config or introduce ways to disable

/// Sensor Mode used for measuring. Normal is measuring at regular times,
/// Forced only once and then enters Sleep.
pub static SENSOR_MODE: SensorMode = SensorMode::Normal;

// Oversampling mode
pub static GLOBAL_OVERSAMPLING: Oversampling = Oversampling::Oversampling2X;

/// Enable Temperature measurement.
pub static TEMPERATURE_MEASURING_ENABLED: bool = true;

/// Enable Humidity measurement.
pub static HUMIDITY_MEASURING_ENABLED: bool = true;

/// Enable Pressure measurement.
pub static PRESSURE_MEASURING_ENABLED: bool = true;

static MEASUREMENT_DELAY_SECONDS: u64 = 1;
static MEASUREMENT_DELAY_MILLIS: u32 = 0;

// Delay between measurements.
pub static MEASUREMENT_DELAY: Duration =
    Duration::new(MEASUREMENT_DELAY_SECONDS, MEASUREMENT_DELAY_MILLIS);
