/*
 * File: evp.rs
 * Project: matrix
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/07/2023
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
/// * Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.
#[derive(Gain)]
pub struct EVP<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    gamma: float,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, EVP<B>);

impl<B: Backend> EVP<B> {
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

impl<B: Backend, T: Transducer> Gain<T> for EVP<B> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let g = generate_propagation_matrix(geometry, &self.foci);
        let q = self.backend.evp(self.gamma, &self.amps, g)?;
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
