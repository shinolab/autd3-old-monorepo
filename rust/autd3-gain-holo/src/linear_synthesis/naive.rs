/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, impl_holo, macros::generate_propagation_matrix, Backend, Complex,
    Transpose, VectorXc,
};
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
pub struct Naive<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    pub constraint: Constraint,
    backend: B,
}

impl_holo!(Naive<B>);

impl<B: Backend> Naive<B> {
    pub fn new() -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            backend: B::new(),
            constraint: Constraint::Normalize,
        }
    }
}

impl<B: Backend, T: Transducer> Gain<T> for Naive<B> {
    fn calc(mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = self.foci.len();
        let n = geometry.num_transducers();

        let g = generate_propagation_matrix(geometry, &self.foci);
        let p = VectorXc::from_iterator(m, self.amps.iter().map(|&a| Complex::new(a, 0.0)));
        let mut q = VectorXc::zeros(n);
        self.backend.matrix_mul_vec(
            Transpose::ConjTrans,
            Complex::new(1.0, 0.0),
            &g,
            &p,
            Complex::new(0.0, 0.0),
            &mut q,
        );

        let max_coefficient = self.backend.max_coefficient_c(&q).abs();
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

impl<B: Backend> Default for Naive<B> {
    fn default() -> Self {
        Self::new()
    }
}
