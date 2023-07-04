/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{ConfigSilencer, Drive, GainAdvancedDuty};

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

#[derive(Default)]
pub struct Stop {}

impl Stop {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for Stop {
    type H = ConfigSilencer;
    type B = GainAdvancedDuty;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::new(10),
            <Self::B as autd3_driver::operation::GainOp>::new(
                vec![Drive { amp: 0., phase: 0. }; geometry.num_transducers()],
                || vec![4096u16; geometry.num_transducers()],
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

    use super::*;

    #[test]
    fn test_stop() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let mut datagram = Stop::default();
        assert_eq!(
            <Stop as Datagram<LegacyTransducer>>::timeout(&datagram),
            None
        );
        datagram.operation(&geometry).unwrap();

        let mut datagram = Stop::default().with_timeout(Duration::from_millis(100));
        assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
        datagram.operation(&geometry).unwrap();
    }
}
