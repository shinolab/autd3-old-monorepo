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

use crate::{
    datagram::*,
    error::AUTDInternalError,
    geometry::*,
    operation::{ConfigSilencerOp, StopOp},
};

/// Datagram to stop output
#[derive(Default)]
pub struct Stop {}

impl Stop {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for Stop {
    type O1 = ConfigSilencerOp;
    type O2 = StopOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(10), Self::O2::default()))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::time::Duration;

//     use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

//     use super::*;

//     #[test]
//     fn test_stop() {
//         let geometry = GeometryBuilder::<LegacyTransducer>::new()
//             .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
//             .build()
//             .unwrap();

//         let datagram = Stop::default();
//         assert_eq!(
//             <Stop as Datagram<LegacyTransducer>>::timeout(&datagram),
//             None
//         );
//         datagram.operation(&geometry).unwrap();

//         let datagram = Stop::default().with_timeout(Duration::from_millis(100));
//         assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
//         datagram.operation(&geometry).unwrap();
//     }
// }
