/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use crate::{constraint::Constraint, impl_holo, Complex, LinAlgBackend, Trans};
use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive, PI,
};

use autd3_traits::Gain;
use nalgebra::ComplexField;

/// Gain to produce multiple foci with naive linear synthesis
#[derive(Gain)]
pub struct Naive<B: LinAlgBackend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, Naive<B>);

impl<B: LinAlgBackend + 'static> Naive<B> {
    pub fn new(backend: Rc<B>) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            backend,
            constraint: Constraint::Normalize,
        }
    }
}

impl<B: LinAlgBackend + 'static, T: Transducer> Gain<T> for Naive<B> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = geometry.num_transducers();

        let g = self
            .backend
            .generate_propagation_matrix(geometry, &self.foci);

        let p = self.backend.make_complex_v(&self.amps);
        let mut q = self.backend.alloc_zeros_cv(m);
        self.backend.gemv_c(
            Trans::ConjTrans,
            Complex::new(1., 0.),
            &g,
            &p,
            Complex::new(0., 0.),
            &mut q,
        );

        let q = self.backend.to_host_cv(q);

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
