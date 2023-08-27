/*
 * File: delay.rs
 * Project: src
 * Created Date: 01/06/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram to set modulation delay
#[derive(Default)]
pub struct ModDelay {}

impl ModDelay {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for ModDelay {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::ModDelay;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            Self::B::new(geometry.transducers().map(|tr| tr.mod_delay()).collect()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

    use super::*;

    #[test]
    fn test_mod_dealy() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let datagram = ModDelay::new();
        assert_eq!(
            <ModDelay as Datagram<LegacyTransducer>>::timeout(&datagram),
            None
        );
        datagram.operation(&geometry).unwrap();

        let datagram = ModDelay::new().with_timeout(Duration::from_millis(100));
        assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
        datagram.operation(&geometry).unwrap();
    }
}
