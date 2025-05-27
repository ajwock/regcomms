use core::default::Default;
use core::result::Result;
use crate::{
    RegComms,
    RegCommsAddress,
    RegCommsError,
};

#[cfg(feature = "embedded-hal")]
pub struct I2cComms<A: Copy + Default + embedded_hal::i2c::AddressMode, I: embedded_hal::i2c::I2c<A>> {
    pub comms: I,
    pub i2c_address: A,
}

#[cfg(feature = "embedded-hal")]
impl<A: Copy + Default + embedded_hal::i2c::AddressMode, I: embedded_hal::i2c::I2c<A>> I2cComms<A, I> {
    pub fn new(comms: I) -> Self {
        Self {
            comms,
            i2c_address: Default::default(),
        }
    }

    pub fn with_address(self, i2c_address: A) -> Self {
        Self {
            i2c_address,
            ..self
        }
    }

    pub fn set_address(&mut self, i2c_address: A) {
        self.i2c_address = i2c_address;
    }
}

#[cfg(feature = "embedded-hal")]
impl<A: Copy + Default + embedded_hal::i2c::AddressMode, I: embedded_hal::i2c::I2c<A>, const N: usize, R: RegCommsAddress<N>> RegComms<N, R> for I2cComms<A, I> {
    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        let reg_address_bytes = reg_address.to_big_endian();
        match self.comms.write_read(self.i2c_address, &reg_address_bytes, buf) {
            Ok(_) => Ok(buf.len()),
            Err(_) => Err(RegCommsError::Other),
        }
    }

    fn comms_write(&mut self, reg_address: R, buf: &[u8]) -> Result<usize, RegCommsError> {
        let reg_address_bytes = reg_address.to_big_endian();
        let mut ops = [embedded_hal::i2c::Operation::Write(&reg_address_bytes), embedded_hal::i2c::Operation::Write(buf)];
        match self.comms.transaction(self.i2c_address, &mut ops) {
            Ok(_) => Ok(buf.len()),
            Err(_) => Err(RegCommsError::Other),
        }
    }
}

#[cfg(feature = "embedded-hal-async")]
use crate::blockon::block_on;

#[cfg(feature = "embedded-hal-async")]
pub struct I2cCommsAsync<A: Copy + Default + embedded_hal_async::i2c::AddressMode, I: embedded_hal_async::i2c::I2c<A>> {
    comms: I,
    i2c_address: A,
}

#[cfg(feature = "embedded-hal-async")]
impl<A: Copy + Default + embedded_hal_async::i2c::AddressMode, I: embedded_hal_async::i2c::I2c<A>> I2cCommsAsync<A, I> {
    pub fn new(comms: I) -> Self {
        Self {
            comms,
            i2c_address: Default::default(),
        }
    }

    pub fn with_address(self, i2c_address: A) -> Self {
        Self {
            i2c_address,
            ..self
        }
    }

    pub fn set_address(&mut self, i2c_address: A) {
        self.i2c_address = i2c_address;
    }
}

#[cfg(feature = "embedded-hal-async")]
impl<A: Copy + Default + embedded_hal_async::i2c::AddressMode, I: embedded_hal_async::i2c::I2c<A>, const N: usize, R: RegCommsAddress<N>> RegComms<N, R> for I2cCommsAsync<A, I> {

    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        block_on(self.comms_read_async(reg_address, buf))
    }

    fn comms_write(&mut self, reg_address: R, buf: &[u8]) -> Result<usize, RegCommsError> {
        block_on(self.comms_write_async(reg_address, buf))
    }

    async fn comms_read_async(&mut self, reg_address: R, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        let reg_address_bytes = reg_address.to_big_endian();
        match self.comms.write_read(self.i2c_address, &reg_address_bytes, buf).await {
            Ok(_) => Ok(buf.len()),
            Err(_) => Err(RegCommsError::Other),
        }
    }

    async fn comms_write_async(&mut self, reg_address: R, buf: &[u8]) -> Result<usize, RegCommsError> {
        let reg_address_bytes = reg_address.to_big_endian();
        let mut ops = [embedded_hal_async::i2c::Operation::Write(&reg_address_bytes), embedded_hal_async::i2c::Operation::Write(buf)];
        match self.comms.transaction(self.i2c_address, &mut ops).await {
            Ok(_) => Ok(buf.len()),
            Err(_) => Err(RegCommsError::Other),
        }
    }
}
