/*
 * File: backend.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use autd3_driver::{
    datagram::GainFilter,
    defined::float,
    geometry::{Device, Transducer, Vector3},
};
use nalgebra::{Dyn, VecStorage, U1};

use crate::error::HoloError;

pub type Complex = nalgebra::Complex<float>;

pub type MatrixXc = nalgebra::Matrix<Complex, Dyn, Dyn, VecStorage<Complex, Dyn, Dyn>>;
pub type MatrixX = nalgebra::Matrix<float, Dyn, Dyn, VecStorage<float, Dyn, Dyn>>;
pub type VectorXc = nalgebra::Matrix<Complex, Dyn, U1, VecStorage<Complex, Dyn, U1>>;
pub type VectorX = nalgebra::Matrix<float, Dyn, U1, VecStorage<float, Dyn, U1>>;

pub enum Trans {
    NoTrans,
    Trans,
    ConjTrans,
}

/// Calculation backend
pub trait LinAlgBackend {
    type MatrixXc;
    type MatrixX;
    type VectorXc;
    type VectorX;

    fn new() -> Result<Rc<Self>, HoloError>;

    fn generate_propagation_matrix<T: Transducer>(
        &self,
        devices: &[&Device<T>],
        foci: &[Vector3],
        filter: &GainFilter,
    ) -> Result<Self::MatrixXc, HoloError>;

    fn alloc_v(&self, size: usize) -> Result<Self::VectorX, HoloError>;
    fn alloc_m(&self, rows: usize, cols: usize) -> Result<Self::MatrixX, HoloError>;
    fn alloc_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError>;
    fn alloc_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError>;
    fn alloc_zeros_v(&self, size: usize) -> Result<Self::VectorX, HoloError>;
    fn alloc_zeros_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError>;
    fn alloc_zeros_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError>;

    fn to_host_v(&self, v: Self::VectorX) -> Result<VectorX, HoloError>;
    fn to_host_m(&self, v: Self::MatrixX) -> Result<MatrixX, HoloError>;
    fn to_host_cv(&self, v: Self::VectorXc) -> Result<VectorXc, HoloError>;
    fn to_host_cm(&self, v: Self::MatrixXc) -> Result<MatrixXc, HoloError>;

    fn cols_c(&self, m: &Self::MatrixXc) -> Result<usize, HoloError>;

    #[allow(clippy::wrong_self_convention)]
    fn from_slice_v(&self, v: &[float]) -> Result<Self::VectorX, HoloError>;
    #[allow(clippy::wrong_self_convention)]
    fn from_slice_m(
        &self,
        rows: usize,
        cols: usize,
        v: &[float],
    ) -> Result<Self::MatrixX, HoloError>;
    #[allow(clippy::wrong_self_convention)]
    fn from_slice_cv(&self, v: &[float]) -> Result<Self::VectorXc, HoloError>;
    #[allow(clippy::wrong_self_convention)]
    fn from_slice2_cv(&self, r: &[float], i: &[float]) -> Result<Self::VectorXc, HoloError>;
    #[allow(clippy::wrong_self_convention)]
    fn from_slice2_cm(
        &self,
        rows: usize,
        cols: usize,
        r: &[float],
        i: &[float],
    ) -> Result<Self::MatrixXc, HoloError>;

    fn copy_from_slice_v(&self, v: &[float], dst: &mut Self::VectorX) -> Result<(), HoloError>;

    fn copy_to_v(&self, src: &Self::VectorX, dst: &mut Self::VectorX) -> Result<(), HoloError>;
    fn copy_to_m(&self, src: &Self::MatrixX, dst: &mut Self::MatrixX) -> Result<(), HoloError>;

    fn clone_v(&self, v: &Self::VectorX) -> Result<Self::VectorX, HoloError>;
    fn clone_m(&self, v: &Self::MatrixX) -> Result<Self::MatrixX, HoloError>;
    fn clone_cv(&self, v: &Self::VectorXc) -> Result<Self::VectorXc, HoloError>;
    fn clone_cm(&self, v: &Self::MatrixXc) -> Result<Self::MatrixXc, HoloError>;

    fn make_complex2_v(
        &self,
        real: &Self::VectorX,
        imag: &Self::VectorX,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError>;

    fn get_col_c(
        &self,
        a: &Self::MatrixXc,
        col: usize,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError>;
    fn set_cv(&self, i: usize, val: Complex, v: &mut Self::VectorXc) -> Result<(), HoloError>;
    fn set_col_c(
        &self,
        a: &Self::VectorXc,
        col: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;
    fn set_row_c(
        &self,
        a: &Self::VectorXc,
        row: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;

    fn get_diagonal_c(&self, a: &Self::MatrixXc, v: &mut Self::VectorXc) -> Result<(), HoloError>;
    fn create_diagonal(&self, v: &Self::VectorX, a: &mut Self::MatrixX) -> Result<(), HoloError>;
    fn create_diagonal_c(
        &self,
        v: &Self::VectorXc,
        a: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;
    fn get_diagonal(&self, a: &Self::MatrixX, v: &mut Self::VectorX) -> Result<(), HoloError>;

    fn abs_cv(&self, a: &Self::VectorXc, b: &mut Self::VectorX) -> Result<(), HoloError>;
    fn real_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError>;
    fn imag_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError>;
    fn scale_assign_v(&self, a: float, b: &mut Self::VectorX) -> Result<(), HoloError>;
    fn scale_assign_cv(&self, a: Complex, b: &mut Self::VectorXc) -> Result<(), HoloError>;
    fn conj_assign_v(&self, b: &mut Self::VectorXc) -> Result<(), HoloError>;
    fn sqrt_assign_v(&self, v: &mut Self::VectorX) -> Result<(), HoloError>;
    fn normalize_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError>;
    fn reciprocal_assign_c(&self, v: &mut Self::VectorXc) -> Result<(), HoloError>;
    fn pow_assign_v(&self, a: float, v: &mut Self::VectorX) -> Result<(), HoloError>;
    fn exp_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError>;

    fn concat_row_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;
    fn concat_col_cv(
        &self,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
        c: &mut Self::VectorXc,
    ) -> Result<(), HoloError>;
    fn concat_col_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;

    fn max_v(&self, m: &Self::VectorX) -> Result<float, HoloError>;
    fn max_eigen_vector_c(&self, m: Self::MatrixXc) -> Result<Self::VectorXc, HoloError>;

    fn hadamard_product_assign_cv(
        &self,
        x: &Self::VectorXc,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError>;
    fn hadamard_product_cv(
        &self,
        x: &Self::VectorXc,
        y: &Self::VectorXc,
        z: &mut Self::VectorXc,
    ) -> Result<(), HoloError>;
    fn hadamard_product_cm(
        &self,
        x: &Self::MatrixXc,
        y: &Self::MatrixXc,
        z: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;

    fn dot(&self, x: &Self::VectorX, y: &Self::VectorX) -> Result<float, HoloError>;
    fn dot_c(&self, x: &Self::VectorXc, y: &Self::VectorXc) -> Result<Complex, HoloError>;

    fn add_v(
        &self,
        alpha: float,
        a: &Self::VectorX,
        b: &mut Self::VectorX,
    ) -> Result<(), HoloError>;
    fn add_m(
        &self,
        alpha: float,
        a: &Self::MatrixX,
        b: &mut Self::MatrixX,
    ) -> Result<(), HoloError>;

    #[allow(clippy::too_many_arguments)]
    fn gevv_c(
        &self,
        trans_a: Trans,
        trans_b: Trans,
        alpha: Complex,
        a: &Self::VectorXc,
        x: &Self::VectorXc,
        beta: Complex,
        y: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;

    fn gemv_c(
        &self,
        trans: Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        x: &Self::VectorXc,
        beta: Complex,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError>;

    #[allow(clippy::too_many_arguments)]
    fn gemm_c(
        &self,
        trans_a: Trans,
        trans_b: Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        beta: Complex,
        y: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;

    #[allow(clippy::too_many_arguments)]
    fn pseudo_inverse_svd(
        &self,
        a: Self::MatrixXc,
        alpha: float,
        u: &mut Self::MatrixXc,
        s: &mut Self::MatrixXc,
        vt: &mut Self::MatrixXc,
        buf: &mut Self::MatrixXc,
        b: &mut Self::MatrixXc,
    ) -> Result<(), HoloError>;
    fn solve_inplace(&self, a: &Self::MatrixX, x: &mut Self::VectorX) -> Result<(), HoloError>;
    fn solve_inplace_h(&self, a: Self::MatrixXc, x: &mut Self::VectorXc) -> Result<(), HoloError>;

    fn reduce_col(&self, a: &Self::MatrixX, b: &mut Self::VectorX) -> Result<(), HoloError>;

    fn scaled_to_cv(
        &self,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
        c: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        let mut tmp = self.clone_cv(a)?;
        self.normalize_assign_cv(&mut tmp)?;
        self.hadamard_product_cv(&tmp, b, c)?;
        Ok(())
    }

    fn scaled_to_assign_cv(
        &self,
        a: &Self::VectorXc,
        b: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        self.normalize_assign_cv(b)?;
        self.hadamard_product_assign_cv(a, b)?;
        Ok(())
    }

    fn gen_back_prop(
        &self,
        _m: usize,
        n: usize,
        transfer: &Self::MatrixXc,
        amps: &Self::VectorXc,
        b: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        let mut tmp = self.alloc_zeros_cm(n, n)?;

        self.gemm_c(
            Trans::NoTrans,
            Trans::ConjTrans,
            Complex::new(1., 0.),
            transfer,
            transfer,
            Complex::new(0., 0.),
            &mut tmp,
        )?;

        let mut denominator = self.alloc_cv(n)?;
        self.get_diagonal_c(&tmp, &mut denominator)?;
        self.reciprocal_assign_c(&mut denominator)?;
        self.hadamard_product_assign_cv(amps, &mut denominator)?;

        self.create_diagonal_c(&denominator, &mut tmp)?;

        self.gemm_c(
            Trans::ConjTrans,
            Trans::NoTrans,
            Complex::new(1., 0.),
            transfer,
            &tmp,
            Complex::new(0., 0.),
            b,
        )
    }
}
