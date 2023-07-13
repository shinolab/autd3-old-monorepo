/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/07/2023
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

/// Naive linear synthesis
#[derive(Gain)]
pub struct Naive<B: Backend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, Naive<B>);

impl<B: Backend + 'static> Naive<B> {
    pub fn new(backend: Rc<B>) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            backend,
            constraint: Constraint::Normalize,
        }
    }
}

impl<B: Backend + 'static, T: Transducer> Gain<T> for Naive<B> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let g = generate_propagation_matrix(geometry, &self.foci);
        let q = self.backend.naive(&self.amps, g)?;
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
