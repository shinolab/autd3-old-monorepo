/*
 * File: synchronize.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::NullHeader;

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

#[derive(Default)]
pub struct Synchronize {}

impl Synchronize {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for Synchronize {
    type H = NullHeader;
    type B = T::Sync;

    fn operation(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Default::default(),
            <Self::B as autd3_driver::SyncOp>::new(|| {
                geometry.transducers().map(|tr| tr.cycle()).collect()
            }),
        ))
    }

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

    use super::*;

    #[test]
    fn test_sync() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let mut datagram = Synchronize::new();
        assert_eq!(
            <Synchronize as Datagram<LegacyTransducer>>::timeout(&datagram),
            Some(Duration::from_millis(200))
        );
        datagram.operation(&geometry).unwrap();

        let mut datagram = Synchronize::new().with_timeout(Duration::from_millis(100));
        assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
        datagram.operation(&geometry).unwrap();
    }
}
