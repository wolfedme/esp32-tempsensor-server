#![deny(unsafe_code)]

mod config;

use crate::config::{GLOBAL_OVERSAMPLING, HUMIDITY_MEASURING_ENABLED, MEASUREMENT_DELAY, PRESSURE_MEASURING_ENABLED, SENSOR_MODE, TEMPERATURE_MEASURING_ENABLED};

use std::time::Duration;
use anyhow::{Error, Result};
use bme280_rs::{Bme280, Oversampling};
use bme280_rs::Configuration as BME280_Configuration;
use esp_idf_hal::delay::{Delay};
use esp_idf_hal::i2c::I2cConfig;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::hal::i2c::I2cDriver;


fn main() -> Result<()> {
    log::info!("Starting BME280 Server");
    let mut bme280 = match init() {
        Ok(bme280) => bme280,
        Err(e) => {
            log::error!("Error while initialising: {}", e);
            return Err(Error::msg("Error while initialising"));
        }
    };

    loop {
        match print_measurement(&mut bme280) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Error while measuring: {}", e);
                return Err(Error::msg("Error while measuring"));
            }
        }
    };
    // TODO: Error Handling
    // Ok(())
}

fn init() ->  Result<Bme280<I2cDriver<'static>, Delay>>{
    log::info!("Initialising...");

    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let i2c_bus = init_i2c_bus()?;

    log::info!("Standard delay of {:?} between measurements", MEASUREMENT_DELAY);

    if Duration::as_millis(&MEASUREMENT_DELAY) < 10 {
        log::warn!("Delay is smaller than 10. This could starve FreeRTOS IDLE tasks.");
    }

    // TODO: Check docs if this takes millis?
    let delay = Delay::new(MEASUREMENT_DELAY.as_millis() as u32);
    let mut bme280 = Bme280::new(i2c_bus, delay);

    log::info!("
    Setting sampling configuration to:\n\n

    Sensor Mode: {:?}\n
    Global Oversampling: {:?}\n
    ", SENSOR_MODE, GLOBAL_OVERSAMPLING);

    let configuration = generate_sampling_configuration();
    bme280.set_sampling_configuration(configuration)?;

    log::info!("Initialising completed.");

    Ok(bme280)
}

fn print_measurement(bme280: &mut Bme280<I2cDriver, Delay>) -> Result<()> {
    // TODO: Error Handling & more sophisticated code
    // TODO: Enable support for disabled measurements
    let measurements = bme280.read_sample()?;
    log::info!("Temperature: {}°C", measurements.0.unwrap());
    log::info!("Pressure: {}hPa", measurements.1.unwrap());
    log::info!("Humidity: {}%", measurements.2.unwrap());
    Ok(())
}

fn generate_sampling_configuration() -> BME280_Configuration {
    let temperature_sampling = match TEMPERATURE_MEASURING_ENABLED {
        true => GLOBAL_OVERSAMPLING,
        false => Oversampling::Skip,
    };

    let humidity_sampling = match HUMIDITY_MEASURING_ENABLED {
        true => GLOBAL_OVERSAMPLING,
        false => Oversampling::Skip,
    };

    let pressure_sampling = match PRESSURE_MEASURING_ENABLED {
        true => GLOBAL_OVERSAMPLING,
        false => Oversampling::Skip,
    };

    BME280_Configuration::default()
        .with_temperature_oversampling(temperature_sampling)
        .with_pressure_oversampling(pressure_sampling)
        .with_humidity_oversampling(humidity_sampling)
        .with_sensor_mode(SENSOR_MODE)
}

fn init_i2c_bus() -> Result<I2cDriver<'static>> {
    log::info!("Taking I2C at i2c0");

    let peripherals = Peripherals::take()?;
    let i2c = peripherals.i2c0; // TODO: Check I2C Bus address
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let config = I2cConfig::new();
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    Ok(i2c)
}
