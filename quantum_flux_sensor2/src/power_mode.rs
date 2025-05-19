use regcomms::{RegCommsError, RegComms};
use crate::QuantumFluxSensor;
pub struct PowerMode<'a, C: RegComms<4, u32>>(pub &'a mut QuantumFluxSensor<C>);
impl<'a, C: RegComms<4, u32>> PowerMode<'a, C> {
    pub fn read(&mut self) -> Result<PowerModeVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x1, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(PowerModeVal(val))
    }
    pub async fn read_async(&mut self) -> Result<PowerModeVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read_async(0x1, &mut buf, crate::AccessProc::Standard).await?;
        let val = u8::from_be_bytes(buf);
        Ok(PowerModeVal(val))
    }
    pub fn write(&mut self, val: PowerModeVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x1, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
    pub async fn write_async(&mut self, val: PowerModeVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write_async(0x1, &buf, crate::AccessProc::Standard).await?;
        Ok(())
    }
}
pub struct PowerModeVal(pub u8);
impl PowerModeVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn pulsed<'a>(&'a mut self) -> Pulsed<'a> {
        Pulsed(self)
    }
    pub fn poweron_mode<'a>(&'a mut self) -> PoweronMode<'a> {
        PoweronMode(self)
    }
}
pub struct Pulsed<'a>(pub &'a mut PowerModeVal);
impl<'a> Pulsed<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PowerModeVal {
        self.0.0 &= !(1 << 7);
        self.0.0 |= !(!(val as u8) << 7);
        self.0
    }
    pub fn set_bit(self) -> &'a mut PowerModeVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PowerModeVal {
        self.assign(false)
    }
}
pub struct PoweronMode<'a>(pub &'a mut PowerModeVal);
impl<'a> PoweronMode<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 3) & !(!0 << 3)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PowerModeVal {
        self.0.0 &= !(!(!0 << 3) << 3);
        self.0.0 |= ((val as u8) & !(!0 << 3)) << 3;
        self.0
    }
}
