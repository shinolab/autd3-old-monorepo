/*
 * File: gs.rs
 * Project: linear_synthesis
 * Created Date: 29/05/2021
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

/// Reference
/// * Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
#[derive(Gain)]
pub struct GS<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    pub repeat: usize,
    pub constraint: Constraint,
    backend: B,
}

impl_holo!(GS<B>);

impl<B: Backend> GS<B> {
    pub fn new() -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            repeat: 100,
            backend: B::new(),
            constraint: Constraint::Normalize,
        }
    }
}

impl<B: Backend, T: Transducer> Gain<T> for GS<B> {
    fn calc(mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = self.foci.len();
        let n = geometry.num_transducers();

        let g = generate_propagation_matrix(geometry, &self.foci);

        let q0 = VectorXc::from_element(n, Complex::new(1., 0.));
        let mut q = q0.clone();

        let mut gamma = VectorXc::zeros(m);
        let mut p = VectorXc::zeros(m);
        let mut xi = VectorXc::zeros(n);
        for _ in 0..self.repeat {
            self.backend.matrix_mul_vec(
                Transpose::NoTrans,
                Complex::new(1., 0.),
                &g,
                &q,
                Complex::new(0., 0.),
                &mut gamma,
            );
            for i in 0..m {
                p[i] = gamma[i] / gamma[i].abs() * self.amps[i];
            }
            self.backend.matrix_mul_vec(
                Transpose::ConjTrans,
                Complex::new(1., 0.),
                &g,
                &p,
                Complex::new(0., 0.),
                &mut xi,
            );
            for i in 0..n {
                q[i] = xi[i] / xi[i].abs() * q0[i];
            }
        }

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

impl<B: Backend> Default for GS<B> {
    fn default() -> Self {
        Self::new()
    }
}
