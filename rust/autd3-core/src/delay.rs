/*
 * File: delay.rs
 * Project: src
 * Created Date: 01/06/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    sendable::Sendable,
};

pub struct ModDelay {}

impl<T: Transducer> Sendable<T> for ModDelay {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::ModDelay;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Default::default())
    }

    fn body_operation(&mut self, geometry: &Geometry<T>) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            geometry.transducers().map(|tr| tr.mod_delay()).collect(),
        ))
    }
}
