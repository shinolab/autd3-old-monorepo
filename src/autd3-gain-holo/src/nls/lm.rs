/*
 * File: lm.rs
 * Project: nls
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use crate::{constraint::Constraint, impl_holo, Complex, LinAlgBackend, Trans};
use autd3_core::{
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive, PI,
};
use autd3_traits::Gain;

/// Gain to produce multiple foci with Levenberg-Marquardt algorithm
///
/// References
/// * Levenberg, Kenneth. "A method for the solution of certain non-linear problems in least squares." Quarterly of applied mathematics 2.2 (1944): 164-168.
/// * Marquardt, Donald W. "An algorithm for least-squares estimation of nonlinear parameters." Journal of the society for Industrial and Applied Mathematics 11.2 (1963): 431-441.
/// * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
#[derive(Gain)]
pub struct LM<B: LinAlgBackend + 'static> {
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

impl<B: LinAlgBackend> LM<B> {
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

impl<B: LinAlgBackend> LM<B> {
    fn make_t(&self, zero: &B::VectorX, x: &B::VectorX, t: &mut B::VectorXc) {
        self.backend.make_complex2_v(zero, x, t);
        self.backend.scale_assign_cv(Complex::new(-1., 0.), t);
        self.backend.exp_assign_cv(t);
    }

    fn calc_jtj_jtf(
        &self,
        t: &B::VectorXc,
        bhb: &B::MatrixXc,
        tth: &mut B::MatrixXc,
        bhb_tth: &mut B::MatrixXc,
        bhb_tth_i: &mut B::MatrixX,
        jtj: &mut B::MatrixX,
        jtf: &mut B::VectorX,
    ) {
        self.backend.gevv_c(
            Trans::NoTrans,
            Trans::ConjTrans,
            Complex::new(1., 0.),
            &t,
            &t,
            Complex::new(0., 0.),
            tth,
        );
        self.backend.hadamard_product_cm(bhb, tth, bhb_tth);

        self.backend.real_cm(bhb_tth, jtj);
        self.backend.imag_cm(bhb_tth, bhb_tth_i);

        self.backend.reduce_col(bhb_tth_i, jtf);
    }

    fn calc_fx(
        &self,
        zero: &B::VectorX,
        x: &B::VectorX,
        bhb: &B::MatrixXc,
        tmp: &mut B::VectorXc,
        t: &mut B::VectorXc,
    ) -> float {
        self.backend.make_complex2_v(zero, x, t);
        self.backend.exp_assign_cv(t);
        self.backend.gemv_c(
            Trans::NoTrans,
            Complex::new(1., 0.),
            bhb,
            t,
            Complex::new(0., 0.),
            tmp,
        );
        self.backend.dot_c(t, tmp).re
    }
}

impl<B: LinAlgBackend, T: Transducer> Gain<T> for LM<B> {
    #[allow(clippy::many_single_char_names)]
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let m = geometry.num_transducers();
        let n = self.foci.len();

        let n_param = n + m;

        let mut bhb = self.backend.alloc_zeros_cm(n_param, n_param);
        {
            let mut amps = self.backend.from_slice_cv(&self.amps);

            let mut p = self.backend.alloc_cm(n, n);
            self.backend
                .scale_assign_cv(Complex::new(-1., 0.), &mut amps);
            self.backend.create_diagonal_c(&amps, &mut p);

            let g = self
                .backend
                .generate_propagation_matrix(geometry, &self.foci);

            let mut b = self.backend.alloc_cm(n, n_param);
            self.backend.concat_col_cm(&g, &p, &mut b);

            self.backend.gemm_c(
                Trans::ConjTrans,
                Trans::NoTrans,
                Complex::new(1., 0.),
                &b,
                &b,
                Complex::new(0., 0.),
                &mut bhb,
            );
        }

        let mut x = self.backend.alloc_v(n_param);
        self.backend.copy_from_slice_v(&self.initial, &mut x);

        let mut nu = 2.0;

        let zero = self.backend.alloc_zeros_v(n_param);

        let mut t = self.backend.alloc_cv(n_param);
        self.make_t(&zero, &x, &mut t);

        let mut tth = self.backend.alloc_cm(n_param, n_param);
        let mut bhb_tth = self.backend.alloc_cm(n_param, n_param);
        let mut bhb_tth_i = self.backend.alloc_m(n_param, n_param);
        let mut a = self.backend.alloc_m(n_param, n_param);
        let mut g = self.backend.alloc_v(n_param);
        self.calc_jtj_jtf(
            &t,
            &bhb,
            &mut tth,
            &mut bhb_tth,
            &mut bhb_tth_i,
            &mut a,
            &mut g,
        );

        let mut a_diag = self.backend.alloc_v(n_param);
        self.backend.get_diagonal(&a, &mut a_diag);
        let a_max = self.backend.max_v(&a_diag);

        let mut mu = self.tau * a_max;

        let mut tmp = self.backend.alloc_zeros_cv(n_param);
        let mut fx = self.calc_fx(&zero, &x, &bhb, &mut tmp, &mut t);

        let ones = vec![1.0; n_param];
        let ones = self.backend.from_slice_v(&ones);
        let mut identity = self.backend.alloc_m(n_param, n_param);
        self.backend.create_diagonal(&ones, &mut identity);

        let mut h_lm = self.backend.alloc_v(n_param);
        let mut x_new = self.backend.alloc_v(n_param);
        let mut tmp_mat = self.backend.alloc_m(n_param, n_param);
        let mut tmp_vec = self.backend.alloc_v(n_param);
        for _ in 0..self.k_max {
            if self.backend.max_v(&g) <= self.eps_1 {
                break;
            }

            self.backend.copy_to_m(&a, &mut tmp_mat);
            self.backend.add_m(mu, &identity, &mut tmp_mat);

            self.backend.copy_to_v(&g, &mut h_lm);

            self.backend.solve_inplace(&tmp_mat, &mut h_lm)?;

            if self.backend.dot(&h_lm, &h_lm).sqrt()
                <= self.eps_2 * (self.backend.dot(&x, &x).sqrt() + self.eps_2)
            {
                break;
            }

            self.backend.copy_to_v(&x, &mut x_new);
            self.backend.add_v(-1., &h_lm, &mut x_new);

            let fx_new = self.calc_fx(&zero, &x_new, &bhb, &mut tmp, &mut t);

            self.backend.copy_to_v(&g, &mut tmp_vec);
            self.backend.add_v(mu, &h_lm, &mut tmp_vec);

            let l0_lhlm = self.backend.dot(&h_lm, &tmp_vec) / 2.;

            let rho = (fx - fx_new) / l0_lhlm;
            fx = fx_new;

            if rho > 0. {
                self.backend.copy_to_v(&x_new, &mut x);

                self.make_t(&zero, &x, &mut t);

                self.calc_jtj_jtf(
                    &t,
                    &bhb,
                    &mut tth,
                    &mut bhb_tth,
                    &mut bhb_tth_i,
                    &mut a,
                    &mut g,
                );

                mu *= float::max(1. / 3., float::powi(1. - (2. * rho - 1.), 3));
                nu = 2.;
            } else {
                mu *= nu;
                nu *= 2.;
            }
        }

        let x = self.backend.to_host_v(x);
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
