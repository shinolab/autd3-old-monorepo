/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::{DatagramHeader, Empty, Filled, Sendable},
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

#[derive(Default)]
pub struct Clear {}

impl Clear {
    pub fn new() -> Self {
        Self {}
    }
}

impl DatagramHeader for Clear {
    type O = autd3_driver::Clear;

    fn operation(&mut self) -> Result<Self::O, AUTDInternalError> {
        Ok(Default::default())
    }
}

impl<T: Transducer> Sendable<T> for Clear {
    type H = Filled;
    type B = Empty;
    type O = <Self as DatagramHeader>::O;

    fn operation(&mut self, _: &Geometry<T>) -> Result<Self::O, AUTDInternalError> {
        <Self as DatagramHeader>::operation(self)
    }
}
