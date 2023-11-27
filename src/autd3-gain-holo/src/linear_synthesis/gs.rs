/*
 * File: gs.rs
 * Project: linear_synthesis
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, rc::Rc};

use crate::{
    constraint::EmissionConstraint, helper::generate_result, impl_holo, Amplitude, Complex,
    LinAlgBackend, Trans,
};
use autd3_derive::Gain;

use autd3_driver::{
    defined::T4010A1_AMPLITUDE,
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};

/// Gain to produce multiple foci with GS algorithm
///
/// Reference
/// * Marzo, Asier, and Bruce W. Drinkwater. "Holographic acoustic tweezers." Proceedings of the National Academy of Sciences 116.1 (2019): 84-89.
#[derive(Gain)]
pub struct GS<B: LinAlgBackend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<Amplitude>,
    repeat: usize,
    constraint: EmissionConstraint,
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
            constraint: EmissionConstraint::Normalize,
        }
    }

    pub fn with_repeat(self, repeat: usize) -> Self {
        Self { repeat, ..self }
    }

    pub fn repeat(&self) -> usize {
        self.repeat
    }
}

impl<B: LinAlgBackend> Gain for GS<B> {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        let g = self
            .backend
            .generate_propagation_matrix(geometry, &self.foci, &filter)?;

        let m = self.backend.cols_c(&g)?;
        let n = self.foci.len();
        let ones = vec![T4010A1_AMPLITUDE; m];

        let mut b = self.backend.alloc_cm(m, n)?;
        self.backend.gen_back_prop(m, n, &g, &mut b)?;

        let mut q = self.backend.from_slice_cv(&ones)?;

        let q0 = self.backend.from_slice_cv(&ones)?;

        let amps = self.backend.from_slice_cv(self.amps_as_slice())?;
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
                Trans::NoTrans,
                Complex::new(1., 0.),
                &b,
                &p,
                Complex::new(0., 0.),
                &mut q,
            )?;

            self.backend.scaled_to_assign_cv(&q0, &mut q)?;
        }

        self.backend
            .scale_assign_cv(Complex::new(1. / T4010A1_AMPLITUDE, 0.0), &mut q)?;

        generate_result(
            geometry,
            self.backend.to_host_cv(q)?,
            &self.constraint,
            filter,
        )
    }
}
