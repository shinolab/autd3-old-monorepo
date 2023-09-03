/*
 * File: mod_delay.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    operation::{Operation, TypeTag},
};

#[derive(Default)]
pub struct ModDelayOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for ModDelayOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);

        assert!(tx.len() >= 2 + device.num_transducers() * std::mem::size_of::<u16>());

        tx[0] = TypeTag::ModDelay as u8;

        unsafe {
            let dst = std::slice::from_raw_parts_mut(
                (&mut tx[2..]).as_mut_ptr() as *mut u16,
                device.num_transducers(),
            );
            dst.iter_mut()
                .zip(device.iter())
                .for_each(|(d, s)| *d = s.mod_delay());
        }

        Ok(2 + device.num_transducers() * std::mem::size_of::<u16>())
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<u16>()
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
    use rand::prelude::*;

    use super::*;
    use crate::geometry::{tests::create_device, LegacyTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn mod_delay() {
        let mut devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx =
            vec![0x00u8; (2 + NUM_TRANS_IN_UNIT * std::mem::size_of::<u16>()) * NUM_DEVICE];

        let mut rng = rand::thread_rng();
        devices.iter_mut().for_each(|dev| {
            dev.iter_mut().for_each(|tr| {
                tr.set_mod_delay(rng.gen_range(0x0000..0xFFFFu16));
            })
        });

        let mut op = ModDelayOp::default();

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());
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
                TypeTag::ModDelay as u8
            );
            tx.chunks(2)
                .skip((1 + NUM_TRANS_IN_UNIT) * dev.idx())
                .skip(1)
                .zip(dev.iter())
                .for_each(|(d, tr)| {
                    let delay = tr.mod_delay();
                    assert_eq!(d[0], (delay & 0xFF) as u8);
                    assert_eq!(d[1], (delay >> 8) as u8);
                })
        });
    }
}
