/*
 * File: backend.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use nalgebra::{Dyn, Matrix, VecStorage, U1};
use std::ops::{AddAssign, Mul, MulAssign};

pub type Complex = nalgebra::Complex<f64>;
pub type MatrixXc = Matrix<Complex, Dyn, Dyn, VecStorage<Complex, Dyn, Dyn>>;
pub type MatrixX = Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>>;
pub type VectorXc = Matrix<Complex, Dyn, U1, VecStorage<Complex, Dyn, U1>>;
pub type VectorX = Matrix<f64, Dyn, U1, VecStorage<f64, Dyn, U1>>;

pub enum Transpose {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
    ConjNoTrans = 114,
}

pub trait Backend {
    fn hadamard_product(a: &MatrixXc, b: &MatrixXc, c: &mut MatrixXc);
    fn real(a: &MatrixXc, b: &mut MatrixX);
    fn imag(a: &VectorXc, b: &mut VectorX);
    fn pseudo_inverse_svd(matrix: MatrixXc, alpha: f64, result: &mut MatrixXc);
    fn max_eigen_vector(matrix: MatrixXc) -> VectorXc;
    fn matrix_add(alpha: f64, a: &MatrixX, beta: f64, b: &mut MatrixX);
    fn matrix_mul(
        trans_a: Transpose,
        trans_b: Transpose,
        alpha: Complex,
        a: &MatrixXc,
        b: &MatrixXc,
        beta: Complex,
        c: &mut MatrixXc,
    );
    fn matrix_mul_vec(
        trans_a: Transpose,
        alpha: Complex,
        a: &MatrixXc,
        b: &VectorXc,
        beta: Complex,
        c: &mut VectorXc,
    );
    fn vector_add(alpha: f64, a: &VectorX, b: &mut VectorX);
    fn solve_ch(a: MatrixXc, b: &mut VectorXc) -> bool;
    fn solve_g(a: MatrixX, b: &mut VectorX) -> bool;
    fn dot(a: &VectorX, b: &VectorX) -> f64;
    fn dot_c(a: &VectorXc, b: &VectorXc) -> Complex;
    fn max_coefficient(a: &VectorX) -> f64;
    fn max_coefficient_c(a: &VectorXc) -> f64;
    fn concat_row(a: MatrixXc, b: &MatrixXc) -> MatrixXc;
    fn concat_col(a: MatrixXc, b: &MatrixXc) -> MatrixXc;
}

pub struct NalgebraBackend {}

impl Backend for NalgebraBackend {
    fn hadamard_product(a: &MatrixXc, b: &MatrixXc, c: &mut MatrixXc) {
        *c = a.component_mul(b);
    }

    fn real(a: &MatrixXc, b: &mut MatrixX) {
        *b = a.map(|x| x.re);
    }

    fn imag(a: &VectorXc, b: &mut VectorX) {
        *b = a.map(|x| x.im);
    }

    fn pseudo_inverse_svd(matrix: MatrixXc, alpha: f64, result: &mut MatrixXc) {
        let svd = matrix.svd(true, true);
        let s_inv = MatrixXc::from_diagonal(
            &svd.singular_values
                .map(|s| Complex::new(s / (s * s + alpha * alpha), 0.)),
        );
        *result = match (&svd.v_t, &svd.u) {
            (Some(v_t), Some(u)) => v_t.adjoint() * s_inv * u.adjoint(),
            _ => unreachable!(),
        };
    }

    fn max_eigen_vector(matrix: MatrixXc) -> VectorXc {
        let eig = nalgebra::SymmetricEigen::new(matrix);
        eig.eigenvectors.column(eig.eigenvalues.imax()).into()
    }

    fn matrix_add(alpha: f64, a: &MatrixX, beta: f64, b: &mut MatrixX) {
        b.mul_assign(beta);
        b.add_assign(a.mul(alpha));
    }

    fn matrix_mul(
        trans_a: Transpose,
        trans_b: Transpose,
        alpha: Complex,
        a: &MatrixXc,
        b: &MatrixXc,
        beta: Complex,
        c: &mut MatrixXc,
    ) {
        c.mul_assign(beta);
        match (trans_a, trans_b) {
            (Transpose::NoTrans, Transpose::NoTrans) => c.add_assign(a.mul(b).mul(alpha)),
            (Transpose::NoTrans, Transpose::Trans) => c.add_assign(a.mul(b.transpose()).mul(alpha)),
            (Transpose::NoTrans, Transpose::ConjTrans) => {
                c.add_assign(a.mul(b.adjoint()).mul(alpha))
            }
            (Transpose::NoTrans, Transpose::ConjNoTrans) => {
                c.add_assign(a.mul(b.conjugate()).mul(alpha))
            }
            (Transpose::Trans, Transpose::NoTrans) => c.add_assign(a.transpose().mul(b).mul(alpha)),
            (Transpose::Trans, Transpose::Trans) => {
                c.add_assign(a.transpose().mul(b.transpose()).mul(alpha))
            }
            (Transpose::Trans, Transpose::ConjTrans) => {
                c.add_assign(a.transpose().mul(b.adjoint()).mul(alpha))
            }
            (Transpose::Trans, Transpose::ConjNoTrans) => {
                c.add_assign(a.transpose().mul(b.conjugate()).mul(alpha))
            }
            (Transpose::ConjTrans, Transpose::NoTrans) => {
                c.add_assign(a.adjoint().mul(b).mul(alpha))
            }
            (Transpose::ConjTrans, Transpose::Trans) => {
                c.add_assign(a.adjoint().mul(b.transpose()).mul(alpha))
            }
            (Transpose::ConjTrans, Transpose::ConjTrans) => {
                c.add_assign(a.adjoint().mul(b.adjoint()).mul(alpha))
            }
            (Transpose::ConjTrans, Transpose::ConjNoTrans) => {
                c.add_assign(a.adjoint().mul(b.conjugate()).mul(alpha))
            }
            (Transpose::ConjNoTrans, Transpose::NoTrans) => {
                c.add_assign(a.conjugate().mul(b).mul(alpha))
            }
            (Transpose::ConjNoTrans, Transpose::Trans) => {
                c.add_assign(a.conjugate().mul(b.transpose()).mul(alpha))
            }
            (Transpose::ConjNoTrans, Transpose::ConjTrans) => {
                c.add_assign(a.conjugate().mul(b.adjoint()).mul(alpha))
            }
            (Transpose::ConjNoTrans, Transpose::ConjNoTrans) => {
                c.add_assign(a.conjugate().mul(b.conjugate()).mul(alpha))
            }
        };
    }

    fn matrix_mul_vec(
        trans_a: Transpose,
        alpha: Complex,
        a: &MatrixXc,
        b: &VectorXc,
        beta: Complex,
        c: &mut VectorXc,
    ) {
        c.mul_assign(beta);
        match trans_a {
            Transpose::NoTrans => c.add_assign(a.mul(b).mul(alpha)),
            Transpose::Trans => c.add_assign(a.transpose().mul(b).mul(alpha)),
            Transpose::ConjTrans => c.add_assign(a.adjoint().mul(b).mul(alpha)),
            Transpose::ConjNoTrans => c.add_assign(a.conjugate().mul(b).mul(alpha)),
        };
    }

    fn vector_add(alpha: f64, a: &VectorX, b: &mut VectorX) {
        b.add_assign(a.mul(alpha));
    }

    fn solve_ch(a: MatrixXc, b: &mut VectorXc) -> bool {
        a.qr().solve_mut(b)
    }

    fn solve_g(a: MatrixX, b: &mut VectorX) -> bool {
        a.qr().solve_mut(b)
    }

    fn dot(a: &VectorX, b: &VectorX) -> f64 {
        a.dot(b)
    }

    fn dot_c(a: &VectorXc, b: &VectorXc) -> Complex {
        a.dot(b)
    }

    fn max_coefficient(a: &VectorX) -> f64 {
        a.camax()
    }

    fn max_coefficient_c(a: &VectorXc) -> f64 {
        a.camax()
    }

    fn concat_row(a: MatrixXc, b: &MatrixXc) -> MatrixXc {
        let arows = a.nrows();
        let acols = a.ncols();
        let mut new_mat = a.resize(arows + b.nrows(), acols, Default::default());
        new_mat
            .view_mut((arows, 0), (b.nrows(), b.ncols()))
            .copy_from(b);

        new_mat
    }

    fn concat_col(a: MatrixXc, b: &MatrixXc) -> MatrixXc {
        let arows = a.nrows();
        let acols = a.ncols();
        let mut new_mat = a.resize(arows, acols + b.ncols(), Default::default());
        new_mat
            .view_mut((0, acols), (b.nrows(), b.ncols()))
            .copy_from(b);

        new_mat
    }
}
