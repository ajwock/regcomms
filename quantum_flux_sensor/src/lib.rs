#![no_std]
use core::result::Result;
use core::default::Default;
mod who_am_i;
mod power_mode;
mod lepton_config;
mod quark_config;
mod boson_config;
mod lepton_data;
mod quark_data;
mod boson_data;
mod fifo_config;
mod blk_sel_w;
mod maddr_w;
mod m_w;
mod blk_sel_r;
mod maddr_r;
mod m_r;
mod fifo_config5;
mod handwritten;
use regcomms::{RegComms, RegCommsError, RegCommsAccessProc};
use spin::once::Once;
#[derive(Default)]
pub struct StandardAccessProc;
impl<Comms: RegComms<4, u32>> RegCommsAccessProc<QuantumFluxSensor<Comms>, 4, u32> for StandardAccessProc {
    fn proc_read(&self, peripheral: &mut QuantumFluxSensor<Comms>, reg_address: u32, buf: &mut [u8]) -> Result<(), RegCommsError> {
        peripheral.comms.comms_read(reg_address, buf)
    }
    async fn proc_read_async(&self, peripheral: &mut QuantumFluxSensor<Comms>, reg_address: u32, buf: &mut [u8]) -> Result<(), RegCommsError> {
        peripheral.comms.comms_read_async(reg_address, buf).await
    }
    fn proc_write(&self, peripheral: &mut QuantumFluxSensor<Comms>, reg_address: u32, buf: &[u8]) -> Result<(), RegCommsError> {
        peripheral.comms.comms_write(reg_address, buf)
    }
    async fn proc_write_async(&self, peripheral: &mut QuantumFluxSensor<Comms>, reg_address: u32, buf: &[u8]) -> Result<(), RegCommsError> {
        peripheral.comms.comms_write_async(reg_address, buf).await
    }
}
static MREG_1: Once<crate::handwritten::Mreg1> = Once::new();
static STANDARD: Once<StandardAccessProc> = Once::new();
pub struct QuantumFluxSensor<Comms: RegComms<4, u32>> {
    comms: Comms,
    mreg_1: &'static crate::handwritten::Mreg1,
    standard: &'static StandardAccessProc,
}
impl<Comms: RegComms<4, u32>> QuantumFluxSensor<Comms> {
    pub fn new(comms: Comms) -> Self {
        Self {
             comms,
            mreg_1: MREG_1.call_once(|| Default::default()),
            standard: STANDARD.call_once(|| Default::default()),
        }
    }
    pub fn who_am_i<'a>(&'a mut self) -> who_am_i::WhoAmI<'a, Comms> {
        who_am_i::WhoAmI(self)
    }
    pub fn power_mode<'a>(&'a mut self) -> power_mode::PowerMode<'a, Comms> {
        power_mode::PowerMode(self)
    }
    pub fn lepton_config<'a>(&'a mut self) -> lepton_config::LeptonConfig<'a, Comms> {
        lepton_config::LeptonConfig(self)
    }
    pub fn quark_config<'a>(&'a mut self) -> quark_config::QuarkConfig<'a, Comms> {
        quark_config::QuarkConfig(self)
    }
    pub fn boson_config<'a>(&'a mut self) -> boson_config::BosonConfig<'a, Comms> {
        boson_config::BosonConfig(self)
    }
    pub fn lepton_data<'a>(&'a mut self) -> lepton_data::LeptonData<'a, Comms> {
        lepton_data::LeptonData(self)
    }
    pub fn quark_data<'a>(&'a mut self) -> quark_data::QuarkData<'a, Comms> {
        quark_data::QuarkData(self)
    }
    pub fn boson_data<'a>(&'a mut self) -> boson_data::BosonData<'a, Comms> {
        boson_data::BosonData(self)
    }
    pub fn fifo_config<'a>(&'a mut self) -> fifo_config::FifoConfig<'a, Comms> {
        fifo_config::FifoConfig(self)
    }
    pub fn blk_sel_w<'a>(&'a mut self) -> blk_sel_w::BlkSelW<'a, Comms> {
        blk_sel_w::BlkSelW(self)
    }
    pub fn maddr_w<'a>(&'a mut self) -> maddr_w::MaddrW<'a, Comms> {
        maddr_w::MaddrW(self)
    }
    pub fn m_w<'a>(&'a mut self) -> m_w::MW<'a, Comms> {
        m_w::MW(self)
    }
    pub fn blk_sel_r<'a>(&'a mut self) -> blk_sel_r::BlkSelR<'a, Comms> {
        blk_sel_r::BlkSelR(self)
    }
    pub fn maddr_r<'a>(&'a mut self) -> maddr_r::MaddrR<'a, Comms> {
        maddr_r::MaddrR(self)
    }
    pub fn m_r<'a>(&'a mut self) -> m_r::MR<'a, Comms> {
        m_r::MR(self)
    }
    pub fn fifo_config5<'a>(&'a mut self) -> fifo_config5::FifoConfig5<'a, Comms> {
        fifo_config5::FifoConfig5(self)
    }
}
