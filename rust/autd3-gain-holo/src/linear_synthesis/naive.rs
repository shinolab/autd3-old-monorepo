/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, impl_holo_gain, macros::generate_propagation_matrix, Backend, Complex,
    Transpose, VectorXc,
};
use anyhow::Result;
use autd3_core::geometry::{Geometry, Transducer, Vector3};

use nalgebra::ComplexField;
use std::{f64::consts::PI, marker::PhantomData};

/// Naive linear synthesis
pub struct Naive<B: Backend, C: Constraint, T: Transducer> {
    op: T::Gain,
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    backend: PhantomData<B>,
    constraint: C,
}

impl<B: Backend, C: Constraint, T: Transducer> Naive<B, C, T> {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C) -> Self {
        assert!(foci.len() == amps.len());
        Self {
            op: Default::default(),
            foci,
            amps,
            backend: PhantomData,
            constraint,
        }
    }

    fn calc(&mut self, geometry: &Geometry<T>) -> Result<()> {
        let m = self.foci.len();
        let n = geometry.num_transducers();

        let g = generate_propagation_matrix(geometry, &self.foci);
        let p = VectorXc::from_iterator(m, self.amps.iter().map(|&a| Complex::new(a, 0.0)));
        let mut q = VectorXc::zeros(n);
        B::matrix_mul_vec(
            Transpose::ConjTrans,
            Complex::new(1.0, 0.0),
            &g,
            &p,
            Complex::new(0.0, 0.0),
            &mut q,
        );

        let max_coefficient = B::max_coefficient_c(&q).abs();
        geometry.transducers().for_each(|tr| {
            let phase = q[tr.id()].argument() + PI;
            let amp = self.constraint.convert(q[tr.id()].abs(), max_coefficient);
            self.op.set_drive(tr.id(), amp, phase);
        });

        Ok(())
    }
}

impl_holo_gain!(Naive);
