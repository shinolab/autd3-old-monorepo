/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#![allow(unknown_lints)]
#![allow(clippy::manual_slice_size_calculation)]

mod cusolver;

use std::{ffi::CStr, fmt::Display, rc::Rc};

use autd3_core::float;
use autd3_gain_holo::{HoloError, LinAlgBackend, MatrixX, MatrixXc, VectorX, VectorXc};
use cuda_sys::cublas::{
    cublasOperation_t_CUBLAS_OP_C, cublasOperation_t_CUBLAS_OP_N, cublasOperation_t_CUBLAS_OP_T,
};
use thiserror::Error;

#[cfg(feature = "single_float")]
type CuComplex = cuda_sys::cublas::cuFloatComplex;
#[cfg(not(feature = "single_float"))]
#[repr(C)]
#[repr(align(16))]
struct CuComplex(cuda_sys::cublas::cuDoubleComplex);

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
    CuComplex(cuda_sys::cublas::cuDoubleComplex { x, y })
}

#[link(name = "autd3_cuda_kernel", kind = "static")]
extern "C" {
    fn cu_generate_propagation_matrix(
        positions: *const float,
        foci: *const float,
        wavenums: *const float,
        attens: float,
        row: u32,
        col: u32,
        dst: *mut CuComplex,
    );

    fn cu_normalize(x: *const CuComplex, row: u32, col: u32, y: *mut CuComplex);
    fn cu_scaled_to(
        a: *const CuComplex,
        b: *const CuComplex,
        row: u32,
        col: u32,
        c: *mut CuComplex,
    );

    fn cu_get_diagonal(x: *const float, row: u32, col: u32, y: *mut float);
    fn cu_get_diagonal_c(x: *const CuComplex, row: u32, col: u32, y: *mut CuComplex);
    fn cu_set_diagonal(x: *const float, n: u32, y: *mut float);
    fn cu_set_diagonal_c(x: *const CuComplex, n: u32, y: *mut CuComplex);
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
    fn cu_make_complex(re: *const float, row: u32, col: u32, dst: *mut CuComplex);
    fn cu_make_complex2(
        re: *const float,
        im: *const float,
        row: u32,
        col: u32,
        dst: *mut CuComplex,
    );
    fn cu_pow(a: *const float, p: float, row: u32, col: u32, b: *mut float);

    fn cu_conj(a: *const CuComplex, row: u32, col: u32, b: *mut CuComplex);

    fn cu_calc_singular_inv(a: *const float, row: u32, col: u32, alpha: float, b: *mut CuComplex);

    fn cu_exp(a: *const CuComplex, row: u32, col: u32, b: *mut CuComplex);
    fn cu_real(a: *const CuComplex, row: u32, col: u32, b: *mut float);
    fn cu_imag(a: *const CuComplex, row: u32, col: u32, b: *mut float);

    fn cu_reduce_col(mat: *const float, m: u32, n: u32, result: *mut float);
}

fn convert_trans(trans: autd3_gain_holo::Trans) -> u32 {
    match trans {
        autd3_gain_holo::Trans::NoTrans => cublasOperation_t_CUBLAS_OP_N,
        autd3_gain_holo::Trans::Trans => cublasOperation_t_CUBLAS_OP_T,
        autd3_gain_holo::Trans::ConjTrans => cublasOperation_t_CUBLAS_OP_C,
    }
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
            return Err(CUDABackendError::CuSOLVERError(err).into());
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
        v
    }};
    ($ty:ty, $r:expr, $c:expr) => {{
        let mut v: *mut $ty = std::ptr::null_mut();
        cuda_call!(cuda_sys::cudart::cudaMalloc(
            &mut v as *mut *mut $ty as _,
            std::mem::size_of::<$ty>() * $r * $c,
        ));
        v
    }};
}

macro_rules! alloc_zeroed {
    ($ty:ty, $len:expr) => {{
        let v = alloc_uninitialized!($ty, $len);
        cuda_call!(cuda_sys::cudart::cudaMemset(
            v as _,
            0,
            std::mem::size_of::<$ty>() * $len
        ));
        v
    }};
    ($ty:ty, $r:expr, $c:expr) => {{
        let v = alloc_uninitialized!($ty, $r, $c);
        cuda_call!(cuda_sys::cudart::cudaMemset(
            v as _,
            0,
            std::mem::size_of::<$ty>() * $r * $c
        ));
        v
    }};
}

macro_rules! free {
    ($p:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaFree($p as _))
    }};
}

macro_rules! cpy_host_to_device {
    ($ty:ty, $src:expr, $dst:expr, $len:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            $dst as _,
            $src as _,
            std::mem::size_of::<$ty>() * $len,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyHostToDevice,
        ))
    }};
}

macro_rules! cpy_device_to_device {
    ($ty:ty, $src:expr, $dst:expr, $len:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            $dst as _,
            $src as _,
            std::mem::size_of::<$ty>() * $len,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
        ))
    }};
}

macro_rules! cpy_device_to_host {
    ($ty:ty, $src:expr, $dst:expr, $len:expr) => {{
        cuda_call!(cuda_sys::cudart::cudaMemcpy(
            $dst as _,
            $src as _,
            std::mem::size_of::<$ty>() * $len,
            cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToHost,
        ))
    }};
}

pub struct CuVectorX {
    pub(crate) ptr: *mut float,
    pub(crate) len: usize,
}

impl Drop for CuVectorX {
    fn drop(&mut self) {
        unsafe {
            let _ = cuda_sys::cudart::cudaFree(self.ptr as _);
        }
    }
}

pub struct CuVectorXc {
    pub(crate) ptr: *mut CuComplex,
    pub(crate) len: usize,
}

impl Drop for CuVectorXc {
    fn drop(&mut self) {
        unsafe {
            let _ = cuda_sys::cudart::cudaFree(self.ptr as _);
        }
    }
}

pub struct CuMatrixX {
    pub(crate) ptr: *mut float,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl Drop for CuMatrixX {
    fn drop(&mut self) {
        unsafe {
            let _ = cuda_sys::cudart::cudaFree(self.ptr as _);
        }
    }
}

pub struct CuMatrixXc {
    pub(crate) ptr: *mut CuComplex,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl Drop for CuMatrixXc {
    fn drop(&mut self) {
        unsafe {
            let _ = cuda_sys::cudart::cudaFree(self.ptr as _);
        }
    }
}

/// Backend using CUDA
pub struct CUDABackend {
    handle: cuda_sys::cublas::cublasHandle_t,
    handle_s: cusolver::cusolverDnHandle_t,
}

impl Drop for CUDABackend {
    fn drop(&mut self) {
        unsafe {
            cuda_sys::cublas::cublasDestroy_v2(self.handle);
            cusolver::cusolverDnDestroy(self.handle_s);
        }
    }
}

impl LinAlgBackend for CUDABackend {
    type MatrixXc = CuMatrixXc;
    type MatrixX = CuMatrixX;
    type VectorXc = CuVectorXc;
    type VectorX = CuVectorX;

    fn new() -> Result<Rc<Self>, HoloError> {
        let mut handle: cuda_sys::cublas::cublasHandle_t = std::ptr::null_mut();
        unsafe {
            cublas_call!(cuda_sys::cublas::cublasCreate_v2(&mut handle as _));
        }

        let mut handle_s: cusolver::cusolverDnHandle_t = std::ptr::null_mut();
        unsafe { cusolver_call!(cusolver::cusolverDnCreate(&mut handle_s as _)) }

        Ok(Rc::new(Self { handle, handle_s }))
    }

    fn generate_propagation_matrix<T: autd3_core::geometry::Transducer>(
        &self,
        geometry: &autd3_core::geometry::Geometry<T>,
        foci: &[autd3_core::geometry::Vector3],
    ) -> Result<Self::MatrixXc, HoloError> {
        let rows = foci.len();
        let cols = geometry.num_transducers();
        let positions = geometry
            .transducers()
            .flat_map(|t| t.position().iter().copied())
            .collect::<Vec<_>>();
        let foci = foci
            .iter()
            .flat_map(|f| f.iter().copied())
            .collect::<Vec<_>>();
        let wavenums = geometry
            .transducers()
            .map(|t| t.wavenumber(geometry.sound_speed))
            .collect::<Vec<_>>();
        unsafe {
            let p_positions = alloc_uninitialized!(float, positions.len());
            cpy_host_to_device!(float, positions.as_ptr(), p_positions, positions.len());
            let p_foci = alloc_uninitialized!(float, foci.len());
            cpy_host_to_device!(float, foci.as_ptr(), p_foci, foci.len());
            let p_wavenums = alloc_uninitialized!(float, wavenums.len());
            cpy_host_to_device!(float, wavenums.as_ptr(), p_wavenums, wavenums.len());
            let ptr = alloc_uninitialized!(CuComplex, rows, cols);
            cu_call!(cu_generate_propagation_matrix(
                p_positions,
                p_foci,
                p_wavenums,
                geometry.attenuation,
                rows as _,
                cols as _,
                ptr
            ));
            Ok(Self::MatrixXc { ptr, rows, cols })
        }
    }

    fn alloc_v(&self, size: usize) -> Result<Self::VectorX, HoloError> {
        unsafe {
            Ok(Self::VectorX {
                ptr: alloc_uninitialized!(float, size),
                len: size,
            })
        }
    }

    fn alloc_m(&self, rows: usize, cols: usize) -> Result<Self::MatrixX, HoloError> {
        unsafe {
            Ok(Self::MatrixX {
                ptr: alloc_uninitialized!(float, rows * cols),
                rows,
                cols,
            })
        }
    }

    fn alloc_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError> {
        unsafe {
            Ok(Self::VectorXc {
                ptr: alloc_uninitialized!(CuComplex, size),
                len: size,
            })
        }
    }

    fn alloc_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError> {
        unsafe {
            Ok(Self::MatrixXc {
                ptr: alloc_uninitialized!(CuComplex, rows * cols),
                rows,
                cols,
            })
        }
    }

    fn alloc_zeros_v(&self, size: usize) -> Result<Self::VectorX, HoloError> {
        unsafe {
            Ok(Self::VectorX {
                ptr: alloc_zeroed!(float, size),
                len: size,
            })
        }
    }

    fn alloc_zeros_cv(&self, size: usize) -> Result<Self::VectorXc, HoloError> {
        unsafe {
            Ok(Self::VectorXc {
                ptr: alloc_zeroed!(CuComplex, size),
                len: size,
            })
        }
    }

    fn alloc_zeros_cm(&self, rows: usize, cols: usize) -> Result<Self::MatrixXc, HoloError> {
        unsafe {
            Ok(Self::MatrixXc {
                ptr: alloc_zeroed!(CuComplex, rows * cols),
                rows,
                cols,
            })
        }
    }

    fn to_host_v(&self, v: Self::VectorX) -> Result<VectorX, HoloError> {
        let mut dst = VectorX::zeros(v.len);
        unsafe { cpy_device_to_host!(float, v.ptr, dst.as_mut_ptr(), v.len) }
        Ok(dst)
    }

    fn to_host_m(&self, v: Self::MatrixX) -> Result<MatrixX, HoloError> {
        let mut dst = MatrixX::zeros(v.rows, v.cols);
        unsafe { cpy_device_to_host!(float, v.ptr, dst.as_mut_ptr(), v.rows * v.cols) }
        Ok(dst)
    }

    fn to_host_cv(&self, v: Self::VectorXc) -> Result<VectorXc, HoloError> {
        let mut dst = VectorXc::zeros(v.len);
        unsafe { cpy_device_to_host!(CuComplex, v.ptr, dst.as_mut_ptr(), v.len) }
        Ok(dst)
    }

    fn to_host_cm(&self, v: Self::MatrixXc) -> Result<MatrixXc, HoloError> {
        let mut dst = MatrixXc::zeros(v.rows, v.cols);
        unsafe { cpy_device_to_host!(CuComplex, v.ptr, dst.as_mut_ptr(), v.rows * v.cols) }
        Ok(dst)
    }

    fn from_slice_v(&self, v: &[float]) -> Result<Self::VectorX, HoloError> {
        unsafe {
            let len = v.len();
            let ptr = alloc_uninitialized!(float, len);
            cpy_host_to_device!(float, v.as_ptr(), ptr, len);
            Ok(Self::VectorX { ptr, len })
        }
    }

    fn from_slice_m(
        &self,
        rows: usize,
        cols: usize,
        v: &[float],
    ) -> Result<Self::MatrixX, HoloError> {
        unsafe {
            let len = v.len();
            let ptr = alloc_uninitialized!(float, len);
            cpy_host_to_device!(float, v.as_ptr(), ptr, len);
            Ok(Self::MatrixX { ptr, rows, cols })
        }
    }

    fn from_slice_cv(&self, v: &[float]) -> Result<Self::VectorXc, HoloError> {
        unsafe {
            let len = v.len();
            let re = alloc_uninitialized!(float, len);
            cpy_host_to_device!(float, v.as_ptr(), re, len);
            let ptr = alloc_uninitialized!(CuComplex, len);
            cu_call!(cu_make_complex(re, len as _, 1, ptr));
            Ok(Self::VectorXc { ptr, len })
        }
    }

    fn from_slice2_cv(&self, r: &[float], i: &[float]) -> Result<Self::VectorXc, HoloError> {
        unsafe {
            let len = r.len();
            let re = alloc_uninitialized!(float, len);
            cpy_host_to_device!(float, r.as_ptr(), re, len);
            let im = alloc_uninitialized!(float, len);
            cpy_host_to_device!(float, i.as_ptr(), im, len);
            let ptr = alloc_uninitialized!(CuComplex, len);
            cu_call!(cu_make_complex2(re, im, len as _, 1, ptr));
            Ok(Self::VectorXc { ptr, len })
        }
    }

    fn from_slice2_cm(
        &self,
        rows: usize,
        cols: usize,
        r: &[float],
        i: &[float],
    ) -> Result<Self::MatrixXc, HoloError> {
        unsafe {
            let len = r.len();
            let re = alloc_uninitialized!(float, len);
            cpy_host_to_device!(float, r.as_ptr(), re, len);
            let im = alloc_uninitialized!(float, len);
            cpy_host_to_device!(float, i.as_ptr(), im, len);
            let ptr = alloc_uninitialized!(CuComplex, len);
            cu_call!(cu_make_complex2(re, im, rows as _, cols as _, ptr));
            Ok(Self::MatrixXc { ptr, rows, cols })
        }
    }

    fn copy_from_slice_v(&self, v: &[float], dst: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            cpy_host_to_device!(float, v.as_ptr(), dst.ptr, v.len());
        }
        Ok(())
    }

    fn copy_to_v(&self, src: &Self::VectorX, dst: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            cpy_device_to_device!(float, src.ptr, dst.ptr, src.len);
        }
        Ok(())
    }

    fn copy_to_m(&self, src: &Self::MatrixX, dst: &mut Self::MatrixX) -> Result<(), HoloError> {
        unsafe {
            cpy_device_to_device!(float, src.ptr, dst.ptr, src.rows * src.cols);
        }
        Ok(())
    }

    fn clone_v(&self, v: &Self::VectorX) -> Result<Self::VectorX, HoloError> {
        unsafe {
            let len = v.len;
            let ptr = alloc_uninitialized!(float, len);
            cpy_device_to_device!(float, v.ptr, ptr, len);
            Ok(Self::VectorX { ptr, len })
        }
    }

    fn clone_m(&self, v: &Self::MatrixX) -> Result<Self::MatrixX, HoloError> {
        unsafe {
            let len = v.rows * v.cols;
            let ptr = alloc_uninitialized!(float, len);
            cpy_device_to_device!(float, v.ptr, ptr, len);
            Ok(Self::MatrixX {
                ptr,
                rows: v.rows,
                cols: v.cols,
            })
        }
    }

    fn clone_cv(&self, v: &Self::VectorXc) -> Result<Self::VectorXc, HoloError> {
        unsafe {
            let len = v.len;
            let ptr = alloc_uninitialized!(CuComplex, len);
            cpy_device_to_device!(CuComplex, v.ptr, ptr, len);
            Ok(Self::VectorXc { ptr, len })
        }
    }

    fn clone_cm(&self, v: &Self::MatrixXc) -> Result<Self::MatrixXc, HoloError> {
        unsafe {
            let len = v.rows * v.cols;
            let ptr = alloc_uninitialized!(CuComplex, len);
            cpy_device_to_device!(CuComplex, v.ptr, ptr, len);
            Ok(Self::MatrixXc {
                ptr,
                rows: v.rows,
                cols: v.cols,
            })
        }
    }

    fn make_complex2_v(
        &self,
        real: &Self::VectorX,
        imag: &Self::VectorX,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_make_complex2(
                real.ptr,
                imag.ptr,
                real.len as _,
                1,
                v.ptr
            ));
        }
        Ok(())
    }

    fn get_col_c(
        &self,
        a: &Self::MatrixXc,
        col: usize,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        let row = a.rows;
        unsafe {
            cu_call!(cuda_sys::cudart::cudaMemcpy(
                v.ptr as _,
                a.ptr.add(col * row) as _,
                std::mem::size_of::<CuComplex>() * row,
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
            ));
        }
        Ok(())
    }

    fn set_cv(
        &self,
        i: usize,
        val: autd3_gain_holo::Complex,
        v: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cuda_sys::cudart::cudaMemcpy(
                v.ptr.add(i) as _,
                &val as *const _ as _,
                std::mem::size_of::<CuComplex>(),
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyHostToDevice,
            ));
        }
        Ok(())
    }

    fn set_col_c(
        &self,
        a: &Self::VectorXc,
        col: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cuda_sys::cudart::cudaMemcpy(
                v.ptr.add(col * v.rows + start) as _,
                a.ptr.add(start) as _,
                std::mem::size_of::<CuComplex>() * (end - start),
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
            ));
        }
        Ok(())
    }

    fn set_row_c(
        &self,
        a: &Self::VectorXc,
        row: usize,
        start: usize,
        end: usize,
        v: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        unsafe {
            let rows = v.rows;
            let src_p = a.ptr;
            let dst_p = v.ptr;
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasCcopy_v2(
                self.handle,
                (end - start) as _,
                src_p.add(start),
                1,
                dst_p.add(row + start * rows),
                rows as _,
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasZcopy_v2(
                self.handle,
                (end - start) as _,
                src_p.add(start) as _,
                1,
                dst_p.add(row + start * rows) as _,
                rows as _,
            ));
        }
        Ok(())
    }

    fn get_diagonal_c(&self, a: &Self::MatrixXc, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_get_diagonal_c(
                a.ptr as _,
                a.rows as _,
                a.cols as _,
                v.ptr as _
            ));
        }
        Ok(())
    }

    fn create_diagonal(&self, v: &Self::VectorX, a: &mut Self::MatrixX) -> Result<(), HoloError> {
        unsafe {
            cuda_call!(cuda_sys::cudart::cudaMemset(
                a.ptr as _,
                0,
                std::mem::size_of::<float>() * a.rows * a.cols
            ));
            cu_call!(cu_set_diagonal(v.ptr as _, v.len as _, a.ptr));
        }
        Ok(())
    }

    fn create_diagonal_c(
        &self,
        v: &Self::VectorXc,
        a: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cuda_call!(cuda_sys::cudart::cudaMemset(
                a.ptr as _,
                0,
                std::mem::size_of::<CuComplex>() * a.rows * a.cols
            ));
            cu_call!(cu_set_diagonal_c(v.ptr as _, v.len as _, a.ptr as _));
        }
        Ok(())
    }

    fn get_diagonal(&self, a: &Self::MatrixX, v: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_get_diagonal(
                a.ptr as _,
                a.rows as _,
                a.cols as _,
                v.ptr as _
            ));
        }
        Ok(())
    }

    fn abs_cv(&self, a: &Self::VectorXc, b: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_abs(a.ptr as _, a.len as _, 1, b.ptr as _));
        }
        Ok(())
    }

    fn real_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_real(a.ptr as _, a.rows as _, a.cols as _, b.ptr as _));
        }
        Ok(())
    }

    fn imag_cm(&self, a: &Self::MatrixXc, b: &mut Self::MatrixX) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_imag(a.ptr as _, a.rows as _, a.cols as _, b.ptr as _));
        }
        Ok(())
    }

    fn scale_assign_v(&self, a: float, b: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasSscal_v2(
                self.handle,
                b.len as _,
                &a as _,
                b.ptr as _,
                1
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasDscal_v2(
                self.handle,
                b.len as _,
                &a as _,
                b.ptr as _,
                1
            ));
        }
        Ok(())
    }

    fn scale_assign_cv(
        &self,
        a: autd3_gain_holo::Complex,
        b: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        let a = make_complex(a.re, a.im);
        unsafe {
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasCscal_v2(
                self.handle,
                b.len as _,
                &a as *const _ as _,
                b.ptr,
                1
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasZscal_v2(
                self.handle,
                b.len as _,
                &a as *const _ as _,
                b.ptr as _,
                1
            ));
        }
        Ok(())
    }

    fn conj_assign_v(&self, b: &mut Self::VectorXc) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_conj(b.ptr as _, b.len as _, 1, b.ptr as _));
        }
        Ok(())
    }

    fn sqrt_assign_v(&self, v: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_sqrt(v.ptr as _, v.len as _, 1, v.ptr as _));
        }
        Ok(())
    }

    fn normalize_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_normalize(v.ptr as _, v.len as _, 1, v.ptr as _));
        }
        Ok(())
    }

    fn reciprocal_assign_c(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_reciprocal(v.ptr as _, v.len as _, 1, v.ptr as _));
        }
        Ok(())
    }

    fn pow_assign_v(&self, a: float, v: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_pow(v.ptr as _, a, v.len as _, 1, v.ptr as _));
        }
        Ok(())
    }

    fn exp_assign_cv(&self, v: &mut Self::VectorXc) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_exp(v.ptr as _, v.len as _, 1, v.ptr as _));
        }
        Ok(())
    }

    fn concat_row_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        unsafe {
            for i in 0..a.cols {
                cuda_call!(cuda_sys::cudart::cudaMemcpy(
                    c.ptr.add(i * (a.rows + b.rows)) as _,
                    a.ptr.add(i * a.rows) as _,
                    std::mem::size_of::<CuComplex>() * a.rows,
                    cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
                ));
                cuda_call!(cuda_sys::cudart::cudaMemcpy(
                    c.ptr.add(i * (a.rows + b.rows) + a.rows) as _,
                    b.ptr.add(i * b.rows) as _,
                    std::mem::size_of::<CuComplex>() * b.rows,
                    cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
                ));
            }
        }
        Ok(())
    }

    fn concat_col_cv(
        &self,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
        c: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cuda_call!(cuda_sys::cudart::cudaMemcpy(
                c.ptr as _,
                a.ptr as _,
                a.len * std::mem::size_of::<CuComplex>(),
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice
            ));
            cuda_call!(cuda_sys::cudart::cudaMemcpy(
                c.ptr.add(a.len) as _,
                b.ptr as _,
                b.len * std::mem::size_of::<CuComplex>(),
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice
            ));
        }
        Ok(())
    }

    fn concat_col_cm(
        &self,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        c: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cuda_call!(cuda_sys::cudart::cudaMemcpy(
                c.ptr as _,
                a.ptr as _,
                a.rows * a.cols * std::mem::size_of::<CuComplex>(),
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice
            ));
            cuda_call!(cuda_sys::cudart::cudaMemcpy(
                c.ptr.add(a.rows * a.cols) as _,
                b.ptr as _,
                b.rows * b.cols * std::mem::size_of::<CuComplex>(),
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice
            ));
        }
        Ok(())
    }

    fn max_v(&self, m: &Self::VectorX) -> Result<float, HoloError> {
        // TODO: impl with GPU
        let mut tmp: Vec<float> = vec![0.; m.len];
        unsafe {
            cpy_device_to_host!(float, m.ptr, tmp.as_mut_ptr(), m.len);
        }
        Ok(tmp.into_iter().fold(0., float::max))
    }

    fn max_eigen_vector_c(&self, m: Self::MatrixXc) -> Result<Self::VectorXc, HoloError> {
        unsafe {
            let max_ev = alloc_uninitialized!(CuComplex, m.cols);

            let dw = alloc_uninitialized!(float, m.cols);

            let mut workspace_in_bytes_on_device: u64 = 0;
            let mut workspace_in_bytes_on_host: u64 = 0;
            cusolver_call!(cusolver::cusolverDnXsyevd_bufferSize(
                self.handle_s,
                std::ptr::null_mut(),
                cusolver::cusolverEigMode_t::CUSOLVER_EIG_MODE_VECTOR,
                cusolver::cublasFillMode_t::CUBLAS_FILL_MODE_UPPER,
                m.cols as _,
                AUTD_C,
                m.ptr as _,
                m.cols as _,
                AUTD_R,
                dw as _,
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
                m.cols as _,
                AUTD_C,
                m.ptr as _,
                m.cols as _,
                AUTD_R,
                dw as _,
                AUTD_C,
                workspace_buffer_on_device as _,
                workspace_in_bytes_on_device,
                workspace_buffer_on_host as _,
                workspace_in_bytes_on_host,
                info as _,
            ));

            free!(dw);
            free!(info);
            free!(workspace_buffer_on_device);

            cuda_call!(cuda_sys::cudart::cudaMemcpy(
                max_ev as _,
                m.ptr.add(m.cols * (m.cols - 1)) as _,
                std::mem::size_of::<CuComplex>() * m.cols,
                cuda_sys::cudart::cudaMemcpyKind_cudaMemcpyDeviceToDevice,
            ));
            Ok(Self::VectorXc {
                ptr: max_ev,
                len: m.cols,
            })
        }
    }

    fn hadamard_product_assign_cv(
        &self,
        x: &Self::VectorXc,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_hadamard_product(
                x.ptr as _, y.ptr as _, x.len as _, 1, y.ptr as _
            ));
        }
        Ok(())
    }

    fn hadamard_product_cv(
        &self,
        x: &Self::VectorXc,
        y: &Self::VectorXc,
        z: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_hadamard_product(
                x.ptr as _, y.ptr as _, x.len as _, 1, z.ptr as _
            ));
        }
        Ok(())
    }

    fn hadamard_product_cm(
        &self,
        x: &Self::MatrixXc,
        y: &Self::MatrixXc,
        z: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_hadamard_product(
                x.ptr as _,
                y.ptr as _,
                x.rows as _,
                x.cols as _,
                z.ptr as _
            ));
        }
        Ok(())
    }

    fn dot(&self, x: &Self::VectorX, y: &Self::VectorX) -> Result<float, HoloError> {
        unsafe {
            let mut d: float = 0.;
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasSdot_v2(
                self.handle,
                x.len as _,
                x.ptr as _,
                1,
                y.ptr as _,
                1,
                &mut d as _,
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasDdot_v2(
                self.handle,
                x.len as _,
                x.ptr as _,
                1,
                y.ptr as _,
                1,
                &mut d as _,
            ));
            Ok(d)
        }
    }

    fn dot_c(
        &self,
        x: &Self::VectorXc,
        y: &Self::VectorXc,
    ) -> Result<autd3_gain_holo::Complex, HoloError> {
        unsafe {
            let mut d = autd3_gain_holo::Complex::new(0., 0.);
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasCdotc_v2(
                self.handle,
                x.len as _,
                x.ptr as _,
                1,
                y.ptr as _,
                1,
                &mut d as *mut _ as _,
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasZdotc_v2(
                self.handle,
                x.len as _,
                x.ptr as _,
                1,
                y.ptr as _,
                1,
                &mut d as *mut _ as _,
            ));
            Ok(d)
        }
    }

    fn add_v(
        &self,
        alpha: float,
        a: &Self::VectorX,
        b: &mut Self::VectorX,
    ) -> Result<(), HoloError> {
        unsafe {
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasSaxpy_v2(
                self.handle,
                a.len as _,
                &alpha as _,
                a.ptr as _,
                1,
                b.ptr as _,
                1
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasDaxpy_v2(
                self.handle,
                a.len as _,
                &alpha as _,
                a.ptr as _,
                1,
                b.ptr as _,
                1
            ));
            Ok(())
        }
    }

    fn add_m(
        &self,
        alpha: float,
        a: &Self::MatrixX,
        b: &mut Self::MatrixX,
    ) -> Result<(), HoloError> {
        unsafe {
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasSaxpy_v2(
                self.handle,
                (a.rows * a.cols) as _,
                &alpha as _,
                a.ptr as _,
                1,
                b.ptr as _,
                1
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasDaxpy_v2(
                self.handle,
                (a.rows * a.cols) as _,
                &alpha as _,
                a.ptr as _,
                1,
                b.ptr as _,
                1
            ));
            Ok(())
        }
    }

    fn gevv_c(
        &self,
        trans_a: autd3_gain_holo::Trans,
        trans_b: autd3_gain_holo::Trans,
        alpha: autd3_gain_holo::Complex,
        a: &Self::VectorXc,
        x: &Self::VectorXc,
        beta: autd3_gain_holo::Complex,
        y: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        let transa = convert_trans(trans_a);
        let transb = convert_trans(trans_b);
        let alpha = make_complex(alpha.re, alpha.im);
        let beta = make_complex(beta.re, beta.im);

        unsafe {
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasCgemm_v2(
                self.handle,
                transa,
                transb,
                y.rows as _,
                y.cols as _,
                if transa == cublasOperation_t_CUBLAS_OP_N {
                    1
                } else {
                    a.len
                } as _,
                &alpha as _,
                a.ptr,
                a.len as _,
                x.ptr,
                x.len as _,
                &beta as _,
                y.ptr,
                y.rows as _,
            ));

            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasZgemm_v2(
                self.handle,
                transa,
                transb,
                y.rows as _,
                y.cols as _,
                if transa == cublasOperation_t_CUBLAS_OP_N {
                    1
                } else {
                    a.len
                } as _,
                &alpha as *const _ as _,
                a.ptr as _,
                a.len as _,
                x.ptr as _,
                x.len as _,
                &beta as *const _ as _,
                y.ptr as _,
                y.rows as _,
            ));
        }
        Ok(())
    }

    fn gemv_c(
        &self,
        trans: autd3_gain_holo::Trans,
        alpha: autd3_gain_holo::Complex,
        a: &Self::MatrixXc,
        x: &Self::VectorXc,
        beta: autd3_gain_holo::Complex,
        y: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        let trans = convert_trans(trans);

        let alpha = make_complex(alpha.re, alpha.im);
        let beta = make_complex(beta.re, beta.im);

        unsafe {
            let m = a.rows;
            let n = a.cols;
            let lda = m;

            let ap = a.ptr;
            let bp = x.ptr;
            let cp = y.ptr;

            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasCgemv_v2(
                self.handle,
                trans,
                m as _,
                n as _,
                &alpha as _,
                ap,
                lda as _,
                bp,
                1,
                &beta as _,
                cp,
                1,
            ));
            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasZgemv_v2(
                self.handle,
                trans,
                m as _,
                n as _,
                &alpha as *const _ as _,
                ap as _,
                lda as _,
                bp as _,
                1,
                &beta as *const _ as _,
                cp as _,
                1,
            ));

            Ok(())
        }
    }

    fn gemm_c(
        &self,
        trans_a: autd3_gain_holo::Trans,
        trans_b: autd3_gain_holo::Trans,
        alpha: autd3_gain_holo::Complex,
        a: &Self::MatrixXc,
        b: &Self::MatrixXc,
        beta: autd3_gain_holo::Complex,
        y: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        let transa = convert_trans(trans_a);
        let transb = convert_trans(trans_b);

        let alpha = make_complex(alpha.re, alpha.im);
        let beta = make_complex(beta.re, beta.im);

        unsafe {
            #[cfg(feature = "single_float")]
            cublas_call!(cuda_sys::cublas::cublasCgemm_v2(
                self.handle,
                transa,
                transb,
                y.rows as _,
                y.cols as _,
                if transa == cublasOperation_t_CUBLAS_OP_N {
                    a.cols
                } else {
                    a.rows
                } as _,
                &alpha as _,
                a.ptr,
                a.rows as _,
                b.ptr,
                b.rows as _,
                &beta as _,
                y.ptr,
                y.rows as _,
            ));

            #[cfg(not(feature = "single_float"))]
            cublas_call!(cuda_sys::cublas::cublasZgemm_v2(
                self.handle,
                transa,
                transb,
                y.rows as _,
                y.cols as _,
                if transa == cublasOperation_t_CUBLAS_OP_N {
                    a.cols
                } else {
                    a.rows
                } as _,
                &alpha as *const _ as _,
                a.ptr as _,
                a.rows as _,
                b.ptr as _,
                b.rows as _,
                &beta as *const _ as _,
                y.ptr as _,
                y.rows as _,
            ));
        }
        Ok(())
    }

    fn pseudo_inverse_svd(
        &self,
        a: Self::MatrixXc,
        alpha: float,
        u: &mut Self::MatrixXc,
        s: &mut Self::MatrixXc,
        vt: &mut Self::MatrixXc,
        buf: &mut Self::MatrixXc,
        b: &mut Self::MatrixXc,
    ) -> Result<(), HoloError> {
        unsafe {
            let m = a.rows;
            let n = a.cols;

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
                a.ptr as _,
                lda as _,
                AUTD_R,
                ds as _,
                AUTD_C,
                u.ptr as _,
                ldu as _,
                AUTD_C,
                vt.ptr as _,
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
                a.ptr as _,
                lda as _,
                AUTD_R,
                ds as _,
                AUTD_C,
                u.ptr as _,
                ldu as _,
                AUTD_C,
                vt.ptr as _,
                ldv as _,
                AUTD_C,
                workspace_buffer_on_device as _,
                workspace_in_bytes_on_device,
                workspace_buffer_on_host as _,
                workspace_in_bytes_on_host,
                info as _,
                &mut h_err_sigma as _,
            ));

            cu_call!(cu_calc_singular_inv(ds, n as _, m as _, alpha, s.ptr));

            self.gemm_c(
                autd3_gain_holo::Trans::NoTrans,
                autd3_gain_holo::Trans::ConjTrans,
                autd3_gain_holo::Complex::new(1., 0.),
                s,
                u,
                autd3_gain_holo::Complex::new(0., 0.),
                buf,
            )?;
            self.gemm_c(
                autd3_gain_holo::Trans::NoTrans,
                autd3_gain_holo::Trans::NoTrans,
                autd3_gain_holo::Complex::new(1., 0.),
                vt,
                buf,
                autd3_gain_holo::Complex::new(0., 0.),
                b,
            )?;

            free!(ds);
            free!(info);
            free!(workspace_buffer_on_device);
        }
        Ok(())
    }

    fn solve_inplace(&self, a: &Self::MatrixX, x: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            let n = a.cols;
            let lda = a.rows;
            let ldb = x.len;

            let ap = a.ptr;
            let bp = x.ptr;

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
                workspace_buffer_on_device as _,
                workspace_in_bytes_on_device,
                workspace_buffer_on_host as _,
                workspace_in_bytes_on_host,
                info as _,
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
                info as _,
            ));

            free!(info);
            free!(workspace_buffer_on_device);
        }
        Ok(())
    }

    fn solve_inplace_h(&self, a: Self::MatrixXc, x: &mut Self::VectorXc) -> Result<(), HoloError> {
        unsafe {
            let n = a.cols;
            let lda = a.rows;
            let ldb = x.len;

            let ap = a.ptr;
            let bp = x.ptr;

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
                workspace_buffer_on_device as _,
                workspace_in_bytes_on_device,
                workspace_buffer_on_host as _,
                workspace_in_bytes_on_host,
                info as _,
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
                info as _,
            ));

            free!(info);
            free!(workspace_buffer_on_device);
        }
        Ok(())
    }

    fn reduce_col(&self, a: &Self::MatrixX, b: &mut Self::VectorX) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_reduce_col(
                a.ptr as _,
                a.rows as _,
                a.cols as _,
                b.ptr as _,
            ));
        }
        Ok(())
    }

    fn scaled_to_cv(
        &self,
        a: &Self::VectorXc,
        b: &Self::VectorXc,
        c: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_scaled_to(
                a.ptr as _, b.ptr as _, a.len as _, 1, c.ptr as _
            ));
        }
        Ok(())
    }

    fn scaled_to_assign_cv(
        &self,
        a: &Self::VectorXc,
        b: &mut Self::VectorXc,
    ) -> Result<(), HoloError> {
        unsafe {
            cu_call!(cu_scaled_to(
                b.ptr as _, a.ptr as _, a.len as _, 1, b.ptr as _
            ));
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "test-utilities"))]
mod tests {
    use super::*;

    use autd3_gain_holo::test_utilities::test_utils::*;

    #[test]
    fn test_cuda_backend() {
        LinAlgBackendTestHelper::<100, CUDABackend>::new()
            .expect("failet to initialize CUDABackend")
            .test()
            .expect("Faild to test CUDABackend");
    }
}
