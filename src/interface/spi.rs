diff --git a/spi_lps22.rs b/spi_lps25.rs
index 9030c65..afc4b1f 100755
--- a/spi_lps22.rs
+++ b/spi_lps25.rs
@@ -1,12 +1,11 @@
 //! SPI Interface
-//! THIS ALL HAS TO BE MODIFIED - CURRENTLY JUST A COPY-PASTE FROM ANOTHER CRATE
-
-
+//! 
+//! TO DO: COMPLETE THIS (CURRENTLY JUST A COPY-PASTE FROM ANOTHER CRATE)
+//! 
 use super::Interface;
 use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};
 
 /*
-
 /// R/W bit should be high for SPI Read operation
 const SPI_READ: u8 = 0x80;
 /// Magnetometer MS bit. When 0, does not increment the address; when 1, increments the address in multiple reads. (Refer to page 34)
