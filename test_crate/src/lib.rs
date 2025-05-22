#![allow(dead_code)]
use regcomms::{RegCommsAddress, RegComms, RegCommsError};

pub struct MockedQuantumFluxComms {
    address_space: Vec<Vec<(u64, Vec<u8>)>>,
}

const BLK_SEL_W_ADDRESS: u64 = 0x100;
const MADDR_W_ADDRESS: u64 = 0x101;
const M_W_ADDRESS: u64 = 0x105;

const BLK_SEL_R_ADDRESS: u64 = 0x110;
const MADDR_R_ADDRESS: u64 = 0x111;
const M_R_ADDRESS: u64 = 0x115;

impl MockedQuantumFluxComms {
    fn new(v: Vec<Vec<(u64, Vec<u8>)>>) -> Self {
        Self {
            address_space: v,
        }
    }

    fn read_address_in_block(&self, address: u64, buf: &mut [u8], block_sel: usize) -> Result<usize, RegCommsError> {
        let Some((segaddr, seg)) = self.address_space[block_sel].iter().find(|(segaddr, seg)| address >= *segaddr && address < (*segaddr + seg.len() as u64)) else {
            return Err(RegCommsError::Other)
        };
        let seg_offset = (address - segaddr) as usize;
        let bytes_available = seg.len() - seg_offset as usize;
        let read_len = std::cmp::min(buf.len(), bytes_available);
        let read_end = seg_offset + read_len;
        let out_dst = &mut buf[0..read_len];
        let out_src = &seg[seg_offset..read_end];
        out_dst.copy_from_slice(out_src);
        Ok(read_len)

    }

    fn read_address(&self, address: u64, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        if address == M_R_ADDRESS {
            let mut blksel_buf = [0u8];
            let mut maddr_r_buf = [0u8; 4];
            self.read_address_in_block(BLK_SEL_R_ADDRESS, &mut blksel_buf, 0).unwrap();
            self.read_address_in_block(MADDR_R_ADDRESS, &mut maddr_r_buf, 0).unwrap();
            let maddress_r = u64_from_be_buf(&maddr_r_buf);
            self.read_address_in_block(maddress_r, buf, blksel_buf[0] as usize)
        } else {
            self.read_address_in_block(address, buf, 0)
        }
    }

    fn write_address_in_block(&mut self, address: u64, buf: &[u8], block_sel: usize) -> Result<usize, RegCommsError> {
        let Some((segaddr, seg)) = self.address_space[block_sel].iter_mut().find(|(segaddr, seg)| address >= *segaddr && address < (*segaddr + seg.len() as u64)) else {
            return Err(RegCommsError::Other)
        };
        let seg_offset = (address - *segaddr) as usize;
        let seg_len = seg.len();
        let bytes_available = seg_len - seg_offset as usize;
        let write_len = std::cmp::min(buf.len(), bytes_available);
        let write_end = seg_offset + write_len;
        let w_src  = &buf[0..write_len];
        let w_dst = &mut seg[seg_offset..write_end];
        w_dst.copy_from_slice(w_src);
        Ok(write_len)

    }

    fn write_address(&mut self, address: u64, buf: &[u8]) -> Result<usize, RegCommsError> {
        if address == M_W_ADDRESS {
            let mut blksel_buf = [0u8];
            let mut maddr_w_buf = [0u8; 4];
            self.read_address_in_block(BLK_SEL_W_ADDRESS, &mut blksel_buf, 0).unwrap();
            self.read_address_in_block(MADDR_W_ADDRESS, &mut maddr_w_buf, 0).unwrap();
            let maddress_w = u64_from_be_buf(&maddr_w_buf);
            self.write_address_in_block(maddress_w, buf, blksel_buf[0] as usize)
        } else {
            self.write_address_in_block(address, buf, 0)
        }
    }
}

fn u64_from_be_buf(buf: &[u8]) -> u64 {
    u64_from_le_iter(buf.iter().rev())
}

fn u64_from_le_iter<'a, I: IntoIterator<Item = &'a u8>>(it: I) -> u64 {
    it.into_iter().enumerate().fold(0, |acc, (index, &byte)| acc + byte as u64 * 256u64.pow(index as u32))
}

fn u64_from_regcommaddress<const N: usize, R: RegCommsAddress<N>>(num: R) -> u64 {
    let reg_arr = num.to_little_endian();
    let reg_addr_slc = reg_arr.as_slice();
    u64_from_le_iter(reg_addr_slc)
}

impl<const N: usize, R: RegCommsAddress<N>> RegComms<N, R> for MockedQuantumFluxComms {
    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<(), RegCommsError> {
        let u64_address = u64_from_regcommaddress(reg_address);
        self.read_address(u64_address, buf).map(|_| ())
    }

    fn comms_write(&mut self, reg_address: R, buf: &[u8]) -> Result<(), RegCommsError> {
        let u64_address = u64_from_regcommaddress(reg_address);
        self.write_address(u64_address, buf).map(|_| ())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quantum_flux_sensor::QuantumFluxSensor;

    #[test]
    fn test_alternative_access_proc() {
        let comm_peripheral = MockedQuantumFluxComms::new(vec![vec![(0x1, vec![0x0]), (0x16, vec![0xe0, 0xe0, 0xe0]), (0x20, vec![0xe3]), (0x100, vec![0x00; 6]), (0x110, vec![0x00; 6])], vec![(0x1, vec![0x55])]]);
        let mut sensor = QuantumFluxSensor::new(comm_peripheral);
        let mut fifo_config5 = sensor.fifo_config5().read().unwrap();
        assert_eq!(fifo_config5.get(), 0b01010101);
        fifo_config5.fifo_20_bit_ext().set_bit();
        assert_eq!(fifo_config5.get(), 0b11010101);
        fifo_config5.fifo_excludes().set(0);
        assert_eq!(fifo_config5.get(), 0b11000000);
        sensor.fifo_config5().modify(|mut val| {
            val.fifo_20_bit_ext().set_bit()
                .fifo_excludes().set(0);
            val
        }).unwrap();
        let fifo_config5 = sensor.fifo_config5().read().unwrap();
        assert_eq!(fifo_config5.get(), 0b11000000);
    }

    #[test]
    fn test_quantum_flux_sensor() {
        let comm_peripheral = MockedQuantumFluxComms::new(vec![vec![(0x1, vec![0x0]), (0x16, vec![0xe0, 0xe0, 0xe0]), (0x20, vec![0xe3])]]);
        let mut sensor = QuantumFluxSensor::new(comm_peripheral);
        let mut power_mode = sensor.power_mode().read().unwrap();
        assert_eq!(power_mode.pulsed().bit_is_set(), false);
        assert_eq!(power_mode.poweron_mode().bits(), 0);
        let mut fifo_config = sensor.fifo_config().read().unwrap();
        assert_eq!(fifo_config.fifo_src().bits(), 0x7);
        assert_eq!(fifo_config.fifo_en().bit_is_set(), false);
        assert_eq!(fifo_config.fifo_fmt().bits(), 0x3);
        assert_eq!(fifo_config.get(), 0b11100011);
        fifo_config.fifo_src().set(0x5);
        assert_eq!(fifo_config.get(), 0b10100011);
        fifo_config.fifo_fmt().set(0);
        assert_eq!(fifo_config.get(), 0b10100000);
        // We should just casually ignore the over-step here
        fifo_config.fifo_fmt().set(0xff);
        assert_eq!(fifo_config.get(), 0b10100011);
        fifo_config.fifo_en().set_bit();
        assert_eq!(fifo_config.get(), 0b10100111);
        fifo_config.fifo_fmt().set(0);
        assert_eq!(fifo_config.get(), 0b10100100);
        fifo_config.fifo_fmt().reset();
        assert_eq!(fifo_config.get(), 0b10100111);
        fifo_config.fifo_src().set(0b010);
        assert_eq!(fifo_config.get(), 0b01000111);
        fifo_config.fifo_fmt().set(0b10);
        assert_eq!(fifo_config.get(), 0b01000110);
        fifo_config.fifo_src().reset();
        assert_eq!(fifo_config.get(), 0b11100110);
        fifo_config.fifo_en().reset();
        assert_eq!(fifo_config.get(), 0b11100010);
        fifo_config.fifo_decimation().set(0b11);
        assert_eq!(fifo_config.get(), 0b11111010);
        fifo_config.fifo_decimation().reset();
        assert_eq!(fifo_config.get(), 0b11100010);
        fifo_config.set(0);
        assert_eq!(fifo_config.get(), 0);
        sensor.fifo_config().write(fifo_config).unwrap();
        let fifo_config = sensor.fifo_config().read().unwrap();
        assert_eq!(fifo_config.get(), 0);
        sensor.fifo_config().reset().unwrap();
        let fifo_config = sensor.fifo_config().read().unwrap();
        assert_eq!(fifo_config.get(), 0b11100011);
   }


    async fn embassy_test() {
        let comm_peripheral = MockedQuantumFluxComms::new(vec![vec![(0x1, vec![0x0]), (0x16, vec![0xe0, 0xe0, 0xe0]), (0x20, vec![0xe3])]]);
        let mut sensor = QuantumFluxSensor::new(comm_peripheral);
        let mut power_mode = sensor.power_mode().read_async().await.unwrap();
        assert_eq!(power_mode.pulsed().bit_is_set(), false);
        assert_eq!(power_mode.poweron_mode().bits(), 0);
        let mut fifo_config = sensor.fifo_config().read_async().await.unwrap();
        assert_eq!(fifo_config.fifo_src().bits(), 0x7);
        assert_eq!(fifo_config.fifo_en().bit_is_set(), false);
        assert_eq!(fifo_config.fifo_fmt().bits(), 0x3);
        assert_eq!(fifo_config.get(), 0b11100011);
        fifo_config.fifo_src().set(0x5);
        assert_eq!(fifo_config.get(), 0b10100011);
        fifo_config.fifo_fmt().set(0);
        assert_eq!(fifo_config.get(), 0b10100000);
        // We should just casually ignore the over-step here
        fifo_config.fifo_fmt().set(0xff);
        assert_eq!(fifo_config.get(), 0b10100011);
        fifo_config.fifo_en().set_bit();
        assert_eq!(fifo_config.get(), 0b10100111);
    }

    use embassy_sync::signal::Signal;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use core::future::Future;
    use core::task::Poll;
    async fn signal_pass<F: FnOnce() -> Fut, Fut: Future<Output = ()>>(signal: &'static Signal<CriticalSectionRawMutex, ()>, test_fn: F) {
        test_fn().await;
        signal.signal(());
    }

    #[embassy_executor::task]
    async fn embassy_test_task(signal: &'static Signal<CriticalSectionRawMutex, ()>) {
        signal_pass(&signal, async || embassy_test().await).await;
    }

    #[test]
    fn embassy_concurrency_test() {
        static TEST_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
        let handle = std::thread::spawn(|| {
            let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
            executor.run(|spawner| {
                spawner.spawn(embassy_test_task(&TEST_SIGNAL)).unwrap();
            });
        });
        loop {
            match embassy_futures::poll_once(TEST_SIGNAL.wait()) {
                Poll::Ready(()) => break,
                Poll::Pending => (),
            }
            if handle.is_finished() {
                match handle.join() {
                    Ok(()) => panic!("Did not recieve pass signal but executor thread joined anyway"),
                    Err(e) => std::panic::resume_unwind(e),
                }
            }
        }
    }
}
