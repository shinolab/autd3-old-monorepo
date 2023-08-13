/*
 * File: evp.rs
 * Project: matrix
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/08/2023
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
use autd3_derive::Gain;
use nalgebra::ComplexField;

/// Gain to produce multiple foci by solving Eigen Value Problem
///
/// Reference
/// * Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.
#[derive(Gain)]
pub struct EVP<B: LinAlgBackend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    gamma: float,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, EVP<B>);

impl<B: LinAlgBackend + 'static> EVP<B> {
    pub fn new(backend: Rc<B>) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            gamma: 1.0,
            backend,
            constraint: Constraint::Uniform(1.),
        }
    }

    pub fn with_gamma(self, gamma: float) -> Self {
        Self { gamma, ..self }
    }

    pub fn gamma(&self) -> float {
        self.gamma
    }
}

impl<B: LinAlgBackend + 'static, T: Transducer> Gain<T> for EVP<B> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = geometry.num_transducers();
        let n = self.foci.len();

        let mut q = self.backend.alloc_zeros_cv(m)?;

        {
            let g = self
                .backend
                .generate_propagation_matrix(geometry, &self.foci)?;
            let amps = self.backend.from_slice_cv(&self.amps)?;

            let mut x = self.backend.alloc_cm(m, n)?;
            self.backend.gen_back_prop(m, n, &g, &amps, &mut x)?;

            let mut r = self.backend.alloc_cm(n, n)?;
            self.backend.gemm_c(
                Trans::NoTrans,
                Trans::NoTrans,
                Complex::new(1., 0.),
                &g,
                &x,
                Complex::new(0., 0.),
                &mut r,
            )?;

            let mut max_ev = self.backend.max_eigen_vector_c(r)?;

            let mut sigma = self.backend.alloc_cm(m, m)?;
            {
                let mut sigma_tmp = self.backend.alloc_zeros_cv(m)?;
                self.backend.gemv_c(
                    Trans::Trans,
                    Complex::new(1., 0.),
                    &g,
                    &amps,
                    Complex::new(0., 0.),
                    &mut sigma_tmp,
                )?;

                let mut sigma_tmp_real = self.backend.alloc_v(m)?;
                self.backend.abs_cv(&sigma_tmp, &mut sigma_tmp_real)?;
                self.backend
                    .scale_assign_v(1. / (n as float), &mut sigma_tmp_real)?;
                self.backend.sqrt_assign_v(&mut sigma_tmp_real)?;
                self.backend.pow_assign_v(self.gamma, &mut sigma_tmp_real)?;
                let zero = self.backend.alloc_zeros_v(m)?;
                self.backend
                    .make_complex2_v(&sigma_tmp_real, &zero, &mut sigma_tmp)?;
                self.backend.create_diagonal_c(&sigma_tmp, &mut sigma)?;
            }

            let mut gr = self.backend.alloc_cm(n + m, m)?;
            self.backend.concat_row_cm(&g, &sigma, &mut gr)?;

            self.backend.normalize_assign_cv(&mut max_ev)?;
            self.backend
                .hadamard_product_assign_cv(&amps, &mut max_ev)?;

            let fn_ = self.backend.alloc_zeros_cv(m)?;
            let mut f = self.backend.alloc_cv(m + n)?;
            self.backend.concat_col_cv(&max_ev, &fn_, &mut f)?;

            let mut gtg = self.backend.alloc_zeros_cm(m, m)?;
            self.backend.gemm_c(
                Trans::ConjTrans,
                Trans::NoTrans,
                Complex::new(1., 0.),
                &gr,
                &gr,
                Complex::new(0., 0.),
                &mut gtg,
            )?;
            self.backend.gemv_c(
                Trans::ConjTrans,
                Complex::new(1., 0.),
                &gr,
                &f,
                Complex::new(0., 0.),
                &mut q,
            )?;
            self.backend.solve_inplace_h(gtg, &mut q)?;
        }

        let q = self.backend.to_host_cv(q)?;
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
