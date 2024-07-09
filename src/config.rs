use bme280::Oversampling;
use bme280::SensorMode;
use std::time::Duration;

// TODO: Introduce SENSOR_MODE, TEMPERATURE-, HUMIDITY- and PRESSURE_MEASURING feature toggles.

/// Sensor Mode used for measuring. Normal is measuring at regular times,
/// Forced only once and then enters Sleep.
pub static _SENSOR_MODE: SensorMode = SensorMode::Normal;

// Oversampling mode
pub static GLOBAL_OVERSAMPLING: Oversampling = Oversampling::Oversampling2X;

/// Enable Temperature measurement.
pub static _TEMPERATURE_MEASURING_ENABLED: bool = true;

/// Enable Humidity measurement.
pub static _HUMIDITY_MEASURING_ENABLED: bool = true;

/// Enable Pressure measurement.
pub static _PRESSURE_MEASURING_ENABLED: bool = true;

static MEASUREMENT_DELAY_SECONDS: u64 = 1;
static MEASUREMENT_DELAY_MILLIS: u32 = 0;

// Delay between measurements.
pub static MEASUREMENT_DELAY: Duration =
    Duration::new(MEASUREMENT_DELAY_SECONDS, MEASUREMENT_DELAY_MILLIS);
