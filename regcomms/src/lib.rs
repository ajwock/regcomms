#![no_std]
#![allow(async_fn_in_trait)]

#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-async"))]
pub mod i2c;

#[cfg(any(feature = "embedded-hal-async"))]
mod blockon;

use core::default::Default;
use core::result::Result;

#[derive(Copy, Clone, Debug)]
pub enum RegCommsError {
    Other,
    IncompleteTransfer,
}

pub trait RegCommsAddress<const N: usize>: Copy {
    fn to_big_endian(self) -> [u8; N];
    fn to_little_endian(self) -> [u8; N];
    fn from_big_endian(bytes: [u8; N]) -> Self;
    fn from_little_endian(bytes: [u8; N]) -> Self;
}

pub trait RegComms<const N: usize, R: RegCommsAddress<N>> {
    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<usize, RegCommsError>;
    fn comms_write(&mut self, reg_address: R, buf: &[u8]) -> Result<usize, RegCommsError>;

    async fn comms_read_async<'a>(
        &'a mut self,
        reg_address: R,
        buf: &'a mut [u8],
    ) -> Result<usize, RegCommsError> {
        self.comms_read(reg_address, buf)
    }

    async fn comms_write_async<'a>(
        &'a mut self,
        reg_address: R,
        buf: &'a [u8],
    ) -> Result<usize, RegCommsError> {
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

impl RegCommsAddress<2> for u16 {
    fn to_big_endian(self) -> [u8; 2] {
        self.to_be_bytes()
    }
    fn to_little_endian(self) -> [u8; 2] {
        self.to_le_bytes()
    }
    fn from_big_endian(bytes: [u8; 2]) -> Self {
        Self::from_be_bytes(bytes)
    }
    fn from_little_endian(bytes: [u8; 2]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

impl RegCommsAddress<4> for u32 {
    fn to_big_endian(self) -> [u8; 4] {
        self.to_be_bytes()
    }
    fn to_little_endian(self) -> [u8; 4] {
        self.to_le_bytes()
    }
    fn from_big_endian(bytes: [u8; 4]) -> Self {
        Self::from_be_bytes(bytes)
    }
    fn from_little_endian(bytes: [u8; 4]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

impl RegCommsAddress<8> for u64 {
    fn to_big_endian(self) -> [u8; 8] {
        self.to_be_bytes()
    }
    fn to_little_endian(self) -> [u8; 8] {
        self.to_le_bytes()
    }
    fn from_big_endian(bytes: [u8; 8]) -> Self {
        Self::from_be_bytes(bytes)
    }
    fn from_little_endian(bytes: [u8; 8]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

pub trait RegCommsAccessProc<Peripheral, const N: usize, R: RegCommsAddress<N>>: Default + Send + Sync {
    fn proc_read(&self, peripheral: &mut Peripheral, reg_address: R, buf: &mut [u8]) -> Result<usize, RegCommsError>;
    async fn proc_read_async(&self, peripheral: &mut Peripheral, reg_address: R, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        self.proc_read(peripheral, reg_address, buf)
    }
    fn proc_write(&self, peripheral: &mut Peripheral, reg_address: R, buf: &[u8]) -> Result<usize, RegCommsError>;
    async fn proc_write_async(&self, peripheral: &mut Peripheral, reg_address: R, buf: &[u8]) -> Result<usize, RegCommsError> {
        self.proc_write(peripheral, reg_address, buf)
    }
}
