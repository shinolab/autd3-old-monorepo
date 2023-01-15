/*
 * File: delay.rs
 * Project: src
 * Created Date: 01/06/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::{DatagramBody, Empty, Filled, Sendable},
    geometry::{Geometry, Transducer},
};
use anyhow::Result;

pub struct ModDelay {}

impl<T: Transducer> DatagramBody<T> for ModDelay {
    type O = autd3_driver::ModDelay;

    fn operation(&mut self, geometry: &Geometry<T>) -> Result<Self::O> {
        let delays = geometry.transducers().map(|tr| tr.mod_delay()).collect();
        Ok(Self::O::new(delays))
    }
}

impl<T: Transducer> Sendable<T> for ModDelay {
    type H = Empty;
    type B = Filled;
    type O = <Self as DatagramBody<T>>::O;

    fn operation(&mut self, geometry: &Geometry<T>) -> Result<Self::O> {
        <Self as DatagramBody<T>>::operation(self, geometry)
    }
}
