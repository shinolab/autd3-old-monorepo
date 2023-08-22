/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use crate::{
    constraint::Constraint, helper::generate_result, impl_holo, Complex, LinAlgBackend, Trans,
};
use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::{Gain, GainFilter},
    geometry::{Geometry, Transducer, Vector3},
    Drive,
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

impl<B: LinAlgBackend, T: Transducer> Gain<T> for Naive<B> {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        let g = self
            .backend
            .generate_propagation_matrix(geometry, &self.foci, &filter)?;

        let m = self.backend.cols_c(&g)?;

        let p = self.backend.from_slice_cv(&self.amps)?;
        let mut q = self.backend.alloc_zeros_cv(m)?;
        self.backend.gemv_c(
            Trans::ConjTrans,
            Complex::new(1., 0.),
            &g,
            &p,
            Complex::new(0., 0.),
            &mut q,
        )?;

        generate_result(
            geometry,
            self.backend.to_host_cv(q)?,
            &self.constraint,
            filter,
        )
    }
}
