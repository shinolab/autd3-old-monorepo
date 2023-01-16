/*
 * File: gs.rs
 * Project: linear_synthesis
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
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
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive,
};
use autd3_traits::Gain;
use nalgebra::ComplexField;
use std::{f64::consts::PI, marker::PhantomData};

/// Reference
/// * Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
#[derive(Gain)]
pub struct GS<B: Backend, C: Constraint> {
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    repeat: usize,
    backend: PhantomData<B>,
    constraint: C,
}

impl<B: Backend, C: Constraint> GS<B, C> {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C) -> Self {
        Self::with_param(foci, amps, constraint, 100)
    }

    pub fn with_param(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C, repeat: usize) -> Self {
        assert!(foci.len() == amps.len());
        Self {
            foci,
            amps,
            repeat,
            backend: PhantomData,
            constraint,
        }
    }
}

impl<B: Backend, C: Constraint, T: Transducer> Gain<T> for GS<B, C> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>> {
        let m = self.foci.len();
        let n = geometry.num_transducers();

        let g = generate_propagation_matrix(geometry, &self.foci);

        let q0 = VectorXc::from_element(n, Complex::new(1., 0.));
        let mut q = q0.clone();

        let mut gamma = VectorXc::zeros(m);
        let mut p = VectorXc::zeros(m);
        let mut xi = VectorXc::zeros(n);
        for _ in 0..self.repeat {
            B::matrix_mul_vec(
                Transpose::NoTrans,
                Complex::new(1., 0.),
                &g,
                &q,
                Complex::new(0., 0.),
                &mut gamma,
            );
            for i in 0..m {
                p[i] = gamma[i] / gamma[i].abs() * self.amps[i];
            }
            B::matrix_mul_vec(
                Transpose::ConjTrans,
                Complex::new(1., 0.),
                &g,
                &p,
                Complex::new(0., 0.),
                &mut xi,
            );
            for i in 0..n {
                q[i] = xi[i] / xi[i].abs() * q0[i];
            }
        }

        let max_coefficient = B::max_coefficient_c(&q).abs();
        Ok(geometry
            .transducers()
            .map(|tr| {
                let phase = q[tr.id()].argument() + PI;
                let amp = self.constraint.convert(q[tr.id()].abs(), max_coefficient);
                Drive { amp, phase }
            })
            .collect())
    }
}
