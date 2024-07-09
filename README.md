
# Rust BME280 Sensor Server

This project is a Rust-based server running on an ESP32 microcontroller, designed for reading temperature, humidity, and air pressure data from a BME280 sensor. Built using the `esp-idf-svc` and `esp-idf-hal` crates, it is based on the [esp-idf-template](https://github.com/esp-rs/esp-idf-template).

## Features

- **Temperature Monitoring**
- **Humidity Monitoring**
- **Air Pressure Monitoring**
- **ESP-IDF Integration:** Utilizes ESP-IDF services and hardware abstraction layer for seamless integration with ESP32.

## Roadmap

1. **Simple Measurement Loop:**
   - Implement a loop to periodically read and log humidity, temperature, and air pressure values from the BME280 sensor.
   
2. **MQTT Publishing:**
   - Publish the collected measurement values to an MQTT broker for remote monitoring and integration with other systems.

3. **Homebridge Plugin:**
   - Develop a Homebridge plugin to expose the sensor data to Apple's HomeKit ecosystem, allowing for smart home integrations.

## Prerequisites

- **Rust Toolchain:** Ensure you have the Rust toolchain installed. Installation instructions are available at [rustup.rs](https://rustup.rs/).
- **ESP-IDF:** Install the ESP-IDF development environment. Follow the [ESP-IDF Getting Started Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html).
- **BME280 Sensor:** Connect a BME280 sensor to your ESP32 development board.

## Configuration
The `config.rs` contains various constants used for configuring e.g. `OVERSAMPLE` and `MEASUREMENT_DELAY`.

## Usage
Once deployed, the server will begin reading data from the BME280 sensor. Future updates will include MQTT support and a Homebridge plugin for smart home integrations.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request to contribute to this project.
