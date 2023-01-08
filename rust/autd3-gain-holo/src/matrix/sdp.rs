/*
 * File: sdp.rs
 * Project: matrix
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, impl_holo_gain, macros::generate_propagation_matrix, Backend, Complex,
    MatrixXc, Transpose, VectorXc,
};
use anyhow::Result;
use autd3_core::geometry::{Geometry, Transducer, Vector3};
use nalgebra::ComplexField;
use rand::{thread_rng, Rng};
use std::{f64::consts::PI, marker::PhantomData, ops::MulAssign};

/// Reference
/// * Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.
pub struct SDP<B: Backend, C: Constraint, T: Transducer> {
    op: T::Gain,
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    alpha: f64,
    lambda: f64,
    repeat: usize,
    backend: PhantomData<B>,
    constraint: C,
}

impl<B: Backend, C: Constraint, T: Transducer> SDP<B, C, T> {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C) -> Self {
        Self::with_params(foci, amps, constraint, 1e-3, 0.9, 100)
    }

    pub fn with_params(
        foci: Vec<Vector3>,
        amps: Vec<f64>,
        constraint: C,
        alpha: f64,
        lambda: f64,
        repeat: usize,
    ) -> Self {
        assert!(foci.len() == amps.len());
        Self {
            op: Default::default(),
            foci,
            amps,
            alpha,
            lambda,
            repeat,
            backend: PhantomData,
            constraint,
        }
    }

    fn calc(&mut self, geometry: &Geometry<T>) -> Result<()> {
        let m = self.foci.len();
        let n = geometry.num_transducers();

        let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            m,
            self.amps.iter().map(|&a| Complex::new(a, 0.)),
        ));
        let b = generate_propagation_matrix(geometry, &self.foci);
        let mut pseudo_inv_b = MatrixXc::zeros(n, m);
        B::pseudo_inverse_svd(b.clone(), self.alpha, &mut pseudo_inv_b);

        let mut mm = MatrixXc::identity(m, m);
        B::matrix_mul(
            Transpose::NoTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &b,
            &pseudo_inv_b,
            Complex::new(-1., 0.),
            &mut mm,
        );
        let mut tmp = MatrixXc::zeros(m, m);
        B::matrix_mul(
            Transpose::NoTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &p,
            &mm,
            Complex::new(0., 0.),
            &mut tmp,
        );
        B::matrix_mul(
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
            mat.slice_mut((idx, 0), (1, idx))
                .copy_from(&vec.slice((0, 0), (idx, 1)).adjoint());
            mat.slice_mut((idx, idx + 1), (1, m - idx - 1))
                .copy_from(&vec.slice((0, 0), (m - idx - 1, 1)).adjoint());
            mat.slice_mut((0, idx), (idx, 1))
                .copy_from(&vec.slice((0, 0), (idx, 1)));
            mat.slice_mut((idx + 1, idx), (m - idx - 1, 1))
                .copy_from(&vec.slice((0, 0), (m - idx - 1, 1)));
        }

        for _ in 0..self.repeat {
            let ii = (m as f64 * rng.gen_range(0.0..1.0)) as usize;

            let mut mmc: VectorXc = mm.column(ii).into();
            mmc[ii] = Complex::new(0., 0.);

            B::matrix_mul_vec(
                Transpose::NoTrans,
                Complex::new(1., 0.),
                &x_mat,
                &mmc,
                Complex::new(0., 0.),
                &mut x,
            );
            let gamma = B::dot_c(&x, &mmc);
            if gamma.real() > 0.0 {
                x.mul_assign(Complex::new((self.lambda / gamma.real()).sqrt(), 0.));
                set_bcd_result(&mut x_mat, &x, ii);
            } else {
                set_bcd_result(&mut x_mat, &zero, ii);
            }
        }

        let u = B::max_eigen_vector(x_mat);

        let mut ut = VectorXc::zeros(m);
        B::matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &p,
            &u,
            Complex::new(0., 0.),
            &mut ut,
        );

        let mut q = VectorXc::zeros(n);
        B::matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &pseudo_inv_b,
            &ut,
            Complex::new(0., 0.),
            &mut q,
        );

        let max_coefficient = B::max_coefficient_c(&q).abs();
        geometry.transducers().for_each(|tr| {
            let phase = q[tr.id()].argument() + PI;
            let amp = self.constraint.convert(q[tr.id()].abs(), max_coefficient);
            self.op.set_drive(tr.id(), amp, phase);
        });

        Ok(())
    }
}

impl_holo_gain!(SDP);
