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

use crate::{Complex, HoloError, LinAlgBackend, MatrixXc, Trans};

use super::bench_utils::{gen_foci, generate_geometry};

const TEST_SIZE: usize = 2;

pub struct LinAlgBackendTestHelper<B: LinAlgBackend> {
    backend: Rc<B>,
}

impl<B: LinAlgBackend> LinAlgBackendTestHelper<B> {
    pub fn new() -> Result<Self, HoloError> {
        Ok(Self { backend: B::new()? })
    }

    pub fn test(&self) {
        self.test_alloc_cv();
        self.test_alloc_cm();
        self.test_alloc_zeros_cv();
        self.test_alloc_zeros_cm();

        self.test_clone_cv();
        self.test_clone_cm();

        self.test_make_complex_v();
        self.test_make_complex2_v();
        self.test_make_complex_m();
        self.test_make_complex2_m();

        self.test_get_diagonal_c();
        self.test_create_diagonal_c();

        self.test_normalize_cv();
        self.test_reciprocal_c();

        self.test_hadamard_product_assign_cv();
        self.test_hadamard_product_cv();

        self.test_gemv_c();
        self.test_gemm_c();

        self.test_generate_propagation_matrix();
        self.test_gen_back_prop();
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
        self.backend.make_complex2_v(&real, &imag)
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
        self.backend.make_complex2_m(rows, cols, &real, &imag)
    }

    fn test_alloc_cv(&self) {
        let v = self.backend.alloc_cv(TEST_SIZE);
        let v = self.backend.to_host_cv(v);

        assert_eq!(TEST_SIZE, v.len());
    }

    fn test_alloc_cm(&self) {
        let m = self.backend.alloc_cm(TEST_SIZE, 2 * TEST_SIZE);
        let m = self.backend.to_host_cm(m);

        assert_eq!(TEST_SIZE, m.nrows());
        assert_eq!(2 * TEST_SIZE, m.ncols());
    }

    fn test_alloc_zeros_cv(&self) {
        let v = self.backend.alloc_cv(TEST_SIZE);
        let v = self.backend.to_host_cv(v);

        assert_eq!(TEST_SIZE, v.len());
        assert!(v.iter().all(|&v| v == Complex::new(0., 0.)));
    }

    fn test_alloc_zeros_cm(&self) {
        let m = self.backend.alloc_cm(TEST_SIZE, 2 * TEST_SIZE);
        let m = self.backend.to_host_cm(m);

        assert_eq!(TEST_SIZE, m.nrows());
        assert_eq!(2 * TEST_SIZE, m.ncols());
        assert!(m.iter().all(|&v| v == Complex::new(0., 0.)));
    }

    fn test_clone_cv(&self) {
        let c = self.make_random_cv(TEST_SIZE);
        let c2 = self.backend.clone_cv(&c);

        let c = self.backend.to_host_cv(c);
        let c2 = self.backend.to_host_cv(c2);

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c.re, c2.re);
            assert_eq!(c.im, c2.im);
        });
    }

    fn test_clone_cm(&self) {
        let c = self.make_random_cm(TEST_SIZE, TEST_SIZE);
        let c2 = self.backend.clone_cm(&c);

        let c = self.backend.to_host_cm(c);
        let c2 = self.backend.to_host_cm(c2);

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c.re, c2.re);
            assert_eq!(c.im, c2.im);
        });
    }

    fn test_make_complex_v(&self) {
        let rng = rand::thread_rng();

        let real: Vec<float> = rng
            .sample_iter(rand::distributions::Standard)
            .take(TEST_SIZE)
            .collect();

        let c = self.backend.make_complex_v(&real);
        let c = self.backend.to_host_cv(c);

        assert_eq!(TEST_SIZE, c.len());
        real.iter().zip(c.iter()).for_each(|(r, c)| {
            assert_eq!(r, &c.re);
            assert_eq!(0.0, c.im);
        });
    }

    fn test_make_complex2_v(&self) {
        let mut rng = rand::thread_rng();

        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(TEST_SIZE)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(TEST_SIZE)
            .collect();

        let c = self.backend.make_complex2_v(&real, &imag);
        let c = self.backend.to_host_cv(c);

        assert_eq!(TEST_SIZE, c.len());
        real.iter()
            .zip(imag.iter())
            .zip(c.iter())
            .for_each(|((r, i), c)| {
                assert_eq!(r, &c.re);
                assert_eq!(i, &c.im);
            });
    }

    fn test_make_complex_m(&self) {
        let rng = rand::thread_rng();

        let real: Vec<float> = rng
            .sample_iter(rand::distributions::Standard)
            .take(TEST_SIZE * 2 * TEST_SIZE)
            .collect();

        let c = self.backend.make_complex_m(TEST_SIZE, 2 * TEST_SIZE, &real);
        let c = self.backend.to_host_cm(c);

        assert_eq!(TEST_SIZE, c.nrows());
        assert_eq!(2 * TEST_SIZE, c.ncols());
        (0..2 * TEST_SIZE).for_each(|col| {
            (0..TEST_SIZE).for_each(|row| {
                assert_eq!(real[col * TEST_SIZE + row], c[(row, col)].re);
                assert_eq!(0.0, c[(row, col)].im);
            })
        });
    }

    fn test_make_complex2_m(&self) {
        let mut rng = rand::thread_rng();

        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(TEST_SIZE * 2 * TEST_SIZE)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(TEST_SIZE * 2 * TEST_SIZE)
            .collect();

        let c = self
            .backend
            .make_complex2_m(TEST_SIZE, 2 * TEST_SIZE, &real, &imag);
        let c = self.backend.to_host_cm(c);

        assert_eq!(TEST_SIZE, c.nrows());
        assert_eq!(2 * TEST_SIZE, c.ncols());
        (0..2 * TEST_SIZE).for_each(|col| {
            (0..TEST_SIZE).for_each(|row| {
                assert_eq!(real[col * TEST_SIZE + row], c[(row, col)].re);
                assert_eq!(imag[col * TEST_SIZE + row], c[(row, col)].im);
            })
        });
    }

    fn test_get_diagonal_c(&self) {
        let c = self.make_random_cm(TEST_SIZE, TEST_SIZE);

        let mut diagonal = self.backend.alloc_cv(TEST_SIZE);
        self.backend.get_diagonal_c(&c, &mut diagonal);

        let c = self.backend.to_host_cm(c);
        let diagonal = self.backend.to_host_cv(diagonal);
        (0..TEST_SIZE).for_each(|i| {
            assert_eq!(c[i * TEST_SIZE + i].re, diagonal[i].re);
            assert_eq!(c[i * TEST_SIZE + i].im, diagonal[i].im);
        });
    }

    fn test_create_diagonal_c(&self) {
        let diagonal = self.make_random_cv(TEST_SIZE);

        let mut c = self.backend.alloc_cm(TEST_SIZE, TEST_SIZE);

        self.backend.create_diagonal_c(&diagonal, &mut c);

        let diagonal = self.backend.to_host_cv(diagonal);
        let c = self.backend.to_host_cm(c);
        (0..TEST_SIZE).for_each(|i| {
            (0..TEST_SIZE).for_each(|j| {
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

    fn test_normalize_cv(&self) {
        let mut v = self.make_random_cv(TEST_SIZE);

        let vc = self.backend.clone_cv(&v);

        self.backend.normalize_cv(&mut v);

        let v = self.backend.to_host_cv(v);
        let vc = self.backend.to_host_cv(vc);
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            let norm = vc.abs();
            assert_approx_eq::assert_approx_eq!(vc.re / norm, v.re);
            assert_approx_eq::assert_approx_eq!(vc.im / norm, v.im);
        });
    }

    fn test_reciprocal_c(&self) {
        let mut v = self.make_random_cv(TEST_SIZE);
        let vc = self.backend.clone_cv(&v);

        self.backend.reciprocal_c(&mut v);

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

    fn test_hadamard_product_assign_cv(&self) {
        let a = self.make_random_cv(TEST_SIZE);
        let mut b = self.make_random_cv(TEST_SIZE);
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
        let a = self.make_random_cv(TEST_SIZE);
        let b = self.make_random_cv(TEST_SIZE);
        let mut c = self.backend.alloc_cv(TEST_SIZE);

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
        let m = TEST_SIZE;
        let n = 2 * TEST_SIZE;

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
        let m = TEST_SIZE;
        let n = 2 * TEST_SIZE;
        let k = 3 * TEST_SIZE;

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
