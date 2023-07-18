/*
 * File: gs.rs
 * Project: linear_synthesis
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use crate::{constraint::Constraint, impl_holo, macros::generate_propagation_matrix, Backend};
use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive, PI,
};
use autd3_traits::Gain;
use nalgebra::ComplexField;

/// Gain to produce multiple foci with GS algorithm
///
/// Reference
/// * Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
#[derive(Gain)]
pub struct GS<B: Backend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    repeat: usize,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, GS<B>);

impl<B: Backend + 'static> GS<B> {
    pub fn new(backend: Rc<B>) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            repeat: 100,
            backend,
            constraint: Constraint::Normalize,
        }
    }

    pub fn with_repeat(self, repeat: usize) -> Self {
        Self { repeat, ..self }
    }

    pub fn repeat(&self) -> usize {
        self.repeat
    }
}

impl<B: Backend + 'static, T: Transducer> Gain<T> for GS<B> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let g = generate_propagation_matrix(geometry, &self.foci);
        let q = self.backend.gs(self.repeat, &self.amps, g)?;
        let max_coefficient = q.camax().abs();
        Ok(geometry
            .transducers()
            .map(|tr| {
                let phase = q[tr.idx()].argument() + PI;
                let amp = self.constraint.convert(q[tr.idx()].abs(), max_coefficient);
                Drive { amp, phase }
            })
            .collect())
    }
}
