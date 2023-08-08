/*
 * File: gspat.rs
 * Project: linear_synthesis
 * Created Date: 29/05/2021
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

/// Gain to produce multiple foci with GS-PAT algorithm
///
/// Reference
/// * Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.
#[derive(Gain)]
pub struct GSPAT<B: LinAlgBackend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    repeat: usize,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, GSPAT<B>);

impl<B: LinAlgBackend + 'static> GSPAT<B> {
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

impl<B: LinAlgBackend + 'static, T: Transducer> Gain<T> for GSPAT<B> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = geometry.num_transducers();
        let n = self.foci.len();

        let mut q = self.backend.alloc_zeros_cv(m);

        {
            let g = self
                .backend
                .generate_propagation_matrix(geometry, &self.foci);
            let amps = self.backend.make_complex_v(&self.amps);

            let mut b = self.backend.alloc_cm(m, n);
            self.backend.gen_back_prop(n, &g, &amps, &mut b);

            let mut r = self.backend.alloc_zeros_cm(n, n);
            self.backend.gemm_c(
                Trans::NoTrans,
                Trans::NoTrans,
                Complex::new(1., 0.),
                &g,
                &b,
                Complex::new(0., 0.),
                &mut r,
            );

            let mut p = self.backend.clone_cv(&amps);
            let mut gamma = self.backend.clone_cv(&amps);
            self.backend.gemv_c(
                Trans::NoTrans,
                Complex::new(1., 0.),
                &r,
                &p,
                Complex::new(0., 0.),
                &mut gamma,
            );
            for _ in 0..self.repeat {
                self.backend.normalize_cv(&mut gamma);
                self.backend.hadamard_product_cv(&gamma, &amps, &mut p);
                self.backend.gemv_c(
                    Trans::NoTrans,
                    Complex::new(1., 0.),
                    &r,
                    &p,
                    Complex::new(0., 0.),
                    &mut gamma,
                );
            }

            let mut tmp = self.backend.clone_cv(&gamma);
            self.backend.reciprocal_c(&mut tmp);
            self.backend.normalize_cv(&mut gamma);
            self.backend.hadamard_product_assign_cv(&tmp, &mut p);
            self.backend.hadamard_product_assign_cv(&amps, &mut p);
            self.backend.hadamard_product_assign_cv(&amps, &mut p);

            self.backend.gemv_c(
                Trans::NoTrans,
                Complex::new(1., 0.),
                &b,
                &p,
                Complex::new(0., 0.),
                &mut q,
            );
        }

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
