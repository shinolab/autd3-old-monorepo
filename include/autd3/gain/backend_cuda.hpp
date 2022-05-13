// File: backend.hpp
// Project: gain
// Created Date: 10/09/2021
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include "backend.hpp"

namespace autd3::gain::holo {

/**
 * \brief Backend for HoloGain
 */
class CUDABackend final : public Backend {
 public:
  CUDABackend(int device_idx = 0);
  ~CUDABackend() override = default;
  CUDABackend(const CUDABackend& v) noexcept = default;
  CUDABackend& operator=(const CUDABackend& obj) = default;
  CUDABackend(CUDABackend&& obj) = default;
  CUDABackend& operator=(CUDABackend&& obj) = default;

  void copy(const MatrixXc& src, MatrixXc& dst) override;

  void conj(const VectorXc& src, VectorXc& dst) override;

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override;

  void set(size_t i, complex value, VectorXc& dst) override;
  void set_row(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override;
  void set_col(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override;

  void get_col(const MatrixXc& src, size_t i, VectorXc& dst) override;

  complex max_abs_element(const VectorXc& src) override;

  void scale(complex value, VectorXc& dst) override;

  complex dot(const VectorXc& a, const VectorXc& b) override;

  void mul(TRANSPOSE trans_a, TRANSPOSE trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) override;
  void mul(TRANSPOSE trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) override;

  void max_eigen_vector(const MatrixXc& src, VectorXc& dst) override;

  void pseudo_inverse_svd(const MatrixXc& src, double alpha, const MatrixXc& u, const MatrixXc& s, const MatrixXc& vt, const MatrixXc& buf,
                          MatrixXc& dst) override;

  void generate_transfer_matrix(const std::vector<core::Vector3>& foci, const std::vector<core::Vector3>& transducers, MatrixXc& dst) override;

  static BackendPtr create() { return std::make_shared<CUDABackend>(); }
};

}  // namespace autd3::gain::holo
