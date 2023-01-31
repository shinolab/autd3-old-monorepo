/*
 * File: lm.rs
 * Project: nls
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, error::HoloError, macros::generate_propagation_matrix, Backend,
    Complex, MatrixX, MatrixXc, Transpose, VectorX, VectorXc,
};
use anyhow::Result;
use autd3_core::{
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Amp, Drive, Phase,
};
use autd3_traits::Gain;
use nalgebra::ComplexField;
use std::{f64::consts::PI, marker::PhantomData};

/// References
/// * K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.
/// * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
/// * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
#[derive(Gain)]
pub struct LM<B: Backend, C: Constraint> {
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    eps_1: f64,
    eps_2: f64,
    tau: f64,
    k_max: usize,
    initial: Vec<f64>,
    backend: PhantomData<B>,
    constraint: C,
}

impl<B: Backend, C: Constraint> LM<B, C> {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, constraint: C) -> Self {
        Self::with_param(foci, amps, constraint, 1e-8, 1e-8, 1e-3, 5, vec![])
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_param(
        foci: Vec<Vector3>,
        amps: Vec<f64>,
        constraint: C,
        eps_1: f64,
        eps_2: f64,
        tau: f64,
        k_max: usize,
        initial: Vec<f64>,
    ) -> Self {
        assert!(foci.len() == amps.len());
        Self {
            foci,
            amps,
            eps_1,
            eps_2,
            tau,
            k_max,
            initial,
            backend: PhantomData,
            constraint,
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn make_bhb<T: Transducer>(
        geometry: &Geometry<T>,
        amps: &[f64],
        foci: &[Vector3],
        m: usize,
        n: usize,
    ) -> MatrixXc {
        let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            m,
            amps.iter().map(|a| Complex::new(-a, 0.)),
        ));
        let g = generate_propagation_matrix(geometry, foci);
        let b = B::concat_col(g, &p);
        let mut bhb = MatrixXc::zeros(m + n, m + n);
        B::matrix_mul(
            Transpose::ConjTrans,
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &b,
            &b,
            Complex::new(0., 0.),
            &mut bhb,
        );
        bhb
    }

    fn calc_t_th(x: &VectorX, tth: &mut MatrixXc) {
        let len = x.len();
        let t = MatrixXc::from_iterator(len, 1, x.iter().map(|v| Complex::new(0., -v).exp()));
        B::matrix_mul(
            Transpose::NoTrans,
            Transpose::ConjTrans,
            Complex::new(1., 0.),
            &t,
            &t,
            Complex::new(0., 0.),
            tth,
        );
    }
}

impl<B: Backend, C: Constraint, T: Transducer> Gain<T> for LM<B, C> {
    #[allow(clippy::many_single_char_names)]
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>> {
        let m = self.foci.len();
        let n = geometry.num_transducers();
        let n_param = n + m;

        let bhb = Self::make_bhb(geometry, &self.amps, &self.foci, m, n);

        let mut x = VectorX::zeros(n_param);
        x.slice_mut((0, 0), (self.initial.len(), 1))
            .copy_from_slice(&self.initial);

        let mut nu = 2.0;

        let mut tth = MatrixXc::zeros(n_param, n_param);
        Self::calc_t_th(&x, &mut tth);

        let mut bhb_tth = MatrixXc::zeros(n_param, n_param);
        B::hadamard_product(&bhb, &tth, &mut bhb_tth);

        let mut a = MatrixX::zeros(n_param, n_param);
        B::real(&bhb_tth, &mut a);

        let mut g = VectorX::zeros(n_param);
        B::imag(&bhb_tth.column_sum(), &mut g);

        let a_max = a.diagonal().max();

        let mut mu = self.tau * a_max;

        let mut t = VectorXc::from_iterator(x.len(), x.iter().map(|&v| Complex::new(0., v).exp()));

        let mut tmp_vec_c = VectorXc::zeros(n_param);
        B::matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &bhb,
            &t,
            Complex::new(0., 0.),
            &mut tmp_vec_c,
        );
        let mut fx = B::dot_c(&t, &tmp_vec_c).real();

        let identity = MatrixX::identity(n_param, n_param);
        let mut tmp_vec = VectorX::zeros(n_param);
        let mut x_new = VectorX::zeros(n_param);
        let mut h_lm = VectorX::zeros(n_param);
        for _ in 0..self.k_max {
            if B::max_coefficient(&g).abs() <= self.eps_1 {
                break;
            }

            let mut tmp_mat = a.clone();
            B::matrix_add(mu, &identity, 1.0, &mut tmp_mat);
            h_lm.copy_from(&g);
            if !B::solve_g(tmp_mat, &mut h_lm) {
                return Err(HoloError::SolveFailed.into());
            }
            if h_lm.norm() <= self.eps_2 * (x.norm() * self.eps_2) {
                break;
            }

            x_new.copy_from(&x);
            B::vector_add(-1.0, &h_lm, &mut x_new);
            t = VectorXc::from_iterator(
                x_new.len(),
                x_new.iter().map(|&v| Complex::new(0., v).exp()),
            );

            B::matrix_mul_vec(
                Transpose::NoTrans,
                Complex::new(1., 0.),
                &bhb,
                &t,
                Complex::new(0., 0.),
                &mut tmp_vec_c,
            );
            let fx_new = B::dot_c(&t, &tmp_vec_c).real();

            tmp_vec.copy_from(&g);
            B::vector_add(mu, &h_lm, &mut tmp_vec);

            let l0_lhlm = B::dot(&h_lm, &tmp_vec) / 2.0;
            let rho = (fx - fx_new) / l0_lhlm;
            fx = fx_new;

            if rho > 0. {
                x.copy_from(&x_new);
                Self::calc_t_th(&x, &mut tth);
                B::hadamard_product(&bhb, &tth, &mut bhb_tth);
                B::real(&bhb_tth, &mut a);
                B::imag(&bhb_tth.column_sum(), &mut g);

                const THIRD: f64 = 1. / 3.;
                mu *= THIRD.max((1. - (2. * rho - 1.)).powf(3.0));
                nu = 2.0;
            } else {
                mu *= nu;
                nu *= 2.0;
            }
        }

        Ok(geometry
            .transducers()
            .map(|tr| {
                let phase = x[tr.id()].rem_euclid(2.0 * PI);
                let amp = self.constraint.convert(1.0, 1.0);
                Drive {
                    amp: Amp::new(amp),
                    phase: Phase::new(phase),
                }
            })
            .collect())
    }
}
