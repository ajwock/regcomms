use reg_comms::{RegCommsError, RegComms};
use crate::Lsm6Dsv;
pub struct FuncCfgAccess<'a, C: RegComms<1, u8>>(pub &'a mut Lsm6Dsv<C>);
impl<'a, C: RegComms<1, u8>> FuncCfgAccess<'a, C> {
    pub fn read(&mut self) -> Result<FuncCfgAccessVal, RegCommsError> {
        let mut buf = [0u8; 1];
        self.0.comms_read(0x1, &mut buf, crate::AccessProc::Standard)?;
        let val = u8::from_be_bytes(buf);
        Ok(FuncCfgAccessVal(val))
    }
    pub fn write(&mut self, val: FuncCfgAccessVal) -> Result<(), RegCommsError> {
        let buf = val.0.to_be_bytes();
        self.0.comms_write(0x1, &buf, crate::AccessProc::Standard)?;
        Ok(())
    }
}
pub struct FuncCfgAccessVal(pub u8);
impl FuncCfgAccessVal {
    pub fn get(&self) -> u8 {
        self.0
    }
    pub fn zero() -> Self {
         Self(0)
    }
    pub fn emb_func_reg_access<'a>(&'a mut self) -> EmbFuncRegAccess<'a> {
        EmbFuncRegAccess(self)
    }
    pub fn shub_reg_access<'a>(&'a mut self) -> ShubRegAccess<'a> {
        ShubRegAccess(self)
    }
    pub fn fsm_wr_ctrl_en<'a>(&'a mut self) -> FsmWrCtrlEn<'a> {
        FsmWrCtrlEn(self)
    }
    pub fn sw_por<'a>(&'a mut self) -> SwPor<'a> {
        SwPor(self)
    }
}
pub struct EmbFuncRegAccess<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> EmbFuncRegAccess<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 7) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(!(val as u8) << 7);
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
}
pub struct ShubRegAccess<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> ShubRegAccess<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 6) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(!(val as u8) << 6);
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
}
pub struct FsmWrCtrlEn<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> FsmWrCtrlEn<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 3) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(!(val as u8) << 3);
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
}
pub struct SwPor<'a>(pub &'a mut FuncCfgAccessVal);
impl<'a> SwPor<'a> {
    pub fn bit(&self) -> bool {
        ((self.0.0 >> 2) & 1) != 0
    }
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    pub fn assign(self, val: bool) -> &'a mut FuncCfgAccessVal {
        self.0.0 &= !(!(val as u8) << 2);
        self.0
    }
    pub fn set_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(true)
    }
    pub fn clear_bit(self) -> &'a mut FuncCfgAccessVal {
        self.assign(false)
    }
}
