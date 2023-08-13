/*
 * File: gs.rs
 * Project: linear_synthesis
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

/// Gain to produce multiple foci with GS algorithm
///
/// Reference
/// * Marzo, Asier, and Bruce W. Drinkwater. "Holographic acoustic tweezers." Proceedings of the National Academy of Sciences 116.1 (2019): 84-89.
#[derive(Gain)]
pub struct GS<B: LinAlgBackend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    repeat: usize,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, GS<B>);

impl<B: LinAlgBackend + 'static> GS<B> {
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

impl<B: LinAlgBackend + 'static, T: Transducer> Gain<T> for GS<B> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = geometry.num_transducers();
        let n = self.foci.len();

        let ones = vec![1.; m];
        let mut q = self.backend.from_slice_cv(&ones)?;

        {
            let g = self
                .backend
                .generate_propagation_matrix(geometry, &self.foci)?;

            let amps = self.backend.from_slice_cv(&self.amps)?;

            let q0 = self.backend.from_slice_cv(&ones)?;

            let mut p = self.backend.alloc_zeros_cv(n)?;
            for _ in 0..self.repeat {
                self.backend.gemv_c(
                    Trans::NoTrans,
                    Complex::new(1., 0.),
                    &g,
                    &q,
                    Complex::new(0., 0.),
                    &mut p,
                )?;
                self.backend.scaled_to_assign_cv(&amps, &mut p)?;

                self.backend.gemv_c(
                    Trans::ConjTrans,
                    Complex::new(1., 0.),
                    &g,
                    &p,
                    Complex::new(0., 0.),
                    &mut q,
                )?;

                self.backend.scaled_to_assign_cv(&q0, &mut q)?;
            }
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
