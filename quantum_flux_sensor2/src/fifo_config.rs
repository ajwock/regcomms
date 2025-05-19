use regcomms::{RegCommsError, RegComms};
use crate::QuantumFluxSensor;
pub struct FifoConfig<'a, C: RegComms<4, u32>>(pub &'a mut QuantumFluxSensor<C>);
impl<'a, C: RegComms<4, u32>> FifoConfig<'a, C> {
    pub fn read(&mut self) -> Result<FifoConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x20, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<FifoConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x20, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(FifoConfigVal(val))
    }
    pub fn write(&mut self, val: FifoConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x20, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: FifoConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x20, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct FifoConfigVal(pub u8);
impl FifoConfigVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn fifo_src<'a>(&'a mut self) -> FifoSrc<'a> {
        FifoSrc(self)
    }
    pub fn fifo_fmt<'a>(&'a mut self) -> FifoFmt<'a> {
        FifoFmt(self)
    }
    pub fn fifo_en<'a>(&'a mut self) -> FifoEn<'a> {
        FifoEn(self)
    }
    pub fn fifo_decimation<'a>(&'a mut self) -> FifoDecimation<'a> {
        FifoDecimation(self)
    }
}
pub struct FifoSrc<'a>(pub &'a mut FifoConfigVal);
impl<'a> FifoSrc<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoConfigVal {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 5;
        self.0
    }
}
pub struct FifoFmt<'a>(pub &'a mut FifoConfigVal);
impl<'a> FifoFmt<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoConfigVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
}
pub struct FifoEn<'a>(pub &'a mut FifoConfigVal);
impl<'a> FifoEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FifoConfigVal {
        self.0.0 &= !(1 << 2);
        self.0.0 |= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut FifoConfigVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FifoConfigVal {
        self.assign(false)
    }
}
pub struct FifoDecimation<'a>(pub &'a mut FifoConfigVal);
impl<'a> FifoDecimation<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut FifoConfigVal {
        self.0.0 &= !(!(!0 << 2) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 3;
        self.0
    }
}
