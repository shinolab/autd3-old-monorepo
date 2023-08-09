/*
 * File: test_utils.rs
 * Project: test_utilities
 * Created Date: 09/08/2023
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
use rand::Rng;

use autd3_core::{
    acoustics::{propagate_tr, Sphere},
    float,
};

use crate::{Complex, HoloError, LinAlgBackend, MatrixXc, Trans, VectorXc};

use super::bench_utils::{gen_foci, generate_geometry};

pub struct LinAlgBackendTestHelper<const N: usize, B: LinAlgBackend> {
    backend: Rc<B>,
}

impl<const N: usize, B: LinAlgBackend> LinAlgBackendTestHelper<N, B> {
    pub fn new() -> Result<Self, HoloError> {
        Ok(Self { backend: B::new()? })
    }

    pub fn test(&self) {
        self.test_alloc_v();
        self.test_alloc_cv();
        self.test_alloc_cm();
        self.test_alloc_zeros_v();
        self.test_alloc_zeros_cv();
        self.test_alloc_zeros_cm();

        self.test_from_slice_v();
        self.test_from_slice_cv();
        self.test_from_slice2_cv();
        self.test_from_slice2_cm();

        self.test_clone_v();
        self.test_clone_cv();
        self.test_clone_cm();

        self.test_make_complex2_v();

        self.test_get_diagonal_c();
        self.test_create_diagonal_c();

        self.test_abs_cv();
        self.test_scale_assign_v();
        self.test_sqrt_assign_v();
        self.test_normalize_assign_cv();
        self.test_reciprocal_assign_c();
        self.test_pow_assign_v();

        self.test_concat_row_cm();
        self.test_concat_col_cv();

        self.test_max_eigen_vector_c();

        self.test_hadamard_product_assign_cv();
        self.test_hadamard_product_cv();

        self.test_gemv_c();
        self.test_gemm_c();

        self.test_solve_inplace_h();

        self.test_generate_propagation_matrix();
        self.test_gen_back_prop();
    }

    fn make_random_v(&self, size: usize) -> B::VectorX {
        let mut rng = rand::thread_rng();
        let v: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(size)
            .collect();
        self.backend.from_slice_v(&v)
    }

    fn make_random_cv(&self, size: usize) -> B::VectorXc {
        let mut rng = rand::thread_rng();
        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(size)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(size)
            .collect();
        self.backend.from_slice2_cv(&real, &imag)
    }

    fn make_random_cm(&self, rows: usize, cols: usize) -> B::MatrixXc {
        let mut rng = rand::thread_rng();
        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(rows * cols)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(rows * cols)
            .collect();
        self.backend.from_slice2_cm(rows, cols, &real, &imag)
    }

    fn test_alloc_v(&self) {
        let v = self.backend.alloc_v(N);
        let v = self.backend.to_host_v(v);

        assert_eq!(N, v.len());
    }

    fn test_alloc_cv(&self) {
        let v = self.backend.alloc_cv(N);
        let v = self.backend.to_host_cv(v);

        assert_eq!(N, v.len());
    }

    fn test_alloc_cm(&self) {
        let m = self.backend.alloc_cm(N, 2 * N);
        let m = self.backend.to_host_cm(m);

        assert_eq!(N, m.nrows());
        assert_eq!(2 * N, m.ncols());
    }

    fn test_alloc_zeros_v(&self) {
        let v = self.backend.alloc_v(N);
        let v = self.backend.to_host_v(v);

        assert_eq!(N, v.len());
        assert!(v.iter().all(|&v| v == 0.));
    }

    fn test_alloc_zeros_cv(&self) {
        let v = self.backend.alloc_cv(N);
        let v = self.backend.to_host_cv(v);

        assert_eq!(N, v.len());
        assert!(v.iter().all(|&v| v == Complex::new(0., 0.)));
    }

    fn test_alloc_zeros_cm(&self) {
        let m = self.backend.alloc_cm(N, 2 * N);
        let m = self.backend.to_host_cm(m);

        assert_eq!(N, m.nrows());
        assert_eq!(2 * N, m.ncols());
        assert!(m.iter().all(|&v| v == Complex::new(0., 0.)));
    }

    fn test_from_slice_v(&self) {
        let rng = rand::thread_rng();

        let v: Vec<float> = rng
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();

        let c = self.backend.from_slice_v(&v);
        let c = self.backend.to_host_v(c);

        assert_eq!(N, c.len());
        v.iter().zip(c.iter()).for_each(|(&r, &c)| {
            assert_eq!(r, c);
        });
    }

    fn test_from_slice_cv(&self) {
        let rng = rand::thread_rng();

        let real: Vec<float> = rng
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();

        let c = self.backend.from_slice_cv(&real);
        let c = self.backend.to_host_cv(c);

        assert_eq!(N, c.len());
        real.iter().zip(c.iter()).for_each(|(r, c)| {
            assert_eq!(r, &c.re);
            assert_eq!(0.0, c.im);
        });
    }

    fn test_from_slice2_cv(&self) {
        let mut rng = rand::thread_rng();

        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();

        let c = self.backend.from_slice2_cv(&real, &imag);
        let c = self.backend.to_host_cv(c);

        assert_eq!(N, c.len());
        real.iter()
            .zip(imag.iter())
            .zip(c.iter())
            .for_each(|((r, i), c)| {
                assert_eq!(r, &c.re);
                assert_eq!(i, &c.im);
            });
    }

    fn test_from_slice2_cm(&self) {
        let mut rng = rand::thread_rng();

        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N * 2 * N)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N * 2 * N)
            .collect();

        let c = self.backend.from_slice2_cm(N, 2 * N, &real, &imag);
        let c = self.backend.to_host_cm(c);

        assert_eq!(N, c.nrows());
        assert_eq!(2 * N, c.ncols());
        (0..2 * N).for_each(|col| {
            (0..N).for_each(|row| {
                assert_eq!(real[col * N + row], c[(row, col)].re);
                assert_eq!(imag[col * N + row], c[(row, col)].im);
            })
        });
    }

    fn test_clone_v(&self) {
        let c = self.make_random_v(N);
        let c2 = self.backend.clone_v(&c);

        let c = self.backend.to_host_v(c);
        let c2 = self.backend.to_host_v(c2);

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c, c2);
        });
    }

    fn test_clone_cv(&self) {
        let c = self.make_random_cv(N);
        let c2 = self.backend.clone_cv(&c);

        let c = self.backend.to_host_cv(c);
        let c2 = self.backend.to_host_cv(c2);

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c.re, c2.re);
            assert_eq!(c.im, c2.im);
        });
    }

    fn test_clone_cm(&self) {
        let c = self.make_random_cm(N, N);
        let c2 = self.backend.clone_cm(&c);

        let c = self.backend.to_host_cm(c);
        let c2 = self.backend.to_host_cm(c2);

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c.re, c2.re);
            assert_eq!(c.im, c2.im);
        });
    }

    fn test_make_complex2_v(&self) {
        let real = self.make_random_v(N);
        let imag = self.make_random_v(N);

        let mut c = self.backend.alloc_cv(N);
        self.backend.make_complex2_v(&real, &imag, &mut c);

        let real = self.backend.to_host_v(real);
        let imag = self.backend.to_host_v(imag);
        let c = self.backend.to_host_cv(c);
        real.iter()
            .zip(imag.iter())
            .zip(c.iter())
            .for_each(|((r, i), c)| {
                assert_eq!(r, &c.re);
                assert_eq!(i, &c.im);
            });
    }

    fn test_get_diagonal_c(&self) {
        let c = self.make_random_cm(N, N);

        let mut diagonal = self.backend.alloc_cv(N);
        self.backend.get_diagonal_c(&c, &mut diagonal);

        let c = self.backend.to_host_cm(c);
        let diagonal = self.backend.to_host_cv(diagonal);
        (0..N).for_each(|i| {
            assert_eq!(c[i * N + i].re, diagonal[i].re);
            assert_eq!(c[i * N + i].im, diagonal[i].im);
        });
    }

    fn test_create_diagonal_c(&self) {
        let diagonal = self.make_random_cv(N);

        let mut c = self.backend.alloc_cm(N, N);

        self.backend.create_diagonal_c(&diagonal, &mut c);

        let diagonal = self.backend.to_host_cv(diagonal);
        let c = self.backend.to_host_cm(c);
        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                if i == j {
                    assert_eq!(diagonal[i].re, c[(i, j)].re);
                    assert_eq!(diagonal[i].im, c[(i, j)].im);
                } else {
                    assert_eq!(0.0, c[(i, j)].re);
                    assert_eq!(0.0, c[(i, j)].im);
                }
            })
        });
    }

    fn test_abs_cv(&self) {
        let v = self.make_random_cv(N);

        let mut abs = self.backend.alloc_v(N);
        self.backend.abs_cv(&v, &mut abs);

        let v = self.backend.to_host_cv(v);
        let abs = self.backend.to_host_v(abs);
        v.iter().zip(abs.iter()).for_each(|(v, abs)| {
            assert_approx_eq::assert_approx_eq!(v.abs(), abs);
        });
    }

    fn test_scale_assign_v(&self) {
        let mut v = self.make_random_v(N);
        let vc = self.backend.clone_v(&v);
        let mut rng = rand::thread_rng();
        let scale = rng.gen();

        self.backend.scale_assign_v(scale, &mut v);

        let v = self.backend.to_host_v(v);
        let vc = self.backend.to_host_v(vc);
        v.iter().zip(vc.iter()).for_each(|(&v, &vc)| {
            assert_approx_eq::assert_approx_eq!(scale * vc, v);
        });
    }

    fn test_sqrt_assign_v(&self) {
        let v = self.make_random_cv(N);
        let mut abs = self.backend.alloc_v(N);
        self.backend.abs_cv(&v, &mut abs);
        let absc = self.backend.clone_v(&abs);

        self.backend.sqrt_assign_v(&mut abs);

        let abs = self.backend.to_host_v(abs);
        let absc = self.backend.to_host_v(absc);
        abs.iter().zip(absc.iter()).for_each(|(&v, &vc)| {
            assert_approx_eq::assert_approx_eq!(vc.sqrt(), v);
        });
    }

    fn test_normalize_assign_cv(&self) {
        let mut v = self.make_random_cv(N);

        let vc = self.backend.clone_cv(&v);

        self.backend.normalize_assign_cv(&mut v);

        let v = self.backend.to_host_cv(v);
        let vc = self.backend.to_host_cv(vc);
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            let norm = vc.abs();
            assert_approx_eq::assert_approx_eq!(vc.re / norm, v.re);
            assert_approx_eq::assert_approx_eq!(vc.im / norm, v.im);
        });
    }

    fn test_reciprocal_assign_c(&self) {
        let mut v = self.make_random_cv(N);
        let vc = self.backend.clone_cv(&v);

        self.backend.reciprocal_assign_c(&mut v);

        let v = self.backend.to_host_cv(v);
        let vc = self.backend.to_host_cv(vc);
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            let r = vc.re;
            let i = vc.im;
            let reciprocal = Complex::new(r / (r * r + i * i), -i / (r * r + i * i));
            assert_approx_eq::assert_approx_eq!(reciprocal.re, v.re);
            assert_approx_eq::assert_approx_eq!(reciprocal.im, v.im);
        });
    }

    fn test_pow_assign_v(&self) {
        let mut v = self.make_random_v(N);
        let vc = self.backend.clone_v(&v);

        let mut rng = rand::thread_rng();
        let e = rng.gen();
        self.backend.pow_assign_v(e, &mut v);

        let v = self.backend.to_host_v(v);
        let vc = self.backend.to_host_v(vc);
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            assert_approx_eq::assert_approx_eq!(vc.powf(e), v);
        });
    }

    fn test_concat_row_cm(&self) {
        let a = self.make_random_cm(N, N);
        let b = self.make_random_cm(2 * N, N);
        let mut c = self.backend.alloc_cm(N + 2 * N, N);

        self.backend.concat_row_cm(&a, &b, &mut c);

        let a = self.backend.to_host_cm(a);
        let b = self.backend.to_host_cm(b);
        let c = self.backend.to_host_cm(c);
        (0..N).for_each(|col| (0..N).for_each(|row| assert_eq!(a[(row, col)], c[(row, col)])));
        (0..N).for_each(|col| {
            (0..2 * N).for_each(|row| assert_eq!(b[(row, col)], c[(N + row, col)]))
        });
    }

    fn test_concat_col_cv(&self) {
        let a = self.make_random_cv(N);
        let b = self.make_random_cv(2 * N);
        let mut c = self.backend.alloc_cv(N);

        self.backend.concat_col_cv(&a, &b, &mut c);

        let a = self.backend.to_host_cv(a);
        let b = self.backend.to_host_cv(b);
        let c = self.backend.to_host_cv(c);
        (0..N).for_each(|i| assert_eq!(a[i], c[i]));
        (0..2 * N).for_each(|i| assert_eq!(b[i], c[N + i]));
    }

    fn test_max_eigen_vector_c(&self) {
        let gen_unitary = |size| -> MatrixXc {
            let mut rng = rand::thread_rng();
            let tmp = MatrixXc::from_iterator(
                size,
                size,
                (0..size * size).map(|_| Complex::new(rng.gen(), rng.gen())),
            );

            let hermite = tmp.adjoint() * &tmp;
            (hermite * Complex::new(0.0, 1.0)).exp()
        };

        let u = gen_unitary(N);

        let mut rng = rand::thread_rng();
        let mut lambda_vals: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();
        lambda_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let lambda = MatrixXc::from_diagonal(&VectorXc::from_iterator(
            N,
            lambda_vals.iter().map(|&v| Complex::new(v, 0.)),
        ));

        let a = &u * &lambda * u.adjoint();

        let real = a.iter().map(|c| c.re).collect::<Vec<float>>();
        let imag = a.iter().map(|c| c.im).collect::<Vec<float>>();
        let a = self.backend.from_slice2_cm(N, N, &real, &imag);

        let b = self.backend.max_eigen_vector_c(a);
        let b = self.backend.to_host_cv(b);

        let max_idx = u
            .transpose()
            .rows(N - 1, 1)
            .iter()
            .enumerate()
            .max_by(|(_, value0), (_, value1)| {
                value0.norm_sqr().partial_cmp(&value1.norm_sqr()).unwrap()
            })
            .map(|(idx, _)| idx)
            .unwrap();

        let k = b[max_idx] / u.transpose().rows(N - 1, 1)[max_idx];
        let expected = u.transpose().rows(N - 1, 1) * k;

        (0..N).for_each(|i| {
            assert_approx_eq::assert_approx_eq!(b[i].re, expected[i].re);
            assert_approx_eq::assert_approx_eq!(b[i].im, expected[i].im);
        });
    }

    fn test_hadamard_product_assign_cv(&self) {
        let a = self.make_random_cv(N);
        let mut b = self.make_random_cv(N);
        let bc = self.backend.clone_cv(&b);

        self.backend.hadamard_product_assign_cv(&a, &mut b);

        let a = self.backend.to_host_cv(a);
        let b = self.backend.to_host_cv(b);
        let bc = self.backend.to_host_cv(bc);

        b.iter()
            .zip(a.iter())
            .zip(bc.iter())
            .for_each(|((b, a), bc)| {
                assert_approx_eq::assert_approx_eq!(a.re * bc.re - a.im * bc.im, b.re);
                assert_approx_eq::assert_approx_eq!(a.re * bc.im + a.im * bc.re, b.im);
            });
    }

    fn test_hadamard_product_cv(&self) {
        let a = self.make_random_cv(N);
        let b = self.make_random_cv(N);
        let mut c = self.backend.alloc_cv(N);

        self.backend.hadamard_product_cv(&a, &b, &mut c);

        let a = self.backend.to_host_cv(a);
        let b = self.backend.to_host_cv(b);
        let c = self.backend.to_host_cv(c);

        c.iter()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((c, a), b)| {
                assert_approx_eq::assert_approx_eq!(a.re * b.re - a.im * b.im, c.re);
                assert_approx_eq::assert_approx_eq!(a.re * b.im + a.im * b.re, c.im);
            });
    }

    fn test_gemv_c(&self) {
        let m = N;
        let n = 2 * N;

        let mut rng = rand::thread_rng();

        {
            let a = self.make_random_cm(m, n);
            let b = self.make_random_cv(n);
            let mut c = self.make_random_cv(m);
            let cc = self.backend.clone_cv(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemv_c(Trans::NoTrans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cv(b);
            let c = self.backend.to_host_cv(c);
            let cc = self.backend.to_host_cv(cc);
            let expected = a * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(n, m);
            let b = self.make_random_cv(n);
            let mut c = self.make_random_cv(m);
            let cc = self.backend.clone_cv(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemv_c(Trans::Trans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cv(b);
            let c = self.backend.to_host_cv(c);
            let cc = self.backend.to_host_cv(cc);
            let expected = a.transpose() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(n, m);
            let b = self.make_random_cv(n);
            let mut c = self.make_random_cv(m);
            let cc = self.backend.clone_cv(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemv_c(Trans::ConjTrans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cv(b);
            let c = self.backend.to_host_cv(c);
            let cc = self.backend.to_host_cv(cc);
            let expected = a.adjoint() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }
    }

    fn test_gemm_c(&self) {
        let m = N;
        let n = 2 * N;
        let k = 3 * N;

        let mut rng = rand::thread_rng();

        {
            let a = self.make_random_cm(m, k);
            let b = self.make_random_cm(k, n);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::NoTrans, Trans::NoTrans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(m, k);
            let b = self.make_random_cm(n, k);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::NoTrans, Trans::Trans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a * b.transpose() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(m, k);
            let b = self.make_random_cm(n, k);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend.gemm_c(
                Trans::NoTrans,
                Trans::ConjTrans,
                alpha,
                &a,
                &b,
                beta,
                &mut c,
            );

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a * b.adjoint() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(k, m);
            let b = self.make_random_cm(k, n);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::Trans, Trans::NoTrans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a.transpose() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(k, m);
            let b = self.make_random_cm(n, k);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::Trans, Trans::Trans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a.transpose() * b.transpose() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(k, m);
            let b = self.make_random_cm(n, k);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::Trans, Trans::ConjTrans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a.transpose() * b.adjoint() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(k, m);
            let b = self.make_random_cm(k, n);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend.gemm_c(
                Trans::ConjTrans,
                Trans::NoTrans,
                alpha,
                &a,
                &b,
                beta,
                &mut c,
            );

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a.adjoint() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(k, m);
            let b = self.make_random_cm(n, k);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::ConjTrans, Trans::Trans, alpha, &a, &b, beta, &mut c);

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a.adjoint() * b.transpose() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cm(k, m);
            let b = self.make_random_cm(n, k);
            let mut c = self.make_random_cm(m, n);
            let cc = self.backend.clone_cm(&c);

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend.gemm_c(
                Trans::ConjTrans,
                Trans::ConjTrans,
                alpha,
                &a,
                &b,
                beta,
                &mut c,
            );

            let a = self.backend.to_host_cm(a);
            let b = self.backend.to_host_cm(b);
            let c = self.backend.to_host_cm(c);
            let cc = self.backend.to_host_cm(cc);
            let expected = a.adjoint() * b.adjoint() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }
    }

    fn test_solve_inplace_h(&self) {
        let tmp = self.make_random_cm(N, N);
        let mut a = self.backend.alloc_zeros_cm(N, N);
        self.backend.gemm_c(
            Trans::NoTrans,
            Trans::ConjTrans,
            Complex::new(1., 0.),
            &tmp,
            &tmp,
            Complex::new(0., 0.),
            &mut a,
        );

        let x = self.make_random_cv(N);

        let mut b = self.backend.alloc_zeros_cv(N);
        self.backend.gemv_c(
            Trans::NoTrans,
            Complex::new(1., 0.),
            &a,
            &x,
            Complex::new(0., 0.),
            &mut b,
        );

        self.backend
            .solve_inplace_h(a, &mut b)
            .expect("Failed to solve_inplace_h");

        let x = self.backend.to_host_cv(x);
        let b = self.backend.to_host_cv(b);
        b.iter().zip(x.iter()).for_each(|(b, x)| {
            assert_approx_eq::assert_approx_eq!(b.re, x.re);
            assert_approx_eq::assert_approx_eq!(b.im, x.im);
        });
    }

    fn test_generate_propagation_matrix(&self) {
        let geometry = generate_geometry::<autd3_core::geometry::LegacyTransducer>(4);
        let foci = gen_foci(4).map(|(p, _)| p).collect::<Vec<_>>();

        let reference = {
            let mut g = MatrixXc::zeros(foci.len(), geometry.num_transducers());
            let transducers = geometry.transducers().collect::<Vec<_>>();
            (0..foci.len()).for_each(|i| {
                (0..geometry.num_transducers()).for_each(|j| {
                    g[(i, j)] = propagate_tr::<Sphere, autd3_core::geometry::LegacyTransducer>(
                        &transducers[j],
                        geometry.attenuation,
                        geometry.sound_speed,
                        &foci[i],
                    )
                })
            });
            g
        };

        let g = self.backend.generate_propagation_matrix(&geometry, &foci);
        let g = self.backend.to_host_cm(g);
        reference.iter().zip(g.iter()).for_each(|(r, g)| {
            assert_approx_eq::assert_approx_eq!(r.re, g.re);
            assert_approx_eq::assert_approx_eq!(r.im, g.im);
        });
    }

    fn test_gen_back_prop(&self) {
        let geometry = generate_geometry::<autd3_core::geometry::LegacyTransducer>(4);
        let foci = gen_foci(4).map(|(p, _)| p).collect::<Vec<_>>();

        let m = geometry.num_transducers();
        let n = foci.len();

        let g = self.backend.generate_propagation_matrix(&geometry, &foci);
        let amps = self.make_random_cv(n);

        let mut b = self.backend.alloc_cm(m, n);

        self.backend.gen_back_prop(m, n, &g, &amps, &mut b);
        let amps = self.backend.to_host_cv(amps);
        let g = self.backend.to_host_cm(g);
        let reference = {
            let mut b = MatrixXc::zeros(m, n);
            (0..n).for_each(|i| {
                let x = amps[i] / g.rows(i, 1).iter().map(|x| x.norm_sqr()).sum::<float>();
                (0..m).for_each(|j| {
                    b[(j, i)] = g[(i, j)].conj() * x;
                })
            });
            b
        };

        let b = self.backend.to_host_cm(b);
        reference.iter().zip(b.iter()).for_each(|(r, b)| {
            assert_approx_eq::assert_approx_eq!(r.re, b.re);
            assert_approx_eq::assert_approx_eq!(r.im, b.im);
        });
    }
}
