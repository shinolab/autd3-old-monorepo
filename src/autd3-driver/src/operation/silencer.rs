/*
 * File: silencer.rs
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

pub struct ConfigSilencerOp {
    remains: HashMap<usize, usize>,
    step: u16,
}

impl ConfigSilencerOp {
    pub fn new(step: u16) -> Self {
        Self {
            remains: Default::default(),
            step,
        }
    }
}

impl<T: Transducer> Operation<T> for ConfigSilencerOp {
    fn pack(&mut self, device: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);
        tx[0] = TypeTag::Silencer as u8;
        tx[2] = (self.step & 0xFF) as u8;
        tx[3] = (self.step >> 8) as u8;
        Ok(4)
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        4
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
    use crate::geometry::{tests::create_device, LegacyTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn silencer_op() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut tx = vec![0x00u8; 4 * NUM_DEVICE];

        let mut op = ConfigSilencerOp::new(0x1234);

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 4));

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 1));

        devices.iter().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 4..]).is_ok());
            op.commit(dev);
        });

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        devices.iter().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 4], TypeTag::Silencer as u8);
            assert_eq!(tx[dev.idx() * 4 + 1], 0);
            assert_eq!(tx[dev.idx() * 4 + 2], 0x34);
            assert_eq!(tx[dev.idx() * 4 + 3], 0x12);
        });
    }
}
