/*
 * File: backend.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::float;
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
