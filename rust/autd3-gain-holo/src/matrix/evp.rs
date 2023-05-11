/*
 * File: evp.rs
 * Project: matrix
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
    constraint::Constraint, error::HoloError, impl_holo, macros::generate_propagation_matrix,
    Backend, Complex, MatrixXc, Transpose, VectorXc,
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
/// * Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.
#[derive(Gain)]
pub struct EVP<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    pub gamma: float,
    pub constraint: Constraint,
    backend: B,
}

impl_holo!(EVP<B>);

impl<B: Backend> EVP<B> {
    pub fn new() -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            gamma: 1.0,
            backend: B::new(),
            constraint: Constraint::Uniform(1.),
        }
    }
}

impl<B: Backend, T: Transducer> Gain<T> for EVP<B> {
    fn calc(mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = self.foci.len();
        let n = geometry.num_transducers();

        let g = generate_propagation_matrix(geometry, &self.foci);

        let denomi = g.column_sum();
        let x = g
            .map_with_location(|i, _, a| Complex::new(self.amps[i], 0.0) * a.conj() / denomi[i])
            .transpose();

        let mut r = MatrixXc::zeros(m, m);
        self.backend.matrix_mul(
            Transpose::NoTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &g,
            &x,
            Complex::new(0., 0.),
            &mut r,
        );
        let max_ev = self.backend.max_eigen_vector(r);

        let sigma = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            n,
            g.column_iter()
                .map(|col| {
                    col.iter()
                        .zip(self.amps.iter())
                        .map(|(a, &amp)| a.abs() * amp)
                        .sum()
                })
                .map(|s: float| Complex::new((s / m as float).sqrt().powf(self.gamma), 0.0)),
        ));

        let gr = self.backend.concat_row(g, &sigma);
        let f = VectorXc::from_iterator(
            m + n,
            self.amps
                .iter()
                .zip(max_ev.iter())
                .map(|(amp, &e)| amp * e / e.abs())
                .chain((0..n).map(|_| Complex::new(0., 0.))),
        );

        let mut gtg = MatrixXc::zeros(n, n);
        self.backend.matrix_mul(
            Transpose::ConjTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &gr,
            &gr,
            Complex::new(0., 0.),
            &mut gtg,
        );

        let mut gtf = VectorXc::zeros(n);
        self.backend.matrix_mul_vec(
            Transpose::ConjTrans,
            Complex::new(1., 0.),
            &gr,
            &f,
            Complex::new(0., 0.),
            &mut gtf,
        );

        if !self.backend.solve_ch(gtg, &mut gtf) {
            return Err(HoloError::SolveFailed.into());
        }

        let max_coefficient = self.backend.max_coefficient_c(&gtf).abs();
        Ok(geometry
            .transducers()
            .map(|tr| {
                let phase = gtf[tr.idx()].argument() + PI;
                let amp = self
                    .constraint
                    .convert(gtf[tr.idx()].abs(), max_coefficient);
                Drive { amp, phase }
            })
            .collect())
    }
}

impl<B: Backend> Default for EVP<B> {
    fn default() -> Self {
        Self::new()
    }
}
