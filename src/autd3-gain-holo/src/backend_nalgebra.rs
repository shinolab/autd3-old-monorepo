/*
 * File: backend_nalgebra.rs
 * Project: src
 * Created Date: 07/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use nalgebra::ComplexField;

use autd3_core::{
    acoustics::{propagate_tr, Sphere},
    float,
};

use crate::{error::HoloError, Complex, LinAlgBackend, MatrixX, MatrixXc, VectorX, VectorXc};

/// Backend using nalgebra
#[derive(Default)]
pub struct NalgebraBackend {}

//     fn sdp(
//         &self,
//         alpha: float,
//         repeat: usize,
//         lambda: float,
//         amps: &[float],
//         g: MatrixXc,
//     ) -> Result<VectorXc, HoloError> {
//         let m = g.nrows();

//         let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
//             m,
//             amps.iter().map(|&a| Complex::new(a, 0.)),
//         ));
//         let b = g;

//         let pseudo_inv_b = {
//             let svd = b.clone().svd(true, true);
//             let s_inv = MatrixXc::from_diagonal(
//                 &svd.singular_values
//                     .map(|s| Complex::new(s / (s * s + alpha * alpha), 0.)),
//             );
//             match (&svd.v_t, &svd.u) {
//                 (Some(v_t), Some(u)) => v_t.adjoint() * s_inv * u.adjoint(),
//                 _ => unreachable!(),
//             }
//         };

//         let mm = b * &pseudo_inv_b - MatrixXc::identity(m, m);

//         let mm = &p * mm * &p;

//         let mut x_mat = MatrixXc::identity(m, m);

//         let mut rng = thread_rng();
//         let zero = VectorXc::zeros(m);

//         fn set_bcd_result(mat: &mut MatrixXc, vec: &VectorXc, idx: usize) {
//             let m = vec.len();
//             mat.view_mut((idx, 0), (1, idx))
//                 .copy_from(&vec.view((0, 0), (idx, 1)).adjoint());
//             mat.view_mut((idx, idx + 1), (1, m - idx - 1))
//                 .copy_from(&vec.view((0, 0), (m - idx - 1, 1)).adjoint());
//             mat.view_mut((0, idx), (idx, 1))
//                 .copy_from(&vec.view((0, 0), (idx, 1)));
//             mat.view_mut((idx + 1, idx), (m - idx - 1, 1))
//                 .copy_from(&vec.view((0, 0), (m - idx - 1, 1)));
//         }

//         for _ in 0..repeat {
//             let ii = (m as float * rng.gen_range(0.0..1.0)) as usize;

//             let mut mmc: VectorXc = mm.column(ii).into();
//             mmc[ii] = Complex::new(0., 0.);

//             let mut x = &x_mat * &mmc;
//             let gamma = x.dotc(&mmc);
//             if gamma.real() > 0.0 {
//                 x *= Complex::new((lambda / gamma.real()).sqrt(), 0.);
//                 set_bcd_result(&mut x_mat, &x, ii);
//             } else {
//                 set_bcd_result(&mut x_mat, &zero, ii);
//             }
//         }

//         let u: VectorXc = {
//             let eig = x_mat.symmetric_eigen();
//             eig.eigenvectors.column(eig.eigenvalues.imax()).into()
//         };

//         let ut = p * u;

//         Ok(pseudo_inv_b * ut)
//     }

//     fn lm(
//         &self,
//         eps1: float,
//         eps2: float,
//         tau: float,
//         kmax: usize,
//         initial: &[float],
//         amps: &[float],
//         g: MatrixXc,
//     ) -> Result<VectorX, HoloError> {
//         let m = g.nrows();
//         let n = g.ncols();
//         let n_param = n + m;

//         let bhb = {
//             let p = MatrixXc::from_diagonal(&VectorXc::from_iterator(
//                 m,
//                 amps.iter().map(|a| Complex::new(-a, 0.)),
//             ));

//             let rows = g.nrows();
//             let cols = g.ncols();
//             let mut b = g.resize(rows, cols + p.ncols(), Default::default());
//             b.view_mut((0, cols), (p.nrows(), p.ncols())).copy_from(&p);

//             b.adjoint() * b
//         };

//         let mut x = VectorX::zeros(n_param);
//         x.view_mut((0, 0), (initial.len(), 1))
//             .copy_from_slice(initial);

//         let mut nu = 2.0;

//         fn calc_t_th(x: &VectorX) -> MatrixXc {
//             let len = x.len();
//             let t = MatrixXc::from_iterator(len, 1, x.iter().map(|v| Complex::new(0., -v).exp()));
//             &t * t.adjoint()
//         }

//         let mut tth = calc_t_th(&x);

//         let mut bhb_tth = bhb.component_mul(&tth);

//         let mut a = bhb_tth.map(|v| v.re);
//         let mut g = bhb_tth.column_sum().map(|v| v.im);

//         let a_max = a.diagonal().max();

//         let mut mu = tau * a_max;

//         let mut t = VectorXc::from_iterator(x.len(), x.iter().map(|&v| Complex::new(0., v).exp()));

//         let tmp_vec_c = &bhb * &t;
//         let mut fx = t.dotc(&tmp_vec_c).real();

//         let identity = MatrixX::identity(n_param, n_param);
//         for _ in 0..kmax {
//             if g.camax().abs() <= eps1 {
//                 break;
//             }

//             let tmp_mat = &a + &identity * mu;
//             let mut h_lm = g.clone();
//             if !tmp_mat.qr().solve_mut(&mut h_lm) {
//                 return Err(HoloError::SolveFailed);
//             }
//             if h_lm.norm() <= eps2 * (x.norm() * eps2) {
//                 break;
//             }

//             let x_new = &x - &h_lm;
//             t = VectorXc::from_iterator(
//                 x_new.len(),
//                 x_new.iter().map(|&v| Complex::new(0., v).exp()),
//             );

//             let tmp_vec_c = &bhb * &t;
//             let fx_new = t.dotc(&tmp_vec_c).real();

//             let tmp_vec = &g + mu * &h_lm;

//             let l0_lhlm = h_lm.dot(&tmp_vec) / 2.0;
//             let rho = (fx - fx_new) / l0_lhlm;
//             fx = fx_new;

//             if rho > 0. {
//                 x = x_new;
//                 tth = calc_t_th(&x);
//                 bhb_tth = bhb.component_mul(&tth);
//                 a = bhb_tth.map(|v| v.re);
//                 g = bhb_tth.column_sum().map(|v| v.im);

//                 const THIRD: float = 1. / 3.;
//                 mu *= THIRD.max((1. - (2. * rho - 1.)).powi(3));
//                 nu = 2.0;
//             } else {
//                 mu *= nu;
//                 nu *= 2.0;
//             }
//         }

//         Ok(x)
//     }
// }

impl LinAlgBackend for NalgebraBackend {
    type MatrixXc = MatrixXc;
    type MatrixX = MatrixX;
    type VectorXc = VectorXc;
    type VectorX = VectorX;

    fn new() -> Result<Rc<Self>, HoloError> {
        Ok(Rc::new(Self {}))
    }

    fn generate_propagation_matrix<T: autd3_core::geometry::Transducer>(
        &self,
        geometry: &autd3_core::geometry::Geometry<T>,
        foci: &[autd3_core::geometry::Vector3],
    ) -> Self::MatrixXc {
        MatrixXc::from_iterator(
            foci.len(),
            geometry.num_transducers(),
            geometry.transducers().flat_map(|trans| {
                foci.iter().map(move |fp| {
                    propagate_tr::<Sphere, T>(trans, geometry.attenuation, geometry.sound_speed, fp)
                })
            }),
        )
    }

    fn to_host_cv(&self, v: Self::VectorXc) -> VectorXc {
        v
    }

    fn to_host_v(&self, v: Self::VectorX) -> VectorX {
        v
    }

    fn to_host_cm(&self, v: Self::MatrixXc) -> MatrixXc {
        v
    }

    fn alloc_v(&self, size: usize) -> Self::VectorX {
        Self::VectorX::zeros(size)
    }

    fn alloc_zeros_v(&self, size: usize) -> Self::VectorX {
        Self::VectorX::zeros(size)
    }

    fn alloc_zeros_cv(&self, size: usize) -> Self::VectorXc {
        Self::VectorXc::zeros(size)
    }

    fn from_slice_v(&self, v: &[float]) -> Self::VectorX {
        Self::VectorX::from_row_slice(v)
    }

    fn make_complex2_v(&self, real: &Self::VectorX, imag: &Self::VectorX, v: &mut Self::VectorXc) {
        *v = Self::VectorXc::from_iterator(
            real.len(),
            real.iter()
                .zip(imag.iter())
                .map(|(&r, &i)| Complex::new(r, i)),
        )
    }

    fn gemv_c(
        &self,
        trans: crate::Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        x: &Self::VectorXc,
        beta: Complex,
        y: &mut Self::VectorXc,
    ) {
        match trans {
            crate::Trans::NoTrans => y.gemv(alpha, a, x, beta),
            crate::Trans::Trans => y.gemv_tr(alpha, a, x, beta),
            crate::Trans::ConjTrans => y.gemv_ad(alpha, a, x, beta),
        }
    }

    fn normalize_assign_cv(&self, v: &mut Self::VectorXc) {
        v.apply(|v| *v = *v / v.abs())
    }

    fn hadamard_product_assign_cv(&self, x: &Self::VectorXc, y: &mut Self::VectorXc) {
        y.component_mul_assign(x)
    }

    fn hadamard_product_cv(&self, x: &Self::VectorXc, y: &Self::VectorXc, z: &mut Self::VectorXc) {
        *z = x.component_mul(y)
    }

    fn alloc_cv(&self, size: usize) -> Self::VectorXc {
        Self::VectorXc::zeros(size)
    }

    fn alloc_zeros_cm(&self, rows: usize, cols: usize) -> Self::MatrixXc {
        Self::MatrixXc::zeros(rows, cols)
    }

    fn get_diagonal_c(&self, a: &Self::MatrixXc, v: &mut Self::VectorXc) {
        *v = a.diagonal()
    }

    fn create_diagonal_c(&self, v: &Self::VectorXc, a: &mut Self::MatrixXc) {
        a.fill(Complex::new(0., 0.));
        a.set_diagonal(&v)
    }

    fn reciprocal_assign_c(&self, v: &mut Self::VectorXc) {
        v.apply(|v| *v = Complex::new(1., 0.) / *v);
    }

    fn abs_cv(&self, a: &Self::VectorXc, b: &mut Self::VectorX) {
        *b = a.map(|v| v.abs())
    }

    fn scale_assign_v(&self, a: float, b: &mut Self::VectorX) {
        *b *= a;
    }

    fn sqrt_assign_v(&self, v: &mut Self::VectorX) {
        v.apply(|v| *v = v.sqrt())
    }

    fn gemm_c(
        &self,
        trans_a: crate::Trans,
        trans_b: crate::Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        beta: Complex,
        y: &mut Self::MatrixXc,
    ) {
        match trans_a {
            crate::Trans::NoTrans => match trans_b {
                crate::Trans::NoTrans => y.gemm(alpha, a, b, beta),
                crate::Trans::Trans => y.gemm(alpha, a, &b.transpose(), beta),
                crate::Trans::ConjTrans => y.gemm(alpha, a, &b.adjoint(), beta),
            },
            crate::Trans::Trans => match trans_b {
                crate::Trans::NoTrans => y.gemm_tr(alpha, a, b, beta),
                crate::Trans::Trans => y.gemm_tr(alpha, a, &b.transpose(), beta),
                crate::Trans::ConjTrans => y.gemm_tr(alpha, a, &b.adjoint(), beta),
            },
            crate::Trans::ConjTrans => match trans_b {
                crate::Trans::NoTrans => y.gemm_ad(alpha, a, b, beta),
                crate::Trans::Trans => y.gemm_ad(alpha, a, &b.transpose(), beta),
                crate::Trans::ConjTrans => y.gemm_ad(alpha, a, &b.adjoint(), beta),
            },
        }
    }

    fn alloc_cm(&self, rows: usize, cols: usize) -> Self::MatrixXc {
        Self::MatrixXc::zeros(rows, cols)
    }

    fn clone_v(&self, v: &Self::VectorX) -> Self::VectorX {
        v.clone()
    }

    fn clone_cv(&self, v: &Self::VectorXc) -> Self::VectorXc {
        v.clone()
    }

    fn clone_cm(&self, v: &Self::MatrixXc) -> Self::MatrixXc {
        v.clone()
    }

    fn gen_back_prop(
        &self,
        m: usize,
        n: usize,
        transfer: &Self::MatrixXc,
        amps: &Self::VectorXc,
        b: &mut Self::MatrixXc,
    ) {
        (0..n).for_each(|i| {
            let x = amps[i]
                / transfer
                    .rows(i, 1)
                    .iter()
                    .map(|x| x.norm_sqr())
                    .sum::<float>();
            (0..m).for_each(|j| {
                b[(j, i)] = transfer[(i, j)].conj() * x;
            })
        });
    }

    fn max_eigen_vector_c(&self, m: Self::MatrixXc) -> Self::VectorXc {
        let eig = m.symmetric_eigen();
        eig.eigenvectors.column(eig.eigenvalues.imax()).into()
    }

    fn from_slice_cv(&self, real: &[float]) -> Self::VectorXc {
        Self::VectorXc::from_iterator(real.len(), real.iter().map(|&r| Complex::new(r, 0.)))
    }

    fn from_slice2_cv(&self, r: &[float], i: &[float]) -> Self::VectorXc {
        Self::VectorXc::from_iterator(
            r.len(),
            r.iter().zip(i.iter()).map(|(&r, &i)| Complex::new(r, i)),
        )
    }

    fn from_slice2_cm(&self, rows: usize, cols: usize, r: &[float], i: &[float]) -> Self::MatrixXc {
        Self::MatrixXc::from_iterator(
            rows,
            cols,
            r.iter().zip(i.iter()).map(|(&r, &i)| Complex::new(r, i)),
        )
    }

    fn pow_assign_v(&self, a: float, v: &mut Self::VectorX) {
        v.apply(|v| *v = v.powf(a))
    }

    fn concat_row_cm(&self, a: &Self::MatrixXc, b: &Self::MatrixXc, c: &mut Self::MatrixXc) {
        c.view_mut((0, 0), (a.nrows(), a.ncols())).copy_from(&a);
        c.view_mut((a.nrows(), 0), (b.nrows(), b.ncols()))
            .copy_from(&b);
    }

    fn concat_col_cv(&self, a: &Self::VectorXc, b: &Self::VectorXc, c: &mut Self::VectorXc) {
        *c = VectorXc::from_iterator(a.len() + b.len(), a.iter().chain(b.iter()).cloned());
    }

    fn solve_inplace_h(&self, a: Self::MatrixXc, x: &mut Self::VectorXc) -> Result<(), HoloError> {
        if !a.qr().solve_mut(x) {
            return Err(HoloError::SolveFailed);
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "test-utilities"))]
mod tests {
    use super::*;

    use crate::test_utilities::test_utils::*;

    #[test]
    fn test_nalgebra_backend() {
        LinAlgBackendTestHelper::<10, NalgebraBackend>::new()
            .unwrap()
            .test();
    }
}
