#![deny(unsafe_code)]

mod config;

use std::any::Any;
use crate::config::{
    GLOBAL_OVERSAMPLING, HUMIDITY_MEASURING_ENABLED, MEASUREMENT_DELAY, PRESSURE_MEASURING_ENABLED,
    SENSOR_MODE, TEMPERATURE_MEASURING_ENABLED,
};

use anyhow::{Error, Result};
use bme280_rs::Configuration as BME280_Configuration;
use bme280_rs::{Bme280, Oversampling};
use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::i2c::I2cConfig;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::hal::i2c::I2cDriver;
use std::time::Duration;
use esp_idf_hal::sys::EspError;
use esp_idf_svc::tls::Config;

fn main() -> Result<()> {
    // Links to the final executable
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Debug);

    log::info!("Log Level: {}", log::max_level());
    log::debug!("Linked patches and initialized default EspLogger");

    // Main loop
    log::info!("Starting BME280 Server");
    let mut bme280 = match init() {
        Ok(bme280) => bme280,
        Err(e) => {
            log::error!("Error while initialising: {}", e);
            panic!("{}", e);
        }
    };

    log::info!("BME280\nChip ID: {}\nStatus: {}, Humidity test: {:?}",
        bme280.chip_id()?, bme280.status()?, bme280.read_humidity()?);

    loop {
        log::debug!("Measuring Loop start.");

        match print_measurement(&mut bme280) {
            Ok(_) => {
                log::debug!("print_measurement OK");
            },
            Err(e) => {
                log::error!("Error while measuring: {}", e);
                return Err(Error::msg("Error while measuring"));
            }
        }

        log::debug!("Measuring Loop end. Waiting for {}", MEASUREMENT_DELAY.as_millis());
        // Delay next measurement
        FreeRtos::delay_ms(MEASUREMENT_DELAY.as_millis() as u32);
    }

    // TODO: Error Handling
    Ok(())
}

fn init() -> Result<Bme280<I2cDriver<'static>, Delay>> {
    log::info!("Initialising...");

    let i2c_bus = match init_i2c_bus() {
        Ok(i2c_bus) => i2c_bus,
        Err(e) => {
            return Err(e);
        }
    };

    log::info!(
        "Standard delay of {:?} between measurements",
        MEASUREMENT_DELAY
    );

    if Duration::as_millis(&MEASUREMENT_DELAY) < 10 {
        log::warn!("Delay is smaller than 10ms. This could starve FreeRTOS IDLE tasks.");
    }

    let delay = Delay::new(MEASUREMENT_DELAY.as_millis() as u32);
    let mut bme280 = Bme280::new(i2c_bus, delay);

    log::info!(
        "
    Setting sampling configuration to:\n\n

    Sensor Mode: {:?}\n
    Global Oversampling: {:?}\n
    ",
        SENSOR_MODE,
        GLOBAL_OVERSAMPLING
    );

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

    let peripherals = match Peripherals::take() {
        Ok(peripherals) => peripherals,
        Err(e) => {
            return Err(Error::new(e));
        }
    };

    log::info!("Peripherals taken {:?}", peripherals.type_id());

    let i2c = peripherals.i2c0; // TODO: Check I2C Bus address
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    log::debug!("i2c0, gpio5, gpio6");

    let i2c = match I2cDriver::new(i2c, sda, scl, &I2cConfig::new()) {
        Ok(i2c) => i2c,
        Err(e) => {
            log::error!("Issue while creating I2cDriver: {}: {}", e.code(), e.to_string());
            return Err(Error::new(e));
        }
    };

    log::info!("i2cdriver created with given config.");

    log::info!("I2C at i2c0 taken. TypeId {:?}", i2c.type_id());
    Ok(i2c)
}
