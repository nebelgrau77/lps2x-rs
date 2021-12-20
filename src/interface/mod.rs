//! Interface trait
pub mod spi;
pub use self::spi::SpiInterface;
pub mod i2c;
pub use self::i2c::I2cInterface;

/// Interface Trait. `SpiInterface` and `I2cInterface` implement this.
pub trait Interface {
    type Error;
    /// Writes a byte to a sensor's specified register address.
    /// # Arguments    
    /// * `addr` - register address
    /// * `value` - value to write
    fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
    /// Reads multiple bytes from a sensor's specified register address.
    /// # Arguments    
    /// * `addr` - register address
    /// * `buffer` - buffer to store read data
    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
