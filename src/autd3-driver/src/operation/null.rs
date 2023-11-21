/*
 * File: null.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    error::AUTDInternalError,
    geometry::{Device, Geometry},
    operation::Operation,
};

#[derive(Default)]
pub struct NullOp {}

impl Operation for NullOp {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn pack(&mut self, _: &Device, _: &mut [u8]) -> Result<usize, AUTDInternalError> {
        unreachable!()
    }

    fn required_size(&self, _: &Device) -> usize {
        0
    }

    fn init(&mut self, _: &Geometry) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    fn remains(&self, _: &Device) -> usize {
        0
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn commit(&mut self, _: &Device) {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::tests::create_geometry;

    const NUM_TRANS_IN_UNIT: usize = 249;
    const NUM_DEVICE: usize = 10;

    #[test]
    fn null_op() {
        let geometry = create_geometry(NUM_DEVICE, NUM_TRANS_IN_UNIT);

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
