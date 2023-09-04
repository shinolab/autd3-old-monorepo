/*
 * File: stop.rs
 * Project: operation
 * Created Date: 01/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    defined::Drive,
    error::AUTDInternalError,
    fpga::AdvancedDriveDuty,
    geometry::{Device, Transducer},
    operation::{GainControlFlags, TypeTag},
};

use super::Operation;

#[derive(Default)]
pub struct StopOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for StopOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::Gain as u8;
        tx[1] = GainControlFlags::DUTY.bits();

        assert!(
            tx.len() >= 2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>()
        );

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                (&mut tx[2..]).as_mut_ptr() as *mut AdvancedDriveDuty,
                device.num_transducers(),
            );
            dst.iter_mut()
                .zip(device.iter().map(|tr| tr.cycle()))
                .for_each(|(d, c)| d.set(&Drive { amp: 0., phase: 0. }, c));
        }

        Ok(2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>())
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<AdvancedDriveDuty>()
    }

    fn init(&mut self, devices: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        self.remains = devices.iter().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains.insert(device.idx(), 0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::{device::tests::create_device, LegacyTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn stop_op() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx =
            vec![0x00u8; (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) * NUM_DEVICE];

        let mut op = StopOp::default();

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices.iter().for_each(|dev| {
            assert_eq!(
                op.required_size(dev),
                2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<AdvancedDriveDuty>()
            )
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 1));

        devices.iter().for_each(|dev| {
            assert!(op
                .pack(
                    dev,
                    &mut tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>())..]
                )
                .is_ok());
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>())],
                TypeTag::Gain as u8
            );
            let flag = GainControlFlags::from_bits_truncate(
                tx[dev.idx() * (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) + 1],
            );
            assert!(!flag.contains(GainControlFlags::LEGACY));
            assert!(flag.contains(GainControlFlags::DUTY));
            tx.iter()
                .skip((2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) * dev.idx())
                .skip(2)
                .take(NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>())
                .for_each(|&d| {
                    assert_eq!(d, 0);
                })
        });
    }
}
