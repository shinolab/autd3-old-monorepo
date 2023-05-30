/*
 * File: delay.rs
 * Project: src
 * Created Date: 01/06/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

#[derive(Default)]
pub struct ModDelay {}

impl ModDelay {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for ModDelay {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::ModDelay;

    fn operation(
        &mut self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            Self::B::new(geometry.transducers().map(|tr| tr.mod_delay()).collect()),
        ))
    }
}
