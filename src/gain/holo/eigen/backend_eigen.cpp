// File: backend_eigen.cpp
// Project: eigen
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/gain/backend.hpp"

namespace autd3::gain::holo {

void EigenBackend::init() {}
void EigenBackend::to_host(VectorXc&) {}
void EigenBackend::to_host(MatrixXc&) {}
void EigenBackend::to_host(VectorXd&) {}
void EigenBackend::to_host(MatrixXd&) {}

void EigenBackend::copy_to(const MatrixXc& src, MatrixXc& dst) { dst = src; }
void EigenBackend::copy_to(const MatrixXd& src, MatrixXd& dst) { dst = src; }
void EigenBackend::copy_to(const VectorXd& src, VectorXd& dst) { dst = src; }
void EigenBackend::copy_to(const VectorXc& src, VectorXc& dst) { dst = src; }

void EigenBackend::real(const MatrixXc& src, MatrixXd& re) { re = src.real(); }
void EigenBackend::imag(const MatrixXc& src, MatrixXd& im) { im = src.imag(); }
void EigenBackend::make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) {
  dst.real() = re;
  dst.imag() = im;
}

void EigenBackend::abs(const VectorXc& src, VectorXd& dst) { dst = src.cwiseAbs(); }
void EigenBackend::abs(const VectorXc& src, VectorXc& dst) { dst = src.cwiseAbs(); }
void EigenBackend::sqrt(const VectorXd& src, VectorXd& dst) { dst = src.cwiseSqrt(); }
void EigenBackend::conj(const VectorXc& src, VectorXc& dst) { dst = src.conjugate(); }
void EigenBackend::arg(const VectorXc& src, VectorXc& dst) { dst = src.cwiseQuotient(src.cwiseAbs()); }
void EigenBackend::reciprocal(const VectorXc& src, VectorXc& dst) { dst = src.cwiseInverse(); }
void EigenBackend::exp(const VectorXc& src, VectorXc& dst) { dst = src.array().exp(); }
void EigenBackend::pow(const VectorXd& src, const driver::autd3_float_t p, VectorXd& dst) { dst = src.array().pow(p); }

void EigenBackend::create_diagonal(const VectorXc& src, MatrixXc& dst) {
  dst.fill(ZERO);
  dst.diagonal() = src;
}
void EigenBackend::get_diagonal(const MatrixXc& src, VectorXc& dst) { dst = src.diagonal(); }
void EigenBackend::get_diagonal(const MatrixXd& src, VectorXd& dst) { dst = src.diagonal(); }

void EigenBackend::set(const size_t i, const complex value, VectorXc& dst) { dst(static_cast<Eigen::Index>(i)) = value; }
void EigenBackend::set_row(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) {
  dst.block(static_cast<Eigen::Index>(i), static_cast<Eigen::Index>(begin), 1, end - begin) =
      src.block(static_cast<Eigen::Index>(begin), 0, end - begin, 1).transpose();
}
void EigenBackend::set_col(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) {
  dst.block(static_cast<Eigen::Index>(begin), static_cast<Eigen::Index>(i), end - begin, 1) =
      src.block(static_cast<Eigen::Index>(begin), 0, end - begin, 1);
}

void EigenBackend::get_col(const MatrixXc& src, const size_t i, VectorXc& dst) { dst = src.col(static_cast<Eigen::Index>(i)); }

void EigenBackend::concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) { dst << a, b; }
void EigenBackend::concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) { dst << a, b; }

void EigenBackend::concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) { dst << a, b; }

void EigenBackend::reduce_col(const MatrixXd& src, VectorXd& dst) { dst = src.rowwise().sum(); }

complex EigenBackend::max_abs_element(const VectorXc& src) {
  Eigen::Index idx = 0;
  src.cwiseAbs2().maxCoeff(&idx);
  return src(idx);
}

driver::autd3_float_t EigenBackend::max_element(const VectorXd& src) { return src.maxCoeff(); }

void EigenBackend::scale(const complex value, VectorXc& dst) { dst *= value; }
void EigenBackend::scale(const driver::autd3_float_t value, VectorXd& dst) { dst *= value; }

complex EigenBackend::dot(const VectorXc& a, const VectorXc& b) { return a.dot(b); }
driver::autd3_float_t EigenBackend::dot(const VectorXd& a, const VectorXd& b) { return a.dot(b); }

void EigenBackend::add(const driver::autd3_float_t alpha, const MatrixXd& a, MatrixXd& b) { b += alpha * a; }
void EigenBackend::add(const complex alpha, const MatrixXc& a, MatrixXc& b) { b += alpha * a; }
void EigenBackend::add(const driver::autd3_float_t alpha, const VectorXd& a, VectorXd& b) { b += alpha * a; }
void EigenBackend::add(const complex alpha, const VectorXc& a, VectorXc& b) { b += alpha * a; }

void EigenBackend::mul(const Transpose trans_a, const Transpose trans_b, const complex alpha, const MatrixXc& a, const MatrixXc& b,
                       const complex beta, MatrixXc& c) {
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
void EigenBackend::mul(const Transpose trans_a, const complex alpha, const MatrixXc& a, const VectorXc& b, const complex beta, VectorXc& c) {
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

void EigenBackend::mul(const Transpose trans_a, const Transpose trans_b, const driver::autd3_float_t alpha, const MatrixXd& a, const MatrixXd& b,
                       const driver::autd3_float_t beta, MatrixXd& c) {
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

void EigenBackend::mul(const Transpose trans_a, const driver::autd3_float_t alpha, const MatrixXd& a, const VectorXd& b,
                       const driver::autd3_float_t beta, VectorXd& c) {
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

void EigenBackend::hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) { c.noalias() = a.cwiseProduct(b); }
void EigenBackend::hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) { c.noalias() = a.cwiseProduct(b); }

void EigenBackend::solvet(MatrixXd& a, VectorXd& b) {
  const Eigen::LLT<MatrixXd> llt(a);
  llt.solveInPlace(b);
}

void EigenBackend::solveh(MatrixXc& a, VectorXc& b) {
  const Eigen::LLT<MatrixXc> llt(a);
  llt.solveInPlace(b);
}

void EigenBackend::max_eigen_vector(MatrixXc& src, VectorXc& dst) {
  const Eigen::ComplexEigenSolver<MatrixXc> ces(src);
  auto idx = 0;
  ces.eigenvalues().cwiseAbs2().maxCoeff(&idx);
  dst = ces.eigenvectors().col(idx);
}

void EigenBackend::pseudo_inverse_svd(MatrixXc& src, const driver::autd3_float_t alpha, MatrixXc&, MatrixXc& s, MatrixXc&, MatrixXc&, MatrixXc& dst) {
  const Eigen::BDCSVD svd(src, Eigen::ComputeFullU | Eigen::ComputeFullV);
  s.fill(ZERO);
  auto& singular_values = svd.singularValues();
  const auto size = singular_values.size();
  for (Eigen::Index i = 0; i < size; i++) s(i, i) = singular_values(i) / (singular_values(i) * singular_values(i) + alpha);
  dst.noalias() = svd.matrixV() * s * svd.matrixU().adjoint();
}

void EigenBackend::pseudo_inverse_svd(MatrixXd& src, const driver::autd3_float_t alpha, MatrixXd&, MatrixXd& s, MatrixXd&, MatrixXd&, MatrixXd& dst) {
  const Eigen::BDCSVD svd(src, Eigen::ComputeFullU | Eigen::ComputeFullV);
  s.fill(0.0);
  auto& singular_values = svd.singularValues();
  const auto size = singular_values.size();
  for (Eigen::Index i = 0; i < size; i++) s(i, i) = singular_values(i) / (singular_values(i) * singular_values(i) + alpha);
  dst.noalias() = svd.matrixV() * s * svd.matrixU().adjoint();
}

BackendPtr EigenBackend::create() { return std::make_shared<EigenBackend>(); }

}  // namespace autd3::gain::holo
