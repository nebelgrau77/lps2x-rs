diff --git a/fifo_lps22.rs b/fifo_lps25.rs
index 376af93..5c7a59f 100755
--- a/fifo_lps22.rs
+++ b/fifo_lps25.rs
@@ -1,27 +1,34 @@
 //! Various functions related to FIFO
 //!
-//! TO DO:
-//! - add all the FIFO-related functions
+//! TO DO: improve watermark level reading (?)
+
 
 use super::*;
 
 /// FIFO settings
 #[derive(Debug)]
 pub struct FIFOConfig {
+
     /// Stop on FIFO watermark (enable FIFO watermark use)
     pub enable_watermark: FLAG, // default disabled
+    /// Enable decimating output pressure to 1Hz with FIFO Mean mode
+    pub enable_decimating: FLAG, // default disabled
     /// Select FIFO operation mode (see Table 22 for details)        
     pub fifo_mode: FIFO_MODE, // default Bypass
     /// Set the watermark level
     pub watermark_level: u8, // default 0
+    /// Select sample size for FIFO Mean mode running average (see Table 23 for details)        
+    pub fifo_mean_config: FIFO_MEAN, // default 2-sample
 }
 
 impl Default for FIFOConfig {
     fn default() -> Self {
         FIFOConfig {
-            enable_watermark: FLAG::Disabled,      // disabled
-            fifo_mode: FIFO_MODE::Bypass, // Bypass mode
-            watermark_level: 32u8,        // 0 does not make sense as a default value
+            enable_watermark: FLAG::Disabled,               // disabled
+            enable_decimating: FLAG::Disabled,              // disabled
+            fifo_mode: FIFO_MODE::Bypass,          // Bypass mode
+            watermark_level: 32u8,                  // 0 does not make sense as a default value
+            fifo_mean_config: FIFO_MEAN::_2sample, // 2 samples
         }
     }
 }
@@ -34,12 +41,21 @@ impl FIFOConfig {
         if self.enable_watermark.status() {
             data |= 1 << 5;
         }
+        if self.enable_decimating.status() {
+            data |= 1 << 4;
+        }
         data
     }
     fn f_fifo_ctrl(&self) -> u8 {
         let mut data = 0u8;
+
         data |= self.fifo_mode.value();
-        data |= self.watermark_level;
+
+        let wtm = match self.fifo_mode {
+            FIFO_MODE::FIFO_Mean => self.fifo_mean_config.value(),
+            _ => self.watermark_level,
+        };
+        data |= wtm;
         data
     }
 }
@@ -53,40 +69,30 @@ pub struct FifoStatus {
     pub fifo_level: u8,
 }
 
-impl<T, E> LPS22HB<T>
+impl<T, E> LPS25HB<T>
 where
     T: Interface<Error = E>,
 {
-    // The FIFO buffer is enabled when the FIFO_EN bit in CTRL_REG2 (11h) is set to '1'
-    // and each mode is selected by the FIFO_MODE[2:0] bits in FIFO_CTRL (14h).
-
     /// Enable and configure FIFO
-    pub fn enable_fifo(&mut self, flag: FIFO_ON, config: FIFOConfig) -> Result<(), T::Error> {
+    pub fn configure_fifo(&mut self, flag: FIFO_ON, config: FIFOConfig) -> Result<(), T::Error> {
         match flag {
             FIFO_ON::Enabled => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN),
             FIFO_ON::Disabled => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN),
         }?;
 
-        match config.enable_watermark {
-            FLAG::Enabled => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
-            FLAG::Disabled => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
-        }?;
-
-        /*
         let mut reg_data = [0u8];
         self.interface
             .read(Registers::CTRL_REG2.addr(), &mut reg_data)?;
         reg_data[0] |= config.f_ctrl_reg2();
         self.interface
             .write(Registers::CTRL_REG2.addr(), reg_data[0])?;
-        */
         self.interface
             .write(Registers::FIFO_CTRL.addr(), config.f_fifo_ctrl())?;
 
         Ok(())
     }
 
-    // --- THIS FUNCTIONS COULD BE REMOVED ---
+    // --- THE FOLLOWING SECTION COULD BE REMOVED ---
 
     /*
 
@@ -98,7 +104,7 @@ where
         }
     }
 
-    /// Select FIFO operation mode (see Table 20 for details)
+    /// Select FIFO operation mode (see Table 22 for details)
     pub fn fifo_mode_config(&mut self, mode: FIFO_MODE) -> Result<(), T::Error> {
         let mut reg_data = [0u8];
         self.interface
@@ -110,50 +116,88 @@ where
         Ok(())
     }
 
-     */
+    /// Select sample size for FIFO Mean mode running average (see Table 23 for details)
+    pub fn fifo_mean_config(&mut self, sample: FIFO_MEAN) -> Result<(), T::Error> {
+        let mut reg_data = [0u8];
+        self.interface
+            .read(Registers::FIFO_CTRL.addr(), &mut reg_data)?;
+        let mut payload = reg_data[0];
+        payload &= !Bitmasks::WTM_POINT_MASK;
+        payload |= sample.value();
+        self.interface.write(Registers::FIFO_CTRL.addr(), payload)?;
+        Ok(())
+    }
+
+    /// Stop on FIFO watermark (enable FIFO watermark use)
+    pub fn stop_on_fth(&mut self, flag: bool) -> Result<(), T::Error> {
+        match flag {
+            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
+        }
+    }
 
-    // -- THESE FUNCTIONS CAN BE REPLACED WITH INTERRUPT CONFIGURATION ---
+    /// Enable decimating output pressure to 1Hz with FIFO Mean mode
+    pub fn fifo_decimate_enable(&mut self, flag: bool) -> Result<(), T::Error> {
+        match flag {
+            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_MEAN_DEC),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_MEAN_DEC),
+        }
+    }
 
     /*
+    /// Set the watermark level
+    pub fn set_watermark_level(&mut self, level: u8) -> Result<(), T::Error> {
+        let wtm: u8 = match level {
+            // if the input value exceeds the capacity, default to maximum
+            l if l < 33 => l,
+            _ => 32,
+        };
+        let mut reg_data = [0u8];
+        self.interface
+            .read(Registers::FIFO_CTRL.addr(), &mut reg_data)?;
+        let mut payload = reg_data[0];
+        payload &= !Bitmasks::WTM_MASK;
+        payload |= mode.value();
+        self.interface.write(Registers::FIFO_CTRL.addr(), payload)?;
+        Ok(())
+    }
+     */
 
-    /// FIFO full flag on INT_DRDY pin
-    pub fn fifo_full_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
+    /// FIFO empty flag on INT_DRDY pin
+    pub fn fifo_empty_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FSS5),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FSS5),
+            true => self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_EMPTY),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_EMPTY),
         }
     }
 
     /// FIFO filled up to threshold (watermark) level on INT_DRDY pin
-    pub fn fifo_fth_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
+    pub fn fifo_filled_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FTH),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FTH),
+            true => self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_FTH),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_FTH),
         }
     }
 
     /// FIFO overrun interrupt on INT_DRDY pin
     pub fn fifo_overrun_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_OVR),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_OVR),
+            true => self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_OVR),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_OVR),
         }
     }
 
-     */
+    */
 
-    // --- END OF THE BLOCK
+    // --- END OF THE SECTION THAT COULD BE REMOVED ---
 
     /// Get flags and FIFO level from the FIFO_STATUS register
     pub fn get_fifo_status(&mut self) -> Result<FifoStatus, T::Error> {
         
         let reg_value = self.read_register(Registers::FIFO_STATUS)?;
-        
-
-        let fifo_level_value = self.read_fifo_level()?;
 
         let status = FifoStatus {
-
+            /// Is FIFO filling equal or higher than the threshold?
             fifo_thresh_reached: match reg_value & Bitmasks::FTH_FIFO {
                 0 => false,
                 _ => true,
@@ -163,31 +207,40 @@ where
                 0 => false,
                 _ => true,
             },
-            /*
+            /// Is FIFO empty?
+            fifo_empty: match reg_value & Bitmasks::EMPTY_FIFO {
+                0 => false,
+                _ => true,
+            },
+            
+            /// Read FIFO stored data level
+            
+            // TO DO: REPLACE WITH BITMASKING
+            
+            fifo_level: self.read_fifo_level()?,
+
+          };
+
+        /*
+        let status = FifoStatus {
             /// Is FIFO filling equal or higher than the threshold?
             fifo_thresh_reached: self
                 .is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::FTH_FIFO)?,
             /// Is FIFO full and at least one sample has been overwritten?
             fifo_overrun: self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::OVR)?,
-            */
             /// Is FIFO empty?
-            /// 
-            
-            fifo_empty: match fifo_level_value {
-                0 => true,
-                _ => false,
-            },
-             
+            fifo_empty: self
+                .is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::EMPTY_FIFO)?,
             /// Read FIFO stored data level
-            fifo_level: fifo_level_value,
+            fifo_level: self.read_fifo_level()?,
         };
+        */
         Ok(status)
     }
 
     // --- THESE FUNCTIONS COULD BE REMOVED ---
 
     /*
-
     /// Is FIFO filling equal or higher than the threshold?
     pub fn fifo_threshold_status(&mut self) -> Result<bool, T::Error> {
         self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::FTH_FIFO)
@@ -198,45 +251,26 @@ where
         self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::OVR)
     }
 
+    /// Is FIFO empty?
+    pub fn fifo_empty_status(&mut self) -> Result<bool, T::Error> {
+        self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::EMPTY_FIFO)
+    }
      */
 
+    // --- THIS FUNCTION COULD BE PRIVATE ---
+
     /// Read FIFO stored data level
-    // pub fn read_fifo_level(&mut self) -> Result<u8, T::Error> {
+    //pub fn read_fifo_level(&mut self) -> Result<u8, T::Error> {
     fn read_fifo_level(&mut self) -> Result<u8, T::Error> {
-        let mut data = [0u8; 1];
+        let mut reg_data = [0u8];
         self.interface
-            .read(Registers::FIFO_STATUS.addr(), &mut data)?;
-        let level = data[0] & Bitmasks::FSS_MASK;
-        Ok(level)
-    }
-
-    // --- THESE FUNCTIONS COULD BE REMOVED
-
-    /*
+            .read(Registers::FIFO_STATUS.addr(), &mut reg_data)?;
 
-    /// Stop on FIFO watermark (enable FIFO watermark use)
-    pub fn stop_on_fth(&mut self, flag: bool) -> Result<(), T::Error> {
-        match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
-        }
-    }
-
-    /// Set the watermark level
-    pub fn set_watermark_level(&mut self, level: u8) -> Result<(), T::Error> {
-        let wtm: u8 = match level {
-            // if the input value exceeds the capacity, default to maximum
-            l if l < 33 => l,
-            _ => 32,
+        let fifo_level: u8 = match self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::EMPTY_FIFO)? {
+            true => 0,
+            false => (reg_data[0] & Bitmasks::FSS_MASK) + 1,
         };
-        let mut reg_data = [0u8];
-        self.interface
-            .read(Registers::FIFO_CTRL.addr(), &mut reg_data)?;
-        let mut payload = reg_data[0];
-        payload &= !Bitmasks::WTM_MASK;
-        payload |= mode.value();
-        self.interface.write(Registers::FIFO_CTRL.addr(), payload)?;
-        Ok(())
+
+        Ok(fifo_level)
     }
-    */
 }
