/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::manual_slice_size_calculation)]

mod cusolver;

use std::{ffi::CStr, fmt::Display, rc::Rc};

use autd3_core::float;
use autd3_gain_holo::{Backend, HoloError, MatrixXc, VectorX, VectorXc};
use cuda_sys::cublas::{
    cublasOperation_t_CUBLAS_OP_C, cublasOperation_t_CUBLAS_OP_N, cublasOperation_t_CUBLAS_OP_T,
};
use rand::{thread_rng, Rng};
use thiserror::Error;

#[cfg(feature = "single_float")]
type CuComplex = cuda_sys::cublas::cuFloatComplex;
#[cfg(not(feature = "single_float"))]
type CuComplex = cuda_sys::cublas::cuDoubleComplex;

#[cfg(feature = "single_float")]
const AUTD_R: cusolver::cudaDataType_t = cusolver::cudaDataType_t::CUDA_R_32F;
#[cfg(not(feature = "single_float"))]
const AUTD_R: cusolver::cudaDataType_t = cusolver::cudaDataType_t::CUDA_R_64F;

#[cfg(feature = "single_float")]
const AUTD_C: cusolver::cudaDataType_t = cusolver::cudaDataType_t::CUDA_C_32F;
#[cfg(not(feature = "single_float"))]
const AUTD_C: cusolver::cudaDataType_t = cusolver::cudaDataType_t::CUDA_C_64F;

#[cfg(feature = "single_float")]
fn make_complex(x: float, y: float) -> CuComplex {
    cuda_sys::cublas::cuFloatComplex {
        x,
        y,
        __bindgen_align: [],
    }
}

#[cfg(not(feature = "single_float"))]
fn make_complex(x: float, y: float) -> CuComplex {
    cuda_sys::cublas::cuDoubleComplex { x, y }
}

#[link(name = "autd3_cuda_kernel", kind = "static")]
extern "C" {
    fn cu_gs_normalize(x: *const CuComplex, n: u32, y: *mut CuComplex);
    fn cu_gspat_normalize(x: *const CuComplex, y: *const CuComplex, n: u32, z: *mut CuComplex);
    fn cu_gspat_normalize2(x: *const CuComplex, y: *const CuComplex, n: u32, z: *mut CuComplex);

    fn cu_get_diagonal(x: *const float, row: u32, col: u32, y: *mut float);
    fn cu_get_diagonal_c(x: *const CuComplex, row: u32, col: u32, y: *mut CuComplex);
    fn cu_set_diagonal(x: *const float, row: u32, col: u32, y: *mut float);
    fn cu_set_diagonal_c(x: *const CuComplex, row: u32, col: u32, y: *mut CuComplex);
    fn cu_reciprocal(x: *const CuComplex, row: u32, col: u32, y: *mut CuComplex);
    fn cu_hadamard_product(
        x: *const CuComplex,
        y: *const CuComplex,
        row: u32,
        col: u32,
        z: *mut CuComplex,
    );

    fn cu_abs(a: *const CuComplex, row: u32, col: u32, b: *mut float);
    fn cu_sqrt(a: *const float, row: u32, col: u32, b: *mut float);
    fn cu_make_complex(re: *const float, im: *const float, row: u32, col: u32, dst: *mut CuComplex);
    fn cu_pow(a: *const float, p: float, row: u32, col: u32, b: *mut float);

    fn cu_conj(a: *const CuComplex, row: u32, col: u32, b: *mut CuComplex);

    fn cu_calc_singular_inv(a: *const float, row: u32, col: u32, alpha: float, b: *mut CuComplex);

    fn cu_exp(a: *const CuComplex, row: u32, col: u32, b: *mut CuComplex);
    fn cu_real(a: *const CuComplex, row: u32, col: u32, b: *mut float);
    fn cu_imag(a: *const CuComplex, row: u32, col: u32, b: *mut float);

    fn cu_reduce_col_buffer_size(m: u32, n: u32) -> u32;
    fn cu_reduce_col(mat: *const float, m: u32, n: u32, result: *mut float, buffer: *mut float);
}

#[derive(Error, Debug)]
pub enum CUDABackendError {
    CuBLASError(cuda_sys::cublas::cublasStatus_t),
    CUDAError(cuda_sys::cudart::cudaError_t),
    CuSOLVERError(cusolver::cusolverStatus_t),
}

impl Display for CUDABackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                CUDABackendError::CuBLASError(err) => write!(f, "cuBLAS Error: {:?}", err),
                CUDABackendError::CuSOLVERError(err) => write!(f, "cuSOLVER Error: {:?}", err),
                CUDABackendError::CUDAError(err) => write!(
                    f,
                    "CUDA Error: {}",
                    CStr::from_ptr(cuda_sys::cudart::cudaGetErrorString(*err))
                        .to_str()
                        .unwrap()
                ),
            }
        }
    }
}

impl From<CUDABackendError> for HoloError {
    fn from(value: CUDABackendError) -> Self {
        HoloError::BackendError(value.to_string())
    }
}

macro_rules! cu_call {
    ($f:expr) => {{
        let res = $f;
        let err = cuda_sys::cudart::cudaGetLastError();
        if err != cuda_sys::cudart::cudaError_t::Success {
            return Err(CUDABackendError::CUDAError(err).into());
        }
        res
    }};
}

macro_rules! cuda_call {
    ($f:expr) => {{
        let err = $f;
        if err != cuda_sys::cudart::cudaError_t::Success {
            return Err(CUDABackendError::CUDAError(err).into());
        }
    }};
}

macro_rules! cublas_call {
    ($f:expr) => {{
        let err = $f;
        if err != cuda_sys::cublas::cublasStatus_t::SUCCESS {
            return Err(CUDABackendError::CuBLASError(err).into());
        }
    }};
}

macro_rules! cusolver_call {
    ($f:expr) => {{
        let err = $f;
        if err != cusolver::cusolverStatus_t::CUSOLVER_STATUS_SUCCESS {
            return Err(CUDABackendError::CuSOLVERError(err));
        }
    }};
}

macro_rules! alloc_uninitialized {
    ($ty:ty, $len:expr) => {{
        let mut v: *mut $ty = std::ptr::null_mut();
        cuda_call!(cuda_sys::cudart::cudaMalloc(
            &mut v as *mut *mut $ty as _,
            std::mem::size_of::<$ty>() * $len,
        ));
        (v, $len)
    }};
    ($ty:ty, $r:expr, $c:expr) => {{
        let mut v: *mut $ty = std::ptr::null_mut();
        cuda_call!(cuda_sys::cudart::cudaMalloc(
            &mut v as *mut *mut $ty as _,
            std::mem::size_of::<$ty>() * $r * $c,
        ));
        (v, $r, $c)
    }};
}

macro_rules! alloc_zeroed {
    ($ty:ty, $len:expr) => {{
        let (v, _) = alloc_uninitialized!($ty, $len);
        cuda_call!(cuda_sys::cudart::cudaMemset(
            v as _,
            0,
            std::mem::size_of::<$ty>() * $len
        ));
        (v, $len)
    }};
    ($ty:ty, $r:expr, $c:expr) => {{
        let (v, _, _) = alloc_uninitialized!($ty, $r, $c);
        cuda_call!(cuda_sys::cudart::cudaMemset(
            v as _,
            0,
            std::mem::size_of::<$ty>() * $r * $c
        ));
        (v, $r, $c)
    }};
}

macro_rules! free {
    ($p:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaFree($p.0 as _))
    }};
}

macro_rules! cpy_host_to_device {
    ($ty:ty, $src:expr, $dst:expr, $len:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            $dst.0 as _,
            $src as _,
            std::mem::size_of::<$ty>() * $len,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyHostToDevice,
        ))
    }};
}

macro_rules! cpy_device_to_device {
    ($ty:ty, $src:expr, $dst:expr, $len:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            $dst.0 as _,
            $src.0 as _,
            std::mem::size_of::<$ty>() * $len,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
        ))
    }};
}

macro_rules! cpy_device_to_host {
    ($ty:ty, $src:expr, $dst:expr, $len:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            $dst as _,
            $src.0 as _,
            std::mem::size_of::<$ty>() * $len,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToHost,
        ))
    }};
}

pub struct CUDABackend {
    handle: cuda_sys::cublas::cublasHandle_t,
    handle_s: cusolver::cusolverDnHandle_t,
}

impl CUDABackend {
    pub fn new() -> Result<Rc<Self>, CUDABackendError> {
        let mut handle: cuda_sys::cublas::cublasHandle_t = std::ptr::null_mut();
        unsafe {
            cublas_call!(cuda_sys::cublas::cublasCreate_v2(&mut handle as _));
        }

        let mut handle_s: cusolver::cusolverDnHandle_t = std::ptr::null_mut();
        unsafe { cusolver_call!(cusolver::cusolverDnCreate(&mut handle_s as _)) }

        Ok(Rc::new(Self { handle, handle_s }))
    }
}

impl CUDABackend {
    unsafe fn scale_vec(
        &self,
        value: float,
        vec: (*mut float, usize),
    ) -> Result<(), CUDABackendError> {
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasSscal_v2(
            self.handle,
            vec.1 as _,
            &value as _,
            vec.0 as _,
            1
        ));
        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasDscal_v2(
            self.handle,
            vec.1 as _,
            &value as _,
            vec.0 as _,
            1
        ));
        Ok(())
    }

    unsafe fn scale_c_vec(
        &self,
        value: CuComplex,
        vec: (*mut CuComplex, usize),
    ) -> Result<(), CUDABackendError> {
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasCscal_v2(
            self.handle,
            vec.1 as _,
            &value as _,
            vec.0 as _,
            1
        ));

        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasZscal_v2(
            self.handle,
            vec.1 as _,
            &value as _,
            vec.0 as _,
            1
        ));
        Ok(())
    }

    unsafe fn max_element(&self, a: (*mut float, usize)) -> Result<float, CUDABackendError> {
        let len = a.1;
        let mut tmp: Vec<float> = vec![0.; len];
        cpy_device_to_host!(float, a, tmp.as_mut_ptr(), len);
        Ok(tmp.into_iter().fold(0., float::max))
    }

    unsafe fn add_mat(
        &self,
        alpha: *const float,
        a: (*mut float, usize, usize),
        b: (*mut float, usize, usize),
    ) -> Result<(), CUDABackendError> {
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasSaxpy_v2(
            self.handle,
            (a.1 * a.2) as _,
            alpha,
            a.0 as _,
            1,
            b.0 as _,
            1
        ));
        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasDaxpy_v2(
            self.handle,
            (a.1 * a.2) as _,
            alpha,
            a.0 as _,
            1,
            b.0 as _,
            1
        ));
        Ok(())
    }

    unsafe fn add_vec(
        &self,
        alpha: *const float,
        a: (*mut float, usize),
        b: (*mut float, usize),
    ) -> Result<(), CUDABackendError> {
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasSaxpy_v2(
            self.handle,
            a.1 as _,
            alpha,
            a.0 as _,
            1,
            b.0 as _,
            1
        ));
        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasDaxpy_v2(
            self.handle,
            a.1 as _,
            alpha,
            a.0 as _,
            1,
            b.0 as _,
            1
        ));
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn mul_mat_mat_c(
        &self,
        transa: cuda_sys::cublas::cublasOperation_t,
        transb: cuda_sys::cublas::cublasOperation_t,
        alpha: *const CuComplex,
        a: (*mut CuComplex, usize, usize),
        b: (*mut CuComplex, usize, usize),
        beta: *const CuComplex,
        c: (*mut CuComplex, usize, usize),
    ) -> Result<(), CUDABackendError> {
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasCgemm_v2(
            self.handle,
            transa,
            transb,
            c.1 as _,
            c.2 as _,
            if transa == cublasOperation_t_CUBLAS_OP_N {
                a.2
            } else {
                a.1
            } as _,
            alpha,
            a.0,
            a.1 as _,
            b.0,
            b.1 as _,
            beta,
            c.0,
            c.1 as _,
        ));

        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasZgemm_v2(
            self.handle,
            transa,
            transb,
            c.1 as _,
            c.2 as _,
            if transa == cublasOperation_t_CUBLAS_OP_N {
                a.2
            } else {
                a.1
            } as _,
            alpha,
            a.0,
            a.1 as _,
            b.0,
            b.1 as _,
            beta,
            c.0,
            c.1 as _,
        ));

        Ok(())
    }

    unsafe fn mul_mat_vec_c(
        &self,
        transa: cuda_sys::cublas::cublasOperation_t,
        alpha: *const CuComplex,
        a: (*mut CuComplex, usize, usize),
        b: (*mut CuComplex, usize),
        beta: *const CuComplex,
        c: (*mut CuComplex, usize),
    ) -> Result<(), CUDABackendError> {
        let m = a.1;
        let n = a.2;
        let lda = m;
        let ap = a.0;
        let bp = b.0;
        let cp = c.0;

        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasCgemv_v2(
            self.handle,
            transa,
            m as _,
            n as _,
            alpha,
            ap,
            lda as _,
            bp,
            1,
            beta,
            cp,
            1,
        ));
        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasZgemv_v2(
            self.handle,
            transa,
            m as _,
            n as _,
            alpha,
            ap,
            lda as _,
            bp,
            1,
            beta,
            cp,
            1,
        ));

        Ok(())
    }

    unsafe fn create_backprop(
        &self,
        amps: (*mut CuComplex, usize),
        g: (*mut CuComplex, usize, usize),
        b: (*mut CuComplex, usize, usize),
    ) -> Result<(), CUDABackendError> {
        let one = make_complex(1., 0.);
        let zero = make_complex(0., 0.);

        let tmp = alloc_zeroed!(CuComplex, g.1, g.1);
        self.mul_mat_mat_c(
            cuda_sys::cublas::cublasOperation_t_CUBLAS_OP_N,
            cuda_sys::cublas::cublasOperation_t_CUBLAS_OP_C,
            &one,
            g,
            g,
            &zero,
            tmp,
        )?;

        let denominator = alloc_uninitialized!(CuComplex, g.1);
        cu_call!(cu_get_diagonal_c(tmp.0, g.1 as _, g.1 as _, denominator.0));
        cu_call!(cu_reciprocal(denominator.0, g.1 as _, 1, denominator.0));
        cu_call!(cu_hadamard_product(
            amps.0,
            denominator.0,
            g.1 as _,
            1,
            denominator.0
        ));
        cu_call!(cu_set_diagonal_c(denominator.0, g.1 as _, g.1 as _, tmp.0));

        self.mul_mat_mat_c(
            cuda_sys::cublas::cublasOperation_t_CUBLAS_OP_C,
            cuda_sys::cublas::cublasOperation_t_CUBLAS_OP_N,
            &one,
            g,
            tmp,
            &zero,
            b,
        )?;

        free!(tmp);
        free!(denominator);

        Ok(())
    }

    unsafe fn max_eigen_vector(
        &self,
        r: (*mut CuComplex, usize, usize),
        max_ev: (*mut CuComplex, usize),
    ) -> Result<(), CUDABackendError> {
        let dw = alloc_uninitialized!(float, r.2);

        let mut workspace_in_bytes_on_device: u64 = 0;
        let mut workspace_in_bytes_on_host: u64 = 0;
        cusolver_call!(cusolver::cusolverDnXsyevd_bufferSize(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cusolverEigMode_t::CUSOLVER_EIG_MODE_VECTOR,
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            r.2 as _,
            AUTD_C,
            r.0 as _,
            r.2 as _,
            AUTD_R,
            dw.0 as _,
            AUTD_C,
            &mut workspace_in_bytes_on_device as _,
            &mut workspace_in_bytes_on_host as _,
        ));

        let workspace_buffer_on_device =
            alloc_uninitialized!(u8, workspace_in_bytes_on_device as usize);
        let mut workspace_buffer_on_host_v = vec![0u8; workspace_in_bytes_on_host as usize];
        let workspace_buffer_on_host = if workspace_in_bytes_on_host > 0 {
            workspace_buffer_on_host_v.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };

        let info = alloc_uninitialized!(i32, 1);

        cusolver_call!(cusolver::cusolverDnXsyevd(
            self.handle_s,
            std::ptr::null_mut::<cusolver::cusolverDnParams>(),
            cusolver::cusolverEigMode_t::CUSOLVER_EIG_MODE_VECTOR,
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            r.2 as _,
            AUTD_C,
            r.0 as _,
            r.2 as _,
            AUTD_R,
            dw.0 as _,
            AUTD_C,
            workspace_buffer_on_device.0 as _,
            workspace_in_bytes_on_device,
            workspace_buffer_on_host as _,
            workspace_in_bytes_on_host,
            info.0 as _,
        ));

        free!(dw);
        free!(info);
        free!(workspace_buffer_on_device);

        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            max_ev.0 as _,
            r.0.add(r.2 * (r.2 - 1)) as _,
            std::mem::size_of::<CuComplex>() * r.2,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
        ));

        Ok(())
    }

    unsafe fn solveh(
        &self,
        a: (*mut CuComplex, usize, usize),
        b: (*mut CuComplex, usize),
    ) -> Result<(), CUDABackendError> {
        let n = a.2;
        let lda = a.1;
        let ldb = b.1;

        let ap = a.0;
        let bp = b.0;

        let mut workspace_in_bytes_on_device: u64 = 0;
        let mut workspace_in_bytes_on_host: u64 = 0;
        cusolver_call!(cusolver::cusolverDnXpotrf_bufferSize(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            n as _,
            AUTD_C,
            ap as _,
            lda as _,
            AUTD_C,
            &mut workspace_in_bytes_on_device as _,
            &mut workspace_in_bytes_on_host as _,
        ));

        let workspace_buffer_on_device =
            alloc_uninitialized!(u8, workspace_in_bytes_on_device as usize);
        let mut workspace_buffer_on_host_v = vec![0u8; workspace_in_bytes_on_host as usize];
        let workspace_buffer_on_host = if workspace_in_bytes_on_host > 0 {
            workspace_buffer_on_host_v.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };

        let info = alloc_uninitialized!(i32, 1);

        cusolver_call!(cusolver::cusolverDnXpotrf(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            n as _,
            AUTD_C,
            ap as _,
            lda as _,
            AUTD_C,
            workspace_buffer_on_device.0 as _,
            workspace_in_bytes_on_device,
            workspace_buffer_on_host as _,
            workspace_in_bytes_on_host,
            info.0 as _,
        ));
        cusolver_call!(cusolver::cusolverDnXpotrs(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            n as _,
            1,
            AUTD_C,
            ap as _,
            lda as _,
            AUTD_C,
            bp as _,
            ldb as _,
            info.0 as _,
        ));

        free!(info);
        free!(workspace_buffer_on_device);

        Ok(())
    }

    unsafe fn solvet(
        &self,
        a: (*mut float, usize, usize),
        b: (*mut float, usize),
    ) -> Result<(), CUDABackendError> {
        let n = a.2;
        let lda = a.1;
        let ldb = b.1;

        let ap = a.0;
        let bp = b.0;

        let mut workspace_in_bytes_on_device: u64 = 0;
        let mut workspace_in_bytes_on_host: u64 = 0;
        cusolver_call!(cusolver::cusolverDnXpotrf_bufferSize(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            n as _,
            AUTD_R,
            ap as _,
            lda as _,
            AUTD_R,
            &mut workspace_in_bytes_on_device as _,
            &mut workspace_in_bytes_on_host as _,
        ));

        let workspace_buffer_on_device =
            alloc_uninitialized!(u8, workspace_in_bytes_on_device as usize);
        let mut workspace_buffer_on_host_v = vec![0u8; workspace_in_bytes_on_host as usize];
        let workspace_buffer_on_host = if workspace_in_bytes_on_host > 0 {
            workspace_buffer_on_host_v.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };

        let info = alloc_uninitialized!(i32, 1);

        cusolver_call!(cusolver::cusolverDnXpotrf(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            n as _,
            AUTD_R,
            ap as _,
            lda as _,
            AUTD_R,
            workspace_buffer_on_device.0 as _,
            workspace_in_bytes_on_device,
            workspace_buffer_on_host as _,
            workspace_in_bytes_on_host,
            info.0 as _,
        ));
        cusolver_call!(cusolver::cusolverDnXpotrs(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
            n as _,
            1,
            AUTD_R,
            ap as _,
            lda as _,
            AUTD_R,
            bp as _,
            ldb as _,
            info.0 as _,
        ));

        free!(info);
        free!(workspace_buffer_on_device);

        Ok(())
    }

    unsafe fn concat_row(
        &self,
        a: (*mut CuComplex, usize, usize),
        b: (*mut CuComplex, usize, usize),
        c: (*mut CuComplex, usize, usize),
    ) -> Result<(), CUDABackendError> {
        for i in 0..a.2 {
            cuda_call!(cuda_sys::cudart::cudaMemcpy(
                c.0.add(i * (a.1 + b.1)) as _,
                a.0.add(i * a.1) as _,
                std::mem::size_of::<CuComplex>() * a.1,
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
            ));
            cuda_call!(cuda_sys::cudart::cudaMemcpy(
                c.0.add(i * (a.1 + b.1) + a.1) as _,
                b.0.add(i * b.1) as _,
                std::mem::size_of::<CuComplex>() * b.1,
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
            ));
        }

        Ok(())
    }

    unsafe fn concat_col(
        &self,
        a: (*mut CuComplex, usize, usize),
        b: (*mut CuComplex, usize, usize),
        c: (*mut CuComplex, usize, usize),
    ) -> Result<(), CUDABackendError> {
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            c.0 as _,
            a.0 as _,
            a.1 * a.2 * std::mem::size_of::<CuComplex>(),
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice
        ));
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            c.0.add(a.1 * a.2) as _,
            b.0 as _,
            b.1 * b.2 * std::mem::size_of::<CuComplex>(),
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice
        ));
        Ok(())
    }

    unsafe fn concat_vec(
        &self,
        a: (*mut CuComplex, usize),
        b: (*mut CuComplex, usize),
        c: (*mut CuComplex, usize),
    ) -> Result<(), CUDABackendError> {
        cpy_device_to_device!(CuComplex, a, c, a.1);
        cu_call!(cuda_sys::cudart::cudaMemcpy(
            c.0.add(a.1) as _,
            b.0 as _,
            std::mem::size_of::<CuComplex>() * b.1,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
        ));
        Ok(())
    }

    unsafe fn set_col(
        &self,
        src: (*mut CuComplex, usize),
        i: usize,
        begin: usize,
        end: usize,
        dst: (*mut CuComplex, usize, usize),
    ) -> Result<(), CUDABackendError> {
        let row = dst.1;
        let src_p = src.0;
        let dst_p = dst.0;
        cu_call!(cuda_sys::cudart::cudaMemcpy(
            dst_p.add(i * row + begin) as _,
            src_p.add(begin) as _,
            std::mem::size_of::<CuComplex>() * (end - begin),
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
        ));
        Ok(())
    }

    unsafe fn set_row(
        &self,
        src: (*mut CuComplex, usize),
        i: usize,
        begin: usize,
        end: usize,
        dst: (*mut CuComplex, usize, usize),
    ) -> Result<(), CUDABackendError> {
        let row = dst.1;
        let src_p = src.0;
        let dst_p = dst.0;
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasCcopy_v2(
            self.handle,
            (end - begin) as _,
            src_p.add(begin),
            1,
            dst_p.add(i + begin * row),
            row as _,
        ));
        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasZcopy_v2(
            self.handle,
            (end - begin) as _,
            src_p.add(begin ),
            1,
            dst_p.add(i + begin * row),
            row as _,
        ));
        Ok(())
    }

    unsafe fn dotc(
        &self,
        a: (*mut CuComplex, usize),
        b: (*mut CuComplex, usize),
    ) -> Result<CuComplex, CUDABackendError> {
        let mut d = make_complex(0., 0.);
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasCdotc_v2(
            self.handle,
            a.1 as _,
            a.0 as _,
            1,
            b.0 as _,
            1,
            &mut d as _,
        ));
        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasZdotc_v2(
            self.handle,
            a.1 as _,
            a.0 as _,
            1,
            b.0 as _,
            1,
            &mut d as _,
        ));
        Ok(d)
    }

    unsafe fn dot(
        &self,
        a: (*mut float, usize),
        b: (*mut float, usize),
    ) -> Result<float, CUDABackendError> {
        let mut d: float = 0.;
        #[cfg(feature = "single_float")]
        cublas_call!(cuda_sys::cublas::cublasSdot_v2(
            self.handle,
            a.1 as _,
            a.0 as _,
            1,
            b.0 as _,
            1,
            &mut d as _,
        ));
        #[cfg(not(feature = "single_float"))]
        cublas_call!(cuda_sys::cublas::cublasDdot_v2(
            self.handle,
            a.1 as _,
            a.0 as _,
            1,
            b.0 as _,
            1,
            &mut d as _,
        ));
        Ok(d)
    }

    unsafe fn set(
        &self,
        i: usize,
        value: CuComplex,
        dst: (*mut CuComplex, usize),
    ) -> Result<(), CUDABackendError> {
        cu_call!(cuda_sys::cudart::cudaMemcpy(
            dst.0.add(i) as _,
            &value as *const CuComplex as _,
            std::mem::size_of::<CuComplex>(),
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyHostToDevice,
        ));
        Ok(())
    }

    unsafe fn get_col(
        &self,
        src: (*mut CuComplex, usize, usize),
        i: usize,
        dst: (*mut CuComplex, usize),
    ) -> Result<(), CUDABackendError> {
        let row = src.1;
        cu_call!(cuda_sys::cudart::cudaMemcpy(
            dst.0 as _,
            src.0.add(i * row) as _,
            std::mem::size_of::<CuComplex>() * row,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
        ));
        Ok(())
    }

    unsafe fn pseudo_inverse(
        &self,
        src: (*mut CuComplex, usize, usize),
        alpha: float,
        dst: (*mut CuComplex, usize, usize),
    ) -> Result<(), CUDABackendError> {
        let one = make_complex(1., 0.);
        let zero = make_complex(0., 0.);

        let m = src.1;
        let n = src.2;

        let u = alloc_uninitialized!(CuComplex, m, m);
        let s = alloc_uninitialized!(CuComplex, n, m);
        let vt = alloc_uninitialized!(CuComplex, n, n);

        let lda = m;
        let ldu = m;
        let ldv = n;

        let s_size = m.min(n);
        let ds = alloc_uninitialized!(float, s_size);

        let mut workspace_in_bytes_on_device: u64 = 0;
        let mut workspace_in_bytes_on_host: u64 = 0;
        cusolver_call!(cusolver::cusolverDnXgesvdp_bufferSize(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cusolverEigMode_t::CUSOLVER_EIG_MODE_VECTOR,
            0,
            m as _,
            n as _,
            AUTD_C,
            src.0 as _,
            lda as _,
            AUTD_R,
            ds.0 as _,
            AUTD_C,
            u.0 as _,
            ldu as _,
            AUTD_C,
            vt.0 as _,
            ldv as _,
            AUTD_C,
            &mut workspace_in_bytes_on_device as _,
            &mut workspace_in_bytes_on_host as _,
        ));

        let workspace_buffer_on_device =
            alloc_uninitialized!(u8, workspace_in_bytes_on_device as usize);
        let mut workspace_buffer_on_host_v = vec![0u8; workspace_in_bytes_on_host as usize];
        let workspace_buffer_on_host = if workspace_in_bytes_on_host > 0 {
            workspace_buffer_on_host_v.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };

        let info = alloc_uninitialized!(i32, 1);

        let mut h_err_sigma = 0.;
        cusolver_call!(cusolver::cusolverDnXgesvdp(
            self.handle_s,
            std::ptr::null_mut(),
            cusolver::cusolverEigMode_t::CUSOLVER_EIG_MODE_VECTOR,
            0,
            m as _,
            n as _,
            AUTD_C,
            src.0 as _,
            lda as _,
            AUTD_R,
            ds.0 as _,
            AUTD_C,
            u.0 as _,
            ldu as _,
            AUTD_C,
            vt.0 as _,
            ldv as _,
            AUTD_C,
            workspace_buffer_on_device.0 as _,
            workspace_in_bytes_on_device,
            workspace_buffer_on_host as _,
            workspace_in_bytes_on_host,
            info.0 as _,
            &mut h_err_sigma as _,
        ));

        cu_call!(cu_calc_singular_inv(ds.0, n as _, m as _, alpha, s.0));

        let buf = alloc_zeroed!(CuComplex, n, m);

        self.mul_mat_mat_c(
            cublasOperation_t_CUBLAS_OP_N,
            cublasOperation_t_CUBLAS_OP_C,
            &one,
            s,
            u,
            &zero,
            buf,
        )?;
        self.mul_mat_mat_c(
            cublasOperation_t_CUBLAS_OP_N,
            cublasOperation_t_CUBLAS_OP_N,
            &one,
            vt,
            buf,
            &zero,
            dst,
        )?;

        free!(u);
        free!(s);
        free!(vt);
        free!(buf);
        free!(ds);
        free!(info);
        free!(workspace_buffer_on_device);

        Ok(())
    }

    unsafe fn reduce_col(
        &self,
        a: (*mut float, usize, usize),
        b: (*mut float, usize),
    ) -> Result<(), CUDABackendError> {
        let m = a.1;
        let n = a.2;
        let buf_size = cu_call!(cu_reduce_col_buffer_size(m as _, n as _)) as usize;
        let buffer = alloc_uninitialized!(float, buf_size);
        cu_call!(cu_reduce_col(
            a.0 as _,
            m as _,
            n as _,
            b.0 as _,
            buffer.0 as _
        ));
        free!(buffer);

        Ok(())
    }
}

impl Backend for CUDABackend {
    fn gs(
        &self,
        repeat: usize,
        amps: &[float],
        g: MatrixXc,
    ) -> Result<autd3_gain_holo::VectorXc, HoloError> {
        let m = g.nrows();
        let n = g.ncols();

        let one = make_complex(1., 0.);
        let zero = make_complex(0., 0.);

        unsafe {
            let gp = alloc_uninitialized!(CuComplex, m, n);
            cpy_host_to_device!(CuComplex, g.as_ptr(), gp, m * n);

            let q0 = alloc_uninitialized!(CuComplex, n);
            let tmp = vec![one; n];
            cpy_host_to_device!(CuComplex, tmp.as_ptr(), q0, n);

            let q = alloc_uninitialized!(CuComplex, n);
            cpy_device_to_device!(CuComplex, q0, q, n);

            let p = alloc_zeroed!(CuComplex, m);

            let amps_ = Vec::from_iter(amps.iter().map(|&x| make_complex(x, 0.)));
            let amps = alloc_uninitialized!(CuComplex, m);
            cpy_host_to_device!(CuComplex, amps_.as_ptr(), amps, m);

            for _ in 0..repeat {
                self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_N, &one, gp, q, &zero, p)?;
                cu_call!(cu_gs_normalize(amps.0, m as _, p.0));
                self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_C, &one, gp, p, &zero, q)?;
                cu_call!(cu_gs_normalize(q0.0, n as _, q.0));
            }

            let mut res = VectorXc::zeros(n);
            cpy_device_to_host!(CuComplex, q, res.as_mut_ptr(), n);

            free!(gp);
            free!(q0);
            free!(q);
            free!(p);
            free!(amps);

            Ok(res)
        }
    }

    fn gspat(&self, repeat: usize, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError> {
        let m = g.nrows();
        let n = g.ncols();

        let one = make_complex(1., 0.);
        let zero = make_complex(0., 0.);

        unsafe {
            let gp = alloc_uninitialized!(CuComplex, m, n);
            cpy_host_to_device!(CuComplex, g.as_ptr(), gp, m * n);

            let amps_ = Vec::from_iter(amps.iter().map(|&x| make_complex(x, 0.)));
            let amps = alloc_uninitialized!(CuComplex, m);
            cpy_host_to_device!(CuComplex, amps_.as_ptr(), amps, m);

            let bp = alloc_zeroed!(CuComplex, n, m);
            self.create_backprop(amps, gp, bp)?;

            let r = alloc_zeroed!(CuComplex, m, m);
            self.mul_mat_mat_c(
                cublasOperation_t_CUBLAS_OP_N,
                cublasOperation_t_CUBLAS_OP_N,
                &one,
                gp,
                bp,
                &zero,
                r,
            )?;

            let p = alloc_uninitialized!(CuComplex, m);
            cpy_device_to_device!(CuComplex, amps, p, m);

            let gamma = alloc_zeroed!(CuComplex, m);
            self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_N, &one, r, p, &zero, gamma)?;
            for _ in 0..repeat {
                cu_call!(cu_gspat_normalize(amps.0, gamma.0, m as _, p.0));
                self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_C, &one, r, p, &zero, gamma)?;
            }
            cu_call!(cu_gspat_normalize2(amps.0, gamma.0, m as _, p.0));

            let q = alloc_zeroed!(CuComplex, n);
            self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_N, &one, bp, p, &zero, q)?;

            let mut res = VectorXc::zeros(n);
            cpy_device_to_host!(CuComplex, q, res.as_mut_ptr(), n);

            free!(amps);
            free!(gp);
            free!(bp);
            free!(r);
            free!(p);
            free!(gamma);
            free!(q);

            Ok(res)
        }
    }

    fn naive(&self, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError> {
        let m = g.nrows();
        let n = g.ncols();
        let one = make_complex(1., 0.);
        let zero = make_complex(0., 0.);

        unsafe {
            let gp = alloc_uninitialized!(CuComplex, m, n);
            cpy_host_to_device!(CuComplex, g.as_ptr(), gp, m * n);
            let amps_ = Vec::from_iter(amps.iter().map(|&x| make_complex(x, 0.)));
            let p = alloc_uninitialized!(CuComplex, m);
            cpy_host_to_device!(CuComplex, amps_.as_ptr(), p, m);
            let q = alloc_zeroed!(CuComplex, n);
            self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_C, &one, gp, p, &zero, q)?;
            let mut res = VectorXc::zeros(n);
            cpy_device_to_host!(CuComplex, q, res.as_mut_ptr(), n);

            free!(gp);
            free!(p);
            free!(q);

            Ok(res)
        }
    }

    fn evp(&self, gamma: float, amps: &[float], g: MatrixXc) -> Result<VectorXc, HoloError> {
        let m = g.nrows();
        let n = g.ncols();

        let one = make_complex(1., 0.);
        let zero = make_complex(0., 0.);

        unsafe {
            let amps_ = Vec::from_iter(amps.iter().map(|&x| make_complex(x, 0.)));
            let amps = alloc_uninitialized!(CuComplex, m);
            cpy_host_to_device!(CuComplex, amps_.as_ptr(), amps, m);

            let gp = alloc_uninitialized!(CuComplex, m, n);
            cpy_host_to_device!(CuComplex, g.as_ptr(), gp, m * n);

            let x = alloc_zeroed!(CuComplex, n, m);
            self.create_backprop(amps, gp, x)?;

            let r = alloc_zeroed!(CuComplex, m, m);
            self.mul_mat_mat_c(
                cublasOperation_t_CUBLAS_OP_N,
                cublasOperation_t_CUBLAS_OP_N,
                &one,
                gp,
                x,
                &zero,
                r,
            )?;
            let max_ev = alloc_uninitialized!(CuComplex, m);
            self.max_eigen_vector(r, max_ev)?;

            let sigma = {
                let sigma_tmp = alloc_zeroed!(CuComplex, n);
                self.mul_mat_vec_c(
                    cublasOperation_t_CUBLAS_OP_T,
                    &one,
                    gp,
                    amps,
                    &zero,
                    sigma_tmp,
                )?;

                let sigma_tmp_real = alloc_uninitialized!(float, n);
                cu_call!(cu_abs(sigma_tmp.0, sigma_tmp.1 as _, 1, sigma_tmp_real.0));
                self.scale_vec(1. / m as float, sigma_tmp_real)?;
                cu_call!(cu_sqrt(
                    sigma_tmp_real.0,
                    sigma_tmp_real.1 as _,
                    1,
                    sigma_tmp_real.0
                ));
                cu_call!(cu_pow(
                    sigma_tmp_real.0,
                    gamma,
                    sigma_tmp_real.1 as _,
                    1,
                    sigma_tmp_real.0,
                ));

                let zero = alloc_zeroed!(float, n);
                cu_call!(cu_make_complex(
                    sigma_tmp_real.0,
                    zero.0,
                    sigma_tmp_real.1 as _,
                    1,
                    sigma_tmp.0,
                ));

                let sigma = alloc_uninitialized!(CuComplex, n, n);
                cu_set_diagonal_c(sigma_tmp.0, n as _, n as _, sigma.0);

                free!(sigma_tmp);
                free!(sigma_tmp_real);
                free!(zero);

                sigma
            };

            let gr = alloc_uninitialized!(CuComplex, m + n, n);
            self.concat_row(gp, sigma, gr)?;

            cu_call!(cu_gs_normalize(amps.0, max_ev.1 as _, max_ev.0));

            let zero_ = alloc_zeroed!(CuComplex, n);

            let f = alloc_uninitialized!(CuComplex, m + n);
            self.concat_vec(max_ev, zero_, f)?;

            let gtg = alloc_zeroed!(CuComplex, n, n);
            self.mul_mat_mat_c(
                cublasOperation_t_CUBLAS_OP_C,
                cublasOperation_t_CUBLAS_OP_N,
                &one,
                gr,
                gr,
                &zero,
                gtg,
            )?;

            let gtf = alloc_zeroed!(CuComplex, n);
            self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_C, &one, gr, f, &zero, gtf)?;

            self.solveh(gtg, gtf)?;

            let mut res = VectorXc::zeros(n);
            cpy_device_to_host!(CuComplex, gtf, res.as_mut_ptr(), n);

            free!(amps);
            free!(gp);
            free!(x);
            free!(r);
            free!(max_ev);
            free!(sigma);
            free!(gr);
            free!(zero_);
            free!(f);
            free!(gtg);
            free!(gtf);

            Ok(res)
        }
    }

    fn sdp(
        &self,
        alpha: float,
        repeat: usize,
        lambda: float,
        amps: &[float],
        g: MatrixXc,
    ) -> Result<VectorXc, HoloError> {
        let m = g.nrows();
        let n = g.ncols();

        let one = make_complex(1., 0.);
        let m_one = make_complex(-1.0, 0.0);
        let zero = make_complex(0., 0.);

        unsafe {
            let amps_ = Vec::from_iter(amps.iter().map(|&x| make_complex(x, 0.)));
            let amps = alloc_uninitialized!(CuComplex, m);
            cpy_host_to_device!(CuComplex, amps_.as_ptr(), amps, m);

            let b = alloc_uninitialized!(CuComplex, m, n);
            cpy_host_to_device!(CuComplex, g.as_ptr(), b, m * n);

            let p = alloc_uninitialized!(CuComplex, m, m);
            cu_call!(cu_set_diagonal_c(amps.0, m as _, m as _, p.0));

            let b_tmp = alloc_uninitialized!(CuComplex, m, n);
            cpy_device_to_device!(CuComplex, b, b_tmp, m * n);

            let pseudo_inv_b = alloc_uninitialized!(CuComplex, n, m);
            self.pseudo_inverse(b_tmp, alpha, pseudo_inv_b)?;

            let mm = alloc_uninitialized!(CuComplex, m, m);
            let ones_ = vec![one; m];
            let ones = alloc_uninitialized!(CuComplex, m);
            cpy_host_to_device!(CuComplex, ones_.as_ptr(), ones, m);
            cu_call!(cu_set_diagonal_c(ones.0, m as _, m as _, mm.0));

            self.mul_mat_mat_c(
                cublasOperation_t_CUBLAS_OP_N,
                cublasOperation_t_CUBLAS_OP_N,
                &m_one,
                b,
                pseudo_inv_b,
                &one,
                mm,
            )?;

            let tmp = alloc_zeroed!(CuComplex, m, m);
            self.mul_mat_mat_c(
                cublasOperation_t_CUBLAS_OP_N,
                cublasOperation_t_CUBLAS_OP_N,
                &one,
                p,
                mm,
                &zero,
                tmp,
            )?;
            self.mul_mat_mat_c(
                cublasOperation_t_CUBLAS_OP_N,
                cublasOperation_t_CUBLAS_OP_N,
                &one,
                tmp,
                p,
                &zero,
                mm,
            )?;

            let x_mat = alloc_uninitialized!(CuComplex, m, m);
            cu_call!(cu_set_diagonal_c(ones.0, m as _, m as _, x_mat.0));

            let mut rng = thread_rng();

            let zeros = alloc_zeroed!(CuComplex, m);

            let x = alloc_zeroed!(CuComplex, m);

            let x_conj = alloc_uninitialized!(CuComplex, m);
            let mmc = alloc_uninitialized!(CuComplex, m);
            for _ in 0..repeat {
                let ii = (m as float * rng.gen_range(0.0..1.0)).floor() as usize;

                self.get_col(mm, ii, mmc)?;
                self.set(ii, zero, mmc)?;
                self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_N, &one, x_mat, mmc, &zero, x)?;

                let gamma = self.dotc(x, mmc)?;
                if gamma.x > 0. {
                    let s = make_complex((lambda / gamma.x).sqrt(), 0.);
                    self.scale_c_vec(s, x)?;
                    cu_call!(cu_conj(x.0, x.1 as _, 1, x_conj.0));
                    self.set_row(x_conj, ii, 0, ii, x_mat)?;
                    self.set_row(x_conj, ii, ii + 1, m, x_mat)?;
                    self.set_col(x, ii, 0, ii, x_mat)?;
                    self.set_col(x, ii, ii + 1, m, x_mat)?;
                } else {
                    self.set_row(zeros, ii, 0, ii, x_mat)?;
                    self.set_row(zeros, ii, ii + 1, m, x_mat)?;
                    self.set_col(zeros, ii, 0, ii, x_mat)?;
                    self.set_col(zeros, ii, ii + 1, m, x_mat)?;
                }
            }

            let u = alloc_uninitialized!(CuComplex, m);
            self.max_eigen_vector(x_mat, u)?;

            let ut = alloc_zeroed!(CuComplex, m);
            self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_N, &one, p, u, &zero, ut)?;

            let q = alloc_zeroed!(CuComplex, n);
            self.mul_mat_vec_c(
                cublasOperation_t_CUBLAS_OP_N,
                &one,
                pseudo_inv_b,
                ut,
                &zero,
                q,
            )?;

            let mut res = VectorXc::zeros(n);
            cpy_device_to_host!(CuComplex, q, res.as_mut_ptr(), n);

            free!(amps);
            free!(b);
            free!(p);
            free!(b_tmp);
            free!(pseudo_inv_b);
            free!(mm);
            free!(ones);
            free!(tmp);
            free!(x_mat);
            free!(zeros);
            free!(x);
            free!(x_conj);
            free!(mmc);
            free!(u);
            free!(ut);
            free!(q);

            Ok(res)
        }
    }

    fn lm(
        &self,
        eps1: float,
        eps2: float,
        tau: float,
        kmax: usize,
        initial: &[float],
        amps: &[float],
        g: MatrixXc,
    ) -> Result<autd3_gain_holo::VectorX, HoloError> {
        let m = g.nrows();
        let n = g.ncols();
        let n_param = n + m;

        let one = make_complex(1., 0.);
        let zero = make_complex(0., 0.);

        unsafe {
            let set_t = |zero: (*mut float, usize),
                         x: (*mut float, usize),
                         t: (*mut CuComplex, usize)|
             -> Result<(), CUDABackendError> {
                let len = x.1;
                cu_call!(cu_make_complex(
                    zero.0 as _,
                    x.0 as _,
                    len as _,
                    len as _,
                    t.0 as _
                ));
                self.scale_c_vec(make_complex(-1., 0.), t)?;
                cu_call!(cu_exp(t.0, len as _, 1, t.0));
                Ok(())
            };

            let calc_jtj_jtf = |t: (*mut CuComplex, usize),
                                tth: (*mut CuComplex, usize, usize),
                                bhb: (*mut CuComplex, usize, usize),
                                bhb_tth: (*mut CuComplex, usize, usize),
                                bhb_tth_i: (*mut float, usize, usize),
                                jtj: (*mut float, usize, usize),
                                jtf: (*mut float, usize)|
             -> Result<(), CUDABackendError> {
                self.mul_mat_mat_c(
                    cublasOperation_t_CUBLAS_OP_N,
                    cublasOperation_t_CUBLAS_OP_C,
                    &one,
                    (t.0, t.1, 1),
                    (t.0, t.1, 1),
                    &zero,
                    tth,
                )?;
                cu_call!(cu_hadamard_product(
                    bhb.0 as _,
                    tth.0 as _,
                    n_param as _,
                    n_param as _,
                    bhb_tth.0
                ));
                cu_call!(cu_real(
                    bhb_tth.0 as _,
                    n_param as _,
                    n_param as _,
                    jtj.0 as _
                ));
                cu_call!(cu_imag(bhb_tth.0, n_param as _, n_param as _, bhb_tth_i.0));
                self.reduce_col(bhb_tth_i, jtf)?;
                Ok(())
            };

            let cal_fx = |zeros: (*mut float, usize),
                          x: (*mut float, usize),
                          bhb: (*mut CuComplex, usize, usize),
                          tmp: (*mut CuComplex, usize),
                          t: (*mut CuComplex, usize)|
             -> Result<float, CUDABackendError> {
                cu_call!(cu_make_complex(zeros.0, x.0, n_param as _, 1, t.0));
                cu_call!(cu_exp(t.0, n_param as _, 1, t.0));
                self.mul_mat_vec_c(cublasOperation_t_CUBLAS_OP_N, &one, bhb, t, &zero, tmp)?;
                Ok(self.dotc(t, tmp)?.x)
            };

            let gp = alloc_uninitialized!(CuComplex, m, n);
            cpy_host_to_device!(CuComplex, g.as_ptr(), gp, m * n);

            let bhb = alloc_uninitialized!(CuComplex, n_param, n_param);
            {
                let mamps_ = Vec::from_iter(amps.iter().map(|&x| make_complex(-x, 0.)));
                let mamps = alloc_uninitialized!(CuComplex, m);
                cpy_host_to_device!(CuComplex, mamps_.as_ptr(), mamps, m);

                let p = alloc_uninitialized!(CuComplex, m, m);
                cu_call!(cu_set_diagonal_c(mamps.0, m as _, m as _, p.0));

                let b = alloc_uninitialized!(CuComplex, m, n + m);
                self.concat_col(gp, p, b)?;

                self.mul_mat_mat_c(
                    cublasOperation_t_CUBLAS_OP_C,
                    cublasOperation_t_CUBLAS_OP_N,
                    &one,
                    b,
                    b,
                    &zero,
                    bhb,
                )?;

                free!(mamps);
                free!(p);
                free!(b);
            }

            let x = alloc_uninitialized!(float, n_param);
            cpy_host_to_device!(float, initial.as_ptr(), x, initial.len());

            let mut nu = 2.;

            let zeros = alloc_zeroed!(float, n_param);

            let t = alloc_uninitialized!(CuComplex, n_param);
            set_t(zeros, x, t)?;

            let tth = alloc_uninitialized!(CuComplex, n_param, n_param);
            let bhb_tth = alloc_uninitialized!(CuComplex, n_param, n_param);
            let bhb_tth_i = alloc_uninitialized!(float, n_param, n_param);
            let a = alloc_uninitialized!(float, n_param, n_param);
            let g = alloc_uninitialized!(float, n_param);
            calc_jtj_jtf(t, tth, bhb, bhb_tth, bhb_tth_i, a, g)?;

            let a_diag = alloc_uninitialized!(float, n_param);
            cu_call!(cu_get_diagonal(
                a.0 as _,
                n_param as _,
                n_param as _,
                a_diag.0
            ));
            let a_max = self.max_element(a_diag)?;

            let mut mu = tau * a_max;

            let tmp = alloc_uninitialized!(CuComplex, n_param);
            let t_ = alloc_uninitialized!(CuComplex, n_param);
            let mut fx = cal_fx(zeros, x, bhb, tmp, t_)?;

            let identity = alloc_uninitialized!(float, n_param, n_param);
            {
                let ones_: Vec<float> = vec![1.; n_param];
                let ones = alloc_uninitialized!(float, n_param);
                cpy_host_to_device!(float, ones_.as_ptr(), ones, n_param);
                cu_call!(cu_set_diagonal(
                    ones.0 as _,
                    n_param as _,
                    n_param as _,
                    identity.0
                ));
                free!(ones);
            }

            let tmp_vec = alloc_uninitialized!(float, n_param);
            let h_lm = alloc_uninitialized!(float, n_param);
            let x_new = alloc_uninitialized!(float, n_param);
            let tmp_mat = alloc_uninitialized!(float, n_param, n_param);

            for _ in 0..kmax {
                if self.max_element(g)? <= eps1 {
                    break;
                }

                cpy_device_to_device!(float, a, tmp_mat, n_param * n_param);

                self.add_mat(&mu, identity, tmp_mat)?;

                cpy_device_to_device!(float, g, h_lm, n_param);

                self.solvet(tmp_mat, h_lm)?;

                if self.dot(h_lm, h_lm)? <= eps2 * (self.dot(x, x)? + eps2) {
                    break;
                }

                cpy_device_to_device!(float, x, x_new, n_param);

                let m1 = -1.;
                self.add_vec(&m1, h_lm, x_new)?;

                let fx_new = cal_fx(zeros, x_new, bhb, tmp, t_)?;

                cpy_device_to_device!(float, g, tmp_vec, n_param);

                self.add_vec(&mu, h_lm, tmp_vec)?;

                let l0_lhlm = self.dot(h_lm, tmp_vec)? / 2.;

                let rho = (fx - fx_new) / l0_lhlm;
                fx = fx_new;

                if rho > 0. {
                    cpy_device_to_device!(float, x_new, x, n_param);

                    set_t(zeros, x, t)?;
                    calc_jtj_jtf(t, tth, bhb, bhb_tth, bhb_tth_i, a, g)?;

                    mu *= float::max(1. / 3., (1. - (2. * rho - 1.)).powf(3.));
                    nu = 2.;
                } else {
                    mu *= nu;
                    nu *= 2.;
                }
            }

            let mut res = VectorX::zeros(n);
            cpy_device_to_host!(float, x, res.as_mut_ptr(), n);

            free!(gp);
            free!(bhb);
            free!(x);
            free!(zeros);
            free!(t);
            free!(tth);
            free!(bhb_tth);
            free!(bhb_tth_i);
            free!(a);
            free!(g);
            free!(a_diag);
            free!(tmp);
            free!(t_);
            free!(identity);
            free!(tmp_vec);
            free!(h_lm);
            free!(x_new);
            free!(tmp_mat);

            Ok(res)
        }
    }
}
