/*
 * File: sdp.rs
 * Project: matrix
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, impl_holo, macros::generate_propagation_matrix, Backend, Complex,
    MatrixXc, Transpose, VectorXc,
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
use rand::{thread_rng, Rng};

/// Reference
/// * Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.
#[derive(Gain)]
pub struct SDP<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    pub alpha: float,
    pub lambda: float,
    pub repeat: usize,
    pub constraint: Constraint,
    backend: B,
}

impl_holo!(B, SDP<B>);

impl<B: Backend> SDP<B> {
    pub fn new(backend: B) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            alpha: 1e-3,
            lambda: 0.9,
            repeat: 100,
            backend,
            constraint: Constraint::Normalize,
        }
    }
}

impl<B: Backend, T: Transducer> Gain<T> for SDP<B> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = self.foci.len();
        let n = geometry.num_transducers();

        let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            m,
            self.amps.iter().map(|&a| Complex::new(a, 0.)),
        ));
        let b = generate_propagation_matrix(geometry, &self.foci);
        let mut pseudo_inv_b = MatrixXc::zeros(n, m);
        self.backend
            .pseudo_inverse_svd(b.clone(), self.alpha, &mut pseudo_inv_b);

        let mut mm = MatrixXc::identity(m, m);
        self.backend.matrix_mul(
            Transpose::NoTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &b,
            &pseudo_inv_b,
            Complex::new(-1., 0.),
            &mut mm,
        );
        let mut tmp = MatrixXc::zeros(m, m);
        self.backend.matrix_mul(
            Transpose::NoTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &p,
            &mm,
            Complex::new(0., 0.),
            &mut tmp,
        );
        self.backend.matrix_mul(
            Transpose::NoTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &tmp,
            &p,
            Complex::new(0., 0.),
            &mut mm,
        );
        let mut x_mat = MatrixXc::identity(m, m);

        let mut rng = thread_rng();
        let zero = VectorXc::zeros(m);
        let mut x = VectorXc::zeros(m);

        fn set_bcd_result(mat: &mut MatrixXc, vec: &VectorXc, idx: usize) {
            let m = vec.len();
            mat.view_mut((idx, 0), (1, idx))
                .copy_from(&vec.view((0, 0), (idx, 1)).adjoint());
            mat.view_mut((idx, idx + 1), (1, m - idx - 1))
                .copy_from(&vec.view((0, 0), (m - idx - 1, 1)).adjoint());
            mat.view_mut((0, idx), (idx, 1))
                .copy_from(&vec.view((0, 0), (idx, 1)));
            mat.view_mut((idx + 1, idx), (m - idx - 1, 1))
                .copy_from(&vec.view((0, 0), (m - idx - 1, 1)));
        }

        for _ in 0..self.repeat {
            let ii = (m as float * rng.gen_range(0.0..1.0)) as usize;

            let mut mmc: VectorXc = mm.column(ii).into();
            mmc[ii] = Complex::new(0., 0.);

            self.backend.matrix_mul_vec(
                Transpose::NoTrans,
                Complex::new(1., 0.),
                &x_mat,
                &mmc,
                Complex::new(0., 0.),
                &mut x,
            );
            let gamma = self.backend.dot_c(&x, &mmc);
            if gamma.real() > 0.0 {
                x *= Complex::new((self.lambda / gamma.real()).sqrt(), 0.);
                set_bcd_result(&mut x_mat, &x, ii);
            } else {
                set_bcd_result(&mut x_mat, &zero, ii);
            }
        }

        let u = self.backend.max_eigen_vector(x_mat);

        let mut ut = VectorXc::zeros(m);
        self.backend.matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &p,
            &u,
            Complex::new(0., 0.),
            &mut ut,
        );

        let mut q = VectorXc::zeros(n);
        self.backend.matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &pseudo_inv_b,
            &ut,
            Complex::new(0., 0.),
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
