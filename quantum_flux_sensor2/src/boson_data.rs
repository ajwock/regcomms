use regcomms::{RegCommsError, RegComms};
use crate::QuantumFluxSensor;
pub struct BosonData<'a, C: RegComms<4, u32>>(pub &'a mut QuantumFluxSensor<C>);
impl<'a, C: RegComms<4, u32>> BosonData<'a, C> {
    pub fn read(&mut self) -> Result<BosonDataVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read(0xff000004, &mut buf, crate::AccessProc::Standard)?;
        let val = u16::from_be_bytes(buf);
        Ok(BosonDataVal(val))
    }
    pub async fn read_async(&mut self) -> Result<BosonDataVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read_async(0xff000004, &mut buf, crate::AccessProc::Standard).await?;
        let val = u16::from_be_bytes(buf);
        Ok(BosonDataVal(val))
    }
}
pub struct BosonDataVal(pub u16);
impl BosonDataVal {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn data<'a>(&'a mut self) -> Data<'a> {
        Data(self)
    }
}
pub struct Data<'a>(pub &'a mut BosonDataVal);
impl<'a> Data<'a> {
    pub fn bits(&self) -> u16 {
        self.0.0
    }
}
