//! SPI Interface
use super::Interface;
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

/// R/W bit should be high for SPI Read operation
const SPI_READ: u8 = 0x80;
/// MS bit. When 0, does not increment the address; when 1, increments the address in multiple reads.

#[cfg(feature="lps22hb")]
const MS_BIT: u8 = 0x00;
#[cfg(feature="lps25hb")]
const MS_BIT: u8 = 0x40;


/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
}

/// This combines the SPI Interface and chip select pins
pub struct SpiInterface<SPI, CS> {
    spi: SPI,    
    cs: CS,
}

impl<SPI, CS, CommE, PinE> SpiInterface<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,    
    CS: OutputPin<Error = PinE>,
{
    /// Initializes an Interface with `SPI` instance and chip select `OutputPin`s
    /// # Arguments
    /// * `spi` - SPI instance
    /// * `cs` - Chip Select pin    
    pub fn init(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
}

/// Implementation of `Interface`
impl<SPI, CS, CommE, PinE> Interface for SpiInterface<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,    
{
    type Error = Error<CommE, PinE>;

    fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        let bytes = [addr, value];
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.write(&bytes).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(())
    }
   

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.write(&[SPI_READ | MS_BIT | addr]).map_err(Error::Comm)?;
        self.spi.transfer(buffer).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;        
        Ok(())
    }
    
}