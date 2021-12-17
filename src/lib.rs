//! A platform agnostic driver to interface with LPS22HB and LPS25HB pressure sensor modules.
 //!
 //! This driver allows you to:
 //! - read atmospheric pressure in hPa, see [`read_pressure()`]
 //! - enable single-shot data acquisition, see [`enable_one_shot()`]
 //! - set data rate, see [`set_datarate()`]
 //!
//! [`read_pressure()`]: struct.LPS2x.html#method.read_pressure
//! [`read_temperature()`]: struct.LPS2x.html#method.read_temperature
//! [`enable_one_shot()`]: struct.LPS2x.html#method.enable_one_shot
//! [`set_datarate()`]: struct.LPS2x.html#method.set_datarate
//!
//! __NOTE__: Only I2C interface is supported at the moment.
//!  //!
//! ### Datasheets: 
//! - [LPS22HB](https://www.st.com/resource/en/datasheet/lps22hb.pdf)
//! - [LPS25HB](https://www.st.com/resource/en/datasheet/lps25hb.pdf)
 //!
 //! ## Usage examples (see also examples folder)
 //!
 //! Please find additional examples using hardware in this repository: [examples]
 //!
//! [examples]: https://github.com/nebelgrau77/lps2x-rs/examples
//!
//! ### Read pressure and temperature
//! 
//! ### Initialize the sensor with a chosen interface
//! 
//! ### Read pressure and temperature - one shot 
//!
//! ### Continuous mode
//! - set the Output Data Rate
//! 
//! ### Data availability
//! - check data status
//! 
//! ### FIFO functionality
//! - configure and enable FIFO
//! 
//! ### Interrupts and data ready signal
//! - configure data ready signals
//! - configure interrupts
//! - set reference pressure
//! - autozero functions
//! 
//! ### Other functions 
//! - reboot
//! - software reset

// TO DO: move MULTIBYTE into the interface, as it is different between I2C and SPI 
// TO DO (IDEA): create an init() function with a Config struct. 
// The configuration could include: power on (bool), ODR, block data update (bool), pressure resolution, temperature resolution.
//
// TO DO: REMOVE PUB FROM READ_REGISTER() FUNCTION AFTER THE TESTS 
// 


#![no_std]

#[cfg(not(any(feature = "lps25hb", feature = "lps22hb")))]
compile_error!("One of the sensor models must be selected using --features");

#[cfg(all(feature = "lps25hb", feature = "lps22hb"))]
compile_error!("Only one of the sensor models can be selected");



pub mod sensor;
 
pub mod config;

pub mod register;
use register::{Registers, Bitmasks};
 
pub mod fifo;

pub mod interrupt;
 
pub mod interface;
use interface::Interface;
 
/// Sensor's ID
//const WHOAMI: u8 = 0b10110001; // decimal value 177 (LPS22HB)
// const WHOAMI: u8 = 0b10111101; // decimal value 189 (LPS25HB)
 
/// The output of the temperature sensor must be divided by 100, see p. 10 of the datasheet.
const TEMP_SCALE: f32 = 100.0;
/// The output of the pressure sensor must be divided by 4096, see p. 10 of the datasheet.
// https://www.st.com/resource/en/technical_note/dm00242307-how-to-interpret-pressure-and-temperature-readings-in-the-lps25hb-pressure-sensor-stmicroelectronics.pdf

/// The output of the temperature sensor must be divided by 480, see Table 3 of the datasheet.
const TEMP_SCALE: f32 = 480.0;
/// An offset value must be added to the result. This is NOT mentioned in the LPS25HB datasheet, but is described in the LPS25H datasheet.
#[cfg(lps25hb)]
const TEMP_OFFSET: f32 = 42.5;
/// The output of the pressure sensor must be divided by 4096, see Table 3 of the datasheet.
const PRESS_SCALE: f32 = 4096.0;
 
 /// Holds the driver instance with the selected interface
pub struct LPS2X<T> {

     interface: T,
}
 

impl<T, E> LPS2X<T>
where
     T: Interface<Error = E>,
 {
     /// Create a new instance of the LPS25HB driver.
     pub fn new(interface: T) -> Self {
          LPS2X { interface }
     }
 
     /// Destroy driver instance, return interface instance.
     pub fn destroy(self) -> T {
          self.interface
     }
 

     /// Read a byte from the given register.
     fn read_register(&mut self, address: Registers) -> Result<u8, T::Error> {
     // pub fn read_register(&mut self, address: Registers) -> Result<u8, T::Error> {
         let mut reg_data = [0u8];
         self.interface.read(address.addr(), &mut reg_data)?;
         Ok(reg_data[0])
     }
 
     /// Clear selected bits using a bitmask
     fn clear_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(address.addr(), &mut reg_data)?;
        let payload: u8 = reg_data[0] & !bitmask;
        self.interface.write(address.addr(), payload)?;
        Ok(())
     }
 
     /// Set selected bits using a bitmask
     fn set_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
         let mut reg_data = [0u8];
         self.interface.read(address.addr(), &mut reg_data)?;
         let payload: u8 = reg_data[0] | bitmask;
         self.interface.write(address.addr(), payload)?;
         Ok(())
     }

     /// Check if specific bits are set.
     fn is_register_bit_flag_high(
          &mut self,
          address: Registers,
          bitmask: u8,
     ) -> Result<bool, T::Error> {
          let data = self.read_register(address)?;
          Ok((data & bitmask) != 0)
     }
}
  
/// Output data rate and power mode selection (ODR). (Refer to datasheets)

// THERE ARE CONFLICTS HERE AS SAME FREQUENCIES MAY HAVE DIFFERENT BIT VALUES

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ODR {
    /// Power-down / One-shot mode enabled
    //PowerDown = 0b000,
    /// One-shot mode enabled
    OneShot = 0b000, // KEEPING THIS AS A NAME
    /// 1 Hz
    _1Hz = 0b001, // same for both sensors
    #[cfg(feature = "lps25")]
    /// 7 Hz
    _7Hz = 0b010,
    #[cfg(feature = "lps22")]
    /// 10 Hz
    _10Hz = 0b010,    
    #[cfg(feature = "lps25")]
    /// 12.5 Hz    
    _12_5Hz = 0b011,
    #[cfg(feature = "lps25")]
    /// 25 Hz
    _25Hz = 0b100,
    #[cfg(feature = "lps22")]
    /// 25 Hz
    _25Hz = 0b011,
    #[cfg(feature = "lps22")]
    /// 50 Hz    
    _50Hz = 0b100,
    #[cfg(feature = "lps22")]
    /// 75 Hz
    _75Hz = 0b101,

}
 
impl ODR {
     pub fn value(self) -> u8 {
        (self as u8) << 4 // shifted into the right position, can be used directly
     }
}

/// SPI interface mode
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum SPI_Mode {
    /// 4-wire mode (default)
    _4wire,
    /// 3-wire mode
    _3wire,
}

 

/// FIFO mode selection. (Refer to datasheets)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFO_MODE {
    /// Bypass mode
    Bypass = 0b000,
    /// FIFO mode
    FIFO = 0b001,
    /// Stream mode
    Stream = 0b010,
    /// Stream-to-FIFO mode
    Stream_to_FIFO = 0b011,
    /// Bypass-to-stream mode
    Bypass_to_stream = 0b100,
    #[cfg(feature = "lps22")]
    /// Dynamic-stream mode
    Dynamic_Stream = 0b110, // LPS22-specific
    #[cfg(feature = "lps25")]
    /// FIFO Mean mode
    FIFO_Mean = 0b110, // LPS25-specific
    /// Bypass-to-FIFO mode
    Bypass_to_FIFO = 0b111,
}
 
impl FIFO_MODE {
     pub fn value(self) -> u8 {

        (self as u8) << 5 // shifted into the right position, can be used directly
    }
}

/// FIFO Mean mode running average sample size. (Refer to Table 23)

#[cfg(feature = "lps25")]
#[allow(non_camel_case_types)]
+#[derive(Debug, Clone, Copy)]
pub enum FIFO_MEAN {
    /// 2-sample moving average
    _2sample = 0b00001,
    /// 4-sample moving average
    _4sample = 0b00011,
    /// 8-sample moving average
    _8sample = 0b00111,
    /// 16-sample moving average
    _16sample = 0b01111,
    /// 32-sample moving average
    _32sample = 0b11111,
}

#[cfg(feature = "lps25")]
impl FIFO_MEAN {
    pub fn value(self) -> u8 {
        self as u8 // no need to shift, bits 0:4
     }
}
 

/// INT_DRDY pin configuration. (Refer to datasheets)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_DRDY {
    /// Data signal (see CTRL_REG4)
    DataSignal = 0b00,
    /// Pressure high
    P_high = 0b01,
    /// Pressure low
    P_low = 0b10,
    /// Pressure low or high
    P_low_or_high = 0b011,
}
 
impl INT_DRDY {
     pub fn value(self) -> u8 {
        self as u8 // no need to shift, bits 0:1
     }
}

// Interrupt active setting for the INT_DRDY pin: active high (default) or active low
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_ACTIVE {
    /// Active high
    High,
    /// Active low
    Low,
}

impl INT_ACTIVE {
    pub fn status(self) -> bool {
        let status = match self {
            INT_ACTIVE::High => false,
            INT_ACTIVE::Low => true,
        };
        status
    }
}

/// Interrupt pad setting for INT_DRDY pin: push-pull (default) or open-drain.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_PIN {
    /// Push-pull
    PushPull,
    /// Open drain
    OpenDrain,
}

impl INT_PIN {
    pub fn status(self) -> bool {
        let status = match self {
            INT_PIN::PushPull => false,
            INT_PIN::OpenDrain => true,
        };
        status
    }
}

/// Settings for various FIFO- and interrupt-related flags, Enabled or Disabled
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FLAG {
    /// Enabled (bit set)
    Enabled,
    /// Disabled (bit cleared)
    Disabled,
}

impl FLAG {
    pub fn status(self) -> bool {
        let status = match self {
            FLAG::Disabled => false,
            FLAG::Enabled => true,
        };
        status
    }
}

// FIFO on/off
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFO_ON {
    /// Enabled (bit set)
    Enabled,
    /// Disabled (bit cleared)
    Disabled,
}

impl FIFO_ON {
    pub fn status(self) -> bool {
        let status = match self {
            FIFO_ON::Disabled => false,
            FIFO_ON::Enabled => true,
        };
        status
    }
}



/// Temperature resolution configuration, number of internal average(Refer to Table 18)
#[cfg(feature = "lps25")]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum TEMP_RES {
    /// Nr. internal average 8
    _8 = 0b00,
    /// Nr. internal average 16
    _16 = 0b01,
    /// Nr. internal average 32
    _32 = 0b10,
    /// Nr. internal average 64
    _64 = 0b11,
}

#[cfg(feature = "lps25")]
impl TEMP_RES {
    pub fn value(self) -> u8 {
        (self as u8) << 2 // shifted into the right position, can be used directly
    }
}

/// Pressure resolution configuration, number of internal average(Refer to Table 19)
#[cfg(feature = "lps25")]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum PRESS_RES {
    /// Nr. internal average 8
    _8 = 0b00,
    /// Nr. internal average 32
    _32 = 0b01,
    /// Nr. internal average 128
    _128 = 0b10,
    /// Nr. internal average 512
    _512 = 0b11,
}

#[cfg(feature = "lps25")]
impl PRESS_RES {
    pub fn value(self) -> u8 {
        self as u8 // no need to shift
    }
}

