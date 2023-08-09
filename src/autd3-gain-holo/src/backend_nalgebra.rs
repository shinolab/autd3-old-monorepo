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

    fn from_slice_m(&self, rows: usize, cols: usize, v: &[float]) -> Self::MatrixX {
        Self::MatrixX::from_iterator(rows, cols, v.iter().copied())
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

    fn clone_m(&self, v: &Self::MatrixX) -> Self::MatrixX {
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

    fn get_col_c(&self, a: &Self::MatrixXc, col: usize, v: &mut Self::VectorXc) {
        *v = a.column(col).into();
    }

    fn set_cv(&self, i: usize, val: Complex, v: &mut Self::VectorXc) {
        v[i] = val;
    }

    fn set_col_c(
        &self,
        a: &Self::VectorXc,
        col: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) {
        v.view_mut((start, col), (end - start, 1))
            .copy_from(&a.view((start, 0), (end - start, 1)));
    }

    fn set_row_c(
        &self,
        a: &Self::VectorXc,
        row: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) {
        v.view_mut((row, start), (1, end - start))
            .copy_from(&a.view((start, 0), (end - start, 1)).transpose());
    }

    fn scale_assign_cv(&self, a: Complex, b: &mut Self::VectorXc) {
        b.apply(|x| *x *= a)
    }

    fn conj_assign_v(&self, b: &mut Self::VectorXc) {
        b.apply(|x| *x = x.conj())
    }

    fn dot_c(&self, x: &Self::VectorXc, y: &Self::VectorXc) -> Complex {
        x.dotc(y)
    }

    fn pseudo_inverse_svd(
        &self,
        a: Self::MatrixXc,
        alpha: float,
        _u: &mut Self::MatrixXc,
        _s: &mut Self::MatrixXc,
        _vt: &mut Self::MatrixXc,
        _buf: &mut Self::MatrixXc,
        b: &mut Self::MatrixXc,
    ) {
        let svd = a.svd(true, true);
        let s_inv = MatrixXc::from_diagonal(
            &svd.singular_values
                .map(|s| Complex::new(s / (s * s + alpha * alpha), 0.)),
        );
        match (&svd.v_t, &svd.u) {
            (Some(v_t), Some(u)) => *b = v_t.adjoint() * s_inv * u.adjoint(),
            _ => unreachable!(),
        }
    }

    fn alloc_m(&self, rows: usize, cols: usize) -> Self::MatrixX {
        Self::MatrixX::zeros(rows, cols)
    }

    fn to_host_m(&self, v: Self::MatrixX) -> MatrixX {
        v
    }

    fn copy_from_slice_v(&self, v: &[float], dst: &mut Self::VectorX) {
        dst.view_mut((0, 0), (v.len(), 1)).copy_from_slice(v)
    }

    fn copy_to_v(&self, src: &Self::VectorX, dst: &mut Self::VectorX) {
        dst.copy_from(src)
    }

    fn copy_to_m(&self, src: &Self::MatrixX, dst: &mut Self::MatrixX) {
        dst.copy_from(src)
    }

    fn create_diagonal(&self, v: &Self::VectorX, a: &mut Self::MatrixX) {
        a.fill(0.);
        a.set_diagonal(&v)
    }

    fn get_diagonal(&self, a: &Self::MatrixX, v: &mut Self::VectorX) {
        *v = a.diagonal();
    }

    fn real_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) {
        *b = a.map(|v| v.re);
    }

    fn imag_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) {
        *b = a.map(|v| v.im);
    }

    fn exp_assign_cv(&self, v: &mut Self::VectorXc) {
        v.apply(|v| *v = v.exp())
    }

    fn concat_col_cm(&self, a: &Self::MatrixXc, b: &Self::MatrixXc, c: &mut Self::MatrixXc) {
        c.view_mut((0, 0), (a.nrows(), a.ncols())).copy_from(&a);
        c.view_mut((0, a.ncols()), (b.nrows(), b.ncols()))
            .copy_from(&b);
    }

    fn max_v(&self, m: &Self::VectorX) -> float {
        m.max()
    }

    fn hadamard_product_cm(&self, x: &Self::MatrixXc, y: &Self::MatrixXc, z: &mut Self::MatrixXc) {
        *z = x.component_mul(y)
    }

    fn dot(&self, x: &Self::VectorX, y: &Self::VectorX) -> float {
        x.dot(&y)
    }

    fn add_v(&self, alpha: float, a: &Self::VectorX, b: &mut Self::VectorX) {
        *b += alpha * a;
    }

    fn add_m(&self, alpha: float, a: &Self::MatrixX, b: &mut Self::MatrixX) {
        *b += alpha * a;
    }

    fn gevv_c(
        &self,
        trans_a: crate::Trans,
        trans_b: crate::Trans,
        alpha: Complex,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
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

    fn solve_inplace(&self, a: &Self::MatrixX, x: &mut Self::VectorX) -> Result<(), HoloError> {
        if !a.clone().qr().solve_mut(x) {
            return Err(HoloError::SolveFailed);
        }
        Ok(())
    }

    fn reduce_col(&self, a: &Self::MatrixX, b: &mut Self::VectorX) {
        *b = a.column_sum();
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
