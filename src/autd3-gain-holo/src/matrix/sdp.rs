/*
 * File: sdp.rs
 * Project: matrix
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use crate::{constraint::Constraint, impl_holo, macros::generate_propagation_matrix, Backend};
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
/// * Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.
#[derive(Gain)]
pub struct SDP<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    alpha: float,
    lambda: float,
    repeat: usize,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, SDP<B>);

impl<B: Backend> SDP<B> {
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
}

impl<B: Backend, T: Transducer> Gain<T> for SDP<B> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let g = generate_propagation_matrix(geometry, &self.foci);
        let q = self
            .backend
            .sdp(self.alpha, self.repeat, self.lambda, &self.amps, g)?;
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
