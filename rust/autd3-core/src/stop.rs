/*
 * File: clear.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{ConfigSilencer, Drive, GainAdvancedDuty};

use crate::{error::AUTDInternalError, geometry::*, sendable::Sendable};

#[derive(Default)]
pub struct Stop {}

impl Stop {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(not(feature = "dynamic"))]
impl<T: Transducer> Sendable<T> for Stop {
    type H = ConfigSilencer;
    type B = GainAdvancedDuty;

    fn operation(self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::new(10),
            Self::B::new(
                vec![Drive { amp: 0., phase: 0. }; geometry.num_transducers()],
                vec![4096u16; geometry.num_transducers()],
            ),
        ))
    }
}

#[cfg(feature = "dynamic")]
impl Sendable for Stop {
    fn operation(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3_driver::Operation>,
            Box<dyn autd3_driver::Operation>,
        ),
        AUTDInternalError,
    > {
        Ok((
            Box::new(ConfigSilencer::new(10)),
            Box::new(GainAdvancedDuty::new(
                vec![Drive { amp: 0., phase: 0. }; geometry.num_transducers()],
                vec![4096u16; geometry.num_transducers()],
            )),
        ))
    }
}
