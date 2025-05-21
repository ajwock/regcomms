#![allow(dead_code)]
use regcomms::{RegCommsAddress, RegComms, RegCommsError};

pub struct MockedComms {
    address_space: Vec<(u64, Vec<u8>)>,
}

impl MockedComms {
    fn new(v: Vec<(u64, Vec<u8>)>) -> Self {
        Self {
            address_space: v,
        }
    }

    fn read_address(&self, address: u64, buf: &mut [u8]) -> Result<usize, RegCommsError> {
        let Some((segaddr, seg)) = self.address_space.iter().find(|(segaddr, seg)| address >= *segaddr && address < (*segaddr + seg.len() as u64)) else {
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
}

impl<const N: usize, R: RegCommsAddress<N>> RegComms<N, R> for MockedComms {
    fn comms_read(&mut self, reg_address: R, buf: &mut [u8]) -> Result<(), RegCommsError> {
        let reg_arr = reg_address.to_little_endian();
        let reg_addr_slc = reg_arr.as_slice();
        let u64_address = reg_addr_slc.into_iter().enumerate().fold(0, |acc, (index, &byte)| acc + byte as u64 * 256u64.pow(index as u32));
        self.read_address(u64_address, buf).map(|_| ())
    }

    fn comms_write(&mut self, _reg_address: R, _buf: &[u8]) -> Result<(), RegCommsError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quantum_flux_sensor::QuantumFluxSensor;

    #[test]
    fn test_quantum_flux_sensor() {
        let comm_peripheral = MockedComms::new(vec![(0x1, vec![0x0]), (0x16, vec![0xe0, 0xe0, 0xe0]), (0x20, vec![0xe3])]);
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
   }


    async fn embassy_test() {
        let comm_peripheral = MockedComms::new(vec![(0x1, vec![0x0]), (0x16, vec![0xe0, 0xe0, 0xe0]), (0x20, vec![0xe3])]);
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
