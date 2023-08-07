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

    fn alloc_zeros_cv(&self, size: usize) -> Self::VectorXc;

    fn make_complex_v(&self, real: &[float]) -> Self::VectorXc;

    fn normalize_cv(&self, v: &mut Self::VectorXc);

    fn hadamard_product_cv(&self, x: &Self::VectorXc, y: &mut Self::VectorXc);

    fn c_gemv(
        &self,
        trans: Trans,
        alpha: Complex,
        a: &Self::MatrixXc,
        x: &Self::VectorXc,
        beta: Complex,
        y: &mut Self::VectorXc,
    );
}
