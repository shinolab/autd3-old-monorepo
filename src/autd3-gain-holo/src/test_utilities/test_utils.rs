/*
 * File: test_utils.rs
 * Project: test_utilities
 * Created Date: 09/08/2023
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
use rand::Rng;

use autd3_driver::{
    acoustics::{propagate_tr, Sphere},
    datagram::GainFilter,
    defined::float,
};

#[cfg(feature = "single_float")]
const EPS: float = 1e-3;
#[cfg(not(feature = "single_float"))]
const EPS: float = 1e-6;

use crate::{Complex, HoloError, LinAlgBackend, MatrixXc, Trans, VectorX, VectorXc};

use super::bench_utils::{gen_foci, generate_geometry};

pub struct LinAlgBackendTestHelper<const N: usize, B: LinAlgBackend> {
    backend: Rc<B>,
}

impl<const N: usize, B: LinAlgBackend> LinAlgBackendTestHelper<N, B> {
    pub fn new() -> Result<Self, HoloError> {
        Ok(Self { backend: B::new()? })
    }

    pub fn test(&self) -> Result<(), HoloError> {
        self.test_alloc_v()?;
        println!("test_alloc_v done");
        self.test_alloc_m()?;
        println!("test_alloc_m done");
        self.test_alloc_cv()?;
        println!("test_alloc_cv done");
        self.test_alloc_cm()?;
        println!("test_alloc_cm done");
        self.test_alloc_zeros_v()?;
        println!("test_alloc_zeros_v done");
        self.test_alloc_zeros_cv()?;
        println!("test_alloc_zeros_cv done");
        self.test_alloc_zeros_cm()?;
        println!("test_alloc_zeros_cm done");

        self.test_cols_c()?;
        println!("test_cols_c done");

        self.test_from_slice_v()?;
        println!("test_from_slice_v done");
        self.test_from_slice_m()?;
        println!("test_from_slice_m done");
        self.test_from_slice_cv()?;
        println!("test_from_slice_cv done");
        self.test_from_slice2_cv()?;
        println!("test_from_slice2_cv done");
        self.test_from_slice2_cm()?;
        println!("test_from_slice2_cm done");

        self.test_copy_from_slice_v()?;
        println!("test_copy_from_slice_v done");

        self.test_copy_to_v()?;
        println!("test_copy_to_v done");
        self.test_copy_to_m()?;
        println!("test_copy_to_m done");

        self.test_clone_v()?;
        println!("test_clone_v done");
        self.test_clone_m()?;
        println!("test_clone_m done");
        self.test_clone_cv()?;
        println!("test_clone_cv done");
        self.test_clone_cm()?;
        println!("test_clone_cm done");

        self.test_make_complex2_v()?;
        println!("test_make_complex2_v done");

        self.test_get_col_c()?;
        println!("test_get_col_c done");
        self.set_cv()?;
        println!("set_cv done");
        self.set_col_c()?;
        println!("set_col_c done");
        self.set_row_c()?;
        println!("set_row_c done");

        self.test_get_diagonal_c()?;
        println!("test_get_diagonal_c done");
        self.test_create_diagonal()?;
        println!("test_create_diagonal done");
        self.test_create_diagonal_c()?;
        println!("test_create_diagonal_c done");
        self.test_get_diagonal()?;
        println!("test_get_diagonal done");

        self.test_abs_cv()?;
        println!("test_abs_cv done");
        self.test_real_cm()?;
        println!("test_real_cm done");
        self.test_imag_cm()?;
        println!("test_imag_cm done");
        self.test_scale_assign_v()?;
        println!("test_scale_assign_v done");
        self.test_scale_assign_cv()?;
        println!("test_scale_assign_cv done");
        self.test_conj_assign_v()?;
        println!("test_conj_assign_v done");
        self.test_sqrt_assign_v()?;
        println!("test_sqrt_assign_v done");
        self.test_normalize_assign_cv()?;
        println!("test_normalize_assign_cv done");
        self.test_reciprocal_assign_c()?;
        println!("test_reciprocal_assign_c done");
        self.test_pow_assign_v()?;
        println!("test_pow_assign_v done");
        self.test_exp_assign_cv()?;
        println!("test_exp_assign_cv done");

        self.test_concat_row_cm()?;
        println!("test_concat_row_cm done");
        self.test_concat_col_cv()?;
        println!("test_concat_col_cv done");
        self.test_concat_col_cm()?;
        println!("test_concat_col_cm done");

        self.test_max_v()?;
        println!("test_max_v done");
        self.test_max_eigen_vector_c()?;
        println!("test_max_eigen_vector_c done");

        self.test_hadamard_product_assign_cv()?;
        println!("test_hadamard_product_assign_cv done");
        self.test_hadamard_product_cv()?;
        println!("test_hadamard_product_cv done");
        self.test_hadamard_product_cm()?;
        println!("test_hadamard_product_cm done");

        self.test_dot()?;
        println!("test_dot done");
        self.test_dot_c()?;
        println!("test_dot_c done");

        self.test_add_v()?;
        println!("test_add_v done");
        self.test_add_m()?;
        println!("test_add_m done");

        self.test_gevv_c()?;
        println!("test_gevv_c done");
        self.test_gemv_c()?;
        println!("test_gemv_c done");
        self.test_gemm_c()?;
        println!("test_gemm_c done");

        self.pseudo_inverse_svd()?;
        println!("pseudo_inverse_svd done");
        self.test_solve_inplace()?;
        println!("test_solve_inplace done");
        self.test_solve_inplace_h()?;
        println!("test_solve_inplace_h done");

        self.test_reduce_col()?;
        println!("test_reduce_col done");

        self.test_scaled_to_cv()?;
        println!("test_scaled_to_cv done");
        self.test_scaled_to_assign_cv()?;
        println!("test_scaled_to_assign_cv done");

        self.test_generate_propagation_matrix()?;
        println!("test_generate_propagation_matrix done");
        self.test_generate_propagation_matrix_with_filter()?;
        println!("test_generate_propagation_matrix_with_filter done");
        self.test_gen_back_prop()?;
        println!("test_gen_back_prop done");

        Ok(())
    }

    fn make_random_v(&self, size: usize) -> Result<B::VectorX, HoloError> {
        let mut rng = rand::thread_rng();
        let v: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(size)
            .collect();
        self.backend.from_slice_v(&v)
    }

    fn make_random_m(&self, rows: usize, cols: usize) -> Result<B::MatrixX, HoloError> {
        let mut rng = rand::thread_rng();
        let v: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(rows * cols)
            .collect();
        self.backend.from_slice_m(rows, cols, &v)
    }

    fn make_random_cv(&self, size: usize) -> Result<B::VectorXc, HoloError> {
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

    fn make_random_cm(&self, rows: usize, cols: usize) -> Result<B::MatrixXc, HoloError> {
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

    fn test_alloc_v(&self) -> Result<(), HoloError> {
        let v = self.backend.alloc_v(N)?;
        let v = self.backend.to_host_v(v)?;

        assert_eq!(N, v.len());
        Ok(())
    }

    fn test_alloc_m(&self) -> Result<(), HoloError> {
        let m = self.backend.alloc_m(N, 2 * N)?;
        let m = self.backend.to_host_m(m)?;

        assert_eq!(N, m.nrows());
        assert_eq!(2 * N, m.ncols());
        Ok(())
    }

    fn test_alloc_cv(&self) -> Result<(), HoloError> {
        let v = self.backend.alloc_cv(N)?;
        let v = self.backend.to_host_cv(v)?;

        assert_eq!(N, v.len());
        Ok(())
    }

    fn test_alloc_cm(&self) -> Result<(), HoloError> {
        let m = self.backend.alloc_cm(N, 2 * N)?;
        let m = self.backend.to_host_cm(m)?;

        assert_eq!(N, m.nrows());
        assert_eq!(2 * N, m.ncols());
        Ok(())
    }

    fn test_alloc_zeros_v(&self) -> Result<(), HoloError> {
        let v = self.backend.alloc_v(N)?;
        let v = self.backend.to_host_v(v)?;

        assert_eq!(N, v.len());
        assert!(v.iter().all(|&v| v == 0.));
        Ok(())
    }

    fn test_alloc_zeros_cv(&self) -> Result<(), HoloError> {
        let v = self.backend.alloc_cv(N)?;
        let v = self.backend.to_host_cv(v)?;

        assert_eq!(N, v.len());
        assert!(v.iter().all(|&v| v == Complex::new(0., 0.)));
        Ok(())
    }

    fn test_alloc_zeros_cm(&self) -> Result<(), HoloError> {
        let m = self.backend.alloc_cm(N, 2 * N)?;
        let m = self.backend.to_host_cm(m)?;

        assert_eq!(N, m.nrows());
        assert_eq!(2 * N, m.ncols());
        assert!(m.iter().all(|&v| v == Complex::new(0., 0.)));
        Ok(())
    }

    fn test_cols_c(&self) -> Result<(), HoloError> {
        let m = self.backend.alloc_cm(N, 2 * N)?;

        assert_eq!(2 * N, self.backend.cols_c(&m)?);

        Ok(())
    }

    fn test_from_slice_v(&self) -> Result<(), HoloError> {
        let rng = rand::thread_rng();

        let v: Vec<float> = rng
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();

        let c = self.backend.from_slice_v(&v)?;
        let c = self.backend.to_host_v(c)?;

        assert_eq!(N, c.len());
        v.iter().zip(c.iter()).for_each(|(&r, &c)| {
            assert_eq!(r, c);
        });
        Ok(())
    }

    fn test_from_slice_m(&self) -> Result<(), HoloError> {
        let rng = rand::thread_rng();

        let v: Vec<float> = rng
            .sample_iter(rand::distributions::Standard)
            .take(N * 2 * N)
            .collect();

        let c = self.backend.from_slice_m(N, 2 * N, &v)?;
        let c = self.backend.to_host_m(c)?;

        assert_eq!(N, c.nrows());
        assert_eq!(2 * N, c.ncols());
        (0..2 * N).for_each(|col| {
            (0..N).for_each(|row| {
                assert_eq!(v[col * N + row], c[(row, col)]);
            })
        });
        Ok(())
    }

    fn test_from_slice_cv(&self) -> Result<(), HoloError> {
        let rng = rand::thread_rng();

        let real: Vec<float> = rng
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();

        let c = self.backend.from_slice_cv(&real)?;
        let c = self.backend.to_host_cv(c)?;

        assert_eq!(N, c.len());
        real.iter().zip(c.iter()).for_each(|(r, c)| {
            assert_eq!(r, &c.re);
            assert_eq!(0.0, c.im);
        });
        Ok(())
    }

    fn test_from_slice2_cv(&self) -> Result<(), HoloError> {
        let mut rng = rand::thread_rng();

        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N)
            .collect();

        let c = self.backend.from_slice2_cv(&real, &imag)?;
        let c = self.backend.to_host_cv(c)?;

        assert_eq!(N, c.len());
        real.iter()
            .zip(imag.iter())
            .zip(c.iter())
            .for_each(|((r, i), c)| {
                assert_eq!(r, &c.re);
                assert_eq!(i, &c.im);
            });
        Ok(())
    }

    fn test_from_slice2_cm(&self) -> Result<(), HoloError> {
        let mut rng = rand::thread_rng();

        let real: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N * 2 * N)
            .collect();
        let imag: Vec<float> = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(N * 2 * N)
            .collect();

        let c = self.backend.from_slice2_cm(N, 2 * N, &real, &imag)?;
        let c = self.backend.to_host_cm(c)?;

        assert_eq!(N, c.nrows());
        assert_eq!(2 * N, c.ncols());
        (0..2 * N).for_each(|col| {
            (0..N).for_each(|row| {
                assert_eq!(real[col * N + row], c[(row, col)].re);
                assert_eq!(imag[col * N + row], c[(row, col)].im);
            })
        });
        Ok(())
    }

    fn test_copy_from_slice_v(&self) -> Result<(), HoloError> {
        {
            let mut a = self.backend.alloc_zeros_v(N)?;
            let mut rng = rand::thread_rng();
            let v = (&mut rng)
                .sample_iter(rand::distributions::Standard)
                .take(N / 2)
                .collect::<Vec<float>>();

            self.backend.copy_from_slice_v(&v, &mut a)?;

            let a = self.backend.to_host_v(a)?;
            (0..N / 2).for_each(|i| {
                assert_eq!(v[i], a[i]);
            });
            (N / 2..N).for_each(|i| {
                assert_eq!(0., a[i]);
            });
        }

        {
            let mut a = self.backend.alloc_zeros_v(N)?;
            let v = [];

            self.backend.copy_from_slice_v(&v, &mut a)?;

            let a = self.backend.to_host_v(a)?;
            a.iter().for_each(|&a| {
                assert_eq!(0., a);
            });
        }

        Ok(())
    }

    fn test_copy_to_v(&self) -> Result<(), HoloError> {
        let a = self.make_random_v(N)?;
        let mut b = self.backend.alloc_v(N)?;

        self.backend.copy_to_v(&a, &mut b)?;

        let a = self.backend.to_host_v(a)?;
        let b = self.backend.to_host_v(b)?;
        a.iter().zip(b.iter()).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
        Ok(())
    }

    fn test_copy_to_m(&self) -> Result<(), HoloError> {
        let a = self.make_random_m(N, N)?;
        let mut b = self.backend.alloc_m(N, N)?;

        self.backend.copy_to_m(&a, &mut b)?;

        let a = self.backend.to_host_m(a)?;
        let b = self.backend.to_host_m(b)?;
        a.iter().zip(b.iter()).for_each(|(a, b)| {
            assert_eq!(a, b);
        });
        Ok(())
    }

    fn test_clone_v(&self) -> Result<(), HoloError> {
        let c = self.make_random_v(N)?;
        let c2 = self.backend.clone_v(&c)?;

        let c = self.backend.to_host_v(c)?;
        let c2 = self.backend.to_host_v(c2)?;

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c, c2);
        });
        Ok(())
    }

    fn test_clone_m(&self) -> Result<(), HoloError> {
        let c = self.make_random_m(N, N)?;
        let c2 = self.backend.clone_m(&c)?;

        let c = self.backend.to_host_m(c)?;
        let c2 = self.backend.to_host_m(c2)?;

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c, c2);
        });
        Ok(())
    }

    fn test_clone_cv(&self) -> Result<(), HoloError> {
        let c = self.make_random_cv(N)?;
        let c2 = self.backend.clone_cv(&c)?;

        let c = self.backend.to_host_cv(c)?;
        let c2 = self.backend.to_host_cv(c2)?;

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c.re, c2.re);
            assert_eq!(c.im, c2.im);
        });
        Ok(())
    }

    fn test_clone_cm(&self) -> Result<(), HoloError> {
        let c = self.make_random_cm(N, N)?;
        let c2 = self.backend.clone_cm(&c)?;

        let c = self.backend.to_host_cm(c)?;
        let c2 = self.backend.to_host_cm(c2)?;

        c.iter().zip(c2.iter()).for_each(|(c, c2)| {
            assert_eq!(c.re, c2.re);
            assert_eq!(c.im, c2.im);
        });
        Ok(())
    }

    fn test_make_complex2_v(&self) -> Result<(), HoloError> {
        let real = self.make_random_v(N)?;
        let imag = self.make_random_v(N)?;

        let mut c = self.backend.alloc_cv(N)?;
        self.backend.make_complex2_v(&real, &imag, &mut c)?;

        let real = self.backend.to_host_v(real)?;
        let imag = self.backend.to_host_v(imag)?;
        let c = self.backend.to_host_cv(c)?;
        real.iter()
            .zip(imag.iter())
            .zip(c.iter())
            .for_each(|((r, i), c)| {
                assert_eq!(r, &c.re);
                assert_eq!(i, &c.im);
            });
        Ok(())
    }

    fn test_get_col_c(&self) -> Result<(), HoloError> {
        let a = self.make_random_cm(N, N)?;
        let ac = self.backend.clone_cm(&a)?;

        let ac = self.backend.to_host_cm(ac)?;
        (0..N)
            .map(|col| {
                let mut v = self.backend.alloc_cv(N)?;
                self.backend.get_col_c(&a, col, &mut v)?;
                let v = self.backend.to_host_cv(v)?;
                (0..N).for_each(|row| {
                    assert_eq!(ac[(row, col)].re, v[row].re);
                    assert_eq!(ac[(row, col)].im, v[row].im);
                });
                Ok(())
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn set_cv(&self) -> Result<(), HoloError> {
        let mut a = self.backend.alloc_zeros_cv(N)?;

        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..N);
        let v = Complex::new(rng.gen(), rng.gen());

        self.backend.set_cv(i, v, &mut a)?;

        let a = self.backend.to_host_cv(a)?;
        (0..N).for_each(|j| {
            if j == i {
                assert_eq!(v.re, a[j].re);
                assert_eq!(v.im, a[j].im);
            } else {
                assert_eq!(0.0, a[j].re);
                assert_eq!(0.0, a[j].im);
            }
        });
        Ok(())
    }

    fn set_col_c(&self) -> Result<(), HoloError> {
        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..2 * N);
            let start = rng.gen_range(1..N / 2);
            let end = rng.gen_range(N / 2..N - 1);

            self.backend.set_col_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if col == i && (start <= row && row < end) {
                        assert_eq!(b[row].re, a[(row, col)].re);
                        assert_eq!(b[row].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }

        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..2 * N);
            let start = 0;
            let end = N - 1;

            self.backend.set_col_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if col == i && (start <= row && row < end) {
                        assert_eq!(b[row].re, a[(row, col)].re);
                        assert_eq!(b[row].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }

        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..2 * N);
            let start = 0;
            let end = 0;

            self.backend.set_col_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if col == i && (start <= row && row < end) {
                        assert_eq!(b[row].re, a[(row, col)].re);
                        assert_eq!(b[row].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }

        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..2 * N);
            let start = N - 1;
            let end = N - 1;

            self.backend.set_col_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if col == i && (start <= row && row < end) {
                        assert_eq!(b[row].re, a[(row, col)].re);
                        assert_eq!(b[row].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }
        Ok(())
    }

    fn set_row_c(&self) -> Result<(), HoloError> {
        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(2 * N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..N);
            let start = rng.gen_range(1..N);
            let end = rng.gen_range(N..2 * N - 1);

            self.backend.set_row_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if row == i && (start <= col && col < end) {
                        assert_eq!(b[col].re, a[(row, col)].re);
                        assert_eq!(b[col].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }

        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(2 * N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..N);
            let start = 0;
            let end = 2 * N - 1;

            self.backend.set_row_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if row == i && (start <= col && col < end) {
                        assert_eq!(b[col].re, a[(row, col)].re);
                        assert_eq!(b[col].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }

        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(2 * N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..N);
            let start = 0;
            let end = 0;

            self.backend.set_row_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if row == i && (start <= col && col < end) {
                        assert_eq!(b[col].re, a[(row, col)].re);
                        assert_eq!(b[col].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }

        {
            let mut a = self.backend.alloc_zeros_cm(N, 2 * N)?;
            let b = self.make_random_cv(2 * N)?;

            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..N);
            let start = 2 * N - 1;
            let end = 2 * N - 1;

            self.backend.set_row_c(&b, i, start, end, &mut a)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            (0..N).for_each(|row| {
                (0..2 * N).for_each(|col| {
                    if row == i && (start <= col && col < end) {
                        assert_eq!(b[col].re, a[(row, col)].re);
                        assert_eq!(b[col].im, a[(row, col)].im);
                    } else {
                        assert_eq!(0.0, a[(row, col)].re);
                        assert_eq!(0.0, a[(row, col)].im);
                    }
                })
            });
        }
        Ok(())
    }

    fn test_get_diagonal_c(&self) -> Result<(), HoloError> {
        let c = self.make_random_cm(N, N)?;

        let mut diagonal = self.backend.alloc_cv(N)?;
        self.backend.get_diagonal_c(&c, &mut diagonal)?;

        let c = self.backend.to_host_cm(c)?;
        let diagonal = self.backend.to_host_cv(diagonal)?;
        (0..N).for_each(|i| {
            assert_eq!(c[i * N + i].re, diagonal[i].re);
            assert_eq!(c[i * N + i].im, diagonal[i].im);
        });
        Ok(())
    }

    fn test_create_diagonal(&self) -> Result<(), HoloError> {
        let diagonal = self.make_random_v(N)?;

        let mut c = self.backend.alloc_m(N, N)?;

        self.backend.create_diagonal(&diagonal, &mut c)?;

        let diagonal = self.backend.to_host_v(diagonal)?;
        let c = self.backend.to_host_m(c)?;
        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                if i == j {
                    assert_eq!(diagonal[i], c[(i, j)]);
                } else {
                    assert_eq!(0.0, c[(i, j)]);
                }
            })
        });
        Ok(())
    }

    fn test_create_diagonal_c(&self) -> Result<(), HoloError> {
        let diagonal = self.make_random_cv(N)?;

        let mut c = self.backend.alloc_cm(N, N)?;

        self.backend.create_diagonal_c(&diagonal, &mut c)?;

        let diagonal = self.backend.to_host_cv(diagonal)?;
        let c = self.backend.to_host_cm(c)?;
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
        Ok(())
    }

    fn test_get_diagonal(&self) -> Result<(), HoloError> {
        let m = self.make_random_m(N, N)?;
        let mut diagonal = self.backend.alloc_v(N)?;

        self.backend.get_diagonal(&m, &mut diagonal)?;

        let m = self.backend.to_host_m(m)?;
        let diagonal = self.backend.to_host_v(diagonal)?;
        (0..N).for_each(|i| {
            assert_eq!(m[(i, i)], diagonal[i]);
        });
        Ok(())
    }

    fn test_abs_cv(&self) -> Result<(), HoloError> {
        let v = self.make_random_cv(N)?;

        let mut abs = self.backend.alloc_v(N)?;
        self.backend.abs_cv(&v, &mut abs)?;

        let v = self.backend.to_host_cv(v)?;
        let abs = self.backend.to_host_v(abs)?;
        v.iter().zip(abs.iter()).for_each(|(v, abs)| {
            assert_approx_eq::assert_approx_eq!(v.abs(), abs);
        });
        Ok(())
    }

    fn test_real_cm(&self) -> Result<(), HoloError> {
        let v = self.make_random_cm(N, N)?;
        let mut r = self.backend.alloc_m(N, N)?;

        self.backend.real_cm(&v, &mut r)?;

        let v = self.backend.to_host_cm(v)?;
        let r = self.backend.to_host_m(r)?;
        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                assert_approx_eq::assert_approx_eq!(v[(i, j)].re, r[(i, j)]);
            })
        });
        Ok(())
    }

    fn test_imag_cm(&self) -> Result<(), HoloError> {
        let v = self.make_random_cm(N, N)?;
        let mut r = self.backend.alloc_m(N, N)?;

        self.backend.imag_cm(&v, &mut r)?;

        let v = self.backend.to_host_cm(v)?;
        let r = self.backend.to_host_m(r)?;
        (0..N).for_each(|i| {
            (0..N).for_each(|j| {
                assert_approx_eq::assert_approx_eq!(v[(i, j)].im, r[(i, j)]);
            })
        });
        Ok(())
    }

    fn test_scale_assign_v(&self) -> Result<(), HoloError> {
        let mut v = self.make_random_v(N)?;
        let vc = self.backend.clone_v(&v)?;
        let mut rng = rand::thread_rng();
        let scale = rng.gen();

        self.backend.scale_assign_v(scale, &mut v)?;

        let v = self.backend.to_host_v(v)?;
        let vc = self.backend.to_host_v(vc)?;
        v.iter().zip(vc.iter()).for_each(|(&v, &vc)| {
            assert_approx_eq::assert_approx_eq!(scale * vc, v);
        });
        Ok(())
    }

    fn test_scale_assign_cv(&self) -> Result<(), HoloError> {
        let mut v = self.make_random_cv(N)?;
        let vc = self.backend.clone_cv(&v)?;
        let mut rng = rand::thread_rng();
        let scale = Complex::new(rng.gen(), rng.gen());

        self.backend.scale_assign_cv(scale, &mut v)?;

        let v = self.backend.to_host_cv(v)?;
        let vc = self.backend.to_host_cv(vc)?;
        v.iter().zip(vc.iter()).for_each(|(&v, &vc)| {
            assert_approx_eq::assert_approx_eq!(scale * vc, v);
        });
        Ok(())
    }

    fn test_conj_assign_v(&self) -> Result<(), HoloError> {
        let mut v = self.make_random_cv(N)?;
        let vc = self.backend.clone_cv(&v)?;

        self.backend.conj_assign_v(&mut v)?;

        let v = self.backend.to_host_cv(v)?;
        let vc = self.backend.to_host_cv(vc)?;
        v.iter().zip(vc.iter()).for_each(|(&v, &vc)| {
            assert_eq!(vc.re, v.re);
            assert_eq!(vc.im, -v.im);
        });
        Ok(())
    }

    fn test_sqrt_assign_v(&self) -> Result<(), HoloError> {
        let v = self.make_random_cv(N)?;
        let mut abs = self.backend.alloc_v(N)?;
        self.backend.abs_cv(&v, &mut abs)?;
        let absc = self.backend.clone_v(&abs)?;

        self.backend.sqrt_assign_v(&mut abs)?;

        let abs = self.backend.to_host_v(abs)?;
        let absc = self.backend.to_host_v(absc)?;
        abs.iter().zip(absc.iter()).for_each(|(&v, &vc)| {
            assert_approx_eq::assert_approx_eq!(vc.sqrt(), v);
        });
        Ok(())
    }

    fn test_normalize_assign_cv(&self) -> Result<(), HoloError> {
        let mut v = self.make_random_cv(N)?;

        let vc = self.backend.clone_cv(&v)?;

        self.backend.normalize_assign_cv(&mut v)?;

        let v = self.backend.to_host_cv(v)?;
        let vc = self.backend.to_host_cv(vc)?;
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            let norm = vc.abs();
            assert_approx_eq::assert_approx_eq!(vc.re / norm, v.re);
            assert_approx_eq::assert_approx_eq!(vc.im / norm, v.im);
        });
        Ok(())
    }

    fn test_reciprocal_assign_c(&self) -> Result<(), HoloError> {
        let mut v = self.make_random_cv(N)?;
        let vc = self.backend.clone_cv(&v)?;

        self.backend.reciprocal_assign_c(&mut v)?;

        let v = self.backend.to_host_cv(v)?;
        let vc = self.backend.to_host_cv(vc)?;
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            let r = vc.re;
            let i = vc.im;
            let reciprocal = Complex::new(r / (r * r + i * i), -i / (r * r + i * i));
            assert_approx_eq::assert_approx_eq!(reciprocal.re, v.re);
            assert_approx_eq::assert_approx_eq!(reciprocal.im, v.im);
        });
        Ok(())
    }

    fn test_pow_assign_v(&self) -> Result<(), HoloError> {
        let mut v = self.make_random_v(N)?;
        let vc = self.backend.clone_v(&v)?;

        let mut rng = rand::thread_rng();
        let e = rng.gen();
        self.backend.pow_assign_v(e, &mut v)?;

        let v = self.backend.to_host_v(v)?;
        let vc = self.backend.to_host_v(vc)?;
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            assert_approx_eq::assert_approx_eq!(vc.powf(e), v);
        });
        Ok(())
    }

    fn test_exp_assign_cv(&self) -> Result<(), HoloError> {
        let mut v = self.make_random_cv(N)?;
        let vc = self.backend.clone_cv(&v)?;

        self.backend.exp_assign_cv(&mut v)?;

        let v = self.backend.to_host_cv(v)?;
        let vc = self.backend.to_host_cv(vc)?;
        v.iter().zip(vc.iter()).for_each(|(v, vc)| {
            assert_approx_eq::assert_approx_eq!(vc.exp(), v);
        });
        Ok(())
    }

    fn test_concat_row_cm(&self) -> Result<(), HoloError> {
        let a = self.make_random_cm(N, N)?;
        let b = self.make_random_cm(2 * N, N)?;
        let mut c = self.backend.alloc_cm(N + 2 * N, N)?;

        self.backend.concat_row_cm(&a, &b, &mut c)?;

        let a = self.backend.to_host_cm(a)?;
        let b = self.backend.to_host_cm(b)?;
        let c = self.backend.to_host_cm(c)?;
        (0..N).for_each(|col| (0..N).for_each(|row| assert_eq!(a[(row, col)], c[(row, col)])));
        (0..N).for_each(|col| {
            (0..2 * N).for_each(|row| assert_eq!(b[(row, col)], c[(N + row, col)]))
        });
        Ok(())
    }

    fn test_concat_col_cv(&self) -> Result<(), HoloError> {
        let a = self.make_random_cv(N)?;
        let b = self.make_random_cv(2 * N)?;
        let mut c = self.backend.alloc_cv(N + 2 * N)?;

        self.backend.concat_col_cv(&a, &b, &mut c)?;

        let a = self.backend.to_host_cv(a)?;
        let b = self.backend.to_host_cv(b)?;
        let c = self.backend.to_host_cv(c)?;
        (0..N).for_each(|i| assert_eq!(a[i], c[i]));
        (0..2 * N).for_each(|i| assert_eq!(b[i], c[N + i]));
        Ok(())
    }

    fn test_concat_col_cm(&self) -> Result<(), HoloError> {
        let a = self.make_random_cm(N, N)?;
        let b = self.make_random_cm(N, 2 * N)?;
        let mut c = self.backend.alloc_cm(N, N + 2 * N)?;

        self.backend.concat_col_cm(&a, &b, &mut c)?;

        let a = self.backend.to_host_cm(a)?;
        let b = self.backend.to_host_cm(b)?;
        let c = self.backend.to_host_cm(c)?;
        (0..N).for_each(|col| (0..N).for_each(|row| assert_eq!(a[(row, col)], c[(row, col)])));
        (0..2 * N)
            .for_each(|col| (0..N).for_each(|row| assert_eq!(b[(row, col)], c[(row, N + col)])));
        Ok(())
    }

    fn test_max_v(&self) -> Result<(), HoloError> {
        let v = self.make_random_v(N)?;

        let max = self.backend.max_v(&v)?;

        let v = self.backend.to_host_v(v)?;
        assert_eq!(
            *v.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            max
        );
        Ok(())
    }

    fn test_max_eigen_vector_c(&self) -> Result<(), HoloError> {
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
        let a = self.backend.from_slice2_cm(N, N, &real, &imag)?;

        let b = self.backend.max_eigen_vector_c(a)?;
        let b = self.backend.to_host_cv(b)?;

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
            assert_approx_eq::assert_approx_eq!(b[i].re, expected[i].re, EPS);
            assert_approx_eq::assert_approx_eq!(b[i].im, expected[i].im, EPS);
        });
        Ok(())
    }

    fn test_hadamard_product_assign_cv(&self) -> Result<(), HoloError> {
        let a = self.make_random_cv(N)?;
        let mut b = self.make_random_cv(N)?;
        let bc = self.backend.clone_cv(&b)?;

        self.backend.hadamard_product_assign_cv(&a, &mut b)?;

        let a = self.backend.to_host_cv(a)?;
        let b = self.backend.to_host_cv(b)?;
        let bc = self.backend.to_host_cv(bc)?;

        b.iter()
            .zip(a.iter())
            .zip(bc.iter())
            .for_each(|((b, a), bc)| {
                assert_approx_eq::assert_approx_eq!(a.re * bc.re - a.im * bc.im, b.re);
                assert_approx_eq::assert_approx_eq!(a.re * bc.im + a.im * bc.re, b.im);
            });
        Ok(())
    }

    fn test_hadamard_product_cv(&self) -> Result<(), HoloError> {
        let a = self.make_random_cv(N)?;
        let b = self.make_random_cv(N)?;
        let mut c = self.backend.alloc_cv(N)?;

        self.backend.hadamard_product_cv(&a, &b, &mut c)?;

        let a = self.backend.to_host_cv(a)?;
        let b = self.backend.to_host_cv(b)?;
        let c = self.backend.to_host_cv(c)?;

        c.iter()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((c, a), b)| {
                assert_approx_eq::assert_approx_eq!(a.re * b.re - a.im * b.im, c.re);
                assert_approx_eq::assert_approx_eq!(a.re * b.im + a.im * b.re, c.im);
            });
        Ok(())
    }

    fn test_hadamard_product_cm(&self) -> Result<(), HoloError> {
        let a = self.make_random_cm(N, N)?;
        let b = self.make_random_cm(N, N)?;
        let mut c = self.backend.alloc_cm(N, N)?;

        self.backend.hadamard_product_cm(&a, &b, &mut c)?;

        let a = self.backend.to_host_cm(a)?;
        let b = self.backend.to_host_cm(b)?;
        let c = self.backend.to_host_cm(c)?;
        c.iter()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((c, a), b)| {
                assert_approx_eq::assert_approx_eq!(a.re * b.re - a.im * b.im, c.re);
                assert_approx_eq::assert_approx_eq!(a.re * b.im + a.im * b.re, c.im);
            });
        Ok(())
    }

    fn test_dot(&self) -> Result<(), HoloError> {
        let a = self.make_random_v(N)?;
        let b = self.make_random_v(N)?;

        let dot = self.backend.dot(&a, &b)?;

        let a = self.backend.to_host_v(a)?;
        let b = self.backend.to_host_v(b)?;
        let expect = a.iter().zip(b.iter()).map(|(a, b)| a * b).sum::<float>();
        assert_approx_eq::assert_approx_eq!(dot, expect, EPS);
        Ok(())
    }

    fn test_dot_c(&self) -> Result<(), HoloError> {
        let a = self.make_random_cv(N)?;
        let b = self.make_random_cv(N)?;

        let dot = self.backend.dot_c(&a, &b)?;

        let a = self.backend.to_host_cv(a)?;
        let b = self.backend.to_host_cv(b)?;
        let expect = a
            .iter()
            .zip(b.iter())
            .map(|(a, b)| a.conj() * b)
            .sum::<Complex>();
        assert_approx_eq::assert_approx_eq!(dot.re, expect.re);
        assert_approx_eq::assert_approx_eq!(dot.im, expect.im);
        Ok(())
    }

    fn test_add_v(&self) -> Result<(), HoloError> {
        let a = self.make_random_v(N)?;
        let mut b = self.make_random_v(N)?;
        let bc = self.backend.clone_v(&b)?;

        let mut rng = rand::thread_rng();
        let alpha = rng.gen();

        self.backend.add_v(alpha, &a, &mut b)?;

        let a = self.backend.to_host_v(a)?;
        let b = self.backend.to_host_v(b)?;
        let bc = self.backend.to_host_v(bc)?;
        b.iter()
            .zip(a.iter())
            .zip(bc.iter())
            .for_each(|((b, a), bc)| {
                assert_approx_eq::assert_approx_eq!(alpha * a + bc, b);
            });
        Ok(())
    }

    fn test_add_m(&self) -> Result<(), HoloError> {
        let a = self.make_random_m(N, N)?;
        let mut b = self.make_random_m(N, N)?;
        let bc = self.backend.clone_m(&b)?;

        let mut rng = rand::thread_rng();
        let alpha = rng.gen();

        self.backend.add_m(alpha, &a, &mut b)?;

        let a = self.backend.to_host_m(a)?;
        let b = self.backend.to_host_m(b)?;
        let bc = self.backend.to_host_m(bc)?;
        b.iter()
            .zip(a.iter())
            .zip(bc.iter())
            .for_each(|((b, a), bc)| {
                assert_approx_eq::assert_approx_eq!(alpha * a + bc, b);
            });
        Ok(())
    }

    fn test_gevv_c(&self) -> Result<(), HoloError> {
        let mut rng = rand::thread_rng();

        {
            let a = self.make_random_cv(N)?;
            let b = self.make_random_cv(N)?;
            let mut c = self.make_random_cm(N, N)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gevv_c(Trans::NoTrans, Trans::Trans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cv(a)?;
            let b = self.backend.to_host_cv(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a * b.transpose() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }

        {
            let a = self.make_random_cv(N)?;
            let b = self.make_random_cv(N)?;
            let mut c = self.make_random_cm(N, N)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend.gevv_c(
                Trans::NoTrans,
                Trans::ConjTrans,
                alpha,
                &a,
                &b,
                beta,
                &mut c,
            )?;

            let a = self.backend.to_host_cv(a)?;
            let b = self.backend.to_host_cv(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a * b.adjoint() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im);
            });
        }
        Ok(())
    }

    fn test_gemv_c(&self) -> Result<(), HoloError> {
        let m = N;
        let n = 2 * N;

        let mut rng = rand::thread_rng();

        {
            let a = self.make_random_cm(m, n)?;
            let b = self.make_random_cv(n)?;
            let mut c = self.make_random_cv(m)?;
            let cc = self.backend.clone_cv(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemv_c(Trans::NoTrans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            let c = self.backend.to_host_cv(c)?;
            let cc = self.backend.to_host_cv(cc)?;
            let expected = a * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(n, m)?;
            let b = self.make_random_cv(n)?;
            let mut c = self.make_random_cv(m)?;
            let cc = self.backend.clone_cv(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemv_c(Trans::Trans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            let c = self.backend.to_host_cv(c)?;
            let cc = self.backend.to_host_cv(cc)?;
            let expected = a.transpose() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(n, m)?;
            let b = self.make_random_cv(n)?;
            let mut c = self.make_random_cv(m)?;
            let cc = self.backend.clone_cv(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemv_c(Trans::ConjTrans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cv(b)?;
            let c = self.backend.to_host_cv(c)?;
            let cc = self.backend.to_host_cv(cc)?;
            let expected = a.adjoint() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }
        Ok(())
    }

    fn test_gemm_c(&self) -> Result<(), HoloError> {
        let m = N;
        let n = 2 * N;
        let k = 3 * N;

        let mut rng = rand::thread_rng();

        {
            let a = self.make_random_cm(m, k)?;
            let b = self.make_random_cm(k, n)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::NoTrans, Trans::NoTrans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(m, k)?;
            let b = self.make_random_cm(n, k)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::NoTrans, Trans::Trans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a * b.transpose() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(m, k)?;
            let b = self.make_random_cm(n, k)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

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
            )?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a * b.adjoint() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(k, m)?;
            let b = self.make_random_cm(k, n)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::Trans, Trans::NoTrans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a.transpose() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(k, m)?;
            let b = self.make_random_cm(n, k)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::Trans, Trans::Trans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a.transpose() * b.transpose() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(k, m)?;
            let b = self.make_random_cm(n, k)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::Trans, Trans::ConjTrans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a.transpose() * b.adjoint() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(k, m)?;
            let b = self.make_random_cm(k, n)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

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
            )?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a.adjoint() * b * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(k, m)?;
            let b = self.make_random_cm(n, k)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

            let alpha = Complex::new(rng.gen(), rng.gen());
            let beta = Complex::new(rng.gen(), rng.gen());
            self.backend
                .gemm_c(Trans::ConjTrans, Trans::Trans, alpha, &a, &b, beta, &mut c)?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a.adjoint() * b.transpose() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }

        {
            let a = self.make_random_cm(k, m)?;
            let b = self.make_random_cm(n, k)?;
            let mut c = self.make_random_cm(m, n)?;
            let cc = self.backend.clone_cm(&c)?;

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
            )?;

            let a = self.backend.to_host_cm(a)?;
            let b = self.backend.to_host_cm(b)?;
            let c = self.backend.to_host_cm(c)?;
            let cc = self.backend.to_host_cm(cc)?;
            let expected = a.adjoint() * b.adjoint() * alpha + cc * beta;
            c.iter().zip(expected.iter()).for_each(|(c, expected)| {
                assert_approx_eq::assert_approx_eq!(c.re, expected.re, EPS);
                assert_approx_eq::assert_approx_eq!(c.im, expected.im, EPS);
            });
        }
        Ok(())
    }

    fn pseudo_inverse_svd(&self) -> Result<(), HoloError> {
        let a = self.make_random_cm(N, 5 * N)?;
        let mut b = self.backend.alloc_zeros_cm(5 * N, N)?;
        let mut u = self.backend.alloc_zeros_cm(N, N)?;
        let mut s = self.backend.alloc_zeros_cm(5 * N, N)?;
        let mut vt = self.backend.alloc_zeros_cm(5 * N, 5 * N)?;
        let mut buf = self.backend.alloc_zeros_cm(5 * N, N)?;
        let tmp = self.backend.clone_cm(&a)?;
        self.backend
            .pseudo_inverse_svd(tmp, 0., &mut u, &mut s, &mut vt, &mut buf, &mut b)?;

        let mut c = self.backend.alloc_zeros_cm(N, N)?;
        self.backend.gemm_c(
            Trans::NoTrans,
            Trans::NoTrans,
            Complex::new(1., 0.),
            &a,
            &b,
            Complex::new(0., 0.),
            &mut c,
        )?;

        let c = self.backend.to_host_cm(c)?;

        (0..N).for_each(|col| {
            (0..N).for_each(|row| {
                if row == col {
                    assert_approx_eq::assert_approx_eq!(c[(row, col)].re, 1., 0.1);
                    assert_approx_eq::assert_approx_eq!(c[(row, col)].im, 0., 0.1);
                } else {
                    assert_approx_eq::assert_approx_eq!(c[(row, col)].re, 0., 0.1);
                    assert_approx_eq::assert_approx_eq!(c[(row, col)].im, 0., 0.1);
                }
            })
        });
        Ok(())
    }

    fn test_solve_inplace(&self) -> Result<(), HoloError> {
        let tmp = self.make_random_m(N, N)?;
        let tmp = self.backend.to_host_m(tmp)?;

        let a = &tmp * tmp.adjoint();

        let mut rng = rand::thread_rng();
        let x = VectorX::from_iterator(N, (0..N).map(|_| rng.gen()));

        let b = &a * &x;

        let aa = self.backend.from_slice_m(N, N, a.as_slice())?;
        let mut bb = self.backend.from_slice_v(b.as_slice())?;

        self.backend.solve_inplace(&aa, &mut bb)?;

        let b = self.backend.to_host_v(bb)?;
        b.iter().zip(x.iter()).for_each(|(b, x)| {
            assert_approx_eq::assert_approx_eq!(b, x, EPS);
        });
        Ok(())
    }

    fn test_solve_inplace_h(&self) -> Result<(), HoloError> {
        let tmp = self.make_random_cm(N, N)?;
        let mut a = self.backend.alloc_zeros_cm(N, N)?;
        self.backend.gemm_c(
            Trans::NoTrans,
            Trans::ConjTrans,
            Complex::new(1., 0.),
            &tmp,
            &tmp,
            Complex::new(0., 0.),
            &mut a,
        )?;

        let x = self.make_random_cv(N)?;

        let mut b = self.backend.alloc_zeros_cv(N)?;
        self.backend.gemv_c(
            Trans::NoTrans,
            Complex::new(1., 0.),
            &a,
            &x,
            Complex::new(0., 0.),
            &mut b,
        )?;

        self.backend.solve_inplace_h(a, &mut b)?;

        let x = self.backend.to_host_cv(x)?;
        let b = self.backend.to_host_cv(b)?;
        b.iter().zip(x.iter()).for_each(|(b, x)| {
            assert_approx_eq::assert_approx_eq!(b.re, x.re, EPS);
            assert_approx_eq::assert_approx_eq!(b.im, x.im, EPS);
        });
        Ok(())
    }

    fn test_reduce_col(&self) -> Result<(), HoloError> {
        let a = self.make_random_m(N, N)?;

        let mut b = self.backend.alloc_v(N)?;

        self.backend.reduce_col(&a, &mut b)?;

        let a = self.backend.to_host_m(a)?;
        let b = self.backend.to_host_v(b)?;

        (0..N).for_each(|row| {
            let sum = a.row(row).iter().sum::<float>();
            assert_approx_eq::assert_approx_eq!(sum, b[row]);
        });
        Ok(())
    }

    fn test_scaled_to_cv(&self) -> Result<(), HoloError> {
        let a = self.make_random_cv(N)?;
        let b = self.make_random_cv(N)?;
        let mut c = self.backend.alloc_cv(N)?;

        self.backend.scaled_to_cv(&a, &b, &mut c)?;

        let a = self.backend.to_host_cv(a)?;
        let b = self.backend.to_host_cv(b)?;
        let c = self.backend.to_host_cv(c)?;
        c.iter()
            .zip(a.iter())
            .zip(b.iter())
            .for_each(|((&c, &a), &b)| {
                assert_approx_eq::assert_approx_eq!(c, a / a.abs() * b);
            });

        Ok(())
    }

    fn test_scaled_to_assign_cv(&self) -> Result<(), HoloError> {
        let a = self.make_random_cv(N)?;
        let mut b = self.make_random_cv(N)?;
        let bc = self.backend.clone_cv(&b)?;

        self.backend.scaled_to_assign_cv(&a, &mut b)?;

        let a = self.backend.to_host_cv(a)?;
        let b = self.backend.to_host_cv(b)?;
        let bc = self.backend.to_host_cv(bc)?;
        b.iter()
            .zip(a.iter())
            .zip(bc.iter())
            .for_each(|((&b, &a), &bc)| {
                assert_approx_eq::assert_approx_eq!(b, bc / bc.abs() * a);
            });

        Ok(())
    }

    fn test_generate_propagation_matrix(&self) -> Result<(), HoloError> {
        let geometry = generate_geometry::<autd3_driver::geometry::LegacyTransducer>(4);
        let foci = gen_foci(4).map(|(p, _)| p).collect::<Vec<_>>();

        let reference = {
            let mut g = MatrixXc::zeros(
                foci.len(),
                geometry
                    .iter()
                    .map(|dev| dev.num_transducers())
                    .sum::<usize>(),
            );
            let transducers = geometry
                .iter()
                .flat_map(|dev| dev.iter().map(|tr| (dev.idx(), tr)))
                .collect::<Vec<_>>();
            (0..foci.len()).for_each(|i| {
                (0..transducers.len()).for_each(|j| {
                    g[(i, j)] = propagate_tr::<Sphere, autd3_driver::geometry::LegacyTransducer>(
                        transducers[j].1,
                        geometry[transducers[j].0].attenuation,
                        geometry[transducers[j].0].sound_speed,
                        &foci[i],
                    )
                })
            });
            g
        };

        let g = self
            .backend
            .generate_propagation_matrix(&geometry, &foci, &GainFilter::All)?;
        let g = self.backend.to_host_cm(g)?;
        reference.iter().zip(g.iter()).for_each(|(r, g)| {
            assert_approx_eq::assert_approx_eq!(r.re, g.re);
            assert_approx_eq::assert_approx_eq!(r.im, g.im);
        });
        Ok(())
    }

    fn test_generate_propagation_matrix_with_filter(&self) -> Result<(), HoloError> {
        use autd3_driver::geometry::Transducer;
        use std::collections::HashMap;

        let geometry = generate_geometry::<autd3_driver::geometry::LegacyTransducer>(4);
        let foci = gen_foci(4).map(|(p, _)| p).collect::<Vec<_>>();

        let filter = geometry
            .iter()
            .map(|dev| {
                let mut filter = bitvec::prelude::BitVec::new();
                for tr in dev.iter() {
                    filter.push(tr.local_idx() > dev.num_transducers() / 2);
                }
                (dev.idx(), filter)
            })
            .collect::<HashMap<_, _>>();

        let reference = {
            let transducers = geometry
                .iter()
                .flat_map(|dev| {
                    dev.iter().filter_map(|tr| {
                        if filter[&dev.idx()][tr.local_idx()] {
                            Some((dev.idx(), tr))
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>();

            let mut g = MatrixXc::zeros(foci.len(), transducers.len());
            (0..foci.len()).for_each(|i| {
                (0..transducers.len()).for_each(|j| {
                    g[(i, j)] = propagate_tr::<Sphere, autd3_driver::geometry::LegacyTransducer>(
                        transducers[j].1,
                        geometry[transducers[j].0].attenuation,
                        geometry[transducers[j].0].sound_speed,
                        &foci[i],
                    )
                })
            });
            g
        };

        let g = self.backend.generate_propagation_matrix(
            &geometry,
            &foci,
            &GainFilter::Filter(&filter),
        )?;
        let g = self.backend.to_host_cm(g)?;
        assert_eq!(g.nrows(), foci.len());
        assert_eq!(
            g.ncols(),
            geometry
                .iter()
                .map(|dev| dev.num_transducers() / 2)
                .sum::<usize>()
        );
        reference.iter().zip(g.iter()).for_each(|(r, g)| {
            assert_approx_eq::assert_approx_eq!(r.re, g.re);
            assert_approx_eq::assert_approx_eq!(r.im, g.im);
        });
        Ok(())
    }

    fn test_gen_back_prop(&self) -> Result<(), HoloError> {
        let geometry = generate_geometry::<autd3_driver::geometry::LegacyTransducer>(4);
        let foci = gen_foci(4).map(|(p, _)| p).collect::<Vec<_>>();

        let m = geometry
            .iter()
            .map(|dev| dev.num_transducers())
            .sum::<usize>();
        let n = foci.len();

        let g = self
            .backend
            .generate_propagation_matrix(&geometry, &foci, &GainFilter::All)?;
        let amps = self.make_random_cv(n)?;

        let mut b = self.backend.alloc_cm(m, n)?;

        self.backend.gen_back_prop(m, n, &g, &amps, &mut b)?;
        let amps = self.backend.to_host_cv(amps)?;
        let g = self.backend.to_host_cm(g)?;
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

        let b = self.backend.to_host_cm(b)?;
        reference.iter().zip(b.iter()).for_each(|(r, b)| {
            assert_approx_eq::assert_approx_eq!(r.re, b.re);
            assert_approx_eq::assert_approx_eq!(r.im, b.im);
        });
        Ok(())
    }
}
