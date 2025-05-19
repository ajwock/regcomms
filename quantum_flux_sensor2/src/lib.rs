mod who_am_i;
mod power_mode;
mod lepton_config;
mod quark_config;
mod boson_config;
mod lepton_data;
mod quark_data;
mod boson_data;
mod fifo_config;
use regcomms::{RegComms, RegCommsError};
pub enum AccessProc {
    Standard,
}
pub struct QuantumFluxSensor<C: RegComms<4, u32>>(pub C);
impl<C: RegComms<4, u32>> QuantumFluxSensor<C> {
    pub fn comms_read(&mut self, reg_address: u32, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_read(reg_address, buf)
    }
    pub fn comms_write(&mut self, reg_address: u32, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_write(reg_address, buf)
    }
    pub async fn comms_read_async(&mut self, reg_address: u32, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_read_async(reg_address, buf).await
    }
    pub async fn comms_write_async(&mut self, reg_address: u32, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {
        self.0.comms_write_async(reg_address, buf).await
    }
    pub fn who_am_i<'a>(&'a mut self) -> who_am_i::WhoAmI<'a, C> {
        who_am_i::WhoAmI(self)
    }
    pub fn power_mode<'a>(&'a mut self) -> power_mode::PowerMode<'a, C> {
        power_mode::PowerMode(self)
    }
    pub fn lepton_config<'a>(&'a mut self) -> lepton_config::LeptonConfig<'a, C> {
        lepton_config::LeptonConfig(self)
    }
    pub fn quark_config<'a>(&'a mut self) -> quark_config::QuarkConfig<'a, C> {
        quark_config::QuarkConfig(self)
    }
    pub fn boson_config<'a>(&'a mut self) -> boson_config::BosonConfig<'a, C> {
        boson_config::BosonConfig(self)
    }
    pub fn lepton_data<'a>(&'a mut self) -> lepton_data::LeptonData<'a, C> {
        lepton_data::LeptonData(self)
    }
    pub fn quark_data<'a>(&'a mut self) -> quark_data::QuarkData<'a, C> {
        quark_data::QuarkData(self)
    }
    pub fn boson_data<'a>(&'a mut self) -> boson_data::BosonData<'a, C> {
        boson_data::BosonData(self)
    }
    pub fn fifo_config<'a>(&'a mut self) -> fifo_config::FifoConfig<'a, C> {
        fifo_config::FifoConfig(self)
    }
}
