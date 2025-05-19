use regcomms::{RegCommsError, RegComms};
use crate::QuantumFluxSensor;
pub struct QuarkData<'a, C: RegComms<4, u32>>(pub &'a mut QuantumFluxSensor<C>);
impl<'a, C: RegComms<4, u32>> QuarkData<'a, C> {
    pub fn read(&mut self) -> Result<QuarkDataVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read(0xff000002, &mut buf, crate::AccessProc::Standard)?;
        let val = u16::from_be_bytes(buf);
        Ok(QuarkDataVal(val))
    }
    pub async fn read_async(&mut self) -> Result<QuarkDataVal, RegCommsError> {
        let mut buf = [0u8; 2];
        self.0.comms_read_async(0xff000002, &mut buf, crate::AccessProc::Standard).await?;
        let val = u16::from_be_bytes(buf);
        Ok(QuarkDataVal(val))
    }
}
pub struct QuarkDataVal(pub u16);
impl QuarkDataVal {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn data<'a>(&'a mut self) -> Data<'a> {
        Data(self)
    }
}
pub struct Data<'a>(pub &'a mut QuarkDataVal);
impl<'a> Data<'a> {
    pub fn bits(&self) -> u16 {
        self.0.0
    }
}
