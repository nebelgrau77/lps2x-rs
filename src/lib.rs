diff --git a/lib_lps22.rs b/lib_lps25.rs
index 09dd5af..8f19f16 100755
--- a/lib_lps22.rs
+++ b/lib_lps25.rs
@@ -1,4 +1,4 @@
-//! A platform agnostic driver to interface with LPS22HB pressure sensor module.
+//! A platform agnostic driver to interface with LPS25HB pressure sensor module.
 //!
 //! This driver allows you to:
 //! - read atmospheric pressure in hPa, see [`read_pressure()`]
@@ -6,36 +6,66 @@
 //! - enable single-shot data acquisition, see [`enable_one_shot()`]
 //! - set data rate, see [`set_datarate()`]
 //!
-//! [`read_pressure()`]: struct.LPS22HB.html#method.read_pressure
-//! [`read_temperature()`]: struct.LPS22HB.html#method.read_temperature
-//! [`enable_one_shot()`]: struct.LPS22HB.html#method.enable_one_shot
-//! [`set_datarate()`]: struct.LPS22HB.html#method.set_datarate
+//! [`read_pressure()`]: struct.LPS25HB.html#method.read_pressure
+//! [`read_temperature()`]: struct.LPS25HB.html#method.read_temperature
+//! [`enable_one_shot()`]: struct.LPS25HB.html#method.enable_one_shot
+//! [`set_datarate()`]: struct.LPS25HB.html#method.set_datarate
 //!
-//! __NOTE__: Only I2C interface is supported at the moment.
-//!  //!
-//! ### Datasheet: [LPS22HB](https://www.st.com/resource/en/datasheet/lps22hb.pdf)
+//! __NOTE__: This is an early version of the crate. Only I2C interface is supported at the moment.
+//!  
+//!
+//! ### Datasheet: [LPS25HB](https://www.st.com/resource/en/datasheet/lps25hb.pdf)
 //!
 //! ## Usage examples (see also examples folder)
 //!
 //! Please find additional examples using hardware in this repository: [examples]
 //!
-//! [examples]: https://github.com/nebelgrau77/lps22hb-rs/examples
+//! [examples]: https://github.com/nebelgrau77/lps25hb-rs/examples
 //!
-//! ### Read pressure and temperature
+//! ### Initialize the sensor with a chosen interface
+//! 
+//! ### Read pressure and temperature - one shot
 //!
 //! ```rust
 //!
-//! use lps22hb::interface::{I2cInterface, i2c::I2cAddress};
-//! use lps22hb::*;
+//! use lps25hb::interface::{I2cInterface, i2c::I2cAddress};
+//! use lps25hb::*;
+//!
+//! let mut lps25 = LPS25HB.new(i2c_interface);
 //!
-//! let mut lps22 = LPS22HB.new(i2c_interface);
-//!//!
-//! lps22.one_shot().unwrap();
+//! lps25hb.sensor_on(true).unwrap();
 //!
-//! let pressure = lps22.read_pressure().unwrap();
-//! let temperature = lps22.read_temperature().unwrap();
+//! lps25.one_shot().unwrap();
+//!
+//! let pressure = lps25.read_pressure().unwrap();
+//! let temperature = lps25.read_temperature().unwrap();
 //! ```
 //!
+//! ### Continuous mode
+//! - set the Output Data Rate
+//! 
+//! ### Data availability
+//! - check data status
+//! 
+//! ### FIFO functionality
+//! - configure and enable FIFO
+//! 
+//! ### Interrupts and data ready signal
+//! - configure data ready signals
+//! - configure interrupts
+//! - set reference pressure
+//! - autozero functions
+//! 
+//! ### Other functions 
+//! - reboot
+//! - software reset
+
+// TO DO: move MULTIBYTE into the interface, as it is different between I2C and SPI 
+// TO DO (IDEA): create an init() function with a Config struct. 
+// The configuration could include: power on (bool), ODR, block data update (bool), pressure resolution, temperature resolution.
+//
+// TO DO: REMOVE PUB FROM READ_REGISTER() FUNCTION AFTER THE TESTS 
+// 
 
 #![no_std]
 //#![deny(warnings, missing_docs)]
@@ -43,42 +73,45 @@
 pub mod sensor;
 //use sensor::*;
 
-pub mod config;
-//use config::*;
+pub mod register;
+use register::{Registers, Bitmasks};
 
 pub mod fifo;
 //use fifo::*;
 
+pub mod config;
+//use config::*;
+
 pub mod interrupt;
 //use interrupt::*;
 
-pub mod register;
-use register::{Bitmasks, Registers};
-//use register::*;
-
 pub mod interface;
 use interface::Interface;
 
 /// Sensor's ID
-//const WHOAMI: u8 = 0b10110001; // decimal value 177
+// const WHOAMI: u8 = 0b10111101; // decimal value 189
 
-/// The output of the temperature sensor must be divided by 100, see p. 10 of the datasheet.
-const TEMP_SCALE: f32 = 100.0;
-/// The output of the pressure sensor must be divided by 4096, see p. 10 of the datasheet.
+// https://www.st.com/resource/en/technical_note/dm00242307-how-to-interpret-pressure-and-temperature-readings-in-the-lps25hb-pressure-sensor-stmicroelectronics.pdf
+
+/// The output of the temperature sensor must be divided by 480, see Table 3 of the datasheet.
+const TEMP_SCALE: f32 = 480.0;
+/// An offset value must be added to the result. This is NOT mentioned in the LPS25HB datasheet, but is described in the LPS25H datasheet.
+const TEMP_OFFSET: f32 = 42.5;
+/// The output of the pressure sensor must be divided by 4096, see Table 3 of the datasheet.
 const PRESS_SCALE: f32 = 4096.0;
 
 /// Holds the driver instance with the selected interface
-pub struct LPS22HB<T> {
+pub struct LPS25HB<T> {
     interface: T,
 }
 
-impl<T, E> LPS22HB<T>
+impl<T, E> LPS25HB<T>
 where
     T: Interface<Error = E>,
 {
     /// Create a new instance of the LPS25HB driver.
     pub fn new(interface: T) -> Self {
-        LPS22HB { interface }
+        LPS25HB { interface }
     }
 
     /// Destroy driver instance, return interface instance.
@@ -86,31 +119,12 @@ where
         self.interface
     }
 
-    /*
-    /// Verifies communication with WHO_AM_I register
-    pub fn sensor_is_reachable(&mut self) -> Result<bool, T::Error> {
-        let mut bytes = [0u8; 1];
-        let (who_am_i, register) = (WHOAMI, Registers::WHO_AM_I.addr());
-        self.interface.read(register, &mut bytes)?;
-        Ok(bytes[0] == who_am_i)
-    }
+    /// Read a byte from the given register.
+    /// 
 
-    /// Initializes the sensor with selected settings
-    pub fn begin_sensor(&mut self) -> Result <(), T::Error> {
-        self.interface.write(
-            Registers::CTRL_REG1.addr(),
-            self.sensor.ctrl_reg1(),
-        )?;
-        self.interface.write(
-            Registers::CTRL_REG2.addr(),
-            self.sensor.ctrl_reg2(),
-        )?;
-        Ok(())
-    }
-    */
+    // public for testing
 
-    /// Read a byte from the given register.
-    fn read_register(&mut self, address: Registers) -> Result<u8, T::Error> {
+    pub fn read_register(&mut self, address: Registers) -> Result<u8, T::Error> {
         let mut reg_data = [0u8];
         self.interface.read(address.addr(), &mut reg_data)?;
         Ok(reg_data[0])
@@ -118,8 +132,8 @@ where
 
     /// Clear selected bits using a bitmask
     fn clear_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
-        let mut reg_data = [0u8; 1];
-        self.interface.read(address.addr(), &mut reg_data)?;        
+        let mut reg_data = [0u8];
+        self.interface.read(address.addr(), &mut reg_data)?;
         let payload: u8 = reg_data[0] & !bitmask;
         self.interface.write(address.addr(), payload)?;
         Ok(())
@@ -127,7 +141,7 @@ where
 
     /// Set selected bits using a bitmask
     fn set_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
-        let mut reg_data = [0u8; 1];
+        let mut reg_data = [0u8];
         self.interface.read(address.addr(), &mut reg_data)?;
         let payload: u8 = reg_data[0] | bitmask;
         self.interface.write(address.addr(), payload)?;
@@ -143,29 +157,36 @@ where
         let data = self.read_register(address)?;
         Ok((data & bitmask) != 0)
     }
+
+    /*
+
+    /// FOR DEBUGGING PURPOSES ONLY
+    pub fn get_mask(&mut self, mask: ODR) -> Result<u8, T::Error> {
+        Ok(mask.value())
+    }
+
+    */
 }
 
-/// Output data rate and power mode selection (ODR). (Refer to Table 17)
+/// Output data rate and power mode selection (ODR). (Refer to Table 20)
 #[allow(non_camel_case_types)]
 #[derive(Debug, Clone, Copy)]
 pub enum ODR {
-    /// Power-down / One-shot mode enabled
-    PowerDown = 0b000,
+    /// One-shot mode enabled
+    OneShot = 0b000,
     /// 1 Hz
     _1Hz = 0b001,
-    /// 10 Hz
-    _10Hz = 0b010,
+    /// 7 Hz
+    _7Hz = 0b010,
+    /// 12.5 Hz
+    _12_5Hz = 0b011,
     /// 25 Hz
-    _25Hz = 0b011,
-    /// 50 Hz
-    _50Hz = 0b100,
-    /// 75 Hz
-    _75Hz = 0b101,
+    _25Hz = 0b100,
 }
 
 impl ODR {
     pub fn value(self) -> u8 {
-        (self as u8) << 4
+        (self as u8) << 4 // shifted into the right position, can be used directly
     }
 }
 
@@ -179,7 +200,7 @@ pub enum SPI_Mode {
     _3wire,
 }
 
-/// FIFO mode selection. (Refer to Table 20)
+/// FIFO mode selection. (Refer to Table 22)
 #[allow(non_camel_case_types)]
 #[derive(Debug, Clone, Copy)]
 pub enum FIFO_MODE {
@@ -193,19 +214,41 @@ pub enum FIFO_MODE {
     Stream_to_FIFO = 0b011,
     /// Bypass-to-stream mode
     Bypass_to_stream = 0b100,
-    /// Dynamic-stream mode
-    Dynamic_Stream = 0b110,
+    /// FIFO Mean mode
+    FIFO_Mean = 0b110,
     /// Bypass-to-FIFO mode
     Bypass_to_FIFO = 0b111,
 }
 
 impl FIFO_MODE {
     pub fn value(self) -> u8 {
-        (self as u8) << 5 // shifted into the correct position, can be used directly
+        (self as u8) << 5 // shifted into the right position, can be used directly
+    }
+}
+
+/// FIFO Mean mode running average sample size. (Refer to Table 23)
+#[allow(non_camel_case_types)]
+#[derive(Debug, Clone, Copy)]
+pub enum FIFO_MEAN {
+    /// 2-sample moving average
+    _2sample = 0b00001,
+    /// 4-sample moving average
+    _4sample = 0b00011,
+    /// 8-sample moving average
+    _8sample = 0b00111,
+    /// 16-sample moving average
+    _16sample = 0b01111,
+    /// 32-sample moving average
+    _32sample = 0b11111,
+}
+
+impl FIFO_MEAN {
+    pub fn value(self) -> u8 {
+        self as u8 // no need to shift, bits 0:4
     }
 }
 
-/// INT_DRDY pin configuration. (Refer to Table 19)
+/// INT_DRDY pin configuration. (Refer to Table 21)
 #[allow(non_camel_case_types)]
 #[derive(Debug, Clone, Copy)]
 pub enum INT_DRDY {
@@ -221,7 +264,7 @@ pub enum INT_DRDY {
 
 impl INT_DRDY {
     pub fn value(self) -> u8 {
-        self as u8 // no need to shift, bits 0:1 (INT_S)
+        self as u8 // no need to shift, bits 0:1
     }
 }
 
@@ -265,6 +308,7 @@ impl INT_PIN {
     }
 }
 
+
 /// Settings for various FIFO- and interrupt-related flags, Enabled or Disabled
 #[allow(non_camel_case_types)]
 #[derive(Debug, Clone, Copy)]
@@ -303,4 +347,45 @@ impl FIFO_ON {
         };
         status
     }
+}
+
+
+/// Temperature resolution configuration, number of internal average(Refer to Table 18)
+#[allow(non_camel_case_types)]
+#[derive(Debug, Clone, Copy)]
+pub enum TEMP_RES {
+    /// Nr. internal average 8
+    _8 = 0b00,
+    /// Nr. internal average 16
+    _16 = 0b01,
+    /// Nr. internal average 32
+    _32 = 0b10,
+    /// Nr. internal average 64
+    _64 = 0b11,
+}
+
+impl TEMP_RES {
+    pub fn value(self) -> u8 {
+        (self as u8) << 2 // shifted into the right position, can be used directly
+    }
+}
+
+/// Pressure resolution configuration, number of internal average(Refer to Table 19)
+#[allow(non_camel_case_types)]
+#[derive(Debug, Clone, Copy)]
+pub enum PRESS_RES {
+    /// Nr. internal average 8
+    _8 = 0b00,
+    /// Nr. internal average 32
+    _32 = 0b01,
+    /// Nr. internal average 128
+    _128 = 0b10,
+    /// Nr. internal average 512
+    _512 = 0b11,
+}
+
+impl PRESS_RES {
+    pub fn value(self) -> u8 {
+        self as u8 // no need to shift
+    }
 }
\ No newline at end of file
