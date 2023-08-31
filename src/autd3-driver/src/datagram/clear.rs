/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram for clear all data in devices
#[derive(Default)]
pub struct Clear {}

impl Clear {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for Clear {
    type O1 = crate::operation::ClearOp;
    type O2 = crate::operation::NullOp;

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }

    fn operation(&self, _: &Geometry<T>) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::time::Duration;

//     use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

//     use super::*;

//     #[test]
//     fn test_clear() {
//         let geometry = GeometryBuilder::<LegacyTransducer>::new()
//             .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
//             .build()
//             .unwrap();

//         let datagram = Clear::new();
//         assert_eq!(
//             <Clear as Datagram<LegacyTransducer>>::timeout(&datagram),
//             Some(Duration::from_millis(200))
//         );
//         datagram.operation(&geometry).unwrap();

//         let datagram = Clear::new().with_timeout(Duration::from_millis(100));
//         assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
//         datagram.operation(&geometry).unwrap();
//     }
// }
