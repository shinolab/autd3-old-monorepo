/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, rc::Rc};

use crate::{
    constraint::Constraint, helper::generate_result, impl_holo, Complex, LinAlgBackend, Trans,
};

use autd3_driver::{
    defined::T4010A1_AMPLITUDE,
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};

use autd3_derive::Gain;

/// Gain to produce multiple foci with naive linear synthesis
#[derive(Gain)]
pub struct Naive<B: LinAlgBackend + 'static> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    constraint: Constraint,
    backend: Rc<B>,
}

impl_holo!(B, Naive<B>);

impl<B: LinAlgBackend + 'static> Naive<B> {
    pub fn new(backend: Rc<B>) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            backend,
            constraint: Constraint::Normalize,
        }
    }
}

impl<B: LinAlgBackend> Gain for Naive<B> {
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

        let mut b = self.backend.alloc_cm(m, n)?;
        self.backend.gen_back_prop(m, n, &g, &mut b)?;

        let p = self.backend.from_slice_cv(&self.amps)?;
        let mut q = self.backend.alloc_zeros_cv(m)?;
        self.backend.gemv_c(
            Trans::NoTrans,
            Complex::new(1., 0.),
            &b,
            &p,
            Complex::new(0., 0.),
            &mut q,
        )?;
        self.backend
            .scale_assign_cv(Complex::new(1.0 / T4010A1_AMPLITUDE, 0.), &mut q)?;

        generate_result(
            geometry,
            self.backend.to_host_cv(q)?,
            &self.constraint,
            filter,
        )
    }
}
