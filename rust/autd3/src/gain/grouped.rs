/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use anyhow::Result;

use autd3_core::{
    gain::Gain,
    geometry::{Geometry, Transducer},
    Drive,
};

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

    pub fn add<G: 'a + Gain<T>>(&mut self, id: usize, gain: G) -> Result<()> {
        self.gain_map.insert(id, Box::new(gain));
        Ok(())
    }
}

impl<'a, T: Transducer> Gain<T> for Grouped<'a, T> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>> {
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
                                amp: 0.0,
                                phase: 0.0
                            };
                            geometry.device_map()[i]
                        ]
                    })
            })
            .collect())
    }
}

impl<'a> autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer>
    for Grouped<'a, autd3_core::geometry::LegacyTransducer>
{
    type O = autd3_driver::GainLegacy;

    fn operation(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
    ) -> anyhow::Result<Self::O> {
        let drives = self.calc(geometry)?;
        Ok(Self::O::new(drives))
    }
}

impl<'a> autd3_core::datagram::Sendable<autd3_core::geometry::LegacyTransducer>
    for Grouped<'a, autd3_core::geometry::LegacyTransducer>
{
    type H = autd3_core::datagram::Empty;
    type B = autd3_core::datagram::Filled;
    type O =
        <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer>>::O;

    fn operation(
        &mut self,
        geometry: &Geometry<autd3_core::geometry::LegacyTransducer>,
    ) -> anyhow::Result<Self::O> {
        <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer>>::operation(self, geometry)
    }
}

impl<'a> autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer>
    for Grouped<'a, autd3_core::geometry::NormalTransducer>
{
    type O = autd3_driver::GainNormal;

    fn operation(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
    ) -> anyhow::Result<Self::O> {
        let drives = self.calc(geometry)?;
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(drives, cycles))
    }
}

impl<'a> autd3_core::datagram::Sendable<autd3_core::geometry::NormalTransducer>
    for Grouped<'a, autd3_core::geometry::NormalTransducer>
{
    type H = autd3_core::datagram::Empty;
    type B = autd3_core::datagram::Filled;
    type O =
        <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer>>::O;

    fn operation(
        &mut self,
        geometry: &Geometry<autd3_core::geometry::NormalTransducer>,
    ) -> anyhow::Result<Self::O> {
        <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer>>::operation(self, geometry)
    }
}

impl<'a> autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer>
    for Grouped<'a, autd3_core::geometry::NormalPhaseTransducer>
{
    type O = autd3_driver::GainNormalPhase;

    fn operation(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
    ) -> anyhow::Result<Self::O> {
        let drives = self.calc(geometry)?;
        let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        Ok(Self::O::new(drives, cycles))
    }
}

impl<'a> autd3_core::datagram::Sendable<autd3_core::geometry::NormalPhaseTransducer>
    for Grouped<'a, autd3_core::geometry::NormalPhaseTransducer>
{
    type H = autd3_core::datagram::Empty;
    type B = autd3_core::datagram::Filled;
    type O = <Self as autd3_core::datagram::DatagramBody<
        autd3_core::geometry::NormalPhaseTransducer,
    >>::O;

    fn operation(
        &mut self,
        geometry: &Geometry<autd3_core::geometry::NormalPhaseTransducer>,
    ) -> anyhow::Result<Self::O> {
        <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer>>::operation(self, geometry)
    }
}
