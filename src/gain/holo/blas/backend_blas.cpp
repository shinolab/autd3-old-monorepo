// File: backend_blas.cpp
// Project: blas
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 23/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/gain/backend_blas.hpp"

#define lapack_complex_float std::complex<float>
#define lapack_complex_double std::complex<double>
#ifdef USE_BLAS_MKL
#include "./mkl_cblas.h"
#include "./mkl_lapacke.h"
#else
#include "./cblas.h"
#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4190)
#endif
#include "./lapacke.h"
#if _MSC_VER
#pragma warning(pop)
#endif
#endif

namespace autd3::gain::holo {

#ifdef AUTD3_USE_SINGLE_FLOAT
constexpr auto AUTD_DSCAL = cblas_sscal;
constexpr auto AUTD_GESVD = LAPACKE_sgesdd;
constexpr auto AUTD_GESVDC = LAPACKE_cgesdd;
constexpr auto AUTD_HEEV = LAPACKE_cheev;
constexpr auto AUTD_ZSCAL = cblas_cscal;
constexpr auto AUTD_AXPY = cblas_saxpy;
constexpr auto AUTD_AXPYC = cblas_caxpy;
constexpr auto AUTD_DGEMV = cblas_sgemv;
constexpr auto AUTD_ZGEMV = cblas_cgemv;
constexpr auto AUTD_DGEMM = cblas_sgemm;
constexpr auto AUTD_ZGEMM = cblas_cgemm;
constexpr auto AUTD_DOTC = cblas_cdotc_sub;
constexpr auto AUTD_DOT = cblas_sdot;
constexpr auto AUTD_SYSV = LAPACKE_ssysv;
constexpr auto AUTD_POSVC = LAPACKE_cposv;
constexpr auto AUTD_CPY = cblas_scopy;
constexpr auto AUTD_CPYC = cblas_ccopy;
#else
constexpr auto AUTD_DSCAL = cblas_dscal;
constexpr auto AUTD_GESVD = LAPACKE_dgesdd;
constexpr auto AUTD_GESVDC = LAPACKE_zgesdd;
constexpr auto AUTD_HEEV = LAPACKE_zheev;
constexpr auto AUTD_ZSCAL = cblas_zscal;
constexpr auto AUTD_AXPY = cblas_daxpy;
constexpr auto AUTD_AXPYC = cblas_zaxpy;
constexpr auto AUTD_DGEMV = cblas_dgemv;
constexpr auto AUTD_ZGEMV = cblas_zgemv;
constexpr auto AUTD_DGEMM = cblas_dgemm;
constexpr auto AUTD_ZGEMM = cblas_zgemm;
constexpr auto AUTD_DOTC = cblas_zdotc_sub;
constexpr auto AUTD_DOT = cblas_ddot;
constexpr auto AUTD_SYSV = LAPACKE_dsysv;
constexpr auto AUTD_POSVC = LAPACKE_zposv;
constexpr auto AUTD_CPY = cblas_dcopy;
constexpr auto AUTD_CPYC = cblas_zcopy;
#endif

void BLASBackend::init() {}
void BLASBackend::to_host(VectorXc&) {}
void BLASBackend::to_host(MatrixXc&) {}
void BLASBackend::to_host(VectorXd&) {}
void BLASBackend::to_host(MatrixXd&) {}

void BLASBackend::copy_to(const MatrixXc& src, MatrixXc& dst) { AUTD_CPYC(static_cast<int>(src.size()), src.data(), 1, dst.data(), 1); }
void BLASBackend::copy_to(const MatrixXd& src, MatrixXd& dst) { AUTD_CPY(static_cast<int>(src.size()), src.data(), 1, dst.data(), 1); }
void BLASBackend::copy_to(const VectorXd& src, VectorXd& dst) { AUTD_CPY(static_cast<int>(src.size()), src.data(), 1, dst.data(), 1); }
void BLASBackend::copy_to(const VectorXc& src, VectorXc& dst) { AUTD_CPYC(static_cast<int>(src.size()), src.data(), 1, dst.data(), 1); }

void BLASBackend::real(const MatrixXc& src, MatrixXd& re) { re = src.real(); }
void BLASBackend::imag(const MatrixXc& src, MatrixXd& im) { im = src.imag(); }
void BLASBackend::make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) {
  dst.real() = re;
  dst.imag() = im;
}

void BLASBackend::abs(const VectorXc& src, VectorXd& dst) { dst = src.cwiseAbs(); }
void BLASBackend::abs(const VectorXc& src, VectorXc& dst) { dst = src.cwiseAbs(); }
void BLASBackend::sqrt(const VectorXd& src, VectorXd& dst) { dst = src.cwiseSqrt(); }
void BLASBackend::conj(const VectorXc& src, VectorXc& dst) { dst = src.conjugate(); }
void BLASBackend::arg(const VectorXc& src, VectorXc& dst) { dst = src.cwiseQuotient(src.cwiseAbs()); }
void BLASBackend::reciprocal(const VectorXc& src, VectorXc& dst) { dst = src.cwiseInverse(); }
void BLASBackend::exp(const VectorXc& src, VectorXc& dst) { dst = src.array().exp(); }
void BLASBackend::pow(const VectorXd& src, const driver::autd3_float_t p, VectorXd& dst) { dst = src.array().pow(p); }

void BLASBackend::create_diagonal(const VectorXc& src, MatrixXc& dst) {
  dst.fill(ZERO);
  dst.diagonal() = src;
}
void BLASBackend::get_diagonal(const MatrixXc& src, VectorXc& dst) { dst = src.diagonal(); }
void BLASBackend::get_diagonal(const MatrixXd& src, VectorXd& dst) { dst = src.diagonal(); }

void BLASBackend::set(const size_t i, const complex value, VectorXc& dst) { dst(static_cast<Eigen::Index>(i)) = value; }
void BLASBackend::set_row(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) {
  dst.block(static_cast<Eigen::Index>(i), static_cast<Eigen::Index>(begin), 1, end - begin) =
      src.block(static_cast<Eigen::Index>(begin), 0, end - begin, 1).transpose();
}
void BLASBackend::set_col(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) {
  dst.block(static_cast<Eigen::Index>(begin), static_cast<Eigen::Index>(i), end - begin, 1) =
      src.block(static_cast<Eigen::Index>(begin), 0, end - begin, 1);
}

void BLASBackend::get_col(const MatrixXc& src, const size_t i, VectorXc& dst) { dst = src.col(static_cast<Eigen::Index>(i)); }

void BLASBackend::concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) { dst << a, b; }
void BLASBackend::concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) { dst << a, b; }

void BLASBackend::concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) { dst << a, b; }

void BLASBackend::reduce_col(const MatrixXd& src, VectorXd& dst) { dst = src.rowwise().sum(); }

complex BLASBackend::max_abs_element(const VectorXc& src) {
  Eigen::Index idx = 0;
  src.cwiseAbs2().maxCoeff(&idx);
  return src(idx);
}

driver::autd3_float_t BLASBackend::max_element(const VectorXd& src) { return src.maxCoeff(); }

void BLASBackend::scale(const complex value, VectorXc& dst) { AUTD_ZSCAL(static_cast<int>(dst.size()), &value, dst.data(), 1); }
void BLASBackend::scale(const driver::autd3_float_t value, VectorXd& dst) { AUTD_DSCAL(static_cast<int>(dst.size()), value, dst.data(), 1); }

complex BLASBackend::dot(const VectorXc& a, const VectorXc& b) {
  complex d;
  AUTD_DOTC(static_cast<int>(a.size()), a.data(), 1, b.data(), 1, &d);
  return d;
}
driver::autd3_float_t BLASBackend::dot(const VectorXd& a, const VectorXd& b) {
  return AUTD_DOT(static_cast<int>(a.size()), a.data(), 1, b.data(), 1);
}

void BLASBackend::add(const driver::autd3_float_t alpha, const MatrixXd& a, MatrixXd& b) {
  AUTD_AXPY(static_cast<int>(a.size()), alpha, a.data(), 1, b.data(), 1);
}
void BLASBackend::add(const driver::autd3_float_t alpha, const VectorXd& a, VectorXd& b) {
  AUTD_AXPY(static_cast<int>(a.size()), alpha, a.data(), 1, b.data(), 1);
}
void BLASBackend::add(const complex alpha, const MatrixXc& a, MatrixXc& b) {
  AUTD_AXPYC(static_cast<int>(a.size()), &alpha, a.data(), 1, b.data(), 1);
}
void BLASBackend::add(const complex alpha, const VectorXc& a, VectorXc& b) {
  AUTD_AXPYC(static_cast<int>(a.size()), &alpha, a.data(), 1, b.data(), 1);
}

void BLASBackend::mul(const Transpose trans_a, const Transpose trans_b, const complex alpha, const MatrixXc& a, const MatrixXc& b, const complex beta,
                      MatrixXc& c) {
  const auto lda = static_cast<int>(a.rows());
  const auto ldb = static_cast<int>(b.rows());
  const auto ldc = trans_a == Transpose::NoTrans ? static_cast<int>(a.rows()) : static_cast<int>(a.cols());
  const auto n = trans_b == Transpose::NoTrans ? static_cast<int>(b.cols()) : static_cast<int>(b.rows());
  const auto k = trans_a == Transpose::NoTrans ? static_cast<int>(a.cols()) : static_cast<int>(a.rows());
  AUTD_ZGEMM(CblasColMajor, static_cast<CBLAS_TRANSPOSE>(trans_a), static_cast<CBLAS_TRANSPOSE>(trans_b), ldc, n, k, &alpha, a.data(), lda, b.data(),
             ldb, &beta, c.data(), ldc);
}

void BLASBackend::mul(const Transpose trans_a, const complex alpha, const MatrixXc& a, const VectorXc& b, const complex beta, VectorXc& c) {
  const auto m = static_cast<int>(a.rows());
  const auto n = static_cast<int>(a.cols());
  const auto lda = m;
  AUTD_ZGEMV(CblasColMajor, static_cast<CBLAS_TRANSPOSE>(trans_a), m, n, &alpha, a.data(), lda, b.data(), 1, &beta, c.data(), 1);
}

void BLASBackend::mul(const Transpose trans_a, const Transpose trans_b, const driver::autd3_float_t alpha, const MatrixXd& a, const MatrixXd& b,
                      const driver::autd3_float_t beta, MatrixXd& c) {
  const auto lda = static_cast<int>(a.rows());
  const auto ldb = static_cast<int>(b.rows());
  const auto ldc = trans_a == Transpose::NoTrans ? static_cast<int>(a.rows()) : static_cast<int>(a.cols());
  const auto n = trans_b == Transpose::NoTrans ? static_cast<int>(b.cols()) : static_cast<int>(b.rows());
  const auto k = trans_a == Transpose::NoTrans ? static_cast<int>(a.cols()) : static_cast<int>(a.rows());
  AUTD_DGEMM(CblasColMajor, static_cast<CBLAS_TRANSPOSE>(trans_a), static_cast<CBLAS_TRANSPOSE>(trans_b), ldc, n, k, alpha, a.data(), lda, b.data(),
             ldb, beta, c.data(), ldc);
}

void BLASBackend::mul(const Transpose trans_a, const driver::autd3_float_t alpha, const MatrixXd& a, const VectorXd& b,
                      const driver::autd3_float_t beta, VectorXd& c) {
  const auto m = static_cast<int>(a.rows());
  const auto n = static_cast<int>(a.cols());
  const auto lda = m;
  AUTD_DGEMV(CblasColMajor, static_cast<CBLAS_TRANSPOSE>(trans_a), m, n, alpha, a.data(), lda, b.data(), 1, beta, c.data(), 1);
}

void BLASBackend::hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) { c.noalias() = a.cwiseProduct(b); }
void BLASBackend::hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) { c.noalias() = a.cwiseProduct(b); }

void BLASBackend::solvet(MatrixXd& a, VectorXd& b) {
  const auto n = static_cast<int>(a.cols());
  const auto lda = static_cast<int>(a.rows());
  const auto ldb = static_cast<int>(b.size());
  const auto ipiv = std::make_unique<int[]>(n);
  AUTD_SYSV(CblasColMajor, 'U', n, 1, a.data(), lda, ipiv.get(), b.data(), ldb);
}

void BLASBackend::solveh(MatrixXc& a, VectorXc& b) {
  const auto n = static_cast<int>(a.cols());
  const auto lda = static_cast<int>(a.rows());
  const auto ldb = static_cast<int>(b.size());
  auto ipiv = std::make_unique<int[]>(n);
  AUTD_POSVC(CblasColMajor, 'U', n, 1, a.data(), lda, b.data(), ldb);
}

void BLASBackend::max_eigen_vector(MatrixXc& src, VectorXc& dst) {
  const auto size = src.cols();
  const auto eigenvalues = std::make_unique<driver::autd3_float_t[]>(size);
  AUTD_HEEV(CblasColMajor, 'V', 'U', static_cast<int>(size), src.data(), static_cast<int>(size), eigenvalues.get());
  std::memcpy(dst.data(), src.data() + size * (size - 1), size * sizeof(complex));
}

void BLASBackend::pseudo_inverse_svd(MatrixXc& src, const driver::autd3_float_t alpha, MatrixXc& u, MatrixXc& s, MatrixXc& vt, MatrixXc& buf,
                                     MatrixXc& dst) {
  const auto nc = src.cols();
  const auto nr = src.rows();

  const auto lda = static_cast<int>(nr);
  const auto ldu = static_cast<int>(nr);
  const auto ldvt = static_cast<int>(nc);

  const auto s_size = std::min(nr, nc);
  const auto sigma = std::make_unique<driver::autd3_float_t[]>(s_size);

  AUTD_GESVDC(LAPACK_COL_MAJOR, 'A', static_cast<int>(nr), static_cast<int>(nc), src.data(), lda, sigma.get(), u.data(), ldu, vt.data(), ldvt);
  s.fill(ZERO);
  for (Eigen::Index i = 0; i < static_cast<Eigen::Index>(s_size); i++) s(i, i) = sigma[i] / (sigma[i] * sigma[i] + alpha);

  mul(Transpose::NoTrans, Transpose::ConjTrans, ONE, s, u, ZERO, buf);
  mul(Transpose::ConjTrans, Transpose::NoTrans, ONE, vt, buf, ZERO, dst);
}

void BLASBackend::pseudo_inverse_svd(MatrixXd& src, const driver::autd3_float_t alpha, MatrixXd& u, MatrixXd& s, MatrixXd& vt, MatrixXd& buf,
                                     MatrixXd& dst) {
  const auto nc = src.cols();
  const auto nr = src.rows();

  const auto lda = static_cast<int>(nr);
  const auto ldu = static_cast<int>(nr);
  const auto ldvt = static_cast<int>(nc);

  const auto s_size = std::min(nr, nc);
  const auto sigma = std::make_unique<driver::autd3_float_t[]>(s_size);

  AUTD_GESVD(LAPACK_COL_MAJOR, 'A', static_cast<int>(nr), static_cast<int>(nc), src.data(), lda, sigma.get(), u.data(), ldu, vt.data(), ldvt);
  s.fill(0.0);
  for (Eigen::Index i = 0; i < static_cast<Eigen::Index>(s_size); i++) s(i, i) = sigma[i] / (sigma[i] * sigma[i] + alpha);

  mul(Transpose::NoTrans, Transpose::ConjTrans, 1.0, s, u, 0.0, buf);
  mul(Transpose::ConjTrans, Transpose::NoTrans, 1.0, vt, buf, 0.0, dst);
}

BackendPtr BLASBackend::create() { return std::make_shared<BLASBackend>(); }

}  // namespace autd3::gain::holo
