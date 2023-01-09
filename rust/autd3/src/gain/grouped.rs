/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::error::AUTDError;
use autd3_core::{
    gain::Gain,
    geometry::{Geometry, Transducer},
    GainOp, Operation,
};

/// Gain to produce single focal point
#[derive(Default)]
pub struct Grouped<'a, T: Transducer> {
    op: T::Gain,
    gain_map: HashMap<usize, Box<dyn Gain<T> + 'a>>,
}

impl<'a, T: Transducer> Grouped<'a, T> {
    /// constructor
    pub fn new() -> Self {
        Self {
            op: Default::default(),
            gain_map: HashMap::new(),
        }
    }

    pub fn add<G: 'a + Gain<T>>(&mut self, id: usize, gain: G) -> anyhow::Result<()> {
        self.gain_map.insert(id, Box::new(gain));
        Ok(())
    }

    fn calc(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        self.gain_map.iter_mut().try_for_each(|(dev_id, gain)| {
            if *dev_id >= geometry.num_devices() {
                return Err(AUTDError::GroupedOutOfRange(*dev_id, geometry.num_devices()).into());
            }
            let start = if *dev_id == 0 {
                0
            } else {
                geometry.device_map()[*dev_id - 1]
            };
            let end = start + geometry.device_map()[*dev_id];
            gain.init(geometry)?;
            let gain = gain.drives();
            (start..end).for_each(|idx| {
                self.op.set_drive(idx, gain[idx].amp, gain[idx].phase);
            });
            Ok(())
        })
    }
}

impl<'a> autd3_core::gain::Gain<autd3_core::geometry::LegacyTransducer>
    for Grouped<'a, autd3_core::geometry::LegacyTransducer>
{
    fn drives(&self) -> &[autd3_core::Drive] {
        self.op.drives()
    }
}

impl<'a> autd3_core::gain::Gain<autd3_core::geometry::NormalTransducer>
    for Grouped<'a, autd3_core::geometry::NormalTransducer>
{
    fn drives(&self) -> &[autd3_core::Drive] {
        self.op.drives()
    }
}

impl<'a> autd3_core::gain::Gain<autd3_core::geometry::NormalPhaseTransducer>
    for Grouped<'a, autd3_core::geometry::NormalPhaseTransducer>
{
    fn drives(&self) -> &[autd3_core::Drive] {
        self.op.drives()
    }
}

impl<'a> autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer>
    for Grouped<'a, autd3_core::geometry::LegacyTransducer>
{
    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
    ) -> anyhow::Result<()> {
        self.op.init();
        self.op.drives.resize(
            geometry.num_transducers(),
            autd3_core::Drive {
                amp: 0.0,
                phase: 0.0,
            },
        );
        self.calc(geometry)
    }

    fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<'a> autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer>
    for Grouped<'a, autd3_core::geometry::NormalTransducer>
{
    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
    ) -> anyhow::Result<()> {
        self.op.init();
        self.op.drives.resize(
            geometry.num_transducers(),
            autd3_core::Drive {
                amp: 0.0,
                phase: 0.0,
            },
        );
        self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        self.calc(geometry)
    }

    fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<'a> autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer>
    for Grouped<'a, autd3_core::geometry::NormalPhaseTransducer>
{
    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
    ) -> anyhow::Result<()> {
        self.op.init();
        self.op.drives.resize(
            geometry.num_transducers(),
            autd3_core::Drive {
                amp: 0.0,
                phase: 0.0,
            },
        );
        self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
        self.calc(geometry)
    }

    fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<'a> autd3_core::datagram::Sendable<autd3_core::geometry::LegacyTransducer>
    for Grouped<'a, autd3_core::geometry::LegacyTransducer>
{
    type H = autd3_core::datagram::Empty;
    type B = autd3_core::datagram::Filled;

    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
    ) -> anyhow::Result<()> {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::init(
            self, geometry,
        )
    }

    fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::pack(self, tx)
    }

    fn is_finished(&self) -> bool {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::is_finished(
            self,
        )
    }
}

impl<'a> autd3_core::datagram::Sendable<autd3_core::geometry::NormalTransducer>
    for Grouped<'a, autd3_core::geometry::NormalTransducer>
{
    type H = autd3_core::datagram::Empty;
    type B = autd3_core::datagram::Filled;

    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
    ) -> anyhow::Result<()> {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::init(
            self, geometry,
        )
    }

    fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::pack(self, tx)
    }

    fn is_finished(&self) -> bool {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::is_finished(
            self,
        )
    }
}

impl<'a> autd3_core::datagram::Sendable<autd3_core::geometry::NormalPhaseTransducer>
    for Grouped<'a, autd3_core::geometry::NormalPhaseTransducer>
{
    type H = autd3_core::datagram::Empty;
    type B = autd3_core::datagram::Filled;

    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
    ) -> anyhow::Result<()> {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::init(
            self, geometry,
        )
    }

    fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::pack(
            self, tx,
        )
    }

    fn is_finished(&self) -> bool {
        autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::is_finished(
            self,
        )
    }
}
