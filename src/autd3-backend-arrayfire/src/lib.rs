/*
 * File: lib.rs
 * Project: src
 * Created Date: 04/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(unknown_lints)]
#![allow(clippy::manual_slice_size_calculation)]

use std::rc::Rc;

use arrayfire::*;

use autd3_driver::{
    acoustics::{directivity::Sphere, propagate},
    datagram::GainFilter,
    defined::float,
    geometry::Geometry,
};
use autd3_gain_holo::{HoloError, LinAlgBackend, MatrixX, MatrixXc, Trans, VectorX, VectorXc};

#[cfg(feature = "single_float")]
type AfC = arrayfire::c32;
#[cfg(not(feature = "single_float"))]
type AfC = arrayfire::c64;

pub type AFBackend = arrayfire::Backend;
pub type AFDeviceInfo = (String, String, String, String);

fn convert(trans: Trans) -> MatProp {
    match trans {
        Trans::NoTrans => MatProp::NONE,
        Trans::Trans => MatProp::TRANS,
        Trans::ConjTrans => MatProp::CTRANS,
    }
}

/// Backend using ArrayFire
pub struct ArrayFireBackend {}

impl ArrayFireBackend {
    pub fn get_available_backends() -> Vec<AFBackend> {
        arrayfire::get_available_backends()
    }

    pub fn set_backend(backend: AFBackend) {
        arrayfire::set_backend(backend);
    }

    pub fn set_device(device: i32) {
        arrayfire::set_device(device);
    }

    pub fn get_available_devices() -> Vec<AFDeviceInfo> {
        let cur_dev = arrayfire::get_device();
        let r = (0..arrayfire::device_count())
            .map(|i| {
                arrayfire::set_device(i);
                arrayfire::device_info()
            })
            .collect();
        arrayfire::set_device(cur_dev);
        r
    }
}

impl LinAlgBackend for ArrayFireBackend {
    type MatrixXc = Array<AfC>;
    type MatrixX = Array<float>;
    type VectorXc = Array<AfC>;
    type VectorX = Array<float>;

    fn new() -> Result<Rc<Self>, HoloError> {
        Ok(Rc::new(Self {}))
    }

    fn generate_propagation_matrix<T: autd3_driver::geometry::Transducer>(
        &self,
        geometry: &Geometry<T>,
        foci: &[autd3_driver::geometry::Vector3],
        filter: &GainFilter,
    ) -> Result<Self::MatrixXc, HoloError> {
        let g = match filter {
            GainFilter::All => geometry
                .devices()
                .flat_map(|dev| {
                    dev.iter().flat_map(move |tr| {
                        foci.iter().map(move |fp| {
                            propagate::<Sphere, T>(tr, dev.attenuation, dev.sound_speed, fp)
                        })
                    })
                })
                .collect::<Vec<_>>(),
            GainFilter::Filter(filter) => geometry
                .devices()
                .flat_map(|dev| {
                    dev.iter().filter_map(move |tr| {
                        if let Some(filter) = filter.get(&dev.idx()) {
                            if filter[tr.local_idx()] {
                                Some(foci.iter().map(move |fp| {
                                    propagate::<Sphere, T>(tr, dev.attenuation, dev.sound_speed, fp)
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
                .collect::<Vec<_>>(),
        };
        unsafe {
            Ok(Array::new(
                std::slice::from_raw_parts(g.as_ptr() as *const AfC, g.len()),
                Dim4::new(&[foci.len() as u64, (g.len() / foci.len()) as _, 1, 1]),
            ))
        }
    }

    fn alloc_v(&self, size: usize) -> Result<Self::VectorX, HoloError> {
        Ok(Array::new_empty(Dim4::new(&[size as _, 1, 1, 1])))
    }

    fn alloc_m(&self, rows: usize, cols: usize) -> Result<Self::MatrixX, HoloError> {
        Ok(Array::new_empty(Dim4::new(&[rows as _, cols as _, 1, 1])))
    }

    fn alloc_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError> {
        Ok(Array::new_empty(Dim4::new(&[size as _, 1, 1, 1])))
    }

    fn alloc_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError> {
        Ok(Array::new_empty(Dim4::new(&[rows as _, cols as _, 1, 1])))
    }

    fn alloc_zeros_v(&self, size: usize) -> Result<Self::VectorX, HoloError> {
        Ok(arrayfire::constant(0., Dim4::new(&[size as _, 1, 1, 1])))
    }

    fn alloc_zeros_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError> {
        Ok(arrayfire::constant(
            AfC::new(0., 0.),
            Dim4::new(&[size as _, 1, 1, 1]),
        ))
    }

    fn alloc_zeros_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError> {
        Ok(arrayfire::constant(
            AfC::new(0., 0.),
            Dim4::new(&[rows as _, cols as _, 1, 1]),
        ))
    }

    fn to_host_v(&self, v: Self::VectorX) -> Result<VectorX, HoloError> {
        let mut r = VectorX::zeros(v.elements());
        v.host(r.as_mut_slice());
        Ok(r)
    }

    fn to_host_m(&self, v: Self::MatrixX) -> Result<MatrixX, HoloError> {
        let mut r = MatrixX::zeros(v.dims()[0] as _, v.dims()[1] as _);
        v.host(r.as_mut_slice());
        Ok(r)
    }

    fn to_host_cv(&self, v: Self::VectorXc) -> Result<VectorXc, HoloError> {
        let n = v.elements();
        let mut r = VectorXc::zeros(n);
        unsafe {
            v.host(std::slice::from_raw_parts_mut(
                r.as_mut_ptr() as *mut AfC,
                n,
            ));
        }
        Ok(r)
    }

    fn to_host_cm(&self, v: Self::MatrixXc) -> Result<MatrixXc, HoloError> {
        let n = v.elements();
        let mut r = MatrixXc::zeros(v.dims()[0] as _, v.dims()[1] as _);
        unsafe {
            v.host(std::slice::from_raw_parts_mut(
                r.as_mut_ptr() as *mut AfC,
                n,
            ));
        }
        Ok(r)
    }

    fn from_slice_v(&self, v: &[float]) -> Result<Self::VectorX, HoloError> {
        Ok(Array::new(v, Dim4::new(&[v.len() as _, 1, 1, 1])))
    }

    fn from_slice_m(
        &self,
        rows: usize,
        cols: usize,
        v: &[float],
    ) -> Result<Self::MatrixX, HoloError> {
        Ok(Array::new(v, Dim4::new(&[rows as _, cols as _, 1, 1])))
    }

    fn from_slice_cv(&self, v: &[float]) -> Result<Self::VectorXc, HoloError> {
        let r = Array::new(v, Dim4::new(&[v.len() as _, 1, 1, 1]));
        Ok(arrayfire::cplx(&r))
    }

    fn from_slice2_cv(&self, r: &[float], i: &[float]) -> Result<Self::VectorXc, HoloError> {
        let r = Array::new(r, Dim4::new(&[r.len() as _, 1, 1, 1]));
        let i = Array::new(i, Dim4::new(&[i.len() as _, 1, 1, 1]));
        Ok(arrayfire::cplx2(&r, &i, false).cast())
    }

    fn from_slice2_cm(
        &self,
        rows: usize,
        cols: usize,
        r: &[float],
        i: &[float],
    ) -> Result<Self::MatrixXc, HoloError> {
        let r = Array::new(r, Dim4::new(&[rows as _, cols as _, 1, 1]));
        let i = Array::new(i, Dim4::new(&[rows as _, cols as _, 1, 1]));
        Ok(arrayfire::cplx2(&r, &i, false).cast())
    }

    fn copy_from_slice_v(&self, v: &[float], dst: &mut Self::VectorX) -> Result<(), HoloError> {
        let n = v.len();
        if n == 0 {
            return Ok(());
        }
        let v = self.from_slice_v(v)?;
        let seqs = [Seq::new(0u32, n as u32 - 1, 1)];
        arrayfire::assign_seq(dst, &seqs, &v);
        Ok(())
    }

    fn copy_to_v(&self, src: &Self::VectorX, dst: &mut Self::VectorX) -> Result<(), HoloError> {
        let seqs = [Seq::new(0u32, src.elements() as u32 - 1, 1)];
        arrayfire::assign_seq(dst, &seqs, src);
        Ok(())
    }

    fn copy_to_m(&self, src: &Self::MatrixX, dst: &mut Self::MatrixX) -> Result<(), HoloError> {
        let seqs = [
            Seq::new(0u32, src.dims()[0] as u32 - 1, 1),
            Seq::new(0u32, src.dims()[1] as u32 - 1, 1),
        ];
        arrayfire::assign_seq(dst, &seqs, src);
        Ok(())
    }

    fn clone_v(&self, v: &Self::VectorX) -> Result<Self::VectorX, HoloError> {
        Ok(v.copy())
    }

    fn clone_m(&self, v: &Self::MatrixX) -> Result<Self::MatrixX, HoloError> {
        Ok(v.copy())
    }

    fn clone_cv(&self, v: &Self::VectorXc) -> Result<Self::VectorXc, HoloError> {
        Ok(v.copy())
    }

    fn clone_cm(&self, v: &Self::MatrixXc) -> Result<Self::MatrixXc, HoloError> {
        Ok(v.copy())
    }

    fn make_complex2_v(
        &self,
        real: &Self::VectorX,
        imag: &Self::VectorX,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *v = arrayfire::cplx2(real, imag, false).cast();
        Ok(())
    }

    fn get_col_c(
        &self,
        a: &Self::MatrixXc,
        col: usize,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *v = arrayfire::col(a, col as _);
        Ok(())
    }

    fn set_cv(
        &self,
        i: usize,
        val: autd3_gain_holo::Complex,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        let src = constant(AfC::new(val.re, val.im), Dim4::new(&[1, 1, 1, 1]));
        let seqs = [Seq::new(i as u32, i as u32, 1)];
        arrayfire::assign_seq(v, &seqs, &src);
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
        if start == end {
            return Ok(());
        }
        let seqs_a = [Seq::new(start as u32, end as u32 - 1, 1)];
        let sub_a = index(a, &seqs_a);
        let seqs_b = [
            Seq::new(start as u32, end as u32 - 1, 1),
            Seq::new(col as u32, col as u32, 1),
        ];
        arrayfire::assign_seq(v, &seqs_b, &sub_a);
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
        if start == end {
            return Ok(());
        }
        let seqs_a = [Seq::new(start as u32, end as u32 - 1, 1)];
        let sub_a = index(a, &seqs_a);
        let seqs_b = [
            Seq::new(row as u32, row as u32, 1),
            Seq::new(start as u32, end as u32 - 1, 1),
        ];
        arrayfire::assign_seq(v, &seqs_b, &arrayfire::transpose(&sub_a, false));
        Ok(())
    }

    fn get_diagonal_c(&self, a: &Self::MatrixXc, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        *v = arrayfire::diag_extract(a, 0);
        Ok(())
    }

    fn create_diagonal(&self, v: &Self::VectorX, a: &mut Self::MatrixX) -> Result<(), HoloError> {
        *a = arrayfire::diag_create(v, 0);
        Ok(())
    }

    fn create_diagonal_c(
        &self,
        v: &Self::VectorXc,
        a: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        *a = arrayfire::diag_create(v, 0);
        Ok(())
    }

    fn get_diagonal(&self, a: &Self::MatrixX, v: &mut Self::VectorX) -> Result<(), HoloError> {
        *v = arrayfire::diag_extract(a, 0);
        Ok(())
    }

    fn abs_cv(&self, a: &Self::VectorXc, b: &mut Self::VectorX) -> Result<(), HoloError> {
        *b = arrayfire::abs(a);
        Ok(())
    }

    fn real_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError> {
        *b = arrayfire::real(a);
        Ok(())
    }

    fn imag_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError> {
        *b = arrayfire::imag(a);
        Ok(())
    }

    fn scale_assign_v(&self, a: float, b: &mut Self::VectorX) -> Result<(), HoloError> {
        *b = arrayfire::mul(b, &a, false);
        Ok(())
    }

    fn scale_assign_cv(
        &self,
        a: autd3_gain_holo::Complex,
        b: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        let a = AfC::new(a.re, a.im);
        *b = arrayfire::mul(b, &a, false);
        Ok(())
    }

    fn conj_assign_v(&self, b: &mut Self::VectorXc) -> Result<(), HoloError> {
        *b = arrayfire::conjg(b);
        Ok(())
    }

    fn sqrt_assign_v(&self, v: &mut Self::VectorX) -> Result<(), HoloError> {
        *v = arrayfire::sqrt(v);
        Ok(())
    }

    fn normalize_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        *v = arrayfire::div(v, &arrayfire::abs(v), false);
        Ok(())
    }

    fn reciprocal_assign_c(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        let a = AfC::new(1., 0.);
        *v = arrayfire::div(&a, v, false);
        Ok(())
    }

    fn pow_assign_v(&self, a: float, v: &mut Self::VectorX) -> Result<(), HoloError> {
        *v = arrayfire::pow(v, &a, false);
        Ok(())
    }

    fn exp_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        *v = arrayfire::exp(v);
        Ok(())
    }

    fn concat_row_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        *c = arrayfire::join(0, a, b);
        Ok(())
    }

    fn concat_col_cv(
        &self,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
        c: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *c = arrayfire::join(0, a, b);
        Ok(())
    }

    fn concat_col_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        *c = arrayfire::join(1, a, b);
        Ok(())
    }

    fn max_v(&self, m: &Self::VectorX) -> Result<float, HoloError> {
        Ok(arrayfire::max_all(m).0)
    }

    fn max_eigen_vector_c(&self, m: Self::MatrixXc) -> Result<Self::VectorXc, HoloError> {
        let m = self.to_host_cm(m)?;
        let eig = m.symmetric_eigen();
        let v: VectorXc = eig.eigenvectors.column(eig.eigenvalues.imax()).into();
        unsafe {
            Ok(Array::new(
                std::slice::from_raw_parts(v.as_ptr() as *const AfC, v.len()),
                Dim4::new(&[v.len() as u64, 1, 1, 1]),
            ))
        }
    }

    fn hadamard_product_assign_cv(
        &self,
        x: &Self::VectorXc,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *y = arrayfire::mul(x, y, false);
        Ok(())
    }

    fn hadamard_product_cv(
        &self,
        x: &Self::VectorXc,
        y: &Self::VectorXc,
        z: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        *z = arrayfire::mul(x, y, false);
        Ok(())
    }

    fn hadamard_product_cm(
        &self,
        x: &Self::MatrixXc,
        y: &Self::MatrixXc,
        z: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        *z = arrayfire::mul(x, y, false);
        Ok(())
    }

    fn dot(&self, x: &Self::VectorX, y: &Self::VectorX) -> Result<float, HoloError> {
        let r = arrayfire::dot(x, y, MatProp::NONE, MatProp::NONE);
        let mut v = [0.];
        r.host(&mut v);
        Ok(v[0])
    }

    fn dot_c(
        &self,
        x: &Self::VectorXc,
        y: &Self::VectorXc,
    ) -> Result<autd3_gain_holo::Complex, HoloError> {
        let r = arrayfire::dot(x, y, MatProp::CONJ, MatProp::NONE);
        let mut v = [AfC::new(0., 0.)];
        r.host(&mut v);
        Ok(autd3_gain_holo::Complex::new(v[0].re, v[0].im))
    }

    fn add_v(
        &self,
        alpha: float,
        a: &Self::VectorX,
        b: &mut Self::VectorX,
    ) -> Result<(), HoloError> {
        *b = arrayfire::add(&arrayfire::mul(a, &alpha, false), b, false);
        Ok(())
    }

    fn add_m(
        &self,
        alpha: float,
        a: &Self::MatrixX,
        b: &mut Self::MatrixX,
    ) -> Result<(), HoloError> {
        *b = arrayfire::add(&arrayfire::mul(a, &alpha, false), b, false);
        Ok(())
    }

    fn gevv_c(
        &self,
        trans_a: autd3_gain_holo::Trans,
        trans_b: autd3_gain_holo::Trans,
        alpha: autd3_gain_holo::Complex,
        a: &Self::VectorXc,
        x: &Self::VectorXc,
        beta: autd3_gain_holo::Complex,
        y: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        let alpha = vec![AfC::new(alpha.re, alpha.im)];
        let beta = vec![AfC::new(beta.re, beta.im)];
        let trans_a = convert(trans_a);
        let trans_b = convert(trans_b);
        arrayfire::gemm(y, trans_a, trans_b, alpha, a, x, beta);
        Ok(())
    }

    fn gemv_c(
        &self,
        trans: autd3_gain_holo::Trans,
        alpha: autd3_gain_holo::Complex,
        a: &Self::MatrixXc,
        x: &Self::VectorXc,
        beta: autd3_gain_holo::Complex,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        let alpha = vec![AfC::new(alpha.re, alpha.im)];
        let beta = vec![AfC::new(beta.re, beta.im)];
        let trans = convert(trans);
        arrayfire::gemm(y, trans, MatProp::NONE, alpha, a, x, beta);
        Ok(())
    }

    fn gemm_c(
        &self,
        trans_a: autd3_gain_holo::Trans,
        trans_b: autd3_gain_holo::Trans,
        alpha: autd3_gain_holo::Complex,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        beta: autd3_gain_holo::Complex,
        y: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        let alpha = vec![AfC::new(alpha.re, alpha.im)];
        let beta = vec![AfC::new(beta.re, beta.im)];
        let trans_a = convert(trans_a);
        let trans_b = convert(trans_b);
        arrayfire::gemm(y, trans_a, trans_b, alpha, a, b, beta);
        Ok(())
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
        let (u, s, vt) = arrayfire::svd(&a);
        let m = a.dims()[0];
        let n = a.dims()[1];
        *b = arrayfire::matmul(
            &arrayfire::matmul(
                &vt,
                &arrayfire::join(
                    0,
                    &arrayfire::diag_create(
                        &arrayfire::cplx(&arrayfire::div(
                            &s,
                            &arrayfire::add(
                                &arrayfire::mul(&s, &s, false),
                                &constant(alpha * alpha, Dim4::new(&[s.elements() as _, 1, 1, 1])),
                                false,
                            ),
                            false,
                        )),
                        0,
                    ),
                    &constant(AfC::new(0., 0.), Dim4::new(&[n - m, m, 1, 1])),
                ),
                MatProp::CTRANS,
                MatProp::NONE,
            ),
            &u,
            MatProp::NONE,
            MatProp::CTRANS,
        );
        Ok(())
    }

    fn solve_inplace(&self, a: &Self::MatrixX, x: &mut Self::VectorX) -> Result<(), HoloError> {
        *x = arrayfire::solve(a, x, MatProp::NONE);
        Ok(())
    }

    fn solve_inplace_h(&self, a: Self::MatrixXc, x: &mut Self::VectorXc) -> Result<(), HoloError> {
        *x = arrayfire::solve(&a, x, MatProp::NONE);
        Ok(())
    }

    fn reduce_col(&self, a: &Self::MatrixX, b: &mut Self::VectorX) -> Result<(), HoloError> {
        *b = arrayfire::sum(a, 1);
        Ok(())
    }

    fn cols_c(&self, m: &Self::MatrixXc) -> Result<usize, HoloError> {
        Ok(m.dims()[1] as _)
    }
}

#[cfg(all(test, feature = "test-utilities"))]
mod tests {
    use super::*;

    use autd3_gain_holo::test_utilities::test_utils::*;

    #[test]
    fn test_arrayfire_backend() {
        LinAlgBackendTestHelper::<100, ArrayFireBackend>::new()
            .unwrap()
            .test()
            .expect("Faild to test ArrayFireBackend");
    }
}
