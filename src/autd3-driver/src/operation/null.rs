/*
 * File: null.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Geometry, Transducer},
    operation::Operation,
};

#[derive(Default)]
pub struct NullOp {}

impl<T: Transducer> Operation<T> for NullOp {
    fn pack(&mut self, _: &Device<T>, _: &mut [u8]) -> Result<usize, AUTDInternalError> {
        unreachable!()
    }

    fn required_size(&self, _: &Device<T>) -> usize {
        0
    }

    fn init(&mut self, _: &Geometry<T>) -> Result<(), AUTDInternalError> {
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
    use crate::geometry::{tests::create_geometry, LegacyTransducer};

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn null_op() {
        let geometry = create_geometry::<LegacyTransducer>(NUM_DEVICE, NUM_TRANS_IN_UNIT);

        let mut op = NullOp::default();

        assert!(op.init(&geometry).is_ok());

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.required_size(dev), 0));

        geometry
            .devices()
            .for_each(|dev| assert_eq!(op.remains(dev), 0));
    }
}
