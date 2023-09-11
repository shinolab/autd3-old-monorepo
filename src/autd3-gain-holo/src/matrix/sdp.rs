/*
 * File: sdp.rs
 * Project: matrix
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, rc::Rc};

use rand::Rng;

use crate::{
    constraint::Constraint, helper::generate_result, impl_holo, Complex, LinAlgBackend, Trans,
};
use autd3_derive::Gain;

use autd3_driver::{
    derive::prelude::*,
    geometry::{Device, Vector3},
};

/// Gain to produce multiple foci by solving Semi-Denfinite Programming
///
/// Reference
/// * Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.
#[derive(Gain)]
pub struct SDP<B: LinAlgBackend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    alpha: float,
    lambda: float,
    repeat: usize,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, SDP<B>);

impl<B: LinAlgBackend + 'static> SDP<B> {
    pub fn new(backend: Rc<B>) -> Self {
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

    pub fn with_alpha(self, alpha: float) -> Self {
        Self { alpha, ..self }
    }

    pub fn with_lambda(self, lambda: float) -> Self {
        Self { lambda, ..self }
    }

    pub fn with_repeat(self, repeat: usize) -> Self {
        Self { repeat, ..self }
    }

    pub fn alpha(&self) -> float {
        self.alpha
    }

    pub fn lambda(&self) -> float {
        self.lambda
    }

    pub fn repeat(&self) -> usize {
        self.repeat
    }
}

impl<B: LinAlgBackend, T: Transducer> Gain<T> for SDP<B> {
    fn calc(
        &self,
        devices: &[&Device<T>],
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        let b = self
            .backend
            .generate_propagation_matrix(devices, &self.foci, &filter)?;

        let m = self.backend.cols_c(&b)?;
        let n = self.foci.len();

        let mut q = self.backend.alloc_zeros_cv(m)?;

        let amps = self.backend.from_slice_cv(&self.amps)?;

        let mut p = self.backend.alloc_zeros_cm(n, n)?;
        self.backend.create_diagonal_c(&amps, &mut p)?;

        let mut pseudo_inv_b = self.backend.alloc_zeros_cm(m, n)?;
        let mut u_ = self.backend.alloc_cm(n, n)?;
        let mut s = self.backend.alloc_cm(m, n)?;
        let mut vt = self.backend.alloc_cm(m, m)?;
        let mut buf = self.backend.alloc_zeros_cm(m, n)?;
        let b_tmp = self.backend.clone_cm(&b)?;
        self.backend.pseudo_inverse_svd(
            b_tmp,
            self.alpha,
            &mut u_,
            &mut s,
            &mut vt,
            &mut buf,
            &mut pseudo_inv_b,
        )?;

        let mut mm = self.backend.alloc_cm(n, n)?;
        let ones = vec![1.; n];
        let ones = self.backend.from_slice_cv(&ones)?;
        self.backend.create_diagonal_c(&ones, &mut mm)?;

        self.backend.gemm_c(
            Trans::NoTrans,
            Trans::NoTrans,
            Complex::new(-1., 0.),
            &b,
            &pseudo_inv_b,
            Complex::new(1., 0.),
            &mut mm,
        )?;

        let mut tmp = self.backend.alloc_zeros_cm(n, n)?;
        self.backend.gemm_c(
            Trans::NoTrans,
            Trans::NoTrans,
            Complex::new(1., 0.),
            &p,
            &mm,
            Complex::new(0., 0.),
            &mut tmp,
        )?;
        self.backend.gemm_c(
            Trans::NoTrans,
            Trans::NoTrans,
            Complex::new(1., 0.),
            &tmp,
            &p,
            Complex::new(0., 0.),
            &mut mm,
        )?;

        let mut x_mat = self.backend.alloc_cm(n, n)?;
        self.backend.create_diagonal_c(&ones, &mut x_mat)?;

        let mut rng = rand::thread_rng();

        let zero = self.backend.alloc_zeros_cv(n)?;
        let mut x = self.backend.alloc_zeros_cv(n)?;
        let mut mmc = self.backend.alloc_cv(n)?;
        for _ in 0..self.repeat {
            let ii = (n as float * rng.gen_range(0.0..1.0)) as usize;

            self.backend.get_col_c(&mm, ii, &mut mmc)?;
            self.backend.set_cv(ii, Complex::new(0., 0.), &mut mmc)?;

            self.backend.gemv_c(
                Trans::NoTrans,
                Complex::new(1., 0.),
                &x_mat,
                &mmc,
                Complex::new(0., 0.),
                &mut x,
            )?;

            let gamma = self.backend.dot_c(&x, &mmc)?;
            if gamma.re > 0. {
                self.backend
                    .scale_assign_cv(Complex::new(-(self.lambda / gamma.re).sqrt(), 0.), &mut x)?;

                self.backend.set_col_c(&x, ii, 0, ii, &mut x_mat)?;
                self.backend.set_col_c(&x, ii, ii + 1, n, &mut x_mat)?;

                self.backend.conj_assign_v(&mut x)?;

                self.backend.set_row_c(&x, ii, 0, ii, &mut x_mat)?;
                self.backend.set_row_c(&x, ii, ii + 1, n, &mut x_mat)?;
            } else {
                self.backend.set_col_c(&zero, ii, 0, ii, &mut x_mat)?;
                self.backend.set_col_c(&zero, ii, ii + 1, n, &mut x_mat)?;
                self.backend.set_row_c(&zero, ii, 0, ii, &mut x_mat)?;
                self.backend.set_row_c(&zero, ii, ii + 1, n, &mut x_mat)?;
            }
        }

        let u = self.backend.max_eigen_vector_c(x_mat)?;

        let mut ut = self.backend.alloc_zeros_cv(n)?;
        self.backend.gemv_c(
            Trans::NoTrans,
            Complex::new(1., 0.),
            &p,
            &u,
            Complex::new(0., 0.),
            &mut ut,
        )?;

        self.backend.gemv_c(
            Trans::NoTrans,
            Complex::new(1., 0.),
            &pseudo_inv_b,
            &ut,
            Complex::new(0., 0.),
            &mut q,
        )?;

        generate_result(
            devices,
            self.backend.to_host_cv(q)?,
            &self.constraint,
            filter,
        )
    }
}
