mod func_cfg_access;
mod pin_ctrl;
use reg_comms::{RegComms, RegCommsError};
pub enum AccessProc {
    Standard,
}
pub struct Lsm6Dsv<C: RegComms<1, u8>>(C);
impl<C: RegComms<1, u8>> Lsm6Dsv<C> {
    pub fn comms_read(&mut self, reg_address: u8, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_read(reg_address, buf)
    }
    pub fn comms_write(&mut self, reg_address: u8, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_write(reg_address, buf)
    }
    pub fn func_cfg_access<'a>(&'a mut self) -> func_cfg_access::FuncCfgAccess<'a, C> {
        func_cfg_access::FuncCfgAccess(self)
    }
    pub fn pin_ctrl<'a>(&'a mut self) -> pin_ctrl::PinCtrl<'a, C> {
        pin_ctrl::PinCtrl(self)
    }
}
