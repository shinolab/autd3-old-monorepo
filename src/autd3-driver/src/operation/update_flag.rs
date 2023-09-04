/*
 * File: null.rs
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

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Transducer},
    operation::{Operation, TypeTag},
};

#[derive(Default)]
pub struct UpdateFlagOp {
    remains: HashMap<usize, usize>,
}

impl<T: Transducer> Operation<T> for UpdateFlagOp {
    fn pack(&mut self, _: &Device<T>, tx: &mut [u8]) -> Result<usize, AUTDInternalError> {
        tx[0] = TypeTag::UpdateFlag as u8;
        Ok(2)
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        2
    }

    fn init(&mut self, _: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    fn remains(&self, _: &Device<T>) -> usize {
        0
    }

    fn commit(&mut self, _: &Device<T>) {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{tests::create_device, LegacyTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn null_op() {
        let devices = (0..NUM_DEVICE)
            .map(|i| create_device::<LegacyTransducer>(i, NUM_TRANS_IN_UNIT))
            .collect::<Vec<_>>();

        let mut op = NullOp::default();

        assert!(op.init(&devices.iter().collect::<Vec<_>>()).is_ok());

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.required_size(dev), 0));

        devices
            .iter()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));
    }
}
