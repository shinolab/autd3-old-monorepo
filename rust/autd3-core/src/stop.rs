/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{ConfigSilencer, Drive, GainAdvancedDuty};

use crate::{
    sendable::Sendable,
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

#[derive(Default)]
pub struct Stop {}

impl Stop {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Sendable<T> for Stop {
    type H = ConfigSilencer;
    type B = GainAdvancedDuty;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::new(4096, 10))
    }

    fn body_operation(&mut self, geometry: &Geometry<T>) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            vec![Drive { amp: 0., phase: 0. }; geometry.num_transducers()],
            vec![4096u16; geometry.num_transducers()],
        ))
    }
}
