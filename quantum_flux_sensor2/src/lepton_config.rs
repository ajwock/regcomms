use regcomms::{RegCommsError, RegComms};
use crate::QuantumFluxSensor;
pub struct LeptonConfig<'a, C: RegComms<4, u32>>(pub &'a mut QuantumFluxSensor<C>);
impl<'a, C: RegComms<4, u32>> LeptonConfig<'a, C> {
    pub fn read(&mut self) -> Result<LeptonConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x16, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(LeptonConfigVal(val))
    }
    pub async fn read_async(&mut self) -> Result<LeptonConfigVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x16, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(LeptonConfigVal(val))
    }
    pub fn write(&mut self, val: LeptonConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x16, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: LeptonConfigVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x16, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct LeptonConfigVal(pub u8);
impl LeptonConfigVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn odr<'a>(&'a mut self) -> Odr<'a> {
        Odr(self)
    }
    pub fn dlpf<'a>(&'a mut self) -> Dlpf<'a> {
        Dlpf(self)
    }
    pub fn scale<'a>(&'a mut self) -> Scale<'a> {
        Scale(self)
    }
}
pub struct Odr<'a>(pub &'a mut LeptonConfigVal);
impl<'a> Odr<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 5) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut LeptonConfigVal {
        self.0.0 &= !(!(!0 << 3) << 5);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 5;
        self.0
    }
}
pub struct Dlpf<'a>(pub &'a mut LeptonConfigVal);
impl<'a> Dlpf<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 2) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut LeptonConfigVal {
        self.0.0 &= !(!(!0 << 3) << 2);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 2;
        self.0
    }
}
pub struct Scale<'a>(pub &'a mut LeptonConfigVal);
impl<'a> Scale<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut LeptonConfigVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
}
