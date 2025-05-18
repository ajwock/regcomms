use reg_comms::{RegCommsError, RegComms};
use crate::Lsm6Dsv;
pub struct PinCtrl<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> PinCtrl<'a, C> {
    pub fn read(&mut self) -> Result<PinCtrlVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x2, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(PinCtrlVal(val))
    }
    pub fn write(&mut self, val: PinCtrlVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x2, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
}
pub struct PinCtrlVal(pub u8);
impl PinCtrlVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn sdo_pu_en<'a>(&'a mut self) -> SdoPuEn<'a> {
        SdoPuEn(self)
    }
    pub fn ibhr_por_en<'a>(&'a mut self) -> IbhrPorEn<'a> {
        IbhrPorEn(self)
    }
    pub fn io_pad_strength<'a>(&'a mut self) -> IoPadStrength<'a> {
        IoPadStrength(self)
    }
}
pub struct SdoPuEn<'a>(pub &'a mut PinCtrlVal);
impl<'a> SdoPuEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PinCtrlVal {
        self.0.0 &= !(!(val as u8) << 6);
        self.0
    }
    pub fn set_bit(self) -> &'a mut PinCtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PinCtrlVal {
        self.assign(false)
    }
}
pub struct IbhrPorEn<'a>(pub &'a mut PinCtrlVal);
impl<'a> IbhrPorEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 5) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut PinCtrlVal {
        self.0.0 &= !(!(val as u8) << 5);
        self.0
    }
    pub fn set_bit(self) -> &'a mut PinCtrlVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut PinCtrlVal {
        self.assign(false)
    }
}
pub struct IoPadStrength<'a>(pub &'a mut PinCtrlVal);
impl<'a> IoPadStrength<'a> {
    pub fn bits(&self) -> u8 {
        ((self.0.0 >> 0) & !(!0 << 2)) as u8
    }
    pub fn set(self, val: u8) -> &'a mut PinCtrlVal {
        self.0.0 &= !(!(!0 << 2) << 0);
        self.0.0 |= ((val as u8) & !(!0 << 2)) << 0;
        self.0
    }
}
