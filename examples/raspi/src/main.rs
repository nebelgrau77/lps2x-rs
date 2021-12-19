//! talking to the LPS25HB module over I2C on Raspberry Pi

use rppal::i2c::I2c;

use lps2x::*;
use lps2x::interface::{I2cInterface,
    i2c::I2cAddress};

fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // configure I2C interface for the LPS25HB driver
    let i2c_interface = I2cInterface::init(i2c, I2cAddress::SA0_VCC); // Pololu board

    // create a new driver instance with the I2C interface    
    let mut lps2x = LPS2X::new(i2c_interface);

    // turn the sensor on 
    lps2x.sensor_on(true).unwrap();
      
    // enable Block Data Update
    lps2x.bdu_enable(true).unwrap();
  
    // set data rate to 7Hz
    lps2x.set_datarate(ODR::_7Hz).unwrap();

    let temp = lps2x.read_temperature().unwrap();            
    let press = lps2x.read_pressure().unwrap();
    let id = lps2x.get_device_id().unwrap();

    println!("Device ID: {}\nPressure: {} hPa\nTemperature: {} °C", 
            id, press, temp);

}
