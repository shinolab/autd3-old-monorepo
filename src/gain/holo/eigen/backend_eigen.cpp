// File: backend_eigen.cpp
// Project: eigen
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <Eigen/Dense>

#include "autd3/gain/backend.hpp"

namespace autd3::gain::holo {

class EigenBackendImpl final : public Backend {
 public:
  EigenBackendImpl() = default;
  ~EigenBackendImpl() override = default;
  EigenBackendImpl(const EigenBackendImpl& v) = default;
  EigenBackendImpl& operator=(const EigenBackendImpl& obj) = default;
  EigenBackendImpl(EigenBackendImpl&& obj) = default;
  EigenBackendImpl& operator=(EigenBackendImpl&& obj) = default;

  void init() override {}
  void to_host(VectorXc&) override {}
  void to_host(MatrixXc&) override {}
  void to_host(VectorXd&) override {}
  void to_host(MatrixXd&) override {}

  void copy_to(const MatrixXc& src, MatrixXc& dst) override { dst = src; }
  void copy_to(const MatrixXd& src, MatrixXd& dst) override { dst = src; }
  void copy_to(const VectorXd& src, VectorXd& dst) override { dst = src; }
  void copy_to(const VectorXc& src, VectorXc& dst) override { dst = src; }

  void real(const MatrixXc& src, MatrixXd& re) override { re = src.real(); }
  void imag(const MatrixXc& src, MatrixXd& im) override { im = src.imag(); }
  void make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) override {
    dst.real() = re;
    dst.imag() = im;
  }

  void abs(const VectorXc& src, VectorXd& dst) override { dst = src.cwiseAbs(); }
  void abs(const VectorXc& src, VectorXc& dst) override { dst = src.cwiseAbs(); }
  void sqrt(const VectorXd& src, VectorXd& dst) override { dst = src.cwiseSqrt(); }
  void conj(const VectorXc& src, VectorXc& dst) override { dst = src.conjugate(); }
  void arg(const VectorXc& src, VectorXc& dst) override { dst = src.cwiseQuotient(src.cwiseAbs()); }
  void reciprocal(const VectorXc& src, VectorXc& dst) override { dst = src.cwiseInverse(); }
  void exp(const VectorXc& src, VectorXc& dst) override { dst = src.array().exp(); }
  void pow(const VectorXd& src, const driver::float_t p, VectorXd& dst) override { dst = src.array().pow(p); }

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override {
    dst.fill(ZERO);
    dst.diagonal() = src;
  }
  void get_diagonal(const MatrixXc& src, VectorXc& dst) override { dst = src.diagonal(); }
  void get_diagonal(const MatrixXd& src, VectorXd& dst) override { dst = src.diagonal(); }

  void set(const size_t i, const complex value, VectorXc& dst) override { dst(static_cast<Eigen::Index>(i)) = value; }
  void set_row(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) override {
    dst.block(static_cast<Eigen::Index>(i), static_cast<Eigen::Index>(begin), 1, end - begin) =
        src.block(static_cast<Eigen::Index>(begin), 0, end - begin, 1).transpose();
  }
  void set_col(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) override {
    dst.block(static_cast<Eigen::Index>(begin), static_cast<Eigen::Index>(i), end - begin, 1) =
        src.block(static_cast<Eigen::Index>(begin), 0, end - begin, 1);
  }

  void get_col(const MatrixXc& src, const size_t i, VectorXc& dst) override { dst = src.col(static_cast<Eigen::Index>(i)); }

  void concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override { dst << a, b; }
  void concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override { dst << a, b; }

  void concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) override { dst << a, b; }

  void reduce_col(const MatrixXd& src, VectorXd& dst) override { dst = src.rowwise().sum(); }

  complex max_abs_element(const VectorXc& src) override {
    Eigen::Index idx = 0;
    src.cwiseAbs2().maxCoeff(&idx);
    return src(idx);
  }

  driver::float_t max_element(const VectorXd& src) override { return src.maxCoeff(); }

  void scale(const complex value, VectorXc& dst) override { dst *= value; }
  void scale(const driver::float_t value, VectorXd& dst) override { dst *= value; }

  complex dot(const VectorXc& a, const VectorXc& b) override { return a.dot(b); }
  driver::float_t dot(const VectorXd& a, const VectorXd& b) override { return a.dot(b); }

  void add(const driver::float_t alpha, const MatrixXd& a, MatrixXd& b) override { b += alpha * a; }
  void add(const complex alpha, const MatrixXc& a, MatrixXc& b) override { b += alpha * a; }
  void add(const driver::float_t alpha, const VectorXd& a, VectorXd& b) override { b += alpha * a; }
  void add(const complex alpha, const VectorXc& a, VectorXc& b) override { b += alpha * a; }

  void mul(const Transpose trans_a, const Transpose trans_b, const complex alpha, const MatrixXc& a, const MatrixXc& b, const complex beta,
           MatrixXc& c) override {
    c *= beta;
    switch (trans_a) {
      case Transpose::ConjTrans:
        switch (trans_b) {
          case Transpose::ConjTrans:
            c.noalias() += alpha * (a.adjoint() * b.adjoint());
            break;
          case Transpose::Trans:
            c.noalias() += alpha * (a.adjoint() * b.transpose());
            break;
          case Transpose::NoTrans:
            c.noalias() += alpha * (a.adjoint() * b);
            break;
        }
        break;
      case Transpose::Trans:
        switch (trans_b) {
          case Transpose::ConjTrans:
            c.noalias() += alpha * (a.transpose() * b.adjoint());
            break;
          case Transpose::Trans:
            c.noalias() += alpha * (a.transpose() * b.transpose());
            break;
          case Transpose::NoTrans:
            c.noalias() += alpha * (a.transpose() * b);
            break;
        }
        break;
      case Transpose::NoTrans:
        switch (trans_b) {
          case Transpose::ConjTrans:
            c.noalias() += alpha * (a * b.adjoint());
            break;
          case Transpose::Trans:
            c.noalias() += alpha * (a * b.transpose());
            break;
          case Transpose::NoTrans:
            c.noalias() += alpha * (a * b);
            break;
        }
        break;
    }
  }
  void mul(const Transpose trans_a, const complex alpha, const MatrixXc& a, const VectorXc& b, const complex beta, VectorXc& c) override {
    c *= beta;
    switch (trans_a) {
      case Transpose::ConjTrans:
        c.noalias() += alpha * (a.adjoint() * b);
        break;
      case Transpose::Trans:
        c.noalias() += alpha * (a.transpose() * b);
        break;
      case Transpose::NoTrans:
        c.noalias() += alpha * (a * b);
        break;
    }
  }

  void mul(const Transpose trans_a, const Transpose trans_b, const driver::float_t alpha, const MatrixXd& a, const MatrixXd& b,
           const driver::float_t beta, MatrixXd& c) override {
    c *= beta;
    switch (trans_a) {
      case Transpose::ConjTrans:
      case Transpose::Trans:
        switch (trans_b) {
          case Transpose::ConjTrans:
          case Transpose::Trans:
            c.noalias() += alpha * (a.transpose() * b.transpose());
            break;
          case Transpose::NoTrans:
            c.noalias() += alpha * (a.transpose() * b);
            break;
        }
        break;
      case Transpose::NoTrans:
        switch (trans_b) {
          case Transpose::ConjTrans:
          case Transpose::Trans:
            c.noalias() += alpha * (a * b.transpose());
            break;
          case Transpose::NoTrans:
            c.noalias() += alpha * (a * b);
            break;
        }
        break;
    }
  }

  void mul(const Transpose trans_a, const driver::float_t alpha, const MatrixXd& a, const VectorXd& b, const driver::float_t beta,
           VectorXd& c) override {
    c *= beta;
    switch (trans_a) {
      case Transpose::ConjTrans:
      case Transpose::Trans:
        c.noalias() += alpha * (a.transpose() * b);
        break;
      case Transpose::NoTrans:
        c.noalias() += alpha * (a * b);
        break;
    }
  }

  void hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) override { c.noalias() = a.cwiseProduct(b); }
  void hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) override { c.noalias() = a.cwiseProduct(b); }

  void solvet(MatrixXd& a, VectorXd& b) override {
    const Eigen::LLT<MatrixXd> llt(a);
    llt.solveInPlace(b);
  }

  void solveh(MatrixXc& a, VectorXc& b) override {
    const Eigen::LLT<MatrixXc> llt(a);
    llt.solveInPlace(b);
  }

  void max_eigen_vector(MatrixXc& src, VectorXc& dst) override {
    const Eigen::ComplexEigenSolver<MatrixXc> ces(src);
    auto idx = 0;
    ces.eigenvalues().cwiseAbs2().maxCoeff(&idx);
    dst = ces.eigenvectors().col(idx);
  }

  void pseudo_inverse_svd(MatrixXc& src, const driver::float_t alpha, MatrixXc&, MatrixXc& s, MatrixXc&, MatrixXc&, MatrixXc& dst) override {
    const Eigen::BDCSVD svd(src, Eigen::ComputeFullU | Eigen::ComputeFullV);
    s.fill(ZERO);
    auto& singular_values = svd.singularValues();
    const auto size = singular_values.size();
    for (Eigen::Index i = 0; i < size; i++) s(i, i) = singular_values(i) / (singular_values(i) * singular_values(i) + alpha);
    dst.noalias() = svd.matrixV() * s * svd.matrixU().adjoint();
  }

  void pseudo_inverse_svd(MatrixXd& src, const driver::float_t alpha, MatrixXd&, MatrixXd& s, MatrixXd&, MatrixXd&, MatrixXd& dst) override {
    const Eigen::BDCSVD svd(src, Eigen::ComputeFullU | Eigen::ComputeFullV);
    s.fill(0.0);
    auto& singular_values = svd.singularValues();
    const auto size = singular_values.size();
    for (Eigen::Index i = 0; i < size; i++) s(i, i) = singular_values(i) / (singular_values(i) * singular_values(i) + alpha);
    dst.noalias() = svd.matrixV() * s * svd.matrixU().adjoint();
  }
};

BackendPtr EigenBackend::build() const { return std::make_shared<EigenBackendImpl>(); }

}  // namespace autd3::gain::holo
