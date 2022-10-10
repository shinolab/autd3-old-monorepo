/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, macros::generate_propagation_matrix, Backend, Complex, Transpose,
    VectorXc,
};
use anyhow::Result;
use autd3_core::{
    gain::{Gain, GainProps, IGain},
    geometry::{Geometry, Transducer, Vector3},
    NUM_TRANS_IN_UNIT,
};
use autd3_traits::Gain;
use nalgebra::ComplexField;
use std::{f64::consts::PI, marker::PhantomData};

/// Naive linear synthesis
#[derive(Gain)]
pub struct Naive<B: Backend, T: Transducer, C: Constraint> {
    props: GainProps<T>,
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    backend: PhantomData<B>,
    constraint: C,
}

impl<B: Backend, T: Transducer, C: Constraint> Naive<B, T, C> {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C) -> Self {
        assert!(foci.len() == amps.len());
        Self {
            props: GainProps::default(),
            foci,
            amps,
            backend: PhantomData,
            constraint,
        }
    }
}
impl<B: Backend, T: Transducer, C: Constraint> IGain<T> for Naive<B, T, C> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<()> {
        let m = self.foci.len();
        let n = geometry.num_devices() * NUM_TRANS_IN_UNIT;

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
            let phase = q[tr.id()].argument() / (2.0 * PI) + 0.5;
            let amp = self.constraint.convert(q[tr.id()].abs(), max_coefficient);
            self.props.drives[tr.id()].amp = amp;
            self.props.drives[tr.id()].phase = phase;
        });

        Ok(())
    }
}
