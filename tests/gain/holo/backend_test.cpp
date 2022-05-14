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
  MatrixXc a(2, 2);
  a << complex(0, 1), complex(2, 3), complex(4, 5), complex(6, 7);

  MatrixXc b(2, 2);
  this->backend->copy_to(a, b);
  this->backend->to_host(b);

  ASSERT_EQ(b(0, 0), complex(0, 1));
  ASSERT_EQ(b(0, 1), complex(2, 3));
  ASSERT_EQ(b(1, 0), complex(4, 5));
  ASSERT_EQ(b(1, 1), complex(6, 7));
}

TYPED_TEST(BackendTest, conj) {
  VectorXc a(4);
  a << complex(0, 1), complex(2, 3), complex(4, 5), complex(6, 7);

  VectorXc b(4);
  this->backend->conj(a, b);
  this->backend->to_host(b);

  ASSERT_EQ(b(0), complex(0, -1));
  ASSERT_EQ(b(1), complex(2, -3));
  ASSERT_EQ(b(2), complex(4, -5));
  ASSERT_EQ(b(3), complex(6, -7));
}

TYPED_TEST(BackendTest, create_diagonal) {
  VectorXc a(2);
  a << complex(0.0, 1.0), complex(2.0, 3.0);

  MatrixXc b(2, 2);
  this->backend->create_diagonal(a, b);
  this->backend->to_host(b);

  ASSERT_EQ(b(0, 0), complex(0.0, 1.0));
  ASSERT_EQ(b(1, 0), ZERO);
  ASSERT_EQ(b(0, 1), ZERO);
  ASSERT_EQ(b(1, 1), complex(2.0, 3.0));
}

TYPED_TEST(BackendTest, set) {
  VectorXc a(1);

  this->backend->set(0, complex(10.0, 5.0), a);
  this->backend->to_host(a);

  ASSERT_EQ(a(0, 0), complex(10.0, 5.0));
}

TYPED_TEST(BackendTest, set_row) {
  constexpr size_t n = 10;
  MatrixXc a = MatrixXc::Zero(n, n);

  VectorXc b = VectorXc::Random(n);
  this->backend->set_row(b, 0, 6, 9, a);
  this->backend->to_host(a);

  ASSERT_EQ(a(0, 0), ZERO);
  ASSERT_EQ(a(0, 1), ZERO);
  ASSERT_EQ(a(0, 2), ZERO);
  ASSERT_EQ(a(0, 3), ZERO);
  ASSERT_EQ(a(0, 4), ZERO);
  ASSERT_EQ(a(0, 5), ZERO);
  ASSERT_EQ(a(0, 6), b(6));
  ASSERT_EQ(a(0, 7), b(7));
  ASSERT_EQ(a(0, 8), b(8));
  ASSERT_EQ(a(0, 9), ZERO);
}

TYPED_TEST(BackendTest, set_col) {
  constexpr size_t n = 10;
  MatrixXc a = MatrixXc::Zero(n, n);

  VectorXc b = VectorXc::Random(n);
  this->backend->set_col(b, 7, 2, 5, a);
  this->backend->to_host(a);

  ASSERT_EQ(a(0, 7), ZERO);
  ASSERT_EQ(a(1, 7), ZERO);
  ASSERT_EQ(a(2, 7), b(2));
  ASSERT_EQ(a(3, 7), b(3));
  ASSERT_EQ(a(4, 7), b(4));
  ASSERT_EQ(a(5, 7), ZERO);
  ASSERT_EQ(a(6, 7), ZERO);
  ASSERT_EQ(a(7, 7), ZERO);
  ASSERT_EQ(a(8, 7), ZERO);
  ASSERT_EQ(a(9, 7), ZERO);
}

TYPED_TEST(BackendTest, get_col) {
  MatrixXc a(2, 2);
  a << complex(0.0, 1.0), complex(2.0, 3.0), complex(4.0, 5.0), complex(6.0, 7.0);

  VectorXc b(2);

  this->backend->get_col(a, 0, b);
  this->backend->to_host(b);

  ASSERT_EQ(b(0), complex(0.0, 1.0));
  ASSERT_EQ(b(1), complex(4.0, 5.0));
}

TYPED_TEST(BackendTest, max_abs_element) {
  constexpr Eigen::Index n = 100;
  VectorXc a = VectorXc::Random(n);

  Eigen::Index idx = 0;
  a.cwiseAbs2().maxCoeff(&idx);

  ASSERT_EQ(this->backend->max_abs_element(a), a(idx));
}

TYPED_TEST(BackendTest, scale) {
  VectorXc a(4);
  a << complex(0, 1), complex(2, 3), complex(4, 5), complex(6, 7);

  this->backend->scale(complex(1, 1), a);
  this->backend->to_host(a);

  ASSERT_NEAR_COMPLEX(a(0, 0), complex(-1, 1), 1e-6);
  ASSERT_NEAR_COMPLEX(a(1, 0), complex(-1, 5), 1e-6);
  ASSERT_NEAR_COMPLEX(a(2, 0), complex(-1, 9), 1e-6);
  ASSERT_NEAR_COMPLEX(a(3, 0), complex(-1, 13), 1e-6);
}

TYPED_TEST(BackendTest, dot) {
  constexpr Eigen::Index n = 10;
  VectorXc a = VectorXc::Random(n);
  VectorXc b = VectorXc::Random(n);

  auto expected = complex(0, 0);
  for (Eigen::Index i = 0; i < n; i++) expected += std::conj(a(i)) * b(i);

  ASSERT_NEAR_COMPLEX(this->backend->dot(a, b), expected, 1e-6);
}

TYPED_TEST(BackendTest, mul_matrix) {
  MatrixXc a(2, 2);
  MatrixXc b(2, 2);
  a << complex(0, 1), complex(2, 3), complex(4, 5), complex(6, 7);
  b << complex(8, 9), complex(10, 11), complex(12, 13), complex(14, 15);

  MatrixXc c(2, 2);
  this->backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, a, b, ZERO, c);
  this->backend->to_host(c);

  ASSERT_NEAR_COMPLEX(c(0, 0), complex(-24, 70), 1e-6);
  ASSERT_NEAR_COMPLEX(c(0, 1), complex(-28, 82), 1e-6);
  ASSERT_NEAR_COMPLEX(c(1, 0), complex(-32, 238), 1e-6);
  ASSERT_NEAR_COMPLEX(c(1, 1), complex(-36, 282), 1e-6);

  this->backend->mul(TRANSPOSE::CONJ_TRANS, TRANSPOSE::TRANS, ZERO, a, b, ONE, c);
  this->backend->to_host(c);

  ASSERT_NEAR_COMPLEX(c(0, 0), complex(-24, 70), 1e-6);
  ASSERT_NEAR_COMPLEX(c(0, 1), complex(-28, 82), 1e-6);
  ASSERT_NEAR_COMPLEX(c(1, 0), complex(-32, 238), 1e-6);
  ASSERT_NEAR_COMPLEX(c(1, 1), complex(-36, 282), 1e-6);
}

TYPED_TEST(BackendTest, mul_vec) {
  MatrixXc a(2, 2);
  VectorXc b(2);
  a << complex(0, 1), complex(2, 3), complex(4, 5), complex(6, 7);
  b << complex(8, 9), complex(10, 11);

  VectorXc c(2);
  this->backend->mul(TRANSPOSE::NO_TRANS, ONE, a, b, ZERO, c);
  this->backend->to_host(c);

  ASSERT_NEAR_COMPLEX(c(0), complex(-22, 60), 1e-6);
  ASSERT_NEAR_COMPLEX(c(1), complex(-30, 212), 1e-6);

  this->backend->mul(TRANSPOSE::CONJ_TRANS, ONE, a, b, ONE, c);
  this->backend->to_host(c);

  ASSERT_NEAR_COMPLEX(c(0), complex(82, 46), 1e-6);
  ASSERT_NEAR_COMPLEX(c(1), complex(150, 202), 1e-6);
}

TYPED_TEST(BackendTest, max_eigen_vector) {
  constexpr Eigen::Index n = 5;

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
  constexpr auto n = 5;
  MatrixXc a = MatrixXc::Random(n, n);

  MatrixXc b = MatrixXc::Zero(n, n);
  MatrixXc u(n, n);
  MatrixXc s(n, n);
  MatrixXc vt(n, n);
  MatrixXc buf = MatrixXc::Zero(n, n);
  MatrixXc tmp = a;
  this->backend->pseudo_inverse_svd(tmp, 0.0, u, s, vt, buf, b);

  MatrixXc c = MatrixXc::Zero(n, n);
  this->backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, a, b, ZERO, c);
  this->backend->to_host(c);

  for (Eigen::Index i = 0; i < n; i++)
    for (Eigen::Index j = 0; j < n; j++)
      if (i == j)
        ASSERT_NEAR_COMPLEX(c(i, j), ONE, 0.1);
      else
        ASSERT_NEAR_COMPLEX(c(i, j), ZERO, 0.1);
}
