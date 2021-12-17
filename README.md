# Rust LPS25HB pressure sensor driver

![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

A platform agnostic Rust driver for the ST Microelectronics LPS22HB and LPS25HB pressure sensors,
based on the [`embedded-hal`] traits. Merged from two separate drivers, intended to be used with feature for selecting one of the devices.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

Inspired by and partially based on [another STMicroelectronics driver](https://github.com/lonesometraveler/lsm9ds1).

This driver allows you to:
- check if sensor is reachable
- read pressure and temperature
- set data rate
- configure interrupts generation
- configure FIFO


# UNDER ACTIVE DEVELOPMENT

Basic functions work, see example. Almost all functions added, and should work fine, but many aren't tested yet.

## WORK IN PROGRESS:

This library is work in progress. Not all features are implemented yet. Currently only the I2C interface is implemented. Contributions are welcome.

### TO DO:
- [ ] add device ID check and power up in the `new()` function
- [ ] add an example using FIFO and/or interrupt generation
- [ ] add SPI interface and an example using it
- [ ] add more documentation (main block in the lib.rs)

## The device

The LPS25HB is an ultra-compact piezoresistive absolute pressure sensor which functions as a digital output barometer. The device comprises a sensing element and an IC interface which communicates through I2C or SPI from the sensing element to the application.

Datasheets: 
- [LPS25HB](https://www.st.com/resource/en/datasheet/lps25hb.pdf)
- [LPS22HB](https://www.st.com/resource/en/datasheet/dm00140895.pdf)

For more information regarding the use and configuration of the device, especially the interrupts, data ready signals and FIFO functionalities, refer to:

* [LPS22HB/LPS25HB digital pressure sensors: hardware guidelines for system integration](https://www.st.com/resource/en/application_note/an4672-lps22hblps25hb-digital-pressure-sensors-hardware-guidelines-for-system-integration-stmicroelectronics.pdf)
* [Digital pressure sensors: efficient design tips](https://www.st.com/resource/en/design_tip/dt0132--digital-pressure-sensor-efficient-design-tips-stmicroelectronics.pdf)


## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device, selecting the correct sensor with features.

Please find additional examples using hardware in this repository: [examples]

[examples]: https://github.com/nebelgrau77/lps25hb-rs/tree/main/examples

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/nebelgrau77/lps25hb-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
