/*
 * File: backend_nalgebra.rs
 * Project: src
 * Created Date: 07/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use nalgebra::ComplexField;
use rand::{thread_rng, Rng};

use autd3_core::float;

use crate::{error::HoloError, Backend, Complex, MatrixX, MatrixXc, VectorX, VectorXc};

#[derive(Default)]
pub struct NalgebraBackend {}

impl NalgebraBackend {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}

impl Backend for NalgebraBackend {
    fn gs(&self, repeat: usize, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError> {
        let m = g.nrows();
        let n = g.ncols();

        let q0 = VectorXc::from_element(n, Complex::new(1., 0.));
        let mut q = q0.clone();

        for _ in 0..repeat {
            let mut p = &g * q;
            for i in 0..m {
                p[i] = p[i] / p[i].abs() * amps[i];
            }
            q = g.adjoint() * p;
            for i in 0..n {
                q[i] = q[i] / q[i].abs() * q0[i];
            }
        }

        Ok(q)
    }

    fn gspat(&self, repeat: usize, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError> {
        let m = g.nrows();

        let denomi = g.column_sum();
        let b = g
            .map_with_location(|i, _, a| Complex::new(amps[i], 0.0) * a.conj() / denomi[i])
            .transpose();

        let r = g * &b;

        let mut p = VectorXc::from_iterator(m, amps.iter().map(|&a| Complex::new(a, 0.)));

        let mut gamma = &r * &p;
        for _ in 0..repeat {
            for i in 0..m {
                p[i] = gamma[i] / gamma[i].abs() * amps[i];
            }
            gamma = &r * &p;
        }

        for i in 0..m {
            p[i] = gamma[i] / gamma[i].norm_sqr() * amps[i] * amps[i];
        }

        Ok(b * p)
    }

    fn naive(&self, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError> {
        let m = g.nrows();
        let p = VectorXc::from_iterator(m, amps.iter().map(|&a| Complex::new(a, 0.0)));
        Ok(g.adjoint() * p)
    }

    fn evp(&self, gamma: float, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError> {
        let m = g.nrows();
        let n = g.ncols();

        let denomi = g.column_sum();
        let x = g
            .map_with_location(|i, _, a| Complex::new(amps[i], 0.0) * a.conj() / denomi[i])
            .transpose();

        let r = &g * x;
        let max_ev: VectorXc = {
            let eig = r.symmetric_eigen();
            eig.eigenvectors.column(eig.eigenvalues.imax()).into()
        };

        let sigma = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            n,
            g.column_iter()
                .map(|col| {
                    col.iter()
                        .zip(amps.iter())
                        .map(|(a, &amp)| a.abs() * amp)
                        .sum()
                })
                .map(|s: float| Complex::new((s / m as float).sqrt().powf(gamma), 0.0)),
        ));

        let gr = {
            let arows = g.nrows();
            let acols = g.ncols();
            let mut gr = g.resize(arows + sigma.nrows(), acols, Default::default());
            gr.view_mut((arows, 0), (sigma.nrows(), sigma.ncols()))
                .copy_from(&sigma);

            gr
        };
        let f = VectorXc::from_iterator(
            m + n,
            amps.iter()
                .zip(max_ev.iter())
                .map(|(amp, &e)| amp * e / e.abs())
                .chain((0..n).map(|_| Complex::new(0., 0.))),
        );

        let gtg = gr.adjoint() * &gr;

        let mut gtf = gr.adjoint() * f;

        if !gtg.qr().solve_mut(&mut gtf) {
            return Err(HoloError::SolveFailed);
        }

        Ok(gtf)
    }

    fn sdp(
        &self,
        alpha: float,
        repeat: usize,
        lambda: float,
        amps: &[float],
        g: MatrixXc,
    ) -> Result<VectorXc, HoloError> {
        let m = g.nrows();

        let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            m,
            amps.iter().map(|&a| Complex::new(a, 0.)),
        ));
        let b = g;

        let pseudo_inv_b = {
            let svd = b.clone().svd(true, true);
            let s_inv = MatrixXc::from_diagonal(
                &svd.singular_values
                    .map(|s| Complex::new(s / (s * s + alpha * alpha), 0.)),
            );
            match (&svd.v_t, &svd.u) {
                (Some(v_t), Some(u)) => v_t.adjoint() * s_inv * u.adjoint(),
                _ => unreachable!(),
            }
        };

        let mm = b * &pseudo_inv_b - MatrixXc::identity(m, m);

        let mm = &p * mm * &p;

        let mut x_mat = MatrixXc::identity(m, m);

        let mut rng = thread_rng();
        let zero = VectorXc::zeros(m);

        fn set_bcd_result(mat: &mut MatrixXc, vec: &VectorXc, idx: usize) {
            let m = vec.len();
            mat.view_mut((idx, 0), (1, idx))
                .copy_from(&vec.view((0, 0), (idx, 1)).adjoint());
            mat.view_mut((idx, idx + 1), (1, m - idx - 1))
                .copy_from(&vec.view((0, 0), (m - idx - 1, 1)).adjoint());
            mat.view_mut((0, idx), (idx, 1))
                .copy_from(&vec.view((0, 0), (idx, 1)));
            mat.view_mut((idx + 1, idx), (m - idx - 1, 1))
                .copy_from(&vec.view((0, 0), (m - idx - 1, 1)));
        }

        for _ in 0..repeat {
            let ii = (m as float * rng.gen_range(0.0..1.0)) as usize;

            let mut mmc: VectorXc = mm.column(ii).into();
            mmc[ii] = Complex::new(0., 0.);

            let mut x = &x_mat * &mmc;
            let gamma = x.dotc(&mmc);
            if gamma.real() > 0.0 {
                x *= Complex::new((lambda / gamma.real()).sqrt(), 0.);
                set_bcd_result(&mut x_mat, &x, ii);
            } else {
                set_bcd_result(&mut x_mat, &zero, ii);
            }
        }

        let u: VectorXc = {
            let eig = x_mat.symmetric_eigen();
            eig.eigenvectors.column(eig.eigenvalues.imax()).into()
        };

        let ut = p * u;

        Ok(pseudo_inv_b * ut)
    }

    fn lm(
        &self,
        eps1: float,
        eps2: float,
        tau: float,
        kmax: usize,
        initial: &[float],
        amps: &[float],
        g: MatrixXc,
    ) -> Result<VectorX, HoloError> {
        let m = g.nrows();
        let n = g.ncols();
        let n_param = n + m;

        let bhb = {
            let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
                m,
                amps.iter().map(|a| Complex::new(-a, 0.)),
            ));

            let rows = g.nrows();
            let cols = g.ncols();
            let mut b = g.resize(rows, cols + p.ncols(), Default::default());
            b.view_mut((0, cols), (p.nrows(), p.ncols())).copy_from(&p);

            b.adjoint() * b
        };

        let mut x = VectorX::zeros(n_param);
        x.view_mut((0, 0), (initial.len(), 1))
            .copy_from_slice(initial);

        let mut nu = 2.0;

        fn calc_t_th(x: &VectorX) -> MatrixXc {
            let len = x.len();
            let t = MatrixXc::from_iterator(len, 1, x.iter().map(|v| Complex::new(0., -v).exp()));
            &t * t.adjoint()
        }

        let mut tth = calc_t_th(&x);

        let mut bhb_tth = bhb.component_mul(&tth);

        let mut a = bhb_tth.map(|v| v.re);
        let mut g = bhb_tth.column_sum().map(|v| v.im);

        let a_max = a.diagonal().max();

        let mut mu = tau * a_max;

        let mut t = VectorXc::from_iterator(x.len(), x.iter().map(|&v| Complex::new(0., v).exp()));

        let tmp_vec_c = &bhb * &t;
        let mut fx = t.dotc(&tmp_vec_c).real();

        let identity = MatrixX::identity(n_param, n_param);
        for _ in 0..kmax {
            if g.camax().abs() <= eps1 {
                break;
            }

            let tmp_mat = &a + &identity * mu;
            let mut h_lm = g.clone();
            if !tmp_mat.qr().solve_mut(&mut h_lm) {
                return Err(HoloError::SolveFailed);
            }
            if h_lm.norm() <= eps2 * (x.norm() * eps2) {
                break;
            }

            let x_new = &x - &h_lm;
            t = VectorXc::from_iterator(
                x_new.len(),
                x_new.iter().map(|&v| Complex::new(0., v).exp()),
            );

            let tmp_vec_c = &bhb * &t;
            let fx_new = t.dotc(&tmp_vec_c).real();

            let tmp_vec = &g + mu * &h_lm;

            let l0_lhlm = h_lm.dot(&tmp_vec) / 2.0;
            let rho = (fx - fx_new) / l0_lhlm;
            fx = fx_new;

            if rho > 0. {
                x = x_new;
                tth = calc_t_th(&x);
                bhb_tth = bhb.component_mul(&tth);
                a = bhb_tth.map(|v| v.re);
                g = bhb_tth.column_sum().map(|v| v.im);

                const THIRD: float = 1. / 3.;
                mu *= THIRD.max((1. - (2. * rho - 1.)).powi(3));
                nu = 2.0;
            } else {
                mu *= nu;
                nu *= 2.0;
            }
        }

        Ok(x)
    }
}
