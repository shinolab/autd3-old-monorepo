/*
 * File: amplitude.rs
 * Project: src
 * Created Date: 07/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{float, Drive};

use crate::{error::AUTDInternalError, geometry::*, sendable::Sendable};

pub struct Amplitudes {
    amp: float,
}

impl Amplitudes {
    pub fn uniform(amp: float) -> Self {
        Self { amp }
    }

    pub fn none() -> Self {
        Self::uniform(0.0)
    }
}

#[cfg(not(feature = "dynamic"))]
impl Sendable<AdvancedPhaseTransducer> for Amplitudes {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::GainAdvancedDuty;

    fn operation(
        self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            Self::B::new(
                vec![
                    Drive {
                        phase: 0.0,
                        amp: self.amp,
                    };
                    geometry.num_transducers()
                ],
                geometry.transducers().map(|tr| tr.cycle()).collect(),
            ),
        ))
    }
}

#[cfg(feature = "dynamic")]
impl Sendable for Amplitudes {
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
            Box::new(autd3_driver::NullHeader::default()),
            Box::new(autd3_driver::GainAdvancedDuty::new(
                vec![
                    Drive {
                        phase: 0.0,
                        amp: self.amp,
                    };
                    geometry.num_transducers()
                ],
                geometry.transducers().map(|tr| tr.cycle()).collect(),
            )),
        ))
    }
}
