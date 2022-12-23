// File: backend.hpp
// Project: gain
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/driver/defined.hpp"

namespace autd3::gain::holo {

enum class Transpose { NoTrans = 111, Trans = 112, ConjTrans = 113 };

using complex = std::complex<driver::autd3_float_t>;

constexpr complex ONE = complex(1, 0);
constexpr complex ZERO = complex(0, 0);

using VectorXd = Eigen::Vector<driver::autd3_float_t, -1>;
using VectorXc = Eigen::Vector<complex, -1>;
using MatrixXd = Eigen::Matrix<driver::autd3_float_t, -1, -1, Eigen::ColMajor>;
using MatrixXc = Eigen::Matrix<complex, -1, -1, Eigen::ColMajor>;

/**
 * \brief Backend for Holo
 */
class Backend {
 public:
  Backend() = default;
  virtual ~Backend() = default;
  Backend(const Backend& v) noexcept = default;
  Backend& operator=(const Backend& obj) = default;
  Backend(Backend&& obj) = default;
  Backend& operator=(Backend&& obj) = default;

  virtual void init() = 0;
  virtual void to_host(VectorXc& dst) = 0;
  virtual void to_host(MatrixXc& dst) = 0;
  virtual void to_host(VectorXd& dst) = 0;
  virtual void to_host(MatrixXd& dst) = 0;

  virtual void copy_to(const MatrixXc& src, MatrixXc& dst) = 0;
  virtual void copy_to(const MatrixXd& src, MatrixXd& dst) = 0;
  virtual void copy_to(const VectorXd& src, VectorXd& dst) = 0;
  virtual void copy_to(const VectorXc& src, VectorXc& dst) = 0;

  virtual void real(const MatrixXc& src, MatrixXd& re) = 0;
  virtual void imag(const MatrixXc& src, MatrixXd& im) = 0;
  virtual void make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) = 0;

  virtual void abs(const VectorXc& src, VectorXd& dst) = 0;
  virtual void abs(const VectorXc& src, VectorXc& dst) = 0;
  virtual void sqrt(const VectorXd& src, VectorXd& dst) = 0;
  virtual void conj(const VectorXc& src, VectorXc& dst) = 0;
  virtual void arg(const VectorXc& src, VectorXc& dst) = 0;
  virtual void reciprocal(const VectorXc& src, VectorXc& dst) = 0;
  virtual void exp(const VectorXc& src, VectorXc& dst) = 0;
  virtual void pow(const VectorXd& src, driver::autd3_float_t p, VectorXd& dst) = 0;

  virtual void create_diagonal(const VectorXc& src, MatrixXc& dst) = 0;
  virtual void get_diagonal(const MatrixXc& src, VectorXc& dst) = 0;
  virtual void get_diagonal(const MatrixXd& src, VectorXd& dst) = 0;

  virtual void set(size_t i, complex value, VectorXc& dst) = 0;
  virtual void set_row(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) = 0;
  virtual void set_col(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) = 0;

  virtual void get_col(const MatrixXc& src, size_t i, VectorXc& dst) = 0;

  virtual void concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) = 0;
  virtual void concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) = 0;
  virtual void concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) = 0;

  virtual void reduce_col(const MatrixXd& src, VectorXd& dst) = 0;

  virtual complex max_abs_element(const VectorXc& src) = 0;
  virtual driver::autd3_float_t max_element(const VectorXd& src) = 0;

  virtual void scale(complex value, VectorXc& dst) = 0;
  virtual void scale(driver::autd3_float_t value, VectorXd& dst) = 0;

  virtual complex dot(const VectorXc& a, const VectorXc& b) = 0;
  virtual driver::autd3_float_t dot(const VectorXd& a, const VectorXd& b) = 0;

  virtual void add(driver::autd3_float_t alpha, const MatrixXd& a, MatrixXd& b) = 0;
  virtual void add(complex alpha, const MatrixXc& a, MatrixXc& b) = 0;
  virtual void add(driver::autd3_float_t alpha, const VectorXd& a, VectorXd& b) = 0;
  virtual void add(complex alpha, const VectorXc& a, VectorXc& b) = 0;

  virtual void mul(Transpose trans_a, Transpose trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) = 0;
  virtual void mul(Transpose trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) = 0;
  virtual void mul(Transpose trans_a, Transpose trans_b, driver::autd3_float_t alpha, const MatrixXd& a, const MatrixXd& b,
                   driver::autd3_float_t beta, MatrixXd& c) = 0;
  virtual void mul(Transpose trans_a, driver::autd3_float_t alpha, const MatrixXd& a, const VectorXd& b, driver::autd3_float_t beta, VectorXd& c) = 0;
  virtual void hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) = 0;
  virtual void hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) = 0;

  virtual void solvet(MatrixXd& a, VectorXd& b) = 0;
  virtual void solveh(MatrixXc& a, VectorXc& b) = 0;

  virtual void max_eigen_vector(MatrixXc& src, VectorXc& dst) = 0;

  virtual void pseudo_inverse_svd(MatrixXd& src, driver::autd3_float_t alpha, MatrixXd& u, MatrixXd& s, MatrixXd& vt, MatrixXd& buf,
                                  MatrixXd& dst) = 0;
  virtual void pseudo_inverse_svd(MatrixXc& src, driver::autd3_float_t alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf,
                                  MatrixXc& dst) = 0;
};

using BackendPtr = std::shared_ptr<Backend>;

/**
 * \brief Backend for Holo using Eigen
 */
class EigenBackend final : public Backend {
 public:
  EigenBackend() = default;
  ~EigenBackend() override = default;
  EigenBackend(const EigenBackend& v) = default;
  EigenBackend& operator=(const EigenBackend& obj) = default;
  EigenBackend(EigenBackend&& obj) = default;
  EigenBackend& operator=(EigenBackend&& obj) = default;

  void init() override;
  void to_host(VectorXc& dst) override;
  void to_host(MatrixXc& dst) override;
  void to_host(VectorXd& dst) override;
  void to_host(MatrixXd& dst) override;

  void copy_to(const MatrixXc& src, MatrixXc& dst) override;
  void copy_to(const MatrixXd& src, MatrixXd& dst) override;
  void copy_to(const VectorXd& src, VectorXd& dst) override;
  void copy_to(const VectorXc& src, VectorXc& dst) override;

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
  void pow(const VectorXd& src, driver::autd3_float_t p, VectorXd& dst) override;

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
  driver::autd3_float_t max_element(const VectorXd& src) override;

  void scale(complex value, VectorXc& dst) override;
  void scale(driver::autd3_float_t value, VectorXd& dst) override;

  complex dot(const VectorXc& a, const VectorXc& b) override;
  driver::autd3_float_t dot(const VectorXd& a, const VectorXd& b) override;

  void add(driver::autd3_float_t alpha, const MatrixXd& a, MatrixXd& b) override;
  void add(complex alpha, const MatrixXc& a, MatrixXc& b) override;
  void add(driver::autd3_float_t alpha, const VectorXd& a, VectorXd& b) override;
  void add(complex alpha, const VectorXc& a, VectorXc& b) override;

  void mul(Transpose trans_a, Transpose trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) override;
  void mul(Transpose trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) override;
  void mul(Transpose trans_a, Transpose trans_b, driver::autd3_float_t alpha, const MatrixXd& a, const MatrixXd& b, driver::autd3_float_t beta,
           MatrixXd& c) override;
  void mul(Transpose trans_a, driver::autd3_float_t alpha, const MatrixXd& a, const VectorXd& b, driver::autd3_float_t beta, VectorXd& c) override;
  void hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) override;
  void hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) override;

  void solvet(MatrixXd& a, VectorXd& b) override;
  void solveh(MatrixXc& a, VectorXc& b) override;

  void max_eigen_vector(MatrixXc& src, VectorXc& dst) override;

  void pseudo_inverse_svd(MatrixXd& src, driver::autd3_float_t alpha, MatrixXd& u, MatrixXd& s, MatrixXd& vt, MatrixXd& buf, MatrixXd& dst) override;
  void pseudo_inverse_svd(MatrixXc& src, driver::autd3_float_t alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override;

  static BackendPtr create();
};

}  // namespace autd3::gain::holo
