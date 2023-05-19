/*
 * File: backend.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_gain_holo::Backend;
use autd3capi_common::*;

pub struct DynamicBackend {
    backend_ptr: Box<dyn Backend>,
}

impl DynamicBackend {
    pub fn new(backend_ptr: Box<dyn Backend>) -> Self {
        Self { backend_ptr }
    }
}

impl Backend for DynamicBackend {
    fn hadamard_product(
        &mut self,
        a: &autd3_gain_holo::MatrixXc,
        b: &autd3_gain_holo::MatrixXc,
        c: &mut autd3_gain_holo::MatrixXc,
    ) {
        self.backend_ptr.hadamard_product(a, b, c)
    }

    fn real(&mut self, a: &autd3_gain_holo::MatrixXc, b: &mut autd3_gain_holo::MatrixX) {
        self.backend_ptr.real(a, b)
    }

    fn imag(&mut self, a: &autd3_gain_holo::VectorXc, b: &mut autd3_gain_holo::VectorX) {
        self.backend_ptr.imag(a, b)
    }

    fn pseudo_inverse_svd(
        &mut self,
        matrix: autd3_gain_holo::MatrixXc,
        alpha: float,
        result: &mut autd3_gain_holo::MatrixXc,
    ) {
        self.backend_ptr.pseudo_inverse_svd(matrix, alpha, result)
    }

    fn max_eigen_vector(&mut self, matrix: autd3_gain_holo::MatrixXc) -> autd3_gain_holo::VectorXc {
        self.backend_ptr.max_eigen_vector(matrix)
    }

    fn matrix_add(
        &mut self,
        alpha: float,
        a: &autd3_gain_holo::MatrixX,
        beta: float,
        b: &mut autd3_gain_holo::MatrixX,
    ) {
        self.backend_ptr.matrix_add(alpha, a, beta, b)
    }

    fn matrix_mul(
        &mut self,
        trans_a: autd3_gain_holo::Transpose,
        trans_b: autd3_gain_holo::Transpose,
        alpha: autd3_gain_holo::Complex,
        a: &autd3_gain_holo::MatrixXc,
        b: &autd3_gain_holo::MatrixXc,
        beta: autd3_gain_holo::Complex,
        c: &mut autd3_gain_holo::MatrixXc,
    ) {
        self.backend_ptr
            .matrix_mul(trans_a, trans_b, alpha, a, b, beta, c)
    }

    fn matrix_mul_vec(
        &mut self,
        trans_a: autd3_gain_holo::Transpose,
        alpha: autd3_gain_holo::Complex,
        a: &autd3_gain_holo::MatrixXc,
        b: &autd3_gain_holo::VectorXc,
        beta: autd3_gain_holo::Complex,
        c: &mut autd3_gain_holo::VectorXc,
    ) {
        self.backend_ptr
            .matrix_mul_vec(trans_a, alpha, a, b, beta, c)
    }

    fn vector_add(
        &mut self,
        alpha: float,
        a: &autd3_gain_holo::VectorX,
        b: &mut autd3_gain_holo::VectorX,
    ) {
        self.backend_ptr.vector_add(alpha, a, b)
    }

    fn solve_ch(
        &mut self,
        a: autd3_gain_holo::MatrixXc,
        b: &mut autd3_gain_holo::VectorXc,
    ) -> bool {
        self.backend_ptr.solve_ch(a, b)
    }

    fn solve_g(&mut self, a: autd3_gain_holo::MatrixX, b: &mut autd3_gain_holo::VectorX) -> bool {
        self.backend_ptr.solve_g(a, b)
    }

    fn dot(&mut self, a: &autd3_gain_holo::VectorX, b: &autd3_gain_holo::VectorX) -> float {
        self.backend_ptr.dot(a, b)
    }

    fn dot_c(
        &mut self,
        a: &autd3_gain_holo::VectorXc,
        b: &autd3_gain_holo::VectorXc,
    ) -> autd3_gain_holo::Complex {
        self.backend_ptr.dot_c(a, b)
    }

    fn max_coefficient(&mut self, a: &autd3_gain_holo::VectorX) -> float {
        self.backend_ptr.max_coefficient(a)
    }

    fn max_coefficient_c(&mut self, a: &autd3_gain_holo::VectorXc) -> float {
        self.backend_ptr.max_coefficient_c(a)
    }

    fn concat_row(
        &mut self,
        a: autd3_gain_holo::MatrixXc,
        b: &autd3_gain_holo::MatrixXc,
    ) -> autd3_gain_holo::MatrixXc {
        self.backend_ptr.concat_row(a, b)
    }

    fn concat_col(
        &mut self,
        a: autd3_gain_holo::MatrixXc,
        b: &autd3_gain_holo::MatrixXc,
    ) -> autd3_gain_holo::MatrixXc {
        self.backend_ptr.concat_col(a, b)
    }
}
