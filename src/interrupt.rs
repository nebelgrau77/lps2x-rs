diff --git a/interrupt_lps22.rs b/interrupt_lps25.rs
index 29c04d5..1644e18 100755
--- a/interrupt_lps22.rs
+++ b/interrupt_lps25.rs
@@ -1,53 +1,49 @@
 //! Various functions related to interrupts
-//!
-//! TO DO: 
-//! - "enable" flag in the interrupt enable function must be moved to the config, called "enable_differential"
-//! - interrupt status reading must be in one go, as with latching on it would clear the register by reading the first bit the way it's configured now
-//! 
 
 use super::*;
 
 /// Interrupt pin settings
 #[derive(Debug)]
 pub struct InterruptConfig {
-    /// configure interrupt pin as active high or active low 
-    pub active_high_or_low: INT_ACTIVE, 
+
+    /// configure interrupt pin as active high or active low
+    pub active_high_or_low: INT_ACTIVE,
     /// configure interrupt pin as  push-pull or open drain
     pub pushpull_or_opendrain: INT_PIN,
     /// configure data signal on the interrupt pin
     pub data_signal_config: INT_DRDY,
-    /// enable FIFO full flag on interrupt pin
-    pub enable_fifo_full: FLAG, 
+    /// enable FIFO empty flag on interrupt pin
+    pub enable_fifo_empty: FLAG,
     /// enable FIFO watermark flag on interrupt pin
-    pub enable_fifo_fth: FLAG, 
+    pub enable_fifo_fth: FLAG,
     /// enable FIFO overrun flag on interrupt pin
     pub enable_fifo_overrun: FLAG,
     /// enable data ready signal on interrupt pin
     pub enable_data_ready: FLAG,
+    /// enable computing of differential pressure output
+    pub enable_differential: FLAG,
     /// enable latching interrupt request to INT_SOURCE register
     pub enable_latch_interrupt: FLAG,
     /// enable low pressure event on interrupt pin
     pub enable_low_event: FLAG,
     /// enable hihg pressure event on interrupt pin
     pub enable_high_event: FLAG,
-    /// enable computing of differential pressure output
-    pub enable_differential: FLAG,
 }
 
 impl Default for InterruptConfig {
     fn default() -> Self {
         InterruptConfig {
-            active_high_or_low: INT_ACTIVE::High,                // active high (CTRL_REG3)
-            pushpull_or_opendrain: INT_PIN::PushPull,            // push-pull (CTRL_REG3)
-            data_signal_config: INT_DRDY::DataSignal,            // data signal on INT_DRDY pin (CTRL_REG3)
-            enable_fifo_full: FLAG::Disabled,                    // disabled (CTRL_REG3)
-            enable_fifo_fth: FLAG::Disabled,                     // disabled (CTRL_REG3)
-            enable_fifo_overrun: FLAG::Disabled,                 // disabled (CTRL_REG3)
-            enable_data_ready: FLAG::Disabled,                   // disabled (CTRL_REG3)
-            enable_latch_interrupt: FLAG::Disabled,              // interrupt request not latched (INTERRUPT_CFG)
-            enable_low_event: FLAG::Disabled,                    // disable interrupt request on low pressure event (INTERRUPT_CFG)
-            enable_high_event: FLAG::Disabled,                   // disable interrupt request on low pressure event (INTERRUPT_CFG)
-            enable_differential: FLAG::Disabled,                 // disabled (CTRL_REG1)
+            active_high_or_low: INT_ACTIVE::High,              // active high (CTRL_REG3)
+            pushpull_or_opendrain: INT_PIN::PushPull,          // push-pull (CTRL_REG3)
+            data_signal_config: INT_DRDY::DataSignal,          // data signal on INT_DRDY pin (CTRL_REG3)
+            enable_fifo_empty: FLAG::Disabled,                 // disabled (CTRL_REG4)
+            enable_fifo_fth: FLAG::Disabled,                   // disabled (CTRL_REG4)
+            enable_fifo_overrun: FLAG::Disabled,               // disabled (CTRL_REG4)
+            enable_data_ready: FLAG::Disabled,                 // disabled (CTRL_REG4)
+            enable_differential: FLAG::Disabled,               // disabled (CTRL_REG1)
+            enable_latch_interrupt: FLAG::Disabled,            // inferrupt request not latched (INTERRUPT_CFG)
+            enable_low_event: FLAG::Disabled,                  // disable interrupt request on low pressure event (INTERRUPT_CFG)
+            enable_high_event: FLAG::Disabled,                 // disable interrupt request on low pressure event (INTERRUPT_CFG)
         }
     }
 }
@@ -62,25 +58,28 @@ impl InterruptConfig {
         if self.pushpull_or_opendrain.status() {
             data |= 1 << 6;
         }
-        if self.enable_fifo_full.status() {
-            data |= 1 << 5;
+        // MUST USE THE ACTUAL u8 VALUE HERE
+        data |= self.data_signal_config.value();
+        data
+    }
+    fn int_ctrl_reg4(&self) -> u8 {
+        let mut data = 0u8;
+        if self.enable_fifo_empty.status() {
+            data |= 1 << 3;
         }
         if self.enable_fifo_fth.status() {
-            data |= 1 << 4;
+            data |= 1 << 2;
         }
         if self.enable_fifo_overrun.status() {
-            data |= 1 << 3;
+            data |= 1 << 1;
         }
         if self.enable_data_ready.status() {
-            data |= 1 << 2;
-        }        
-        data |= self.data_signal_config.value();
+            data |= 1;
+        }
         data
-    }    
+    }
     fn int_interrupt_cfg(&self) -> u8 {
-        
         let mut data = 0u8;
-
         if self.enable_latch_interrupt.status() {
             data |= 1 << 2;
         }
@@ -90,7 +89,7 @@ impl InterruptConfig {
         if self.enable_high_event.status() {
             data |= 1;
         }
-        data // this must be OR'ed with the content of the INTERRUPT_CFG
+        data
     }
 }
 
@@ -102,39 +101,39 @@ pub struct IntStatus {
     pub diff_press_high: bool,    
 }
 
-impl<T, E> LPS22HB<T>
+impl<T, E> LPS25HB<T>
 where
     T: Interface<Error = E>,
 {
-    /// Enable interrupts and configure the interrupt pin
-    pub fn configure_interrupts(&mut self, flag: bool, config: InterruptConfig,) -> Result<(), T::Error> {
+    /// Configure interrupt pin and interrupt sources, enable and configure differential pressure interrupts
+    pub fn configure_interrupts(
+        &mut self,
+        //flag: bool,
+        config: InterruptConfig,
+    ) -> Result<(), T::Error> {
         match config.enable_differential {
-            FLAG::Enabled => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
-            FLAG::Disabled => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
+            FLAG::Enabled => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::DIFF_EN),
+            FLAG::Disabled => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::DIFF_EN),
         }?;
+
         self.interface
-            .write(Registers::CTRL_REG3.addr(), config.int_ctrl_reg3())?;        
-        
-        let mut buffer = [0u8;1];
-        self.read_register(Registers::INTERRUPT_CFG)?;        
-        let mut interrupt_cfg = 0u8;
-        interrupt_cfg |= config.int_interrupt_cfg();
-       
+            .write(Registers::CTRL_REG3.addr(), config.int_ctrl_reg3())?;
         self.interface
-            .write(Registers::INTERRUPT_CFG.addr(), interrupt_cfg)?;
+            .write(Registers::CTRL_REG4.addr(), config.int_ctrl_reg4())?;
+        self.interface
+            .write(Registers::INTERRUPT_CFG.addr(), config.int_interrupt_cfg())?;
         Ok(())
     }
-    
+
     // --- THE FOLLOWING SECTION COULD BE REMOVED --- 
 
     /*
-    
-    
+
     /// Configuration of the interrupt generation (enabled/disable)
     pub fn int_generation_enable(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
-            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
-            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
+            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::DIFF_EN),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::DIFF_EN),
         }
     }
 
@@ -146,11 +145,27 @@ where
         }
     }
 
+    /// Enable interrupt on differential pressure low event
+    pub fn diff_press_low_enable(&mut self, flag: bool) -> Result<(), T::Error> {
+        match flag {
+            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PL_E),
+            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PL_E),
+        }
+    }
+
+    /// Enable interrupt on differential pressure high event
+    pub fn diff_press_high_enable(&mut self, flag: bool) -> Result<(), T::Error> {
+        match flag {
+            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PH_E),
+            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PH_E),
+        }
+    }
+
     /// Data-ready signal on INT_DRDY pin
     pub fn data_signal_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
         match flag {
-            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::DRDY),
-            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::DRDY),
+            true => self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::DRDY),
+            false => self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::DRDY),
         }
     }
 
@@ -187,9 +202,34 @@ where
     }
 
     */
-    /*
+
+    // --- END OF THE SECTION THAT COULD BE REMOVED --- 
+
+
     /// Get all the flags from the INT_SOURCE register (NOTE: INT_SOURCE register is cleared by reading it)
-    pub fn get_int_status(&mut self) -> Result<IntSource, T::Error> {        
+    pub fn get_int_status(&mut self) -> Result<IntStatus, T::Error> {        
+                
+        let reg_value = self.read_register(Registers::INT_SOURCE)?;
+
+        let status = IntStatus {
+            /// Has any interrupt event been generated?
+            interrupt_active: match reg_value & Bitmasks::IA {
+                0 => false,
+                _ => true,
+            },
+            /// Has low differential pressure event been generated?
+            diff_press_low: match reg_value & Bitmasks::PL {
+                0 => false,
+                _ => true,
+            },
+            /// Has high differential pressure event been generated?
+            diff_press_high: match reg_value & Bitmasks::PH {
+                0 => false,
+                _ => true,
+            },           
+        };
+
+        /*
         let status = IntSource {
             /// Has any interrupt event been generated? (self clearing)
             interrupt_active: self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::IA)?,
@@ -198,35 +238,10 @@ where
             /// Has high differential pressure event been generated? (self clearing)
             diff_press_high: self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::PH)?,
         };
+        */
         Ok(status)
     }
- */
- /// Get all the flags from the INT_SOURCE register (NOTE: INT_SOURCE register is cleared by reading it)
- pub fn get_int_status(&mut self) -> Result<IntStatus, T::Error> {        
-                
-    let reg_value = self.read_register(Registers::INT_SOURCE)?;
-
-    let status = IntStatus {
-        /// Has any interrupt event been generated?
-        interrupt_active: match reg_value & Bitmasks::IA {
-            0 => false,
-            _ => true,
-        },
-        /// Has low differential pressure event been generated?
-        diff_press_low: match reg_value & Bitmasks::PL {
-            0 => false,
-            _ => true,
-        },
-        /// Has high differential pressure event been generated?
-        diff_press_high: match reg_value & Bitmasks::PH {
-            0 => false,
-            _ => true,
-        },           
-    };
-    Ok(status)
- }
 
-}
     // --- THESE FUNCTIONS COULD BE REMOVED ---
 
     /*
@@ -245,13 +260,5 @@ where
     pub fn high_pressure_event_occurred(&mut self) -> Result<bool, T::Error> {
         self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::PH)
     }
-
-    /// Enable interrupt on differential pressure high event
-    pub fn diff_press_high_enable(&mut self, flag: bool) -> Result<(), T::Error> {
-        match flag {
-            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PHE),
-            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PHE),
-        }
-    }
-
      */
+}
