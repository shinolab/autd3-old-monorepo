/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{error::AUTDInternalError, gain::Gain, geometry::*, sendable::Sendable, Drive};

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
}

impl<'a, T: Transducer> Grouped<'a, T> {
    pub fn add<G: 'a + Gain<T>>(&mut self, id: usize, gain: G) {
        self.gain_map.insert(id, Box::new(gain));
    }

    pub fn add_boxed(&mut self, id: usize, gain: Box<dyn Gain<T>>) {
        self.gain_map.insert(id, gain);
    }
}

impl<'a, T: Transducer> Gain<T> for Grouped<'a, T> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let mut drives = HashMap::new();
        for (i, gain) in &mut self.gain_map {
            let d = gain.calc(geometry)?;
            drives.insert(i, d);
        }

        Ok((0..geometry.num_devices())
            .flat_map(|i| {
                drives
                    .get_mut(&i)
                    .map(|g| {
                        let start = if i == 0 {
                            0
                        } else {
                            geometry.device_map()[i - 1]
                        };
                        let end = start + geometry.device_map()[i];
                        g[start..end].to_vec()
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

    fn operation(
        &mut self,
        geometry: &Geometry<LegacyTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::default(), Self::B::new(self.calc(geometry)?)))
    }
}

impl<'a> Sendable<AdvancedTransducer> for Grouped<'a, AdvancedTransducer> {
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainAdvanced;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            Self::B::new(
                self.calc(geometry)?,
                geometry.transducers().map(|tr| tr.cycle()).collect(),
            ),
        ))
    }
}

impl<'a> Sendable<AdvancedPhaseTransducer> for Grouped<'a, AdvancedPhaseTransducer> {
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainAdvancedPhase;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            Self::B::new(
                self.calc(geometry)?,
                geometry.transducers().map(|tr| tr.cycle()).collect(),
            ),
        ))
    }
}
