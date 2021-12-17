diff --git a/config_lps22.rs b/config_lps25.rs
index fc27482..0b44044 100755
--- a/config_lps22.rs
+++ b/config_lps25.rs
@@ -1,10 +1,9 @@
 //! Various functions related to configuration
 //!
-//! TO DO:
 
 use super::*;
 
-impl<T, E> LPS22HB<T>
+impl<T, E> LPS25HB<T>
 where
     T: Interface<Error = E>,
 {
@@ -20,15 +19,40 @@ where
         Ok(())
     }
 
+    /// Temperature internal average configuration (default 64).
+    pub fn temperature_resolution(&mut self, resolution: TEMP_RES) -> Result<(), T::Error> {
+        let mut reg_data = [0u8];
+        self.interface
+            .read(Registers::RES_CONF.addr(), &mut reg_data)?;
+        let mut payload = reg_data[0];
+        payload &= !Bitmasks::AVGT_MASK;
+        payload |= resolution.value();
+        self.interface.write(Registers::RES_CONF.addr(), payload)?;
+        Ok(())
+    }
+
+    /// Pressure internal average configuration (default 512).
+    pub fn pressure_resolution(&mut self, resolution: PRESS_RES) -> Result<(), T::Error> {
+        let mut reg_data = [0u8];
+        self.interface
+            .read(Registers::RES_CONF.addr(), &mut reg_data)?;
+        let mut payload = reg_data[0];
+        payload &= !Bitmasks::AVGP_MASK;
+        payload |= resolution.value();
+        self.interface.write(Registers::RES_CONF.addr(), payload)?;
+        Ok(())
+    }
+
     // --- THIS FUNCTION CAN BE REMOVED
+
     /*
-    /// Enable single shot data acquisition (self cleared by hardware)
+    /// Enable single shot data acquisition (self cleared by hardware).
     pub fn enable_one_shot(&mut self) -> Result<(), T::Error> {
+        // self.set_datarate(ODR::OneShot)?; // make sure that OneShot mode is enabled
         self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
         Ok(())
     }
      */
-
     /// Enable or disable block data update
     pub fn bdu_enable(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
@@ -37,24 +61,19 @@ where
         }
     }
 
-    /// AUTOZERO: when set to ‘1’, the measured pressure is used
-    /// as the reference in REF_P (0x15, 0x16, 0x17).
-    /// From that point on the output pressure registers are updated and the same value
-    /// is also used for interrupt generation.
-    /// The register content of REF_P is subtracted from the measured pressure.
-    /// PRESS_OUT = measured pressure - REF_P
-    /// P_DIFF_IN = measured pressure - REF_P
-    ///     
+    /// AUTOZERO: when set to ‘1’, the actual pressure output value is copied in
+    /// REF_P_H (0Ah), REF_P_L (09h) and REF_P_XL (08h).
+    /// When this bit is enabled, the register content of REF_P is subtracted from the pressure output value.
     pub fn autozero_config(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
-            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::AUTOZERO),
-            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::AUTOZERO),
+            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::AUTOZERO),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::AUTOZERO),
         }
     }
 
     /// Resets the Autozero function. Self-cleared.
     pub fn autozero_reset(&mut self) -> Result<(), T::Error> {
-        self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::RESET_AZ)
+        self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::RESET_AZ)
     }
 
     /// Disables I2C interface (default 0, I2C enabled)
@@ -73,12 +92,11 @@ where
         }
     }
 
-    /// Register address automatically incremented during a multiple byte access with a serial interface (I2C or SPI).
-    /// Default value: enabled
-    pub fn address_incrementing(&mut self, flag: bool) -> Result<(), T::Error> {
+    /// Turn the sensor on (sensor is in power down by default)
+    pub fn sensor_on(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::IF_ADD_INC),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::IF_ADD_INC),
+            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::PD),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::PD),
         }
     }
 
@@ -87,70 +105,12 @@ where
     /// related to the trimming functions to allow correct behavior of the device itself.
     /// If for any reason the content of the trimming registers is modified,
     /// it is sufficient to use this bit to restore the correct values.
-    /// At the end of the boot process the BOOT bit is set again to ‘0’ by hardware.
-    /// The BOOT bit takes effect after one ODR clock cycle.
     pub fn reboot(&mut self) -> Result<(), T::Error> {
         self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::BOOT)
     }
 
-    /// Is reboot phase running?
-    pub fn reboot_running(&mut self) -> Result<bool, T::Error> {
-        self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::BOOT_STATUS)
-    }
-
     /// Run software reset (resets the device to the power-on configuration, takes 4 usec)
     pub fn software_reset(&mut self) -> Result<(), T::Error> {
         self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::SWRESET)
     }
-
-    // SWITCHING INTO POWER-DOWN COULD BE ADDED TO THIS FUNCTION
-    /// Enable low-power mode (must be done only with the device in power-down mode)
-    pub fn enable_low_power(&mut self) -> Result<(), T::Error> {
-        self.set_register_bit_flag(Registers::RES_CONF, Bitmasks::LC_EN)
-    }
-
-    // LOWPASS FILTER ENABLING AND CONFIGURING COULD BE MOVED TOGETHER
-
-    /// Enable and configure low-pass filter on pressure data in Continuous mode
-    pub fn lowpass_filter(&mut self, enable: bool, configure: bool) -> Result<(), T::Error> {
-        match enable {
-            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
-        }?;
-        match configure {
-            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
-        }?;
-        Ok(())
-    }
-
-    /*
-
-    /// Enable low-pass filter on pressure data in Continuous mode
-    pub fn lowpass_filter_enable(&mut self, flag: bool) -> Result<(), T::Error> {
-        match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
-        }
-    }
-
-    /// Switches the LPFP_CFG bit.
-    /// Depending on the status of the EN_LPFP bit the device bandwith is ODR/9 or ODR/20 (see Table 18)
-    pub fn lowpass_filter_configure(&mut self, flag: bool) -> Result<(), T::Error> {
-        match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
-        }
-    }
-
-     */
-
-    /// Reset low-pass filter.  If the LPFP is active, in order to avoid the transitory phase,
-    /// the filter can be reset by reading this register before generating pressure measurements.
-    pub fn lowpass_filter_reset(&mut self) -> Result<(), T::Error> {
-        let mut _data = [0u8; 1];
-        self.interface
-            .read(Registers::LPFP_RES.addr(), &mut _data)?;
-        Ok(())
-    }
 }
