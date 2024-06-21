#![deny(unsafe_code)]

mod config;

use crate::config::{GLOBAL_OVERSAMPLING, MEASUREMENT_DELAY};
use std::any::Any;

use anyhow::{Error, Result};
use bme280::i2c::BME280;
use bme280::Configuration as Bme280_Config;
use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::hal::i2c::I2cConfig;
use esp_idf_svc::hal::i2c::I2cDriver;
use std::time::Duration;

// TODO: Spread Init & other functionality into logical modules
// TODO: Readable logging framework
// TODO: Introduce async
fn main() -> Result<()> {
    // Links to the final executable
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Debug);

    log::info!("Starting BME280 Server");
    log::info!("Log Level: {}", log::max_level());
    log::debug!("Linked patches and initialized default EspLogger");

    // Main loop
    log::info!("Init BME280 I2C Driver");
    let delay = create_delay();
    let mut bme280 = match init(delay) {
        Ok(bme280) => bme280,
        Err(e) => {
            log::error!("Error while initialising: {}", e);
            panic!("{}", e);
        }
    };

    loop {
        log::debug!("Measuring Loop start.");

        match print_measurement(&mut bme280, delay) {
            Ok(_) => {
                log::debug!("print_measurement OK");
            }
            Err(e) => {
                log::error!("Error while measuring: {}", e);
                return Err(Error::msg("Error while measuring"));
            }
        }

        log::debug!(
            "Measuring Loop end. Waiting for {}",
            MEASUREMENT_DELAY.as_millis()
        );
        // Delay next measurement
        FreeRtos::delay_ms(MEASUREMENT_DELAY.as_millis() as u32);
    }

    // TODO: Error Handling
}

fn init(mut delay: Delay) -> Result<BME280<I2cDriver<'static>>> {
    log::info!("Initialising...");

    let i2c_bus = match init_i2c_bus() {
        Ok(i2c_bus) => i2c_bus,
        Err(e) => {
            return Err(e);
        }
    };

    let mut bme280 = BME280::new_primary(i2c_bus);
    let config = create_bme280_config();
    bme280.init_with_config(&mut delay, config).unwrap(); //TODO: Error handling

    log::info!("Initialising completed.");

    Ok(bme280)
}

fn print_measurement(bme280: &mut BME280<I2cDriver>, mut delay: Delay) -> Result<()> {
    // TODO: Error Handling & more sophisticated code
    // TODO: Enable support for disabled measurements
    let measurements = bme280.measure(&mut delay).unwrap();
    log::info!("Temperature: {}°C", measurements.temperature);
    log::info!("Pressure: {}hPa", measurements.pressure);
    log::info!("Humidity: {}%", measurements.humidity);
    Ok(())
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
            log::error!(
                "Issue while creating I2cDriver: {}: {}",
                e.code(),
                e.to_string()
            );
            return Err(Error::new(e));
        }
    };

    log::info!("i2cdriver created with given config.");

    log::info!("I2C at i2c0 taken. TypeId {:?}", i2c.type_id());
    Ok(i2c)
}

// TODO: Error Handling
fn create_bme280_config() -> Bme280_Config {
    let bme_config = Bme280_Config::default();
    // TODO: Support individual oversampling in config
    bme_config.with_humidity_oversampling(GLOBAL_OVERSAMPLING);
    bme_config.with_pressure_oversampling(GLOBAL_OVERSAMPLING);
    bme_config.with_temperature_oversampling(GLOBAL_OVERSAMPLING);
    // config.with_iir_filter(filter) // TODO

    // TODO: Complete log
    log::info!(
        "
    Setting sampling configuration to:\n\n

    Global Oversampling: {:?}\n
    ",
        GLOBAL_OVERSAMPLING
    );

    bme_config
}

fn create_delay() -> Delay {
    log::info!(
        "Standard delay of {:?} between measurements",
        MEASUREMENT_DELAY
    );

    if Duration::as_millis(&MEASUREMENT_DELAY) < 10 {
        log::warn!("Delay is smaller than 10ms. This could starve FreeRTOS IDLE tasks.");
    }

    return Delay::new(MEASUREMENT_DELAY.as_millis() as u32);
}
