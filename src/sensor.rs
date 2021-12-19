//! Functions related to sensor measurements: reading value or status, setting offset and reference
 
use super::*;
 
#[derive(Debug)]
/// Contents of the STATUS register (pressure and temperature overrun and data availability flags)

pub struct DataStatus {
     pub press_available: bool,
     pub temp_available: bool,
     pub press_overrun: bool,
     pub temp_overrun: bool,
}
 

impl<T, E> LPS2X<T>
where
     T: Interface<Error = E>,
{
     /// Read the device ID ("who am I")
     pub fn get_device_id(&mut self) -> Result<u8, T::Error> {
         let mut data = [0u8; 1];
         self.interface.read(Registers::WHO_AM_I.addr(), &mut data)?;
         let whoami = data[0];
         Ok(whoami)
     }
 

     // FOR THESE FUNCTIONS TO WORK CORECTLY ON LPS25HB, THE MULTIBYTE HAS TO BE ENABLED DIRECTLY IN THE I2C INTERFACE

     /// Calculated pressure reading in hPa
     pub fn read_pressure(&mut self) -> Result<f32, T::Error> {
          let mut data = [0u8; 3];
          self.interface.read(
          Registers::PRESS_OUT_XL.addr(),
          // Registers::PRESS_OUT_XL.addr() | Bitmasks::MULTIBYTE,
          &mut data,
          )?;
          let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
          let pressure = (p as f32) / PRESS_SCALE; // no need to take care of negative values
          Ok(pressure)
     }

     pub fn read_temperature(&mut self) -> Result<f32, T::Error> {
         let mut data = [0u8; 2];
         self.interface.read(
               Registers::TEMP_OUT_L.addr(),
               // Registers::TEMP_OUT_L.addr() | Bitmasks::MULTIBYTE,
               &mut data,
         )?;
         let t: i16 = (data[1] as i16) << 8 | (data[0] as i16);

         #[cfg(feature = "lps22hb")]
         let temperature = (t as f32) / TEMP_SCALE;
         #[cfg(feature = "lps25hb")]
         let temperature = (t as f32) / TEMP_SCALE + TEMP_OFFSET;
      
         Ok(temperature)
     }
 
     /// Calculated reference pressure reading in hPa
     pub fn read_reference_pressure(&mut self) -> Result<f32, T::Error> {
          let mut data = [0u8; 3];
          self.interface.read(Registers::REF_P_XL.addr(), &mut data)?;
          let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
          let pressure: f32 = (p as f32) / PRESS_SCALE;
          Ok(pressure)
     }

     /// Read pressure offset value, 16-bit data that can be used to implement One-Point Calibration (OPC) after soldering.
     pub fn read_pressure_offset(&mut self) -> Result<i16, T::Error> {
          let mut data = [0u8; 2];
          self.interface.read(Registers::RPDS_L.addr(), &mut data)?;
          let o: i16 = (data[1] as i16) << 8 | (data[0] as i16);
          Ok(o)
     }

     /// Read threshold value for pressure interrupt generation
     pub fn read_threshold(&mut self) -> Result<i16, T::Error> {
          let mut data = [0u8; 2];
          self.interface.read(Registers::THS_P_L.addr(), &mut data)?;
          let o: i16 = (data[1] as i16) << 8 | (data[0] as i16);
          Ok(o)
     }


 
    /// Set the pressure offset value (VALUE IN hPA!)

     /// Set threshold value for pressure interrupt generation (VALUE IN hPA!)
     pub fn set_threshold(&mut self, threshold: u16) -> Result<(), T::Error> {
          let mut payload = [0u8; 2];
          // The value is expressed as unsigned number: Interrupt threshold(hPA) = (THS_P)/16.
          let threshold = threshold * 16;
 
          payload[0] = (threshold & 0xff) as u8; // lower byte
          payload[1] = (threshold >> 8) as u8; // upper byte
 
          self.interface.write(Registers::THS_P_L.addr(), payload[0])?;
          self.interface.write(Registers::THS_P_H.addr(), payload[1])?;
 
          Ok(())
     }

     /// Set the pressure offset value (VALUE IN hPA!)
     pub fn set_pressure_offset(&mut self, offset: u16) -> Result<(), T::Error> {
          let mut payload = [0u8; 2];
          let offset = offset * 16;

          payload[0] = (offset & 0xff) as u8; // lower byte
          payload[1] = (offset >> 8) as u8; // upper byte

          self.interface.write(Registers::RPDS_L.addr(), payload[0])?;
          self.interface.write(Registers::RPDS_H.addr(), payload[1])?;

          Ok(())
     }
 
     /// Get all the flags from the STATUS_REG register
     pub fn get_data_status(&mut self) -> Result<DataStatus, T::Error> {         
          let reg_value = self.read_register(Registers::STATUS)?;
         
          // IN LPS25HB THE NAME OF THE REGISTER IS STATUS_REG
          // let reg_value = self.read_register(Registers::STATUS_REG)?;
 
          let status = DataStatus {
               /// Is new pressure data available?
               press_available: match reg_value & Bitmasks::P_DA {
                    0 => false,
                    _ => true,
               },
               /// Is new temperature data available?
               temp_available: match reg_value & Bitmasks::T_DA {
                    0 => false,
                    _ => true,
               },
               
               /// Has new pressure data overwritten the previous one?   
               press_overrun: match reg_value & Bitmasks::P_OR {
                    0 => false,
                    _ => true,
               },
               /// Has new temperature data overwritten the previous one?
               temp_overrun: match reg_value & Bitmasks::T_OR {
                    0 => false,
                    _ => true,
               },
          };
          Ok(status)
     }
 
    /// Triggers the one-shot mode, and a new acquisition starts when it is required.
    /// Enabling this mode is possible only if the device was previously in power-down/one-shot mode.
    /// Once the acquisition is completed and the output registers updated,
    /// the device automatically enters in power-down mode. ONE_SHOT bit self-clears itself,
    /// the new data are available in the output registers and the STATUS bits are updated.    
     pub fn one_shot(&mut self) -> Result<(), T::Error> {          
          self.set_datarate(ODR::OneShot)?; // make sure that OneShot/Power down mode is enabled
          self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
         Ok(())
     }
}
