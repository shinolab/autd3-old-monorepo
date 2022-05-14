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

template <typename T>
std::vector<double> random_vector(T n, const double minimum = -1.0, const double maximum = 1.0) {
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(minimum, maximum);
  std::vector<double> v;
  v.reserve(n);
  for (T i = 0; i < n; ++i) v.emplace_back(dist(engine));
  return v;
}

template <typename T>
std::vector<complex> random_vector_complex(T n, const double minimum = -1.0, const double maximum = 1.0) {
  const auto re = random_vector(n, minimum, maximum);
  const auto im = random_vector(n, minimum, maximum);
  std::vector<complex> v;
  v.reserve(n);
  for (T i = 0; i < n; ++i) v.emplace_back(complex(re[i], im[i]));
  return v;
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

// TYPED_TEST(BackendTest, conj) {
//   auto a = this->_pool.rent_c("a", 2, 2);
//   a->copy_from({complex(0, 1), complex(2, 3), complex(4, 5), complex(6, 7)});
//
//   auto b = this->_pool.rent_c("b", 2, 2);
//
//   b->conj(a);
//
//   ASSERT_EQ(b->at(0, 0), complex(0, -1));
//   ASSERT_EQ(b->at(1, 0), complex(2, -3));
//   ASSERT_EQ(b->at(0, 1), complex(4, -5));
//   ASSERT_EQ(b->at(1, 1), complex(6, -7));
// }
//
// TYPED_TEST(BackendTest, pseudo_inverse_svd_c) {
//   constexpr auto n = 5;
//   auto a = this->_pool.rent_c("a", n, n);
//   a->copy_from(random_vector_complex(n * n));
//
//   auto b = this->_pool.rent_c("b", n, n);
//   auto u = this->_pool.rent_c("u", n, n);
//   auto s = this->_pool.rent_c("s", n, n);
//   auto vt = this->_pool.rent_c("vt", n, n);
//   auto buf = this->_pool.rent_c("buf", n, n);
//   auto mat = this->_pool.rent_c("mat", n, n);
//   mat->copy_from(a);
//   b->pseudo_inverse_svd(mat, 0.0, u, s, vt, buf);
//
//   auto c = this->_pool.rent_c("c", n, n);
//   c->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, 1.0, a, b, 0.0);
//
//   for (Eigen::Index i = 0; i < n; i++)
//     for (Eigen::Index j = 0; j < n; j++) {
//       if (i == j)
//         ASSERT_NEAR_COMPLEX(c->at(i, j), ONE, 0.1);
//       else
//         ASSERT_NEAR_COMPLEX(c->at(i, j), ZERO, 0.1);
//     }
// }
//
// TYPED_TEST(BackendTest, max_eigen_vector) {
//   constexpr Eigen::Index n = 5;
//
//   auto gen_unitary = [](const Eigen::Index size) {
//     Eigen::Matrix<complex, -1, -1, Eigen::ColMajor> tmp(size, size);
//     const auto rand = random_vector_complex(size * size);
//     std::memcpy(tmp.data(), rand.data(), rand.size() * sizeof(complex));
//
//     const Eigen::Matrix<complex, -1, -1, Eigen::ColMajor> hermite = tmp.adjoint() * tmp;
//     Eigen::Matrix<complex, -1, -1, Eigen::ColMajor> u = (complex(0.0, 1.0) * hermite).exp();
//     return u;
//   };
//
//   // generate matrix 'a' from given eigen value 'lambda' and eigen vectors 'u'
//   Eigen::Matrix<complex, -1, -1, Eigen::ColMajor> u = gen_unitary(n);
//   auto lambda_vals = random_vector(n, 1.0, 10.0);
//   std::sort(lambda_vals.begin(), lambda_vals.end());  // maximum eigen value is placed at last
//   Eigen::Matrix<complex, -1, -1, Eigen::ColMajor> lambda = Eigen::Matrix<complex, -1, -1, Eigen::ColMajor>::Zero(n, n);
//   for (Eigen::Index i = 0; i < n; i++) lambda(i, i) = lambda_vals[i];
//   Eigen::Matrix<complex, -1, -1, Eigen::ColMajor> a_vals = u * lambda * u.adjoint();
//
//   auto a = this->_pool.rent_c("a", n, n);
//   a->copy_from(a_vals.data());
//   const auto b = this->_pool.rent_c("b", n, 1);
//   a->max_eigen_vector(b);
//
//   Eigen::MatrixXf::Index max_idx;
//   u.col(n - 1).cwiseAbs2().maxCoeff(&max_idx);
//   const auto k = b->at(max_idx, 0) / u.col(n - 1)(max_idx);
//   const Eigen::Matrix<complex, -1, 1, Eigen::ColMajor> expected = u.col(n - 1) * k;
//
//   for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR_COMPLEX(b->at(i, 0), expected(i), 1e-6);
// }
//
// TYPED_TEST(BackendTest, matrix_mul_c) {
//   auto a = this->_pool.rent_c("a", 2, 2);
//   auto b = this->_pool.rent_c("b", 2, 2);
//   a->copy_from({complex(0, 1), complex(4, 5), complex(2, 3), complex(6, 7)});
//   b->copy_from({complex(8, 9), complex(12, 13), complex(10, 11), complex(14, 15)});
//
//   auto c = this->_pool.rent_c("c", 2, 2);
//
//   c->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, a, b, ZERO);
//
//   ASSERT_NEAR_COMPLEX(c->at(0, 0), complex(-24, 70), 1e-6);
//   ASSERT_NEAR_COMPLEX(c->at(0, 1), complex(-28, 82), 1e-6);
//   ASSERT_NEAR_COMPLEX(c->at(1, 0), complex(-32, 238), 1e-6);
//   ASSERT_NEAR_COMPLEX(c->at(1, 1), complex(-36, 282), 1e-6);
//
//   c->mul(TRANSPOSE::CONJ_TRANS, TRANSPOSE::TRANS, ZERO, a, b, ONE);
//
//   ASSERT_NEAR_COMPLEX(c->at(0, 0), complex(-24, 70), 1e-6);
//   ASSERT_NEAR_COMPLEX(c->at(0, 1), complex(-28, 82), 1e-6);
//   ASSERT_NEAR_COMPLEX(c->at(1, 0), complex(-32, 238), 1e-6);
//   ASSERT_NEAR_COMPLEX(c->at(1, 1), complex(-36, 282), 1e-6);
// }
//
// TYPED_TEST(BackendTest, dot_c) {
//   constexpr Eigen::Index n = 10;
//   auto a_vals = random_vector_complex(n, 1.0, 10.0);
//   auto b_vals = random_vector_complex(n, 1.0, 10.0);
//
//   auto expected = complex(0, 0);
//   for (Eigen::Index i = 0; i < n; i++) expected += std::conj(a_vals[i]) * b_vals[i];
//
//   auto a = this->_pool.rent_c("a", n, 1);
//   a->copy_from(a_vals);
//   auto b = this->_pool.rent_c("b", n, 1);
//   b->copy_from(b_vals);
//
//   ASSERT_NEAR_COMPLEX(a->dot(b), expected, 1e-6);
// }
//
// TYPED_TEST(BackendTest, max_element_c) {
//   constexpr Eigen::Index n = 100;
//   auto vals = random_vector(n, 0.0, 10.0);
//   std::sort(vals.begin(), vals.end());
//   std::vector<complex> vals_c;
//   vals_c.reserve(vals.size());
//   for (const auto v : vals) vals_c.emplace_back(v, 0.0);
//   auto v = this->_pool.rent_c("v", n, 1);
//   v->copy_from(vals_c);
//
//   ASSERT_NEAR(v->max_element(), vals[n - 1], 1e-6);
// }
//
// TYPED_TEST(BackendTest, set_c) {
//   auto a = this->_pool.rent_c("a", 1, 1);
//   a->set(0, 0, complex(10.0, 5.0));
//
//   ASSERT_EQ(a->at(0, 0), complex(10.0, 5.0));
// }
//
// TYPED_TEST(BackendTest, set_col_c) {
//   constexpr size_t n = 10;
//   auto a = this->_pool.rent_c("a", n, n);
//   a->fill(0);
//
//   auto b = this->_pool.rent_c("b", n, 1);
//   auto vals = random_vector_complex(n);
//   b->copy_from(vals);
//
//   a->set_col(9, 7, 10, b);
//   ASSERT_EQ(a->at(0, 9), ZERO);
//   ASSERT_EQ(a->at(1, 9), ZERO);
//   ASSERT_EQ(a->at(2, 9), ZERO);
//   ASSERT_EQ(a->at(3, 9), ZERO);
//   ASSERT_EQ(a->at(4, 9), ZERO);
//   ASSERT_EQ(a->at(5, 9), ZERO);
//   ASSERT_EQ(a->at(6, 9), ZERO);
//   ASSERT_EQ(a->at(7, 9), vals[7]);
//   ASSERT_EQ(a->at(8, 9), vals[8]);
//   ASSERT_EQ(a->at(9, 9), vals[9]);
// }
//
// TYPED_TEST(BackendTest, set_row_c) {
//   constexpr size_t n = 10;
//   auto a = this->_pool.rent_c("a", n, n);
//   a->fill(0);
//
//   auto b = this->_pool.rent_c("b", n, 1);
//   auto vals = random_vector_complex(n);
//   b->copy_from(vals);
//
//   a->set_row(0, 7, 10, b);
//   ASSERT_EQ(a->at(0, 0), ZERO);
//   ASSERT_EQ(a->at(0, 1), ZERO);
//   ASSERT_EQ(a->at(0, 2), ZERO);
//   ASSERT_EQ(a->at(0, 3), ZERO);
//   ASSERT_EQ(a->at(0, 4), ZERO);
//   ASSERT_EQ(a->at(0, 5), ZERO);
//   ASSERT_EQ(a->at(0, 6), ZERO);
//   ASSERT_EQ(a->at(0, 7), vals[7]);
//   ASSERT_EQ(a->at(0, 8), vals[8]);
//   ASSERT_EQ(a->at(0, 9), vals[9]);
// }
//
// TYPED_TEST(BackendTest, get_col_c) {
//   auto a = this->_pool.rent_c("a", 2, 2);
//   a->copy_from({complex(0.0, 1.0), complex(2.0, 3.0), complex(4.0, 5.0), complex(6.0, 71.0)});
//
//   auto b = this->_pool.rent_c("b", 2, 1);
//
//   b->get_col(a, 0);
//
//   ASSERT_EQ(b->at(0, 0), complex(0.0, 1.0));
//   ASSERT_EQ(b->at(1, 0), complex(2.0, 3.0));
// }
//
// TYPED_TEST(BackendTest, create_diagonal_c) {
//   auto a = this->_pool.rent_c("a", 2, 1);
//   a->copy_from({complex(0.0, 1.0), complex(2.0, 3.0)});
//
//   auto b = this->_pool.rent_c("b", 2, 2);
//
//   b->create_diagonal(a);
//
//   ASSERT_EQ(b->at(0, 0), complex(0.0, 1.0));
//   ASSERT_EQ(b->at(1, 0), ZERO);
//   ASSERT_EQ(b->at(0, 1), ZERO);
//   ASSERT_EQ(b->at(1, 1), complex(2.0, 3.0));
// }
