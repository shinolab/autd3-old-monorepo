/*
 * File: update_flag.rs
 * Project: src
 * Created Date: 09/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{NullBody, NullHeader};

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram to update flags (Force fan flag and reads FPGA info flag)
#[derive(Default)]
pub struct UpdateFlags {}

impl UpdateFlags {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for UpdateFlags {
    type H = NullHeader;
    type B = NullBody;

    fn operation(&self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::default()))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

    use super::*;

    #[test]
    fn test_update_flags() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let datagram = UpdateFlags::default();
        assert_eq!(
            <UpdateFlags as Datagram<LegacyTransducer>>::timeout(&datagram),
            None
        );
        datagram.operation(&geometry).unwrap();

        let datagram = UpdateFlags::default().with_timeout(Duration::from_millis(100));
        assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
        datagram.operation(&geometry).unwrap();
    }
}
