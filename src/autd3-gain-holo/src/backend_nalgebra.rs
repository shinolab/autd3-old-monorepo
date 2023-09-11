/*
 * File: backend_nalgebra.rs
 * Project: src
 * Created Date: 07/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use nalgebra::ComplexField;

use autd3_driver::{
    acoustics::{propagate_tr, Sphere},
    datagram::GainFilter,
    defined::float,
    geometry::{Geometry},
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

    fn generate_propagation_matrix<T: autd3_driver::geometry::Transducer>(
        &self,
        geometry: &Geometry<T>,
        foci: &[autd3_driver::geometry::Vector3],
        filter: &GainFilter,
    ) -> Result<Self::MatrixXc, HoloError> {
        match filter {
            GainFilter::All => Ok(MatrixXc::from_iterator(
                foci.len(),
                geometry
                    .devices()
                    .map(|dev| dev.num_transducers())
                    .sum::<usize>(),
                geometry.devices().flat_map(|dev| {
                    dev.iter().flat_map(move |tr| {
                        foci.iter().map(move |fp| {
                            propagate_tr::<Sphere, T>(tr, dev.attenuation, dev.sound_speed, fp)
                        })
                    })
                }),
            )),
            GainFilter::Filter(filter) => {
                let iter = geometry
                    .devices()
                    .flat_map(|dev| {
                        dev.iter().filter_map(move |tr| {
                            if let Some(filter) = filter.get(&dev.idx()) {
                                if filter[tr.local_idx()] {
                                    Some(foci.iter().map(move |fp| {
                                        propagate_tr::<Sphere, T>(
                                            tr,
                                            dev.attenuation,
                                            dev.sound_speed,
                                            fp,
                                        )
                                    }))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                    })
                    .flatten()
                    .collect::<Vec<_>>();
                Ok(MatrixXc::from_iterator(
                    foci.len(),
                    iter.len() / foci.len(),
                    iter,
                ))
            }
        }
    }

    fn to_host_cv(&self, v: Self::VectorXc) -> Result<VectorXc, HoloError> {
        Ok(v)
    }

    fn to_host_v(&self, v: Self::VectorX) -> Result<VectorX, HoloError> {
        Ok(v)
    }

    fn to_host_cm(&self, v: Self::MatrixXc) -> Result<MatrixXc, HoloError> {
        Ok(v)
    }

    fn alloc_v(&self, size: usize) -> Result<Self::VectorX, HoloError> {
        Ok(Self::VectorX::zeros(size))
    }

    fn alloc_zeros_v(&self, size: usize) -> Result<Self::VectorX, HoloError> {
        Ok(Self::VectorX::zeros(size))
    }

    fn alloc_zeros_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError> {
        Ok(Self::VectorXc::zeros(size))
    }

    fn from_slice_v(&self, v: &[float]) -> Result<Self::VectorX, HoloError> {
        Ok(Self::VectorX::from_row_slice(v))
    }

    fn from_slice_m(
        &self,
        rows: usize,
        cols: usize,
        v: &[float],
    ) -> Result<Self::MatrixX, HoloError> {
        Ok(Self::MatrixX::from_iterator(rows, cols, v.iter().copied()))
    }

    fn make_complex2_v(
        &self,
        real: &Self::VectorX,
        imag: &Self::VectorX,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *v = Self::VectorXc::from_iterator(
            real.len(),
            real.iter()
                .zip(imag.iter())
                .map(|(&r, &i)| Complex::new(r, i)),
        );
        Ok(())
    }

    fn gemv_c(
        &self,
        trans: crate::Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        x: &Self::VectorXc,
        beta: Complex,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        match trans {
            crate::Trans::NoTrans => y.gemv(alpha, a, x, beta),
            crate::Trans::Trans => y.gemv_tr(alpha, a, x, beta),
            crate::Trans::ConjTrans => y.gemv_ad(alpha, a, x, beta),
        }
        Ok(())
    }

    fn normalize_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        v.apply(|v| *v /= v.abs());
        Ok(())
    }

    fn hadamard_product_assign_cv(
        &self,
        x: &Self::VectorXc,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        y.component_mul_assign(x);
        Ok(())
    }

    fn hadamard_product_cv(
        &self,
        x: &Self::VectorXc,
        y: &Self::VectorXc,
        z: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *z = x.component_mul(y);
        Ok(())
    }

    fn alloc_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError> {
        Ok(Self::VectorXc::zeros(size))
    }

    fn alloc_zeros_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError> {
        Ok(Self::MatrixXc::zeros(rows, cols))
    }

    fn get_diagonal_c(&self, a: &Self::MatrixXc, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        *v = a.diagonal();
        Ok(())
    }

    fn create_diagonal_c(
        &self,
        v: &Self::VectorXc,
        a: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        a.fill(Complex::new(0., 0.));
        a.set_diagonal(v);
        Ok(())
    }

    fn reciprocal_assign_c(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        v.apply(|v| *v = Complex::new(1., 0.) / *v);
        Ok(())
    }

    fn abs_cv(&self, a: &Self::VectorXc, b: &mut Self::VectorX) -> Result<(), HoloError> {
        *b = a.map(|v| v.abs());
        Ok(())
    }

    fn scale_assign_v(&self, a: float, b: &mut Self::VectorX) -> Result<(), HoloError> {
        *b *= a;
        Ok(())
    }

    fn sqrt_assign_v(&self, v: &mut Self::VectorX) -> Result<(), HoloError> {
        v.apply(|v| *v = v.sqrt());
        Ok(())
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
    ) -> Result<(), HoloError> {
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
        Ok(())
    }

    fn alloc_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError> {
        Ok(Self::MatrixXc::zeros(rows, cols))
    }

    fn clone_v(&self, v: &Self::VectorX) -> Result<Self::VectorX, HoloError> {
        Ok(v.clone())
    }

    fn clone_m(&self, v: &Self::MatrixX) -> Result<Self::MatrixX, HoloError> {
        Ok(v.clone())
    }

    fn clone_cv(&self, v: &Self::VectorXc) -> Result<Self::VectorXc, HoloError> {
        Ok(v.clone())
    }

    fn clone_cm(&self, v: &Self::MatrixXc) -> Result<Self::MatrixXc, HoloError> {
        Ok(v.clone())
    }

    fn gen_back_prop(
        &self,
        m: usize,
        n: usize,
        transfer: &Self::MatrixXc,
        amps: &Self::VectorXc,
        b: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
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
        Ok(())
    }

    fn max_eigen_vector_c(&self, m: Self::MatrixXc) -> Result<Self::VectorXc, HoloError> {
        let eig = m.symmetric_eigen();
        Ok(eig.eigenvectors.column(eig.eigenvalues.imax()).into())
    }

    fn from_slice_cv(&self, real: &[float]) -> Result<Self::VectorXc, HoloError> {
        Ok(Self::VectorXc::from_iterator(
            real.len(),
            real.iter().map(|&r| Complex::new(r, 0.)),
        ))
    }

    fn from_slice2_cv(&self, r: &[float], i: &[float]) -> Result<Self::VectorXc, HoloError> {
        Ok(Self::VectorXc::from_iterator(
            r.len(),
            r.iter().zip(i.iter()).map(|(&r, &i)| Complex::new(r, i)),
        ))
    }

    fn from_slice2_cm(
        &self,
        rows: usize,
        cols: usize,
        r: &[float],
        i: &[float],
    ) -> Result<Self::MatrixXc, HoloError> {
        Ok(Self::MatrixXc::from_iterator(
            rows,
            cols,
            r.iter().zip(i.iter()).map(|(&r, &i)| Complex::new(r, i)),
        ))
    }

    fn pow_assign_v(&self, a: float, v: &mut Self::VectorX) -> Result<(), HoloError> {
        v.apply(|v| *v = v.powf(a));
        Ok(())
    }

    fn concat_row_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        c.view_mut((0, 0), (a.nrows(), a.ncols())).copy_from(a);
        c.view_mut((a.nrows(), 0), (b.nrows(), b.ncols()))
            .copy_from(b);
        Ok(())
    }

    fn concat_col_cv(
        &self,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
        c: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *c = VectorXc::from_iterator(a.len() + b.len(), a.iter().chain(b.iter()).cloned());
        Ok(())
    }

    fn solve_inplace_h(&self, a: Self::MatrixXc, x: &mut Self::VectorXc) -> Result<(), HoloError> {
        if !a.qr().solve_mut(x) {
            return Err(HoloError::SolveFailed);
        }
        Ok(())
    }

    fn get_col_c(
        &self,
        a: &Self::MatrixXc,
        col: usize,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *v = a.column(col).into();
        Ok(())
    }

    fn set_cv(&self, i: usize, val: Complex, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        v[i] = val;
        Ok(())
    }

    fn set_col_c(
        &self,
        a: &Self::VectorXc,
        col: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        v.view_mut((start, col), (end - start, 1))
            .copy_from(&a.view((start, 0), (end - start, 1)));
        Ok(())
    }

    fn set_row_c(
        &self,
        a: &Self::VectorXc,
        row: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        v.view_mut((row, start), (1, end - start))
            .copy_from(&a.view((start, 0), (end - start, 1)).transpose());
        Ok(())
    }

    fn scale_assign_cv(&self, a: Complex, b: &mut Self::VectorXc) -> Result<(), HoloError> {
        b.apply(|x| *x *= a);
        Ok(())
    }

    fn conj_assign_v(&self, b: &mut Self::VectorXc) -> Result<(), HoloError> {
        b.apply(|x| *x = x.conj());
        Ok(())
    }

    fn dot_c(&self, x: &Self::VectorXc, y: &Self::VectorXc) -> Result<Complex, HoloError> {
        Ok(x.dotc(y))
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
    ) -> Result<(), HoloError> {
        let svd = a.svd(true, true);
        let s_inv = MatrixXc::from_diagonal(
            &svd.singular_values
                .map(|s| Complex::new(s / (s * s + alpha * alpha), 0.)),
        );
        match (&svd.v_t, &svd.u) {
            (Some(v_t), Some(u)) => *b = v_t.adjoint() * s_inv * u.adjoint(),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn alloc_m(&self, rows: usize, cols: usize) -> Result<Self::MatrixX, HoloError> {
        Ok(Self::MatrixX::zeros(rows, cols))
    }

    fn to_host_m(&self, v: Self::MatrixX) -> Result<MatrixX, HoloError> {
        Ok(v)
    }

    fn copy_from_slice_v(&self, v: &[float], dst: &mut Self::VectorX) -> Result<(), HoloError> {
        dst.view_mut((0, 0), (v.len(), 1)).copy_from_slice(v);
        Ok(())
    }

    fn copy_to_v(&self, src: &Self::VectorX, dst: &mut Self::VectorX) -> Result<(), HoloError> {
        dst.copy_from(src);
        Ok(())
    }

    fn copy_to_m(&self, src: &Self::MatrixX, dst: &mut Self::MatrixX) -> Result<(), HoloError> {
        dst.copy_from(src);
        Ok(())
    }

    fn create_diagonal(&self, v: &Self::VectorX, a: &mut Self::MatrixX) -> Result<(), HoloError> {
        a.fill(0.);
        a.set_diagonal(v);
        Ok(())
    }

    fn get_diagonal(&self, a: &Self::MatrixX, v: &mut Self::VectorX) -> Result<(), HoloError> {
        *v = a.diagonal();
        Ok(())
    }

    fn real_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError> {
        *b = a.map(|v| v.re);
        Ok(())
    }

    fn imag_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError> {
        *b = a.map(|v| v.im);
        Ok(())
    }

    fn exp_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        v.apply(|v| *v = v.exp());
        Ok(())
    }

    fn concat_col_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        c.view_mut((0, 0), (a.nrows(), a.ncols())).copy_from(a);
        c.view_mut((0, a.ncols()), (b.nrows(), b.ncols()))
            .copy_from(b);
        Ok(())
    }

    fn max_v(&self, m: &Self::VectorX) -> Result<float, HoloError> {
        Ok(m.max())
    }

    fn hadamard_product_cm(
        &self,
        x: &Self::MatrixXc,
        y: &Self::MatrixXc,
        z: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        *z = x.component_mul(y);
        Ok(())
    }

    fn dot(&self, x: &Self::VectorX, y: &Self::VectorX) -> Result<float, HoloError> {
        Ok(x.dot(y))
    }

    fn add_v(
        &self,
        alpha: float,
        a: &Self::VectorX,
        b: &mut Self::VectorX,
    ) -> Result<(), HoloError> {
        *b += alpha * a;
        Ok(())
    }

    fn add_m(
        &self,
        alpha: float,
        a: &Self::MatrixX,
        b: &mut Self::MatrixX,
    ) -> Result<(), HoloError> {
        *b += alpha * a;
        Ok(())
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
    ) -> Result<(), HoloError> {
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
        Ok(())
    }

    fn solve_inplace(&self, a: &Self::MatrixX, x: &mut Self::VectorX) -> Result<(), HoloError> {
        if !a.clone().qr().solve_mut(x) {
            return Err(HoloError::SolveFailed);
        }
        Ok(())
    }

    fn reduce_col(&self, a: &Self::MatrixX, b: &mut Self::VectorX) -> Result<(), HoloError> {
        *b = a.column_sum();
        Ok(())
    }

    fn scaled_to_cv(
        &self,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
        c: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *c = a.zip_map(b, |a, b| a / a.abs() * b);
        Ok(())
    }

    fn scaled_to_assign_cv(
        &self,
        a: &Self::VectorXc,
        b: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        b.zip_apply(a, |b, a| *b = *b / b.abs() * a);
        Ok(())
    }

    fn cols_c(&self, m: &Self::MatrixXc) -> Result<usize, HoloError> {
        Ok(m.ncols())
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
            .test()
            .unwrap();
    }
}
