diff --git a/i2c_lps22.rs b/i2c_lps25.rs
index 685142c..0ac5876 100755
--- a/i2c_lps22.rs
+++ b/i2c_lps25.rs
@@ -2,6 +2,10 @@
 use super::Interface;
 use embedded_hal::blocking::i2c::{Write, WriteRead};
 
+/// Multibyte bit. When 0, does not increment the address; when 1, increments the address in multiple reads. (Refer to page 25)
+/// Must be OR'ed with the register address to enable multibyte data reading (temperature/pressure)
+const MULTIBYTE: u8 = 0b1000_0000;
+
 /// Errors in this crate
 #[derive(Debug)]
 pub enum Error<CommE> {
@@ -51,7 +55,7 @@ where
     type Error = Error<CommE>;
 
     fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
-        //let sensor_addr = self.dev_addr;        
+        //let sensor_addr = self.dev_addr;         
         core::prelude::v1::Ok(
             self.i2c
                 //.write(sensor_addr, &[addr, value])
