// File: backend_test.cpp
// Project: holo
// Created Date: 14/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include <cmath>
#include <random>

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26439 26495 26812)
#endif
#include <gtest/gtest.h>
#if _MSC_VER
#pragma warning(pop)
#endif

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6031 6294 6255 26451 26495 26812)
#endif
#include <unsupported/Eigen/MatrixFunctions>
#if _MSC_VER
#pragma warning(pop)
#endif

#include "autd3/core/utils.hpp"
#include "autd3/gain/backend.hpp"
#include "test_utils.hpp"

using autd3::gain::holo::complex;
using autd3::gain::holo::MatrixXc;
using autd3::gain::holo::ONE;
using autd3::gain::holo::TRANSPOSE;
using autd3::gain::holo::VectorXc;
using autd3::gain::holo::ZERO;

using testing::Types;

template <typename B>
class BackendTest : public testing::Test {
 public:
  BackendTest() : backend(B::create()) {}
  ~BackendTest() override {}
  BackendTest(const BackendTest& v) noexcept = default;
  BackendTest& operator=(const BackendTest& obj) = default;
  BackendTest(BackendTest&& obj) = default;
  BackendTest& operator=(BackendTest&& obj) = default;
  autd3::gain::holo::BackendPtr backend;
};

#define EIGEN3_BACKEND_TYPE autd3::gain::holo::EigenBackend

#ifdef TEST_BACKEND_CUDA
#include "autd3/gain/backend_cuda.hpp"
#define CUDA_BACKEND_TYPE , autd3::gain::holo::CUDABackend
#else
#define CUDA_BACKEND_TYPE
#endif

typedef Types<EIGEN3_BACKEND_TYPE CUDA_BACKEND_TYPE> Implementations;

TYPED_TEST_SUITE(BackendTest, Implementations, );

TYPED_TEST(BackendTest, copy_to) {
  constexpr Eigen::Index m = 1000;
  constexpr Eigen::Index n = 2000;

  MatrixXc a = MatrixXc::Random(m, n);

  MatrixXc b(m, n);
  this->backend->copy_to(a, b);
  this->backend->to_host(b);

  for (Eigen::Index i = 0; i < m; i++)
    for (Eigen::Index j = 0; j < n; j++) ASSERT_EQ(a(i, j), b(i, j));
}

TYPED_TEST(BackendTest, abs) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = VectorXc::Random(n);

  VectorXc b(n);
  this->backend->abs(a, b);
  this->backend->to_host(b);

  for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(std::abs(a(i)), b(i).real(), 1e-6);
}

TYPED_TEST(BackendTest, conj) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = VectorXc::Random(n);

  VectorXc b(n);
  this->backend->conj(a, b);
  this->backend->to_host(b);

  for (Eigen::Index i = 0; i < n; i++) ASSERT_EQ(std::conj(a(i)), b(i));
}

TYPED_TEST(BackendTest, arg) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = VectorXc::Random(n);

  this->backend->arg(a, a);
  this->backend->to_host(a);

  for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(std::abs(a(i)), 1.0, 1e-6);
}

TYPED_TEST(BackendTest, reciprocal) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = 2.0 * VectorXc::Ones(n) + VectorXc::Random(n);

  VectorXc b(n);
  this->backend->reciprocal(a, b);
  this->backend->to_host(b);

  for (Eigen::Index i = 0; i < n; i++) {
    const auto expected = 1.0 / a(i);
    ASSERT_NEAR_COMPLEX(expected, b(i), 1e-6);
  }
}

TYPED_TEST(BackendTest, create_diagonal) {
  constexpr Eigen::Index m = 1000;
  constexpr Eigen::Index n = 2000;

  VectorXc a = VectorXc::Random(m);

  MatrixXc b(m, n);
  this->backend->create_diagonal(a, b);
  this->backend->to_host(b);

  for (int i = 0; i < m; i++)
    for (int j = 0; j < n; j++)
      if (i == j)
        ASSERT_EQ(b(i, j), a(i));
      else
        ASSERT_EQ(b(i, j), ZERO);
}

TYPED_TEST(BackendTest, set) {
  constexpr Eigen::Index m = 1000;

  VectorXc a(m);

  this->backend->set(50, complex(10.0, 5.0), a);
  this->backend->to_host(a);

  ASSERT_EQ(a(50), complex(10.0, 5.0));
}

TYPED_TEST(BackendTest, set_row) {
  constexpr Eigen::Index m = 1000;
  constexpr Eigen::Index n = 2000;

  MatrixXc a = MatrixXc::Zero(m, n);

  VectorXc b = VectorXc::Random(n);
  this->backend->set_row(b, 50, 6, 9, a);
  this->backend->to_host(a);

  for (int i = 0; i < m; i++)
    for (int j = 0; j < n; j++)
      if ((i == 50) && (6 <= j && j < 9))
        ASSERT_EQ(a(i, j), b(j));
      else
        ASSERT_EQ(a(i, j), ZERO);
}

TYPED_TEST(BackendTest, set_col) {
  constexpr Eigen::Index m = 1000;
  constexpr Eigen::Index n = 2000;

  MatrixXc a = MatrixXc::Zero(m, n);

  VectorXc b = VectorXc::Random(m);
  this->backend->set_col(b, 7, 2, 5, a);
  this->backend->to_host(a);

  for (int i = 0; i < m; i++)
    for (int j = 0; j < n; j++)
      if ((j == 7) && (2 <= i && i < 5))
        ASSERT_EQ(a(i, j), b(i));
      else
        ASSERT_EQ(a(i, j), ZERO);
}

TYPED_TEST(BackendTest, get_col) {
  constexpr Eigen::Index m = 1000;
  constexpr Eigen::Index n = 2000;

  MatrixXc a = MatrixXc::Random(m, n);

  VectorXc b(m);

  this->backend->get_col(a, 50, b);
  this->backend->to_host(b);

  for (int i = 0; i < m; i++) ASSERT_EQ(a(i, 50), b(i));
}

TYPED_TEST(BackendTest, max_abs_element) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = VectorXc::Random(n);

  Eigen::Index idx = 0;
  a.cwiseAbs2().maxCoeff(&idx);

  ASSERT_EQ(this->backend->max_abs_element(a), a(idx));
}

TYPED_TEST(BackendTest, scale) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = VectorXc::Random(n);
  VectorXc a_tmp = a;

  this->backend->scale(complex(1, 1), a);
  this->backend->to_host(a);

  for (Eigen::Index i = 0; i < n; i++) {
    const auto expected = complex(1, 1) * a_tmp(i);
    ASSERT_NEAR_COMPLEX(expected, a(i), 1e-6);
  }
}

TYPED_TEST(BackendTest, dot) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = VectorXc::Random(n);
  VectorXc b = VectorXc::Random(n);

  auto expected = complex(0, 0);
  for (Eigen::Index i = 0; i < n; i++) expected += std::conj(a(i)) * b(i);

  ASSERT_NEAR_COMPLEX(this->backend->dot(a, b), expected, 1e-6);
}

TYPED_TEST(BackendTest, mul_matrix) {
  constexpr Eigen::Index n = 1000;
  constexpr Eigen::Index m = 2000;
  constexpr Eigen::Index k = 3000;

  MatrixXc a = MatrixXc::Random(n, m);
  MatrixXc b = MatrixXc::Random(m, m);

  MatrixXc c = MatrixXc::Zero(n, m);
  this->backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, a, b, ZERO, c);
  this->backend->to_host(c);

  MatrixXc expected = a * b;

  for (Eigen::Index i = 0; i < n; i++)
    for (Eigen::Index j = 0; j < m; j++) ASSERT_NEAR_COMPLEX(c(i, j), expected(i, j), 1e-6);

  MatrixXc aa = MatrixXc::Random(k, n);
  MatrixXc bb = MatrixXc::Random(m, k);
  this->backend->mul(TRANSPOSE::CONJ_TRANS, TRANSPOSE::TRANS, 2.0 * ONE, aa, bb, ONE, c);
  this->backend->to_host(c);

  expected += 2.0 * (aa.adjoint() * bb.transpose());

  for (Eigen::Index i = 0; i < n; i++)
    for (Eigen::Index j = 0; j < m; j++) ASSERT_NEAR_COMPLEX(c(i, j), expected(i, j), 1e-6);
}

TYPED_TEST(BackendTest, mul_vec) {
  constexpr Eigen::Index n = 1000;
  constexpr Eigen::Index m = 2000;

  MatrixXc a = MatrixXc::Random(n, m);
  VectorXc b = VectorXc::Random(m);

  VectorXc c = VectorXc::Zero(n);
  this->backend->mul(TRANSPOSE::NO_TRANS, ONE, a, b, ZERO, c);
  this->backend->to_host(c);

  VectorXc expected = a * b;
  for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR_COMPLEX(c(i), expected(i), 1e-6);

  MatrixXc aa = MatrixXc::Random(m, n);
  this->backend->mul(TRANSPOSE::CONJ_TRANS, 3.0 * ONE, aa, b, ONE, c);
  this->backend->to_host(c);

  expected += 3.0 * (aa.adjoint() * b);
  for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR_COMPLEX(c(i), expected(i), 1e-6);
}

TYPED_TEST(BackendTest, hadamard_product) {
  constexpr Eigen::Index n = 1000;

  VectorXc a = VectorXc::Random(n);
  VectorXc b = VectorXc::Random(n);

  VectorXc c(n);
  this->backend->hadamard_product(a, b, c);
  this->backend->to_host(c);

  for (Eigen::Index i = 0; i < n; i++) {
    const auto expected = a(i) * b(i);
    ASSERT_NEAR_COMPLEX(c(i), expected, 1e-6);
  }
}

TYPED_TEST(BackendTest, reduce_col) {
  constexpr Eigen::Index m = 2000;
  constexpr Eigen::Index n = 4000;

  MatrixXc a = MatrixXc::Random(m, n);
  VectorXc b = VectorXc::Random(m);

  this->backend->reduce_col(a, b);
  this->backend->to_host(b);

  for (Eigen::Index i = 0; i < m; i++) {
    complex expected = ZERO;
    for (Eigen::Index k = 0; k < n; k++) expected += a(i, k);
    ASSERT_NEAR_COMPLEX(b(i), expected, 1e-6);
  }
}

TYPED_TEST(BackendTest, max_eigen_vector) {
  constexpr Eigen::Index n = 1000;

  auto gen_unitary = [](const Eigen::Index size) -> MatrixXc {
    const MatrixXc tmp = MatrixXc::Random(size, size);
    const MatrixXc hermite = tmp.adjoint() * tmp;
    return (complex(0.0, 1.0) * hermite).exp();
  };

  // generate matrix 'a' from given eigen value 'lambda' and eigen vectors 'u'
  MatrixXc u = gen_unitary(n);
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(0.0, 1.0);
  std::vector<double> lambda_vals;
  for (Eigen::Index i = 0; i < n; i++) lambda_vals.emplace_back(dist(engine));
  std::sort(lambda_vals.begin(), lambda_vals.end());  // maximum eigen value is placed at last
  MatrixXc lambda = MatrixXc::Zero(n, n);
  for (Eigen::Index i = 0; i < n; i++) lambda(i, i) = lambda_vals[i];
  MatrixXc a = u * lambda * u.adjoint();

  VectorXc b(n);
  this->backend->max_eigen_vector(a, b);
  this->backend->to_host(b);

  Eigen::MatrixXf::Index max_idx;
  u.col(n - 1).cwiseAbs2().maxCoeff(&max_idx);
  const auto k = b(max_idx) / u.col(n - 1)(max_idx);
  const MatrixXc expected = u.col(n - 1) * k;

  for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR_COMPLEX(b(i), expected(i), 1e-6);
}

TYPED_TEST(BackendTest, pseudo_inverse_svd) {
  constexpr auto n = 5000;
  constexpr auto m = 1000;
  MatrixXc a = MatrixXc::Random(m, n);

  MatrixXc b = MatrixXc::Zero(n, m);
  MatrixXc u(m, m);
  MatrixXc s(n, m);
  MatrixXc vt(n, n);
  MatrixXc buf = MatrixXc::Zero(n, m);
  MatrixXc tmp = a;
  this->backend->pseudo_inverse_svd(tmp, 0.0, u, s, vt, buf, b);

  MatrixXc c = MatrixXc::Zero(m, m);
  this->backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, a, b, ZERO, c);
  this->backend->to_host(c);

  for (Eigen::Index i = 0; i < m; i++)
    for (Eigen::Index j = 0; j < m; j++)
      if (i == j)
        ASSERT_NEAR_COMPLEX(c(i, j), ONE, 0.1);
      else
        ASSERT_NEAR_COMPLEX(c(i, j), ZERO, 0.1);
}
