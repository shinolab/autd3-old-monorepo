/*
 * File: lm.rs
 * Project: nls
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/06/2023
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

/// References
/// * K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.
/// * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
/// * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
#[derive(Gain)]
pub struct LM<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    eps_1: float,
    eps_2: float,
    tau: float,
    k_max: usize,
    initial: Vec<float>,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, LM<B>);

impl<B: Backend> LM<B> {
    pub fn new(backend: Rc<B>) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            eps_1: 1e-8,
            eps_2: 1e-8,
            tau: 1e-3,
            k_max: 5,
            initial: vec![],
            backend,
            constraint: Constraint::Normalize,
        }
    }

    pub fn with_eps_1(self, eps_1: float) -> Self {
        Self { eps_1, ..self }
    }

    pub fn with_eps_2(self, eps_2: float) -> Self {
        Self { eps_2, ..self }
    }

    pub fn with_tau(self, tau: float) -> Self {
        Self { tau, ..self }
    }

    pub fn with_k_max(self, k_max: usize) -> Self {
        Self { k_max, ..self }
    }

    pub fn with_initial(self, initial: Vec<float>) -> Self {
        Self { initial, ..self }
    }

    pub fn eps_1(&self) -> float {
        self.eps_1
    }

    pub fn eps_2(&self) -> float {
        self.eps_2
    }

    pub fn tau(&self) -> float {
        self.tau
    }

    pub fn k_max(&self) -> usize {
        self.k_max
    }

    pub fn initial(&self) -> &[float] {
        &self.initial
    }
}

impl<B: Backend, T: Transducer> Gain<T> for LM<B> {
    #[allow(clippy::many_single_char_names)]
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let g = generate_propagation_matrix(geometry, &self.foci);
        let x = self.backend.lm(
            self.eps_1,
            self.eps_2,
            self.tau,
            self.k_max,
            &self.initial,
            &self.amps,
            g,
        )?;
        Ok(geometry
            .transducers()
            .map(|tr| {
                let phase = x[tr.idx()].rem_euclid(2.0 * PI);
                let amp = self.constraint.convert(1.0, 1.0);
                Drive { amp, phase }
            })
            .collect())
    }
}
