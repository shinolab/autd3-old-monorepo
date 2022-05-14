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

#include <memory>

#include "autd3/core/geometry/geometry.hpp"

namespace autd3::gain::holo {

enum class TRANSPOSE { NO_TRANS = 111, TRANS = 112, CONJ_TRANS = 113 };

using complex = std::complex<double>;

constexpr complex ONE = complex(1.0, 0.0);
constexpr complex ZERO = complex(0.0, 0.0);

using VectorXd = Eigen::Vector<double, -1>;
using VectorXc = Eigen::Vector<complex, -1>;
using MatrixXd = Eigen::Matrix<double, -1, -1, Eigen::ColMajor>;
using MatrixXc = Eigen::Matrix<complex, -1, -1, Eigen::ColMajor>;

/**
 * \brief Backend for HoloGain
 */
template <typename T, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Backend {
 public:
  Backend() = default;
  virtual ~Backend() = default;
  Backend(const Backend& v) noexcept = default;
  Backend& operator=(const Backend& obj) = default;
  Backend(Backend&& obj) = default;
  Backend& operator=(Backend&& obj) = default;

  virtual void copy(const MatrixXc& src, MatrixXc& dst) = 0;

  virtual void conj(const VectorXc& src, VectorXc& dst) = 0;

  virtual void create_diagonal(const VectorXc& src, MatrixXc& dst) = 0;

  virtual void set(size_t i, complex value, VectorXc& dst) = 0;
  virtual void set_row(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) = 0;
  virtual void set_col(VectorXc& src, size_t i, size_t begin, size_t end, MatrixXc& dst) = 0;

  virtual void get_col(const MatrixXc& src, size_t i, VectorXc& dst) = 0;

  virtual complex max_abs_element(const VectorXc& src) = 0;

  virtual void scale(complex value, VectorXc& dst) = 0;

  virtual complex dot(const VectorXc& a, const VectorXc& b) = 0;

  virtual void mul(TRANSPOSE trans_a, TRANSPOSE trans_b, complex alpha, const MatrixXc& a, const MatrixXc& b, complex beta, MatrixXc& c) = 0;
  virtual void mul(TRANSPOSE trans_a, complex alpha, const MatrixXc& a, const VectorXc& b, complex beta, VectorXc& c) = 0;

  virtual void max_eigen_vector(const MatrixXc& src, VectorXc& dst) = 0;

  virtual void pseudo_inverse_svd(const MatrixXc& src, double alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) = 0;

  virtual void generate_transfer_matrix(const std::vector<core::Vector3>& foci, const core::Geometry<T>& geometry, MatrixXc& dst) = 0;
};

template <typename T, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
using BackendPtr = std::shared_ptr<Backend<T>>;

/**
 * \brief Backend for HoloGain
 */
template <typename T, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class EigenBackend : public Backend<T> {
 public:
  EigenBackend() = default;
  ~EigenBackend() override = default;
  EigenBackend(const EigenBackend& v) = default;
  EigenBackend& operator=(const EigenBackend& obj) = default;
  EigenBackend(EigenBackend&& obj) = default;
  EigenBackend& operator=(EigenBackend&& obj) = default;

  void copy(const MatrixXc& src, MatrixXc& dst) override = 0;

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

  void pseudo_inverse_svd(const MatrixXc& src, double alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override = 0;

  void generate_transfer_matrix(const std::vector<core::Vector3>& foci, const core::Geometry<T>& geometry, MatrixXc& dst) override = 0;

  static BackendPtr<T> create();
};

}  // namespace autd3::gain::holo
