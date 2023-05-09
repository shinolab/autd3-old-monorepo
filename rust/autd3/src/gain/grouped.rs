/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{
    sendable::Sendable,
    error::AUTDInternalError,
    gain::Gain,
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Geometry, LegacyTransducer, Transducer,
    },
    Drive,
};

use crate::error::AUTDError;

/// Gain to produce single focal point
#[derive(Default)]
pub struct Grouped<'a, T: Transducer> {
    gain_map: HashMap<usize, Box<dyn Gain<T> + 'a>>,
}

impl<'a, T: Transducer> Grouped<'a, T> {
    /// constructor
    pub fn new() -> Self {
        Self {
            gain_map: HashMap::new(),
        }
    }

    pub fn add<G: 'a + Gain<T>>(&mut self, id: usize, gain: G) -> Result<(), AUTDError> {
        self.gain_map.insert(id, Box::new(gain));
        Ok(())
    }
}

impl<'a, T: Transducer> Gain<T> for Grouped<'a, T> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok((0..geometry.num_devices())
            .flat_map(|i| {
                self.gain_map
                    .get_mut(&i)
                    .and_then(|g| match g.calc(geometry) {
                        Ok(g) => {
                            let start = if i == 0 {
                                0
                            } else {
                                geometry.device_map()[i - 1]
                            };
                            let end = start + geometry.device_map()[i];
                            Some(g[start..end].to_vec())
                        }
                        Err(_) => None,
                    })
                    .unwrap_or_else(|| {
                        vec![
                            Drive {
                                phase: 0.0,
                                amp: 0.0,
                            };
                            geometry.device_map()[i]
                        ]
                    })
            })
            .collect())
    }
}

impl<'a> Sendable<LegacyTransducer> for Grouped<'a, LegacyTransducer> {
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainLegacy;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<LegacyTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(self.calc(geometry)?))
    }
}

impl<'a> Sendable<AdvancedTransducer> for Grouped<'a, AdvancedTransducer> {
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainAdvanced;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            self.calc(geometry)?,
            geometry.transducers().map(|tr| tr.cycle()).collect(),
        ))
    }
}

impl<'a> Sendable<AdvancedPhaseTransducer> for Grouped<'a, AdvancedPhaseTransducer> {
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainAdvancedPhase;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            self.calc(geometry)?,
            geometry.transducers().map(|tr| tr.cycle()).collect(),
        ))
    }
}
