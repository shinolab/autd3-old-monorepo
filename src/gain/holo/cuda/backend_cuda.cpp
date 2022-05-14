// File: backend_cuda.cpp
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3/gain/backend_cuda.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26439 26478 26495 26812)
#endif
#include <cublas_v2.h>
#include <cuda_runtime_api.h>
#include <cusolverDn.h>

#include "./kernel.h"
#if _MSC_VER
#pragma warning(pop)
#endif

#include "autd3/core/geometry/normal_transducer.hpp"

namespace autd3::gain::holo {

namespace {
cublasOperation_t convert(const TRANSPOSE trans) {
  switch (trans) {
    case TRANSPOSE::NO_TRANS:
      return CUBLAS_OP_N;
    case TRANSPOSE::CONJ_TRANS:
      return CUBLAS_OP_C;
    case TRANSPOSE::TRANS:
      return CUBLAS_OP_T;
  }
  return CUBLAS_OP_N;
}
}  // namespace

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

  void clear() {
    for (auto& [_, p] : _pool) cudaFree(p);
    _pool.clear();
  }

 private:
  std::unordered_map<std::uintptr_t, void*> _pool;
};

template <typename T>
class CUDABackendImpl final : public CUDABackend<T> {
 public:
  explicit CUDABackendImpl(const int device_idx) {
    cudaSetDevice(device_idx);
    cublasCreate_v2(&_handle);
    cusolverDnCreate(&_handle_s);
  }
  ~CUDABackendImpl() override {
    cublasDestroy_v2(_handle);
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

  void copy(const MatrixXc& src, MatrixXc& dst) override {
    const auto src_p = _pool.get(src);
    const auto dst_p = _pool.get(dst);
    cudaMemcpy(dst_p, src_p, sizeof(complex) * src.size(), cudaMemcpyDeviceToDevice);
  }

  void conj(const VectorXc& src, VectorXc& dst) override {
    const auto size = static_cast<uint32_t>(src.size());
    const auto src_p = static_cast<cuDoubleComplex*>(_pool.get(src));
    const auto dst_p = static_cast<cuDoubleComplex*>(_pool.get(dst));
    cu_conj(src_p, size, 1, dst_p);
  }

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override {
    const auto row = static_cast<uint32_t>(dst.rows());
    const auto col = static_cast<uint32_t>(dst.cols());
    const auto src_p = static_cast<cuDoubleComplex*>(_pool.get(src));
    const auto dst_p = static_cast<cuDoubleComplex*>(_pool.get(dst));
    cu_set_diagonal(src_p, row, col, dst_p);
  }

  void set(const size_t i, const complex value, VectorXc& dst) override {
    const auto dst_p = static_cast<complex*>(_pool.get(dst));
    cudaMemcpy(dst_p + i, &value, sizeof(complex), cudaMemcpyHostToDevice);
  }

  void set_row(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) override {
    const auto row = static_cast<int>(dst.rows());
    const auto src_p = static_cast<cuDoubleComplex*>(_pool.get(src));
    const auto dst_p = static_cast<cuDoubleComplex*>(_pool.get(dst));
    cublasZcopy(_handle, static_cast<int>(end - begin), src_p + begin, 1, dst_p + i + begin * row, row);
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
    return std::sqrt(tmp.cwiseAbs2().maxCoeff());
  }

  void scale(const complex value, VectorXc& dst) override {
    const auto dst_p = static_cast<complex*>(_pool.get(dst));
    cublasZscal_v2(_handle, static_cast<int>(dst.size()), reinterpret_cast<const cuDoubleComplex*>(&value), reinterpret_cast<cuDoubleComplex*>(dst_p),
                   1);
  }

  complex dot(const VectorXc& a, const VectorXc& b) override {
    complex d;
    const auto a_p = static_cast<cuDoubleComplex*>(_pool.get(a));
    const auto b_p = static_cast<cuDoubleComplex*>(_pool.get(b));
    cublasZdotc_v2(_handle, static_cast<int>(a.size()), a_p, 1, b_p, 1, reinterpret_cast<cuDoubleComplex*>(&d));
    return d;
  }

  void mul(const TRANSPOSE trans_a, const TRANSPOSE trans_b, const complex alpha, const MatrixXc& a, const MatrixXc& b, const complex beta,
           MatrixXc& c) override {
    const auto lda = static_cast<int>(a.rows());
    const auto ldb = static_cast<int>(b.rows());
    const auto ldc = trans_a == TRANSPOSE::NO_TRANS ? static_cast<int>(a.rows()) : static_cast<int>(a.cols());
    const auto n = trans_b == TRANSPOSE::NO_TRANS ? static_cast<int>(b.cols()) : static_cast<int>(b.rows());
    const auto k = trans_a == TRANSPOSE::NO_TRANS ? static_cast<int>(a.cols()) : static_cast<int>(a.rows());
    const auto a_p = static_cast<cuDoubleComplex*>(_pool.get(a));
    const auto b_p = static_cast<cuDoubleComplex*>(_pool.get(b));
    const auto c_p = static_cast<cuDoubleComplex*>(_pool.get(c));
    cublasZgemm_v2(_handle, convert(trans_a), convert(trans_b), ldc, n, k, reinterpret_cast<const cuDoubleComplex*>(&alpha), a_p, lda, b_p, ldb,
                   reinterpret_cast<const cuDoubleComplex*>(&beta), c_p, ldc);
  }

  void mul(const TRANSPOSE trans_a, const complex alpha, const MatrixXc& a, const VectorXc& b, const complex beta, VectorXc& c) override {
    const auto lda = static_cast<int>(a.rows());
    const auto ldc = trans_a == TRANSPOSE::NO_TRANS ? static_cast<int>(a.rows()) : static_cast<int>(a.cols());
    const auto n = static_cast<int>(b.size());
    const auto a_p = static_cast<cuDoubleComplex*>(_pool.get(a));
    const auto b_p = static_cast<cuDoubleComplex*>(_pool.get(b));
    const auto c_p = static_cast<cuDoubleComplex*>(_pool.get(c));
    cublasZgemv(_handle, convert(trans_a), ldc, n, reinterpret_cast<const cuDoubleComplex*>(&alpha), a_p, lda, b_p, 1,
                reinterpret_cast<const cuDoubleComplex*>(&beta), c_p, 1);
  }

  void max_eigen_vector(const MatrixXc& src, VectorXc& dst) override {
    const auto size = src.cols();
    const auto src_p = static_cast<complex*>(_pool.get(src));
    const auto dst_p = static_cast<complex*>(_pool.get(dst));

    double* d_w = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&d_w), sizeof(double) * size);

    size_t workspace_in_bytes_on_device;
    size_t workspace_in_bytes_on_host;
    cusolverDnXsyevd_bufferSize(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, CUBLAS_FILL_MODE_UPPER, size, CUDA_C_64F, src_p, size, CUDA_R_64F, d_w,
                                CUDA_C_64F, &workspace_in_bytes_on_device, &workspace_in_bytes_on_host);

    void* workspace_buffer_on_device = nullptr;
    void* workspace_buffer_on_host = nullptr;
    cudaMalloc(&workspace_buffer_on_device, workspace_in_bytes_on_device);
    if (workspace_in_bytes_on_host > 0) workspace_buffer_on_host = malloc(workspace_in_bytes_on_host);

    int* info = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&info), sizeof(int));
    cusolverDnXsyevd(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, CUBLAS_FILL_MODE_UPPER, size, CUDA_C_64F, src_p, size, CUDA_R_64F, d_w, CUDA_C_64F,
                     workspace_buffer_on_device, workspace_in_bytes_on_device, workspace_buffer_on_host, workspace_in_bytes_on_host, info);
    cudaFree(d_w);
    cudaFree(info);
    cudaFree(workspace_buffer_on_device);
    free(workspace_buffer_on_host);

    cudaMemcpy(dst_p, src_p + size * (size - 1), size * sizeof(complex), cudaMemcpyDeviceToDevice);
  }

  void pseudo_inverse_svd(const MatrixXc& src, const double alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override {
    const auto nc = src.cols();
    const auto nr = src.rows();
    const auto src_p = static_cast<complex*>(_pool.get(src));
    const auto u_p = static_cast<complex*>(_pool.get(u));
    const auto v_p = static_cast<complex*>(_pool.get(vt));
    const auto s_p = static_cast<complex*>(_pool.get(s));

    const auto lda = static_cast<int>(nr);
    const auto ldu = static_cast<int>(nr);
    const auto ldv = static_cast<int>(nc);

    const auto s_size = std::min(nr, nc);
    double* d_s = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&d_s), sizeof(double) * s_size);

    size_t workspace_in_bytes_on_device;
    size_t workspace_in_bytes_on_host;

    cusolverDnXgesvdp_bufferSize(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, 0, static_cast<int>(nr), static_cast<int>(nc), CUDA_C_64F, src_p, lda,
                                 CUDA_R_64F, d_s, CUDA_C_64F, u_p, ldu, CUDA_C_64F, v_p, ldv, CUDA_C_64F, &workspace_in_bytes_on_device,
                                 &workspace_in_bytes_on_host);
    void* workspace_buffer_on_device = nullptr;
    void* workspace_buffer_on_host = nullptr;
    cudaMalloc(&workspace_buffer_on_device, workspace_in_bytes_on_device);
    if (workspace_in_bytes_on_host > 0) workspace_buffer_on_host = malloc(workspace_in_bytes_on_host);

    int* info = nullptr;
    cudaMalloc(reinterpret_cast<void**>(&info), sizeof(int));
    double h_err_sigma;
    cusolverDnXgesvdp(_handle_s, nullptr, CUSOLVER_EIG_MODE_VECTOR, 0, static_cast<int>(nr), static_cast<int>(nc), CUDA_C_64F, src_p, lda, CUDA_R_64F,
                      d_s, CUDA_C_64F, u_p, ldu, CUDA_C_64F, v_p, ldv, CUDA_C_64F, workspace_buffer_on_device, workspace_in_bytes_on_device,
                      workspace_buffer_on_host, workspace_in_bytes_on_host, info, &h_err_sigma);

    cu_calc_singular_inv(d_s, static_cast<uint32_t>(s_size), alpha, reinterpret_cast<cuDoubleComplex*>(s_p));

    mul(TRANSPOSE::NO_TRANS, TRANSPOSE::CONJ_TRANS, ONE, s, u, ZERO, buf);
    mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, vt, buf, ZERO, dst);
    cudaFree(d_s);
    cudaFree(info);
    cudaFree(workspace_buffer_on_device);
    free(workspace_buffer_on_host);
  }

  void generate_transfer_matrix(const std::vector<core::Vector3>& foci, const core::Geometry<T>& geometry, MatrixXc& dst) override {
    // FIXME: Implement with CUDA
    for (size_t i = 0; i < foci.size(); i++)
      for (const auto& dev : geometry)
        for (const auto& tr : dev)
          dst(i, tr.id()) = core::propagate(tr.position(), tr.z_direction(), geometry.attenuation, tr.wavenumber(geometry.sound_speed), foci[i]);
  }

 private:
  BufferPool _pool;
  cublasHandle_t _handle = nullptr;
  cusolverDnHandle_t _handle_s = nullptr;
};

template <>
BackendPtr<core::LegacyTransducer> CUDABackend<core::LegacyTransducer>::create(const int device_idx) {
  return std::make_shared<CUDABackendImpl<core::LegacyTransducer>>(device_idx);
}

template <>
BackendPtr<core::NormalTransducer> CUDABackend<core::NormalTransducer>::create(const int device_idx) {
  return std::make_shared<CUDABackendImpl<core::NormalTransducer>>(device_idx);
}

}  // namespace autd3::gain::holo
