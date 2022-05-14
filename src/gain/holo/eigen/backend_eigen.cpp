// File: backend_eigen.cpp
// Project: eigen
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3/gain/backend.hpp"

namespace autd3::gain::holo {

class EigenBackendImpl final : public EigenBackend {
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

  void copy_to(const MatrixXc& src, MatrixXc& dst) override { dst = src; }

  void conj(const VectorXc& src, VectorXc& dst) override { dst = src.conjugate(); }

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override {
    dst.fill(ZERO);
    dst.diagonal() = src;
  }

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

  complex max_abs_element(const VectorXc& src) override {
    Eigen::Index idx = 0;
    src.cwiseAbs2().maxCoeff(&idx);
    return src(idx);
  }

  void scale(const complex value, VectorXc& dst) override { dst *= value; }

  complex dot(const VectorXc& a, const VectorXc& b) override { return a.dot(b); }

  void mul(const TRANSPOSE trans_a, const TRANSPOSE trans_b, const complex alpha, const MatrixXc& a, const MatrixXc& b, const complex beta,
           MatrixXc& c) override {
    c *= beta;
    switch (trans_a) {
      case TRANSPOSE::CONJ_TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            c.noalias() += alpha * (a.adjoint() * b.adjoint());
            break;
          case TRANSPOSE::TRANS:
            c.noalias() += alpha * (a.adjoint() * b.transpose());
            break;
          case TRANSPOSE::NO_TRANS:
            c.noalias() += alpha * (a.adjoint() * b);
            break;
        }
        break;
      case TRANSPOSE::TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            c.noalias() += alpha * (a.transpose() * b.adjoint());
            break;
          case TRANSPOSE::TRANS:
            c.noalias() += alpha * (a.transpose() * b.transpose());
            break;
          case TRANSPOSE::NO_TRANS:
            c.noalias() += alpha * (a.transpose() * b);
            break;
        }
        break;
      case TRANSPOSE::NO_TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            c.noalias() += alpha * (a * b.adjoint());
            break;
          case TRANSPOSE::TRANS:
            c.noalias() += alpha * (a * b.transpose());
            break;
          case TRANSPOSE::NO_TRANS:
            c.noalias() += alpha * (a * b);
            break;
        }
        break;
    }
  }
  void mul(const TRANSPOSE trans_a, const complex alpha, const MatrixXc& a, const VectorXc& b, const complex beta, VectorXc& c) override {
    c *= beta;
    switch (trans_a) {
      case TRANSPOSE::CONJ_TRANS:
        c.noalias() += alpha * (a.adjoint() * b);
        break;
      case TRANSPOSE::TRANS:
        c.noalias() += alpha * (a.transpose() * b);
        break;
      case TRANSPOSE::NO_TRANS:
        c.noalias() += alpha * (a * b);
        break;
    }
  }

  void max_eigen_vector(const MatrixXc& src, VectorXc& dst) override {
    const Eigen::ComplexEigenSolver<MatrixXc> ces(src);
    auto idx = 0;
    ces.eigenvalues().cwiseAbs2().maxCoeff(&idx);
    dst = ces.eigenvectors().col(idx);
  }

  void pseudo_inverse_svd(const MatrixXc& src, const double alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf, MatrixXc& dst) override {
    const Eigen::BDCSVD svd(src, Eigen::ComputeFullU | Eigen::ComputeFullV);
    s.fill(ZERO);
    auto& singular_values = svd.singularValues();
    const auto size = singular_values.size();
    for (Eigen::Index i = 0; i < size; i++) s(i, i) = singular_values(i) / (singular_values(i) * singular_values(i) + alpha);
    dst.noalias() = svd.matrixV() * s * svd.matrixU().adjoint();
  }
};

BackendPtr EigenBackend::create() { return std::make_shared<EigenBackendImpl>(); }

}  // namespace autd3::gain::holo
