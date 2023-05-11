/*
 * File: lm.rs
 * Project: nls
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    constraint::Constraint, error::HoloError, impl_holo, macros::generate_propagation_matrix,
    Backend, Complex, MatrixX, MatrixXc, Transpose, VectorX, VectorXc,
};
use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive, PI,
};
use autd3_traits::Gain;
use nalgebra::ComplexField;

/// References
/// * K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.
/// * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
/// * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
#[derive(Gain)]
pub struct LM<B: Backend> {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    pub eps_1: float,
    pub eps_2: float,
    pub tau: float,
    pub k_max: usize,
    pub initial: Vec<float>,
    pub constraint: Constraint,
    backend: B,
}

impl_holo!(LM<B>);

impl<B: Backend> LM<B> {
    pub fn new() -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            eps_1: 1e-8,
            eps_2: 1e-8,
            tau: 1e-3,
            k_max: 5,
            initial: vec![],
            backend: B::new(),
            constraint: Constraint::Normalize,
        }
    }

    #[allow(clippy::many_single_char_names)]
    fn make_bhb<T: Transducer>(&mut self, geometry: &Geometry<T>, m: usize, n: usize) -> MatrixXc {
        let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            m,
            self.amps.iter().map(|a| Complex::new(-a, 0.)),
        ));
        let g = generate_propagation_matrix(geometry, &self.foci);
        let b = self.backend.concat_col(g, &p);
        let mut bhb = MatrixXc::zeros(m + n, m + n);
        self.backend.matrix_mul(
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

    fn calc_t_th(&mut self, x: &VectorX, tth: &mut MatrixXc) {
        let len = x.len();
        let t = MatrixXc::from_iterator(len, 1, x.iter().map(|v| Complex::new(0., -v).exp()));
        self.backend.matrix_mul(
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

impl<B: Backend, T: Transducer> Gain<T> for LM<B> {
    #[allow(clippy::many_single_char_names)]
    fn calc(mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = self.foci.len();
        let n = geometry.num_transducers();
        let n_param = n + m;

        let bhb = self.make_bhb(geometry, m, n);

        let mut x = VectorX::zeros(n_param);
        x.view_mut((0, 0), (self.initial.len(), 1))
            .copy_from_slice(&self.initial);

        let mut nu = 2.0;

        let mut tth = MatrixXc::zeros(n_param, n_param);
        self.calc_t_th(&x, &mut tth);

        let mut bhb_tth = MatrixXc::zeros(n_param, n_param);
        self.backend.hadamard_product(&bhb, &tth, &mut bhb_tth);

        let mut a = MatrixX::zeros(n_param, n_param);
        self.backend.real(&bhb_tth, &mut a);

        let mut g = VectorX::zeros(n_param);
        self.backend.imag(&bhb_tth.column_sum(), &mut g);

        let a_max = a.diagonal().max();

        let mut mu = self.tau * a_max;

        let mut t = VectorXc::from_iterator(x.len(), x.iter().map(|&v| Complex::new(0., v).exp()));

        let mut tmp_vec_c = VectorXc::zeros(n_param);
        self.backend.matrix_mul_vec(
            Transpose::NoTrans,
            Complex::new(1., 0.),
            &bhb,
            &t,
            Complex::new(0., 0.),
            &mut tmp_vec_c,
        );
        let mut fx = self.backend.dot_c(&t, &tmp_vec_c).real();

        let identity = MatrixX::identity(n_param, n_param);
        let mut tmp_vec = VectorX::zeros(n_param);
        let mut x_new = VectorX::zeros(n_param);
        let mut h_lm = VectorX::zeros(n_param);
        for _ in 0..self.k_max {
            if self.backend.max_coefficient(&g).abs() <= self.eps_1 {
                break;
            }

            let mut tmp_mat = a.clone();
            self.backend.matrix_add(mu, &identity, 1.0, &mut tmp_mat);
            h_lm.copy_from(&g);
            if !self.backend.solve_g(tmp_mat, &mut h_lm) {
                return Err(HoloError::SolveFailed.into());
            }
            if h_lm.norm() <= self.eps_2 * (x.norm() * self.eps_2) {
                break;
            }

            x_new.copy_from(&x);
            self.backend.vector_add(-1.0, &h_lm, &mut x_new);
            t = VectorXc::from_iterator(
                x_new.len(),
                x_new.iter().map(|&v| Complex::new(0., v).exp()),
            );

            self.backend.matrix_mul_vec(
                Transpose::NoTrans,
                Complex::new(1., 0.),
                &bhb,
                &t,
                Complex::new(0., 0.),
                &mut tmp_vec_c,
            );
            let fx_new = self.backend.dot_c(&t, &tmp_vec_c).real();

            tmp_vec.copy_from(&g);
            self.backend.vector_add(mu, &h_lm, &mut tmp_vec);

            let l0_lhlm = self.backend.dot(&h_lm, &tmp_vec) / 2.0;
            let rho = (fx - fx_new) / l0_lhlm;
            fx = fx_new;

            if rho > 0. {
                x.copy_from(&x_new);
                self.calc_t_th(&x, &mut tth);
                self.backend.hadamard_product(&bhb, &tth, &mut bhb_tth);
                self.backend.real(&bhb_tth, &mut a);
                self.backend.imag(&bhb_tth.column_sum(), &mut g);

                const THIRD: float = 1. / 3.;
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
                let phase = x[tr.idx()].rem_euclid(2.0 * PI);
                let amp = self.constraint.convert(1.0, 1.0);
                Drive { amp, phase }
            })
            .collect())
    }
}

impl<B: Backend> Default for LM<B> {
    fn default() -> Self {
        Self::new()
    }
}
