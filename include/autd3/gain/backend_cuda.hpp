// File: backend_cuda.hpp
// Project: gain
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "backend.hpp"

namespace autd3::gain::holo {

/**
 * \brief Backend for Holo using CUDA
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

  void copy_to(const MatrixXd& src, MatrixXd& dst) override = 0;
  void copy_to(const MatrixXc& src, MatrixXc& dst) override = 0;
  void copy_to(const VectorXd& src, VectorXd& dst) override = 0;
  void copy_to(const VectorXc& src, VectorXc& dst) override = 0;

  void abs(const VectorXc& src, VectorXc& dst) override = 0;
  void conj(const VectorXc& src, VectorXc& dst) override = 0;
  void arg(const VectorXc& src, VectorXc& dst) override = 0;
  void reciprocal(const VectorXc& src, VectorXc& dst) override = 0;

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override = 0;
  void get_diagonal(const MatrixXc& src, VectorXc& dst) override = 0;

  void set(size_t i, complex value, VectorXc& dst) override = 0;
  void set_row(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override = 0;
  void set_col(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override = 0;

  void get_col(const MatrixXc& src, size_t i, VectorXc& dst) override = 0;

  complex max_abs_element(const VectorXc& src) override = 0;

  void scale(complex value, VectorXc& dst) override = 0;

  complex dot(const VectorXc& a, const VectorXc& b) override = 0;

  void mul(TRANSPOSE trans_a, TRANSPOSE trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) override = 0;
  void mul(TRANSPOSE trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) override = 0;
  void hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) override = 0;

  void max_eigen_vector(MatrixXc& src, VectorXc& dst) override = 0;

  void pseudo_inverse_svd(MatrixXc& src, double alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override = 0;

  void to_host(VectorXd& dst) override = 0;
  void to_host(MatrixXd& dst) override = 0;
  void real(const MatrixXc& src, MatrixXd& re) override = 0;
  void imag(const MatrixXc& src, MatrixXd& im) override = 0;
  void make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) override = 0;
  void abs(const VectorXc& src, VectorXd& dst) override = 0;
  void sqrt(const VectorXd& src, VectorXd& dst) override = 0;
  void exp(const VectorXc& src, VectorXc& dst) override = 0;
  void pow(const VectorXd& src, double p, VectorXd& dst) override = 0;
  void get_diagonal(const MatrixXd& src, VectorXd& dst) override = 0;
  void concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override = 0;
  void concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override = 0;
  void concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) override = 0;
  void reduce_col(const MatrixXd& src, VectorXd& dst) override = 0;
  double max_element(const VectorXd& src) override = 0;
  void scale(double value, VectorXd& dst) override = 0;
  double dot(const VectorXd& a, const VectorXd& b) override = 0;

  void add(double alpha, const MatrixXd& a, MatrixXd& b) override = 0;
  void add(double alpha, const VectorXd& a, VectorXd& b) override = 0;
  void add(complex alpha, const MatrixXc& a, MatrixXc& b) override = 0;
  void add(complex alpha, const VectorXc& a, VectorXc& b) override = 0;

  void mul(TRANSPOSE trans_a, TRANSPOSE trans_b, double alpha, const MatrixXd& a, const MatrixXd& b, double beta, MatrixXd& c) override = 0;
  void mul(TRANSPOSE trans_a, double alpha, const MatrixXd& a, const VectorXd& b, double beta, VectorXd& c) override = 0;
  void hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) override = 0;
  void solvet(MatrixXd& a, VectorXd& b) override = 0;
  void solveh(MatrixXc& a, VectorXc& b) override = 0;
  void pseudo_inverse_svd(MatrixXd& src, double alpha, MatrixXd& u, MatrixXd& s, MatrixXd& vt, MatrixXd& buf, MatrixXd& dst) override = 0;

  static BackendPtr create(int device_idx = 0);
};

}  // namespace autd3::gain::holo
