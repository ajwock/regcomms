use regcomms::{RegCommsError, RegComms};
use crate::QuantumFluxSensor;
pub struct WhoAmI<'a, C: RegComms<4, u32>>(pub &'a mut QuantumFluxSensor<C>);
impl<'a, C: RegComms<4, u32>> WhoAmI<'a, C> {
    pub fn read(&mut self) -> Result<WhoAmIVal, RegCommsError> {
        let mut buf = [0u8; 4];
        self.0.comms_read(0xffffff08, &mut buf, crate::AccessProc::Standard)?;
        let val = u32::from_be_bytes(buf);
        Ok(WhoAmIVal(val))
    }
    pub async fn read_async(&mut self) -> Result<WhoAmIVal, RegCommsError> {
        let mut buf = [0u8; 4];
        self.0.comms_read_async(0xffffff08, &mut buf, crate::AccessProc::Standard).await?;
        let val = u32::from_be_bytes(buf);
        Ok(WhoAmIVal(val))
    }
}
pub struct WhoAmIVal(pub u32);
impl WhoAmIVal {
    pub fn get(&self) -> u32 {
        self.0
    }
    pub fn id<'a>(&'a mut self) -> Id<'a> {
        Id(self)
    }
}
pub struct Id<'a>(pub &'a mut WhoAmIVal);
impl<'a> Id<'a> {
    pub fn bits(&self) -> u32 {
        self.0.0
    }
}
