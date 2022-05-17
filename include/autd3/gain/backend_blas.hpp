// File: backend_blas.hpp
// Project: gain
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "autd3/gain/backend.hpp"

namespace autd3::gain::holo {

/**
 * \brief Backend for Holo using Eigen
 */
class BLASBackend final : public Backend {
 public:
  BLASBackend() = default;
  ~BLASBackend() override = default;
  BLASBackend(const BLASBackend& v) = default;
  BLASBackend& operator=(const BLASBackend& obj) = default;
  BLASBackend(BLASBackend&& obj) = default;
  BLASBackend& operator=(BLASBackend&& obj) = default;

  void init() override;
  void to_host(VectorXc& dst) override;
  void to_host(MatrixXc& dst) override;
  void to_host(VectorXd& dst) override;
  void to_host(MatrixXd& dst) override;

  void copy_to(const MatrixXc& src, MatrixXc& dst) override;
  void copy_to(const MatrixXd& src, MatrixXd& dst) override;
  void copy_to(const VectorXd& src, VectorXd& dst) override;

  void real(const MatrixXc& src, MatrixXd& re) override;
  void imag(const MatrixXc& src, MatrixXd& im) override;
  void make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) override;

  void abs(const VectorXc& src, VectorXd& dst) override;
  void abs(const VectorXc& src, VectorXc& dst) override;
  void sqrt(const VectorXd& src, VectorXd& dst) override;
  void conj(const VectorXc& src, VectorXc& dst) override;
  void arg(const VectorXc& src, VectorXc& dst) override;
  void reciprocal(const VectorXc& src, VectorXc& dst) override;
  void exp(const VectorXc& src, VectorXc& dst) override;
  void pow(const VectorXd& src, double p, VectorXd& dst) override;

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override;
  void get_diagonal(const MatrixXc& src, VectorXc& dst) override;
  void get_diagonal(const MatrixXd& src, VectorXd& dst) override;

  void set(size_t i, complex value, VectorXc& dst) override;
  void set_row(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override;
  void set_col(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) override;

  void get_col(const MatrixXc& src, size_t i, VectorXc& dst) override;

  void concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override;
  void concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override;
  void concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) override;

  void reduce_col(const MatrixXd& src, VectorXd& dst) override;

  complex max_abs_element(const VectorXc& src) override;
  double max_element(const VectorXd& src) override;

  void scale(complex value, VectorXc& dst) override;
  void scale(double value, VectorXd& dst) override;

  complex dot(const VectorXc& a, const VectorXc& b) override;
  double dot(const VectorXd& a, const VectorXd& b) override;

  void add(double alpha, const MatrixXd& a, MatrixXd& b) override;
  void add(double alpha, const VectorXd& a, VectorXd& b) override;

  void mul(TRANSPOSE trans_a, TRANSPOSE trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) override;
  void mul(TRANSPOSE trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) override;
  void mul(TRANSPOSE trans_a, TRANSPOSE trans_b, double alpha, const MatrixXd& a, const MatrixXd& b, double beta, MatrixXd& c) override;
  void mul(TRANSPOSE trans_a, double alpha, const MatrixXd& a, const VectorXd& b, double beta, VectorXd& c) override;
  void hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) override;
  void hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) override;

  void solvet(MatrixXd& a, VectorXd& b) override;
  void solveh(MatrixXc& a, VectorXc& b) override;

  void max_eigen_vector(MatrixXc& src, VectorXc& dst) override;

  void pseudo_inverse_svd(MatrixXd& src, double alpha, MatrixXd& u, MatrixXd& s, MatrixXd& vt, MatrixXd& buf, MatrixXd& dst) override;
  void pseudo_inverse_svd(MatrixXc& src, double alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override;

  static BackendPtr create();
};

}  // namespace autd3::gain::holo
