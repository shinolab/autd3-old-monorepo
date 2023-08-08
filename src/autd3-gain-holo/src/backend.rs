/*
 * File: backend.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::rc::Rc;

use autd3_core::{
    float,
    geometry::{Geometry, Transducer, Vector3},
};
use nalgebra::{Dyn, VecStorage, U1};

use crate::error::HoloError;

pub type Complex = nalgebra::Complex<float>;

pub type MatrixXc = nalgebra::Matrix<Complex, Dyn, Dyn, VecStorage<Complex, Dyn, Dyn>>;
pub type MatrixX = nalgebra::Matrix<float, Dyn, Dyn, VecStorage<float, Dyn, Dyn>>;
pub type VectorXc = nalgebra::Matrix<Complex, Dyn, U1, VecStorage<Complex, Dyn, U1>>;
pub type VectorX = nalgebra::Matrix<float, Dyn, U1, VecStorage<float, Dyn, U1>>;

/// Calculation backend
pub trait Backend {
    fn gs(&self, repeat: usize, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError>;
    fn gspat(&self, repeat: usize, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError>;
    fn naive(&self, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError>;
    fn evp(&self, gamma: float, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError>;
    fn sdp(
        &self,
        alpha: float,
        repeat: usize,
        lambda: float,
        amps: &[float],
        g: MatrixXc,
    ) -> Result<VectorXc, HoloError>;
    #[allow(clippy::too_many_arguments)]
    fn lm(
        &self,
        eps1: float,
        eps2: float,
        tau: float,
        kmax: usize,
        initial: &[float],
        amps: &[float],
        g: MatrixXc,
    ) -> Result<VectorX, HoloError>;
}

pub enum Trans {
    NoTrans,
    Trans,
    ConjTrans,
}

pub trait LinAlgBackend {
    type MatrixXc;
    type MatrixX;
    type VectorXc;
    type VectorX;

    fn new() -> Result<Rc<Self>, HoloError>;

    fn generate_propagation_matrix<T: Transducer>(
        &self,
        geometry: &Geometry<T>,
        foci: &[Vector3],
    ) -> Self::MatrixXc;

    fn to_host_cv(&self, v: Self::VectorXc) -> VectorXc;

    fn alloc_cv(&self, size: usize) -> Self::VectorXc;
    fn alloc_cm(&self, rows: usize, cols: usize) -> Self::MatrixXc;
    fn alloc_zeros_cv(&self, size: usize) -> Self::VectorXc;
    fn alloc_zeros_cm(&self, rows: usize, cols: usize) -> Self::MatrixXc;

    fn clone_cv(&self, v: &Self::VectorXc) -> Self::VectorXc;

    fn make_complex_v(&self, real: &[float]) -> Self::VectorXc;
    fn get_diagonal_c(&self, a: &Self::MatrixXc, v: &mut Self::VectorXc);
    fn create_diagonal_c(&self, v: &Self::VectorXc, a: &mut Self::MatrixXc);

    fn normalize_cv(&self, v: &mut Self::VectorXc);
    fn reciprocal_c(&self, v: &mut Self::VectorXc);

    fn hadamard_product_assign_cv(&self, x: &Self::VectorXc, y: &mut Self::VectorXc);
    fn hadamard_product_cv(&self, x: &Self::VectorXc, y: &Self::VectorXc, z: &mut Self::VectorXc);

    fn gemv_c(
        &self,
        trans: Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        x: &Self::VectorXc,
        beta: Complex,
        y: &mut Self::VectorXc,
    );

    fn gemm_c(
        &self,
        trans_a: Trans,
        trans_b: Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        beta: Complex,
        y: &mut Self::MatrixXc,
    );

    fn gen_back_prop(
        &self,
        m: usize,
        transfer: &Self::MatrixXc,
        amps: &Self::VectorXc,
        b: &mut Self::MatrixXc,
    ) {
        let mut tmp = self.alloc_zeros_cm(m, m);

        self.gemm_c(
            Trans::NoTrans,
            Trans::ConjTrans,
            Complex::new(1., 0.),
            transfer,
            transfer,
            Complex::new(0., 0.),
            &mut tmp,
        );

        let mut denominator = self.alloc_cv(m);
        self.get_diagonal_c(&tmp, &mut denominator);
        self.reciprocal_c(&mut denominator);
        self.hadamard_product_assign_cv(amps, &mut denominator);

        self.create_diagonal_c(&denominator, &mut tmp);
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
