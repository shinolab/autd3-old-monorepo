/*
 * File: gspat.rs
 * Project: linear_synthesis
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, macros::generate_propagation_matrix, Backend, Complex, MatrixXc,
    Transpose, VectorXc,
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

/// Reference
/// * Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.
///
/// Not yet been implemented with GPU.
#[derive(Gain)]
pub struct GSPAT<B: Backend, T: Transducer, C: Constraint> {
    props: GainProps<T>,
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    repeat: usize,
    backend: PhantomData<B>,
    constraint: C,
}

impl<B: Backend, T: Transducer, C: Constraint> GSPAT<B, T, C> {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C) -> Self {
        Self::with_param(foci, amps, constraint, 100)
    }

    pub fn with_param(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C, repeat: usize) -> Self {
        assert!(foci.len() == amps.len());
        Self {
            props: GainProps::default(),
            foci,
            amps,
            repeat,
            backend: PhantomData,
            constraint,
        }
    }
}

impl<B: Backend, T: Transducer, C: Constraint> IGain<T> for GSPAT<B, T, C> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<()> {
        let m = self.foci.len();
        let n = geometry.num_devices() * NUM_TRANS_IN_UNIT;

        let g = generate_propagation_matrix(geometry, &self.foci);

        let denomi = g.column_sum();
        let b = g
            .map_with_location(|i, _, a| Complex::new(self.amps[i], 0.0) * a.conj() / denomi[i])
            .transpose();

        let mut r = MatrixXc::zeros(m, m);
        B::matrix_mul(
            Transpose::NoTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &g,
            &b,
            Complex::new(0., 0.),
            &mut r,
        );

        let mut p = VectorXc::from_iterator(m, self.amps.iter().map(|&a| Complex::new(a, 0.)));

        let mut gamma = VectorXc::zeros(m);
        B::matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &r,
            &p,
            Complex::new(0., 0.),
            &mut gamma,
        );
        for _ in 0..self.repeat {
            for i in 0..m {
                p[i] = gamma[i] / gamma[i].abs() * self.amps[i];
            }
            B::matrix_mul_vec(
                Transpose::NoTrans,
                Complex::new(1., 0.),
                &r,
                &p,
                Complex::new(0., 0.),
                &mut gamma,
            );
        }

        for i in 0..m {
            p[i] = gamma[i] / gamma[i].norm_sqr() * self.amps[i] * self.amps[i];
        }

        let mut q = VectorXc::zeros(n);
        B::matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &b,
            &p,
            Complex::new(0., 0.),
            &mut q,
        );

        let max_coefficient = B::max_coefficient_c(&q).abs();
        geometry.transducers().for_each(|tr| {
            let phase = q[tr.id()].argument() + PI;
            let amp = self.constraint.convert(q[tr.id()].abs(), max_coefficient);
            self.props.drives[tr.id()].amp = amp;
            self.props.drives[tr.id()].phase = phase;
        });

        Ok(())
    }
}
