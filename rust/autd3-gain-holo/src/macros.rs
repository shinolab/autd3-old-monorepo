/*
 * File: macros.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{Complex, MatrixXc};
use autd3_core::{
    acoustics::directivity_t4010a1 as directivity,
    geometry::{Geometry, Transducer, Vector3},
};
#[allow(unused)]
use nalgebra::ComplexField;

pub fn propagate<T: Transducer>(
    source: &T,
    target: Vector3,
    sound_speed: f64,
    attenuation: f64,
) -> Complex {
    let diff = target - source.position();
    let dist = diff.norm();
    let theta = source.z_direction().angle(&diff);

    let d = directivity(theta);
    let r = d * (-dist * attenuation).exp() / dist;
    let phi = -source.wavenumber(sound_speed) * dist;
    r * Complex::new(0., phi).exp()
}

pub fn generate_propagation_matrix<T: Transducer>(
    geometry: &Geometry<T>,
    foci: &[Vector3],
) -> MatrixXc {
    let m = foci.len();
    let num_trans = geometry.num_transducers();

    let sound_speed = geometry.sound_speed;
    let attenuation = geometry.attenuation;

    MatrixXc::from_iterator(
        m,
        num_trans,
        geometry.transducers().flat_map(|trans| {
            foci.iter()
                .map(move |&fp| propagate(trans, fp, sound_speed, attenuation))
        }),
    )
}

#[macro_export]
macro_rules! impl_holo_gain {
    ($name: ident) => {
use autd3_core::Operation;
use autd3_core::GainOp;

impl<B: Backend, C: Constraint> autd3_core::gain::Gain<autd3_core::geometry::LegacyTransducer>
    for $name<B, C, autd3_core::geometry::LegacyTransducer>
{
    fn drives(&self) -> &[autd3_core::Drive] {
        self.op.drives()
    }
}

impl<B: Backend, C: Constraint> autd3_core::gain::Gain<autd3_core::geometry::NormalTransducer>
    for $name<B, C, autd3_core::geometry::NormalTransducer>
{
    fn drives(&self) -> &[autd3_core::Drive] {
        self.op.drives()
    }
}

impl<B: Backend, C: Constraint> autd3_core::gain::Gain<autd3_core::geometry::NormalPhaseTransducer>
    for $name<B, C, autd3_core::geometry::NormalPhaseTransducer>
{
    fn drives(&self) -> &[autd3_core::Drive] {
        self.op.drives()
    }
}

impl<B: Backend, C: Constraint>
    autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer>
    for $name<B, C, autd3_core::geometry::LegacyTransducer>
{
    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
    ) -> anyhow::Result<()> {
        self.op.init();
        self.op.drives.resize(geometry.num_transducers(), autd3_core::Drive{amp: 0.0, phase: 0.0});
        self.calc(geometry)
    }

    fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
        self.op.pack(tx)
    }

    fn is_finished(&self) -> bool {
        self.op.is_finished()
    }
}

impl<B: Backend, C: Constraint>
    autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer>
    for $name<B, C, autd3_core::geometry::NormalTransducer>
{
    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
    ) -> anyhow::Result<()> {
        self.op.init();
        self.op.drives.resize(geometry.num_transducers(), autd3_core::Drive{amp: 0.0, phase: 0.0});
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

impl<B: Backend, C: Constraint>
    autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer>
    for $name<B, C, autd3_core::geometry::NormalPhaseTransducer>
{
    fn init(
        &mut self,
        geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
    ) -> anyhow::Result<()> {
        self.op.init();
        self.op.drives.resize(geometry.num_transducers(), autd3_core::Drive{amp: 0.0, phase: 0.0});
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

impl<B: Backend, C: Constraint>
    autd3_core::datagram::Sendable<autd3_core::geometry::LegacyTransducer>
    for $name<B, C, autd3_core::geometry::LegacyTransducer>
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

impl<B: Backend, C: Constraint>
    autd3_core::datagram::Sendable<autd3_core::geometry::NormalTransducer>
    for $name<B, C, autd3_core::geometry::NormalTransducer>
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

impl<B: Backend, C: Constraint>
    autd3_core::datagram::Sendable<autd3_core::geometry::NormalPhaseTransducer>
    for $name<B, C, autd3_core::geometry::NormalPhaseTransducer>
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
    };
}
