// File: backend_cuda.cpp
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3/gain/backend_cuda.hpp"

#include <cuda_runtime_api.h>

#include "./kernel.h"

namespace autd3::gain::holo {

CUDABackend::CUDABackend(const int device_idx) { cudaSetDevice(device_idx); }

void CUDABackend::copy(const MatrixXc& src, MatrixXc& dst) {}

void CUDABackend::scale(complex value, VectorXc& dst) {}

complex CUDABackend::dot(const VectorXc& a, const VectorXc& b) { return a.dot(b); }

void CUDABackend::mul(TRANSPOSE trans_a, TRANSPOSE trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) {}

void CUDABackend::mul(TRANSPOSE trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) {}

void CUDABackend::max_eigen_vector(const MatrixXc& src, VectorXc& dst) {}

void CUDABackend::pseudo_inverse_svd(const MatrixXc& src, double alpha, const MatrixXc& u, const MatrixXc& s, const MatrixXc& vt, const MatrixXc& buf,
                                     MatrixXc& dst) {}

void CUDABackend::generate_transfer_matrix(const std::vector<core::Vector3>& foci, const std::vector<core::Vector3>& transducers, MatrixXc& dst) {}

void CUDABackend::conj(const VectorXc& src, VectorXc& dst) {}

void CUDABackend::create_diagonal(const VectorXc& src, MatrixXc& dst) {
  dst.fill(ZERO);
  dst.diagonal() = src;
}

void CUDABackend::set(size_t i, complex value, VectorXc& dst) {}

void CUDABackend::set_row(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) {}

void CUDABackend::set_col(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) {}

void CUDABackend::get_col(const MatrixXc& src, size_t i, VectorXc& dst) {}

complex CUDABackend::max_abs_element(const VectorXc& src) { return std::sqrt(src.cwiseAbs2().maxCoeff()); }

}  // namespace autd3::gain::holo
