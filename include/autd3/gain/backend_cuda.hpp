// File: backend.hpp
// Project: gain
// Created Date: 10/09/2021
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include <vector>

#include "backend.hpp"

namespace autd3::gain::holo {

/**
 * \brief Backend for HoloGain
 */
class CUDABackend : public Backend {
 public:
  CUDABackend() = default;
  ~CUDABackend() override = default;
  CUDABackend(const CUDABackend& v) noexcept = default;
  CUDABackend& operator=(const CUDABackend& obj) = default;
  CUDABackend(CUDABackend&& obj) = default;
  CUDABackend& operator=(CUDABackend&& obj) = default;

  void init() override = 0;
  void to_host(VectorXc&) override = 0;
  void to_host(MatrixXc&) override = 0;

  void copy_to(const MatrixXc& src, MatrixXc& dst) override = 0;

  void conj(const VectorXc& src, VectorXc& dst) override = 0;

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override = 0;

  void set(size_t i, complex value, VectorXc& dst) override = 0;
  void set_row(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override = 0;
  void set_col(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override = 0;

  void get_col(const MatrixXc& src, size_t i, VectorXc& dst) override = 0;

  complex max_abs_element(const VectorXc& src) override = 0;

  void scale(complex value, VectorXc& dst) override = 0;

  complex dot(const VectorXc& a, const VectorXc& b) override = 0;

  void mul(TRANSPOSE trans_a, TRANSPOSE trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) override = 0;
  void mul(TRANSPOSE trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) override = 0;

  void max_eigen_vector(const MatrixXc& src, VectorXc& dst) override = 0;

  void pseudo_inverse_svd(MatrixXc& src, double alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override = 0;

  static BackendPtr create(int device_idx = 0);
};

}  // namespace autd3::gain::holo
