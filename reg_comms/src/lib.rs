#![no_std]
#![allow(async_fn_in_trait)]

#[cfg(feature = "embedded_hal")]
pub struct I2cComms<A: embedded_hal::i2c::AddressMode = embedded_hal::i2c::SevenBitAddress, I: embedded_hal::i2c::I2c<A>> {
    comms: I,
    i2c_address: A,
}

#[cfg(feature = "embedded_hal")]
impl<A: embedded_hal::i2c::AddressMode, I: embedded_hal::i2c::I2c<A>, const N: usize, R: RegCommsAddress<N>> RegComms<R> for I2cComms<A, I> {
    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<(), RegCommsError> {
        let reg_address_bytes = reg_address.to_big_endian();
        match self.comms.write_read(self.i2c_address, reg_address_bytes, buf) {
            Ok(_) => Ok(()),
            Err(_) => Err(RegCommsError::Other),
        }
    }

    fn comms_write(&mut self, reg_address: R, buf: &[u8]) -> Result<(), RegCommsError> {
        todo!()
    }
}


pub enum RegCommsError {
    Other,
}

pub trait RegCommsAddress<const N: usize>: Copy {
    fn to_big_endian(self) -> [u8; N];
    fn to_little_endian(self) -> [u8; N];
    fn from_big_endian(bytes: [u8; N]) -> Self;
    fn from_little_endian(bytes: [u8; N]) -> Self;
}

pub trait RegComms<const N: usize, R: RegCommsAddress<N>> {
    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<(), RegCommsError>;
    fn comms_write(&mut self, reg_address: R, buf: &[u8]) -> Result<(), RegCommsError>;

    async fn comms_read_async<'a>(
        &'a mut self,
        reg_address: R,
        buf: &'a mut [u8],
    ) -> Result<(), RegCommsError> {
        self.comms_read(reg_address, buf)
    }

    async fn comms_write_async<'a>(
        &'a mut self,
        reg_address: R,
        buf: &'a [u8],
    ) -> Result<(), RegCommsError> {
        self.comms_write(reg_address, buf)
    }
}

impl RegCommsAddress<1> for u8 {
    fn to_big_endian(self) -> [u8; 1] {
        [self]
    }
    fn to_little_endian(self) -> [u8; 1] {
        [self]
    }
    fn from_big_endian(bytes: [u8; 1]) -> u8 {
        bytes[0]
    }
    fn from_little_endian(bytes: [u8; 1]) -> u8 {
        bytes[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
