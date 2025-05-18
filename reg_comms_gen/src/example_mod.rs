mod my_reg;
mod my_other_reg;
use reg_comms::{RegComms, RegCommsError};

pub enum AccessProc {
    Standard,
};

struct MyPeripheral<C: RegComms>(C);

impl<C: RegComms> MyPeripheral {
    pub fn comms_read(&mut self, address: u64, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        // TODO, access proc stuff
        self.0.read(address, buf)
    }

    pub fn comms_write(&mut self, address: u64, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.write(address, buf)
    }

    pub fn my_reg<'a>(&'a mut self) -> my_reg::MyReg<'a, C> {
        my_reg::MyReg(self)
    }

    pub fn my_other_reg<'a>(&'a mut self) -> my_other_reg::MyOtherReg<'a, C> {
        my_other_reg::MyOtherReg(self)
    }
}
