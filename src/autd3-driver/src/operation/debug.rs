/*
 * File: debug.rs
 * Project: operation
 * Created Date: 21/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Geometry},
    operation::{Operation, TypeTag},
};

pub struct DebugOutIdxOp {
    remains: HashMap<usize, usize>,
    idx: HashMap<usize, usize>,
}

impl DebugOutIdxOp {
    pub fn new(idx: HashMap<usize, usize>) -> Self {
        Self {
            remains: Default::default(),
            idx,
        }
    }
}

impl Operation for DebugOutIdxOp {
    fn pack(&mut self, device: &Device, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        assert_eq!(self.remains[&device.idx()], 1);
        tx[0] = TypeTag::Debug as u8;
        tx[2] = self.idx.get(&device.idx()).cloned().unwrap_or(0) as u8;
        Ok(4)
    }

    fn required_size(&self, _: &Device) -> usize {
        4
    }

    fn init(&mut self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        self.remains = geometry.devices().map(|device| (device.idx(), 1)).collect();
        Ok(())
    }

    fn remains(&self, device: &Device) -> usize {
        self.remains[&device.idx()]
    }

    fn commit(&mut self, device: &Device) {
        self.remains.insert(device.idx(), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::tests::create_geometry;

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn silencer_op() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut tx = [0x00u8; 4 * NUM_DEVICE];

        let mut op = DebugOutIdxOp::new((0..NUM_DEVICE).map(|i| (i, i)).collect());

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 4));

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 1));

        geometry.devices().for_each(|dev| {
            assert!(op.pack(dev, &mut tx[dev.idx() * 4..]).is_ok());
            op.commit(dev);
        });

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));

        geometry.devices().for_each(|dev| {
            assert_eq!(tx[dev.idx() * 4], TypeTag::Debug as u8);
            assert_eq!(tx[dev.idx() * 4 + 1], 0x00);
            assert_eq!(tx[dev.idx() * 4 + 2], dev.idx() as u8);
            assert_eq!(tx[dev.idx() * 4 + 3], 0x00);
        });
    }
}
