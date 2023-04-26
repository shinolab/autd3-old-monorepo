// File: backend_cuda.cpp
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/gain/backend_cuda.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4102 26439 26478 26495 26812)
#endif
#include <cublas_v2.h>
#include <cuda_runtime_api.h>
#include <cusolverDn.h>

#include "./kernel.h"
#if _MSC_VER
#pragma warning(pop)
#endif

#ifdef AUTD3_USE_SINGLE_FLOAT
#define AUTDCscal cublasCscal
#define AUTDCcopy cublasCcopy
#define AUTDscal cublasSscal
#define AUTDCdotc cublasCdotc
#define AUTDgemv cublasSgemv
#define AUTDgemm cublasSgemm
#define AUTDCgemv cublasCgemv
#define AUTDCgemm cublasCgemm
#define AUTDaxpy cublasSaxpy
#define AUTDCaxpy cublasCaxpy
#define AUTDdot cublasSdot
#define AUTD_R CUDA_R_32F
#define AUTD_C CUDA_C_32F
#else
#define AUTDCscal cublasZscal
#define AUTDCcopy cublasZcopy
#define AUTDscal cublasDscal
#define AUTDCdotc cublasZdotc
#define AUTDgemv cublasDgemv
#define AUTDgemm cublasDgemm
#define AUTDCgemv cublasZgemv
#define AUTDCgemm cublasZgemm
#define AUTDaxpy cublasDaxpy
#define AUTDCaxpy cublasZaxpy
#define AUTDdot cublasDdot
#define AUTD_R CUDA_R_64F
#define AUTD_C CUDA_C_64F
#endif

namespace autd3::gain::holo {

namespace {
cublasOperation_t convert(const Transpose trans) {
  switch (trans) {
    case Transpose::NoTrans:
      return CUBLAS_OP_N;
    case Transpose::ConjTrans:
      return CUBLAS_OP_C;
    case Transpose::Trans:
      return CUBLAS_OP_T;
  }
  return CUBLAS_OP_N;
}
}  // namespace

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26812)
#endif

class BufferPool final {
 public:
  BufferPool() = default;
  ~BufferPool() { clear(); }
  BufferPool(const BufferPool& v) noexcept = default;
  BufferPool& operator=(const BufferPool& obj) = default;
  BufferPool(BufferPool&& obj) = default;
  BufferPool& operator=(BufferPool&& obj) = default;

  void* get(const VectorXc& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) return _pool[key];

    void* dp;
    cudaMalloc(&dp, sizeof(complex) * v.size());
    cudaMemcpy(dp, v.data(), sizeof(complex) * v.size(), cudaMemcpyHostToDevice);
    _pool.emplace(key, dp);
    return dp;
  }

  void* get(const MatrixXc& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) return _pool[key];

    void* dp;
    cudaMalloc(&dp, sizeof(complex) * v.size());
    cudaMemcpy(dp, v.data(), sizeof(complex) * v.size(), cudaMemcpyHostToDevice);
    _pool.emplace(key, dp);
    return dp;
  }

  void* get(const VectorXd& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) return _pool[key];

    void* dp;
    cudaMalloc(&dp, sizeof(driver::float_t) * v.size());
    cudaMemcpy(dp, v.data(), sizeof(driver::float_t) * v.size(), cudaMemcpyHostToDevice);
    _pool.emplace(key, dp);
    return dp;
  }

  void* get(const MatrixXd& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) return _pool[key];

    void* dp;
    cudaMalloc(&dp, sizeof(driver::float_t) * v.size());
    cudaMemcpy(dp, v.data(), sizeof(driver::float_t) * v.size(), cudaMemcpyHostToDevice);
    _pool.emplace(key, dp);
    return dp;
  }

  void clear() {
    for (const auto& [_, p] : _pool) cudaFree(p);
    _pool.clear();
  }

 private:
  std::unordered_map<std::uintptr_t, void*> _pool;
};

class CUDABackendImpl final : public Backend {
 public:
  explicit CUDABackendImpl(const int device_idx) {
    cudaSetDevice(device_idx);
    cublasCreate(&_handle);
    cusolverDnCreate(&_handle_s);
  }
  ~CUDABackendImpl() override {
    cublasDestroy(_handle);
    cusolverDnDestroy(_handle_s);
  }
  CUDABackendImpl(const CUDABackendImpl& v) = default;
  CUDABackendImpl& operator=(const CUDABackendImpl& obj) = default;
  CUDABackendImpl(CUDABackendImpl&& obj) = default;
  CUDABackendImpl& operator=(CUDABackendImpl&& obj) = default;

  void init() override { _pool.clear(); }

  void to_host(VectorXc& dst) override {
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst.data(), dst_p, sizeof(complex) * dst.size(), cudaMemcpyDeviceToHost);
  }
  void to_host(MatrixXc& dst) override {
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst.data(), dst_p, sizeof(complex) * dst.size(), cudaMemcpyDeviceToHost);
  }
  void to_host(VectorXd& dst) override {
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst.data(), dst_p, sizeof(driver::float_t) * dst.size(), cudaMemcpyDeviceToHost);
  }
  void to_host(MatrixXd& dst) override {
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst.data(), dst_p, sizeof(driver::float_t) * dst.size(), cudaMemcpyDeviceToHost);
  }

  void copy_to(const MatrixXc& src, MatrixXc& dst) override {
    const auto src_p = _pool.get(src);
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst_p, src_p, sizeof(complex) * src.size(), cudaMemcpyDeviceToDevice);
  }

  void copy_to(const MatrixXd& src, MatrixXd& dst) override {
    const auto src_p = _pool.get(src);
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst_p, src_p, sizeof(driver::float_t) * src.size(), cudaMemcpyDeviceToDevice);
  }
  void copy_to(const VectorXd& src, VectorXd& dst) override {
    const auto src_p = _pool.get(src);
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst_p, src_p, sizeof(driver::float_t) * src.size(), cudaMemcpyDeviceToDevice);
  }
  void copy_to(const VectorXc& src, VectorXc& dst) override {
    const auto src_p = _pool.get(src);
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst_p, src_p, sizeof(complex) * src.size(), cudaMemcpyDeviceToDevice);
  }

  void abs(const VectorXc& src, VectorXd& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<driver::float_t*>(_pool.get(dst));
    cu_abs(src_p, size, 1, dst_p);
  }

  void abs(const VectorXc& src, VectorXc& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_abs(src_p, size, 1, dst_p);
  }
  void sqrt(const VectorXd& src, VectorXd& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<driver::float_t*>(_pool.get(src));
    const auto dst_p = static_cast<driver::float_t*>(_pool.get(dst));
    cu_sqrt(src_p, size, 1, dst_p);
  }
  void conj(const VectorXc& src, VectorXc& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_conj(src_p, size, 1, dst_p);
  }
  void arg(const VectorXc& src, VectorXc& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_arg(src_p, size, 1, dst_p);
  }
  void reciprocal(const VectorXc& src, VectorXc& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_reciprocal(src_p, size, 1, dst_p);
  }
  void exp(const VectorXc& src, VectorXc& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_exp(src_p, size, 1, dst_p);
  }
  void pow(const VectorXd& src, const driver::float_t p, VectorXd& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<driver::float_t*>(_pool.get(src));
    const auto dst_p = static_cast<driver::float_t*>(_pool.get(dst));
    cu_pow(src_p, p, size, 1, dst_p);
  }

  void real(const MatrixXc& src, MatrixXd& re) override {
    const auto row = static_cast<uint32_t>(src.rows());
    const auto col = static_cast<uint32_t>(src.cols());
    const auto sp = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto rp = static_cast<driver::float_t*>(_pool.get(re));
    cu_real(sp, row, col, rp);
  }
  void imag(const MatrixXc& src, MatrixXd& im) override {
    const auto row = static_cast<uint32_t>(src.rows());
    const auto col = static_cast<uint32_t>(src.cols());
    const auto sp = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto ip = static_cast<driver::float_t*>(_pool.get(im));
    cu_imag(sp, row, col, ip);
  }

  void make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) override {
    const auto row = static_cast<uint32_t>(dst.rows());
    const auto col = static_cast<uint32_t>(dst.cols());
    const auto rp = static_cast<driver::float_t*>(_pool.get(re));
    const auto ip = static_cast<driver::float_t*>(_pool.get(im));
    const auto dp = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_make_complex(rp, ip, row, col, dp);
  }

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override {
    const auto row = static_cast<uint32_t>(dst.rows());
    const auto col = static_cast<uint32_t>(dst.cols());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_set_diagonal(src_p, row, col, dst_p);
  }

  void get_diagonal(const MatrixXc& src, VectorXc& dst) override {
    const auto row = static_cast<uint32_t>(src.rows());
    const auto col = static_cast<uint32_t>(src.cols());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cu_get_diagonal(src_p, row, col, dst_p);
  }
  void get_diagonal(const MatrixXd& src, VectorXd& dst) override {
    const auto row = static_cast<uint32_t>(src.rows());
    const auto col = static_cast<uint32_t>(src.cols());
    const auto src_p = static_cast<driver::float_t*>(_pool.get(src));
    const auto dst_p = static_cast<driver::float_t*>(_pool.get(dst));
    cu_get_diagonal(src_p, row, col, dst_p);
  }

  void set(const size_t i, const complex value, VectorXc& dst) override {
    const auto dst_p = static_cast<complex*>(_pool.get(dst));
    cudaMemcpy(dst_p + i, &value, sizeof(complex), cudaMemcpyHostToDevice);
  }

  void set_row(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) override {
    const auto row = static_cast<int>(dst.rows());
    const auto src_p = static_cast<autd3_complex_t*>(_pool.get(src));
    const auto dst_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    AUTDCcopy(_handle, static_cast<int>(end - begin), src_p + begin, 1, dst_p + i + begin * row, row);
  }

  void set_col(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) override {
    const auto row = dst.rows();
    const auto src_p = static_cast<complex*>(_pool.get(src));
    const auto dst_p = static_cast<complex*>(_pool.get(dst));
    cudaMemcpy(dst_p + i * row + begin, src_p + begin, (end - begin) * sizeof(complex), cudaMemcpyDeviceToDevice);
  }

  void get_col(const MatrixXc& src, const size_t i, VectorXc& dst) override {
    const auto row = src.rows();
    const auto src_p = static_cast<complex*>(_pool.get(src));
    const auto dst_p = static_cast<complex*>(_pool.get(dst));
    cudaMemcpy(dst_p, src_p + i * row, row * sizeof(complex), cudaMemcpyDeviceToDevice);
  }

  complex max_abs_element(const VectorXc& src) override {
    const auto src_p = _pool.get(src);
    VectorXc tmp(src.size());
    cudaMemcpy(tmp.data(), src_p, sizeof(complex) * src.size(), cudaMemcpyDeviceToHost);
    Eigen::Index idx = 0;
    tmp.cwiseAbs2().maxCoeff(&idx);
    return tmp(idx);
  }

  driver::float_t max_element(const VectorXd& src) override {
    const auto src_p = _pool.get(src);
    VectorXd tmp(src.size());
    cudaMemcpy(tmp.data(), src_p, sizeof(driver::float_t) * src.size(), cudaMemcpyDeviceToHost);
    return tmp.maxCoeff();
  }

  void scale(const complex value, VectorXc& dst) override {
    const auto dst_p = static_cast<complex*>(_pool.get(dst));
    AUTDCscal(_handle, static_cast<int>(dst.size()), reinterpret_cast<const autd3_complex_t*>(&value), reinterpret_cast<autd3_complex_t*>(dst_p), 1);
  }

  void scale(const driver::float_t value, VectorXd& dst) override {
    const auto dst_p = static_cast<driver::float_t*>(_pool.get(dst));
    AUTDscal(_handle, static_cast<int>(dst.size()), &value, dst_p, 1);
  }

  complex dot(const VectorXc& a, const VectorXc& b) override {
    complex d;
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    AUTDCdotc(_handle, static_cast<int>(a.size()), a_p, 1, b_p, 1, reinterpret_cast<autd3_complex_t*>(&d));
    return d;
  }

  driver::float_t dot(const VectorXd& a, const VectorXd& b) override {
    driver::float_t d;
    const auto a_p = static_cast<driver::float_t*>(_pool.get(a));
    const auto b_p = static_cast<driver::float_t*>(_pool.get(b));
    AUTDdot(_handle, static_cast<int>(a.size()), a_p, 1, b_p, 1, &d);
    return d;
  }

  void add(const driver::float_t alpha, const MatrixXd& a, MatrixXd& b) override {
    const auto a_p = static_cast<driver::float_t*>(_pool.get(a));
    const auto b_p = static_cast<driver::float_t*>(_pool.get(b));
    AUTDaxpy(_handle, static_cast<int>(a.size()), &alpha, a_p, 1, b_p, 1);
  }

  void add(const driver::float_t alpha, const VectorXd& a, VectorXd& b) override {
    const auto a_p = static_cast<driver::float_t*>(_pool.get(a));
    const auto b_p = static_cast<driver::float_t*>(_pool.get(b));
    AUTDaxpy(_handle, static_cast<int>(a.size()), &alpha, a_p, 1, b_p, 1);
  }

  void add(const complex alpha, const MatrixXc& a, MatrixXc& b) override {
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    AUTDCaxpy(_handle, static_cast<int>(a.size()), reinterpret_cast<const autd3_complex_t*>(&alpha), a_p, 1, b_p, 1);
  }

  void add(const complex alpha, const VectorXc& a, VectorXc& b) override {
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    AUTDCaxpy(_handle, static_cast<int>(a.size()), reinterpret_cast<const autd3_complex_t*>(&alpha), a_p, 1, b_p, 1);
  }

  void mul(const Transpose trans_a, const Transpose trans_b, const complex alpha, const MatrixXc& a, const MatrixXc& b, const complex beta,
           MatrixXc& c) override {
    const auto m = static_cast<int>(c.rows());
    const auto n = static_cast<int>(c.cols());
    const auto k = trans_a == Transpose::NoTrans ? static_cast<int>(a.cols()) : static_cast<int>(a.rows());
    const auto lda = static_cast<int>(a.rows());
    const auto ldb = static_cast<int>(b.rows());
    const auto ldc = static_cast<int>(c.rows());
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    const auto c_p = static_cast<autd3_complex_t*>(_pool.get(c));
    AUTDCgemm(_handle, convert(trans_a), convert(trans_b), m, n, k, reinterpret_cast<const autd3_complex_t*>(&alpha), a_p, lda, b_p, ldb,
              reinterpret_cast<const autd3_complex_t*>(&beta), c_p, ldc);
  }

  void mul(const Transpose trans_a, const complex alpha, const MatrixXc& a, const VectorXc& b, const complex beta, VectorXc& c) override {
    const auto m = static_cast<int>(a.rows());
    const auto n = static_cast<int>(a.cols());
    const auto lda = m;
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    const auto c_p = static_cast<autd3_complex_t*>(_pool.get(c));
    AUTDCgemv(_handle, convert(trans_a), m, n, reinterpret_cast<const autd3_complex_t*>(&alpha), a_p, lda, b_p, 1,
              reinterpret_cast<const autd3_complex_t*>(&beta), c_p, 1);
  }

  void mul(const Transpose trans_a, const Transpose trans_b, const driver::float_t alpha, const MatrixXd& a, const MatrixXd& b,
           const driver::float_t beta, MatrixXd& c) override {
    const auto m = static_cast<int>(c.rows());
    const auto n = static_cast<int>(c.cols());
    const auto k = trans_a == Transpose::NoTrans ? static_cast<int>(a.cols()) : static_cast<int>(a.rows());
    const auto lda = static_cast<int>(a.rows());
    const auto ldb = static_cast<int>(b.rows());
    const auto ldc = static_cast<int>(c.rows());
    const auto a_p = static_cast<driver::float_t*>(_pool.get(a));
    const auto b_p = static_cast<driver::float_t*>(_pool.get(b));
    const auto c_p = static_cast<driver::float_t*>(_pool.get(c));
    AUTDgemm(_handle, convert(trans_a), convert(trans_b), m, n, k, &alpha, a_p, lda, b_p, ldb, &beta, c_p, ldc);
  }
  void mul(const Transpose trans_a, const driver::float_t alpha, const MatrixXd& a, const VectorXd& b, const driver::float_t beta,
           VectorXd& c) override {
    const auto m = static_cast<int>(a.rows());
    const auto n = static_cast<int>(a.cols());
    const auto lda = m;
    const auto a_p = static_cast<driver::float_t*>(_pool.get(a));
    const auto b_p = static_cast<driver::float_t*>(_pool.get(b));
    const auto c_p = static_cast<driver::float_t*>(_pool.get(c));
    AUTDgemv(_handle, convert(trans_a), m, n, &alpha, a_p, lda, b_p, 1, &beta, c_p, 1);
  }

  void hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) override {
    const auto m = static_cast<uint32_t>(a.size());
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    const auto c_p = static_cast<autd3_complex_t*>(_pool.get(c));
    cu_hadamard_product(a_p, b_p, m, 1, c_p);
  }

  void hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) override {
    const auto m = static_cast<uint32_t>(a.rows());
    const auto n = static_cast<uint32_t>(a.cols());
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    const auto c_p = static_cast<autd3_complex_t*>(_pool.get(c));
    cu_hadamard_product(a_p, b_p, m, n, c_p);
  }

  void concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override {
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    const auto c_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cudaMemcpy(c_p, a_p, a.size() * sizeof(complex), cudaMemcpyDeviceToDevice);
    cudaMemcpy(c_p + a.size(), b_p, b.size() * sizeof(complex), cudaMemcpyDeviceToDevice);
  }
  void concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override {
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    const auto c_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    for (Eigen::Index i = 0; i < a.cols(); i++) {
      cudaMemcpy(c_p + i * (a.rows() + b.rows()), a_p + i * a.rows(), a.rows() * sizeof(complex), cudaMemcpyDeviceToDevice);
      cudaMemcpy(c_p + i * (a.rows() + b.rows()) + a.rows(), b_p + i * b.rows(), b.rows() * sizeof(complex), cudaMemcpyDeviceToDevice);
    }
  }
  void concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) override {
    const auto a_p = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto b_p = static_cast<autd3_complex_t*>(_pool.get(b));
    const auto c_p = static_cast<autd3_complex_t*>(_pool.get(dst));
    cudaMemcpy(c_p, a_p, a.size() * sizeof(complex), cudaMemcpyDeviceToDevice);
    cudaMemcpy(c_p + a.size(), b_p, b.size() * sizeof(complex), cudaMemcpyDeviceToDevice);
  }

  void max_eigen_vector(MatrixXc& src, VectorXc& dst) override {
    const auto size = src.cols();
    const auto src_p = static_cast<complex*>(_pool.get(src));
    const auto dst_p = static_cast<complex*>(_pool.get(dst));

    driver::float_t* d_w = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&d_w), sizeof(driver::float_t) * size);

    size_t workspace_in_bytes_on_device;
    size_t workspace_in_bytes_on_host;
    cusolverDnXsyevd_bufferSize(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, CUBLAS_FILL_MODE_UPPER, size, AUTD_C, src_p, size, AUTD_R, d_w, AUTD_C,
                                &workspace_in_bytes_on_device, &workspace_in_bytes_on_host);

    void* workspace_buffer_on_device = nullptr;
    void* workspace_buffer_on_host = nullptr;
    cudaMalloc(&workspace_buffer_on_device, workspace_in_bytes_on_device);
    if (workspace_in_bytes_on_host > 0) workspace_buffer_on_host = malloc(workspace_in_bytes_on_host);

    int* info = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&info), sizeof(int));
    cusolverDnXsyevd(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, CUBLAS_FILL_MODE_UPPER, size, AUTD_C, src_p, size, AUTD_R, d_w, AUTD_C,
                     workspace_buffer_on_device, workspace_in_bytes_on_device, workspace_buffer_on_host, workspace_in_bytes_on_host, info);
    cudaFree(d_w);
    cudaFree(info);
    cudaFree(workspace_buffer_on_device);
    free(workspace_buffer_on_host);

    cudaMemcpy(dst_p, src_p + size * (size - 1), size * sizeof(complex), cudaMemcpyDeviceToDevice);
  }

  void pseudo_inverse_svd(MatrixXc& src, const driver::float_t alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override {
    const auto nc = src.cols();
    const auto nr = src.rows();

    const auto m = static_cast<int>(nr);
    const auto n = static_cast<int>(nc);

    const auto src_p = static_cast<complex*>(_pool.get(src));
    const auto u_p = static_cast<complex*>(_pool.get(u));
    const auto v_p = static_cast<complex*>(_pool.get(vt));
    const auto s_p = static_cast<autd3_complex_t*>(_pool.get(s));

    const auto lda = m;
    const auto ldu = m;
    const auto ldv = n;

    const auto s_size = std::min(nr, nc);
    driver::float_t* d_s = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&d_s), sizeof(driver::float_t) * s_size);

    size_t workspace_in_bytes_on_device;
    size_t workspace_in_bytes_on_host;

    cusolverDnXgesvdp_bufferSize(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, 0, m, n, AUTD_C, src_p, lda, AUTD_R, d_s, AUTD_C, u_p, ldu, AUTD_C,
                                 v_p, ldv, AUTD_C, &workspace_in_bytes_on_device, &workspace_in_bytes_on_host);
    void* workspace_buffer_on_device = nullptr;
    void* workspace_buffer_on_host = nullptr;
    cudaMalloc(&workspace_buffer_on_device, workspace_in_bytes_on_device);
    if (workspace_in_bytes_on_host > 0) workspace_buffer_on_host = malloc(workspace_in_bytes_on_host);

    int* info = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&info), sizeof(int));
    double h_err_sigma;
    cusolverDnXgesvdp(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, 0, m, n, AUTD_C, src_p, lda, AUTD_R, d_s, AUTD_C, u_p, ldu, AUTD_C, v_p, ldv,
                      AUTD_C, workspace_buffer_on_device, workspace_in_bytes_on_device, workspace_buffer_on_host, workspace_in_bytes_on_host, info,
                      &h_err_sigma);

    cu_calc_singular_inv(d_s, static_cast<uint32_t>(n), static_cast<uint32_t>(m), alpha, s_p);

    mul(Transpose::NoTrans, Transpose::ConjTrans, ONE, s, u, ZERO, buf);
    mul(Transpose::NoTrans, Transpose::NoTrans, ONE, vt, buf, ZERO, dst);

    cudaFree(d_s);
    cudaFree(info);
    cudaFree(workspace_buffer_on_device);
    free(workspace_buffer_on_host);
  }

  void pseudo_inverse_svd(MatrixXd& src, const driver::float_t alpha, MatrixXd& u, MatrixXd& s, MatrixXd& vt, MatrixXd& buf, MatrixXd& dst) override {
    const auto nc = src.cols();
    const auto nr = src.rows();

    const auto m = static_cast<int>(nr);
    const auto n = static_cast<int>(nc);

    const auto src_p = static_cast<driver::float_t*>(_pool.get(src));
    const auto u_p = static_cast<driver::float_t*>(_pool.get(u));
    const auto v_p = static_cast<driver::float_t*>(_pool.get(vt));
    const auto s_p = static_cast<driver::float_t*>(_pool.get(s));

    const auto lda = m;
    const auto ldu = m;
    const auto ldv = n;

    const auto s_size = std::min(nr, nc);
    driver::float_t* d_s = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&d_s), sizeof(driver::float_t) * s_size);

    size_t workspace_in_bytes_on_device;
    size_t workspace_in_bytes_on_host;

    cusolverDnXgesvdp_bufferSize(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, 0, m, n, AUTD_R, src_p, lda, AUTD_R, d_s, AUTD_R, u_p, ldu, AUTD_R,
                                 v_p, ldv, AUTD_R, &workspace_in_bytes_on_device, &workspace_in_bytes_on_host);
    void* workspace_buffer_on_device = nullptr;
    void* workspace_buffer_on_host = nullptr;
    cudaMalloc(&workspace_buffer_on_device, workspace_in_bytes_on_device);
    if (workspace_in_bytes_on_host > 0) workspace_buffer_on_host = malloc(workspace_in_bytes_on_host);

    int* info = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&info), sizeof(int));
    double h_err_sigma;
    cusolverDnXgesvdp(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, 0, m, n, AUTD_R, src_p, lda, AUTD_R, d_s, AUTD_R, u_p, ldu, AUTD_R, v_p, ldv,
                      AUTD_R, workspace_buffer_on_device, workspace_in_bytes_on_device, workspace_buffer_on_host, workspace_in_bytes_on_host, info,
                      &h_err_sigma);

    cu_calc_singular_inv(d_s, static_cast<uint32_t>(n), static_cast<uint32_t>(m), alpha, s_p);

    mul(Transpose::NoTrans, Transpose::Trans, 1.0, s, u, 0.0, buf);
    mul(Transpose::NoTrans, Transpose::NoTrans, 1.0, vt, buf, 0.0, dst);

    cudaFree(d_s);
    cudaFree(info);
    cudaFree(workspace_buffer_on_device);
    free(workspace_buffer_on_host);
  }

  void solvet(MatrixXd& a, VectorXd& b) override {
    const auto n = static_cast<int>(a.cols());
    const auto lda = static_cast<int>(a.rows());
    const auto ldb = static_cast<int>(b.rows());

    const auto ap = static_cast<driver::float_t*>(_pool.get(a));
    const auto bp = static_cast<driver::float_t*>(_pool.get(b));

    size_t workspace_in_bytes_on_device;
    size_t workspace_in_bytes_on_host;
    cusolverDnXpotrf_bufferSize(_handle_s, nullptr, CUBLAS_FILL_MODE_UPPER, n, AUTD_R, ap, lda, AUTD_R, &workspace_in_bytes_on_device,
                                &workspace_in_bytes_on_host);

    void* workspace_buffer_on_device = nullptr;
    void* workspace_buffer_on_host = nullptr;
    cudaMalloc(&workspace_buffer_on_device, workspace_in_bytes_on_device);
    if (workspace_in_bytes_on_host > 0) workspace_buffer_on_host = malloc(workspace_in_bytes_on_host);

    int* info = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&info), sizeof(int));
    cusolverDnXpotrf(_handle_s, nullptr, CUBLAS_FILL_MODE_UPPER, n, AUTD_R, ap, lda, AUTD_R, workspace_buffer_on_device, workspace_in_bytes_on_device,
                     workspace_buffer_on_host, workspace_in_bytes_on_host, info);
    cusolverDnXpotrs(_handle_s, nullptr, CUBLAS_FILL_MODE_UPPER, n, 1, AUTD_R, ap, lda, AUTD_R, bp, ldb, info);

    cudaFree(info);
    cudaFree(workspace_buffer_on_device);
    free(workspace_buffer_on_host);
  }

  void solveh(MatrixXc& a, VectorXc& b) override {
    const auto n = static_cast<int>(a.cols());
    const auto lda = static_cast<int>(a.rows());
    const auto ldb = static_cast<int>(b.rows());

    const auto ap = static_cast<autd3_complex_t*>(_pool.get(a));
    const auto bp = static_cast<autd3_complex_t*>(_pool.get(b));

    size_t workspace_in_bytes_on_device;
    size_t workspace_in_bytes_on_host;
    cusolverDnXpotrf_bufferSize(_handle_s, nullptr, CUBLAS_FILL_MODE_UPPER, n, AUTD_C, ap, lda, AUTD_C, &workspace_in_bytes_on_device,
                                &workspace_in_bytes_on_host);

    void* workspace_buffer_on_device = nullptr;
    void* workspace_buffer_on_host = nullptr;
    cudaMalloc(&workspace_buffer_on_device, workspace_in_bytes_on_device);
    if (workspace_in_bytes_on_host > 0) workspace_buffer_on_host = malloc(workspace_in_bytes_on_host);

    int* info = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&info), sizeof(int));
    cusolverDnXpotrf(_handle_s, nullptr, CUBLAS_FILL_MODE_UPPER, n, AUTD_C, ap, lda, AUTD_C, workspace_buffer_on_device, workspace_in_bytes_on_device,
                     workspace_buffer_on_host, workspace_in_bytes_on_host, info);
    cusolverDnXpotrs(_handle_s, nullptr, CUBLAS_FILL_MODE_UPPER, n, 1, AUTD_C, ap, lda, AUTD_C, bp, ldb, info);

    cudaFree(info);
    cudaFree(workspace_buffer_on_device);
    free(workspace_buffer_on_host);
  }

  void reduce_col(const MatrixXd& a, VectorXd& b) override {
    const auto m = static_cast<uint32_t>(a.rows());
    const auto n = static_cast<uint32_t>(a.cols());
    const auto a_p = static_cast<driver::float_t*>(_pool.get(a));
    const auto b_p = static_cast<driver::float_t*>(_pool.get(b));
    driver::float_t* buffer = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&buffer), static_cast<size_t>(m) * BLOCK_SIZE / 2 * sizeof(driver::float_t));
    cu_reduce_col(a_p, m, n, b_p, buffer);
    cudaFree(buffer);
  }

 private:
  BufferPool _pool;
  cublasHandle_t _handle = nullptr;
  cusolverDnHandle_t _handle_s = nullptr;
};

#if _MSC_VER
#pragma warning(pop)
#endif

BackendPtr CUDABackend::build() const { return std::make_shared<CUDABackendImpl>(_device_idx); }

BackendPtr CUDABackend::create(const int device_idx) { return std::make_shared<CUDABackendImpl>(device_idx); }

}  // namespace autd3::gain::holo
