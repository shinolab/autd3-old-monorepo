// File: backend_base.hpp
// Project: gain
// Created Date: 10/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

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

#include "autd3/gain/backend.hpp"
#include "test_utils.hpp"

constexpr Eigen::Index TEST_SIZE = 10;

#ifdef AUTD3_USE_SINGLE_FLOAT
constexpr autd3::driver::autd3_float_t EPS = 1e-3f;
#else
constexpr autd3::driver::autd3_float_t EPS = 1e-6;
#endif

using autd3::gain::holo::complex;
using autd3::gain::holo::MatrixXc;
using autd3::gain::holo::MatrixXd;
using autd3::gain::holo::ONE;
using autd3::gain::holo::Transpose;
using autd3::gain::holo::VectorXc;
using autd3::gain::holo::VectorXd;
using autd3::gain::holo::ZERO;

#define AUTD3_BACKEND_TEST(TestName, BackendType)                                                                                    \
                                                                                                                                     \
  TEST(TestName, copy_to) {                                                                                                          \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXc b(m, n);                                                                                                                \
    backend->copy_to(a, b);                                                                                                          \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_EQ(a(i, j), b(i, j));                                                              \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, copy_to_real) {                                                                                                     \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXd a = MatrixXd::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXd b(m, n);                                                                                                                \
    backend->copy_to(a, b);                                                                                                          \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_EQ(a(i, j), b(i, j));                                                              \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, copy_to_vec_real) {                                                                                                 \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd a = VectorXd::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXd b(m);                                                                                                                   \
    backend->copy_to(a, b);                                                                                                          \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_EQ(a(i), b(i));                                                                      \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, copy_to_vec) {                                                                                                      \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXc b(m);                                                                                                                   \
    backend->copy_to(a, b);                                                                                                          \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_EQ(a(i), b(i));                                                                      \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, real) {                                                                                                             \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXd b(m, n);                                                                                                                \
    backend->real(a, b);                                                                                                             \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_EQ(a(i, j).real(), b(i, j));                                                       \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, imag) {                                                                                                             \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXd b(m, n);                                                                                                                \
    backend->imag(a, b);                                                                                                             \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_EQ(a(i, j).imag(), b(i, j));                                                       \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, make_complex) {                                                                                                     \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd re = VectorXd::Random(m);                                                                                               \
    VectorXd im = VectorXd::Random(m);                                                                                               \
                                                                                                                                     \
    VectorXc a(m);                                                                                                                   \
    backend->make_complex(re, im, a);                                                                                                \
    backend->to_host(a);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_EQ(a(i), complex(re(i), im(i)));                                                     \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, abs) {                                                                                                              \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
                                                                                                                                     \
    VectorXd b(n);                                                                                                                   \
    backend->abs(a, b);                                                                                                              \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(std::abs(a(i)), b(i), EPS);                                                     \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, abs_c) {                                                                                                            \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
                                                                                                                                     \
    VectorXc b(n);                                                                                                                   \
    backend->abs(a, b);                                                                                                              \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(std::abs(a(i)), b(i).real(), EPS);                                              \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, sqrt) {                                                                                                             \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd a = VectorXd::Ones(n) + VectorXd::Random(n);                                                                            \
                                                                                                                                     \
    VectorXd b(n);                                                                                                                   \
    backend->sqrt(a, b);                                                                                                             \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(std::sqrt(a(i)), b(i), EPS);                                                    \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, conj) {                                                                                                             \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
                                                                                                                                     \
    VectorXc b(n);                                                                                                                   \
    backend->conj(a, b);                                                                                                             \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_EQ(std::conj(a(i)), b(i));                                                           \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, arg) {                                                                                                              \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
                                                                                                                                     \
    backend->arg(a, a);                                                                                                              \
    backend->to_host(a);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(std::abs(a(i)), 1, EPS);                                                        \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, reciprocal) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = 2 * VectorXc::Ones(n) + VectorXc::Random(n);                                                                        \
                                                                                                                                     \
    VectorXc b(n);                                                                                                                   \
    backend->reciprocal(a, b);                                                                                                       \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) {                                                                                           \
      const auto expected = static_cast<autd3::driver::autd3_float_t>(1) / a(i);                                                     \
      ASSERT_NEAR_COMPLEX(expected, b(i), EPS);                                                                                      \
    }                                                                                                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, exp) {                                                                                                              \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXc b(m);                                                                                                                   \
    backend->exp(a, b);                                                                                                              \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_NEAR_COMPLEX(std::exp(a(i)), b(i), EPS);                                             \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, pow) {                                                                                                              \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd a = VectorXd::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXd b(m);                                                                                                                   \
    backend->pow(a, 2, b);                                                                                                           \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_NEAR(std::pow(a(i), 2), b(i), EPS);                                                  \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, create_diagonal) {                                                                                                  \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(m);                                                                                                \
                                                                                                                                     \
    MatrixXc b = MatrixXc::Zero(m, n);                                                                                               \
    backend->create_diagonal(a, b);                                                                                                  \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < m; i++)                                                                                                      \
      for (int j = 0; j < n; j++)                                                                                                    \
        if (i == j)                                                                                                                  \
          ASSERT_EQ(b(i, j), a(i));                                                                                                  \
        else                                                                                                                         \
          ASSERT_EQ(b(i, j), ZERO);                                                                                                  \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, get_diagonal) {                                                                                                     \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    VectorXc b(m);                                                                                                                   \
    backend->get_diagonal(a, b);                                                                                                     \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < m; i++) ASSERT_EQ(b(i), a(i, i));                                                                            \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, get_diagonal_real) {                                                                                                \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXd a = MatrixXd::Random(m, n);                                                                                             \
                                                                                                                                     \
    VectorXd b(m);                                                                                                                   \
    backend->get_diagonal(a, b);                                                                                                     \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < m; i++) ASSERT_EQ(b(i), a(i, i));                                                                            \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, set) {                                                                                                              \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a(m);                                                                                                                   \
                                                                                                                                     \
    backend->set(m / 2, complex(10, 5), a);                                                                                          \
    backend->to_host(a);                                                                                                             \
                                                                                                                                     \
    ASSERT_EQ(a(m / 2), complex(10, 5));                                                                                             \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, set_row) {                                                                                                          \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Zero(m, n);                                                                                               \
                                                                                                                                     \
    VectorXc b = VectorXc::Random(n);                                                                                                \
    backend->set_row(b, m / 2, 6, 9, a);                                                                                             \
    backend->to_host(a);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < m; i++)                                                                                                      \
      for (int j = 0; j < n; j++)                                                                                                    \
        if (i == m / 2 && (6 <= j && j < 9))                                                                                         \
          ASSERT_EQ(a(i, j), b(j));                                                                                                  \
        else                                                                                                                         \
          ASSERT_EQ(a(i, j), ZERO);                                                                                                  \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, set_col) {                                                                                                          \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Zero(m, n);                                                                                               \
                                                                                                                                     \
    VectorXc b = VectorXc::Random(m);                                                                                                \
    backend->set_col(b, 7, 2, 5, a);                                                                                                 \
    backend->to_host(a);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < m; i++)                                                                                                      \
      for (int j = 0; j < n; j++)                                                                                                    \
        if (j == 7 && (2 <= i && i < 5))                                                                                             \
          ASSERT_EQ(a(i, j), b(i));                                                                                                  \
        else                                                                                                                         \
          ASSERT_EQ(a(i, j), ZERO);                                                                                                  \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, get_col) {                                                                                                          \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    VectorXc b(m);                                                                                                                   \
                                                                                                                                     \
    backend->get_col(a, n / 2, b);                                                                                                   \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < m; i++) ASSERT_EQ(a(i, n / 2), b(i));                                                                        \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, concal_col) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index k = 3 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
    MatrixXc b = MatrixXc::Random(m, k);                                                                                             \
                                                                                                                                     \
    MatrixXc c(m, n + k);                                                                                                            \
    backend->concat_col(a, b, c);                                                                                                    \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < m; i++)                                                                                                      \
      for (int j = 0; j < n; j++) ASSERT_EQ(c(i, j), a(i, j));                                                                       \
    for (int i = 0; i < m; i++)                                                                                                      \
      for (int j = 0; j < k; j++) ASSERT_EQ(c(i, n + j), b(i, j));                                                                   \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, concal_row_vec) {                                                                                                   \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index k = 3 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
    VectorXc b = VectorXc::Random(k);                                                                                                \
                                                                                                                                     \
    VectorXc c(n + k);                                                                                                               \
    backend->concat_row(a, b, c);                                                                                                    \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < n; i++) ASSERT_EQ(c(i), a(i));                                                                               \
    for (int i = 0; i < k; i++) ASSERT_EQ(c(i + n), b(i));                                                                           \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, concal_row) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index k = 3 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(n, m);                                                                                             \
    MatrixXc b = MatrixXc::Random(k, m);                                                                                             \
                                                                                                                                     \
    MatrixXc c(n + k, m);                                                                                                            \
    backend->concat_row(a, b, c);                                                                                                    \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    for (int i = 0; i < n; i++)                                                                                                      \
      for (int j = 0; j < m; j++) ASSERT_EQ(c(i, j), a(i, j));                                                                       \
    for (int i = 0; i < k; i++)                                                                                                      \
      for (int j = 0; j < m; j++) ASSERT_EQ(c(i + n, j), b(i, j));                                                                   \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, reduce_col) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index n = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXd a = MatrixXd::Random(m, n);                                                                                             \
                                                                                                                                     \
    VectorXd b(m);                                                                                                                   \
    backend->reduce_col(a, b);                                                                                                       \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) {                                                                                           \
      autd3::driver::autd3_float_t expected = 0;                                                                                     \
      for (Eigen::Index k = 0; k < n; k++) expected += a(i, k);                                                                      \
      ASSERT_NEAR(expected, b(i), EPS);                                                                                              \
    }                                                                                                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, max_abs_element) {                                                                                                  \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
                                                                                                                                     \
    Eigen::Index idx = 0;                                                                                                            \
    a.cwiseAbs2().maxCoeff(&idx);                                                                                                    \
                                                                                                                                     \
    ASSERT_EQ(backend->max_abs_element(a), a(idx));                                                                                  \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, max_element) {                                                                                                      \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd a = VectorXd::Random(n);                                                                                                \
                                                                                                                                     \
    const auto expected = a.maxCoeff();                                                                                              \
                                                                                                                                     \
    ASSERT_EQ(backend->max_element(a), expected);                                                                                    \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, scale) {                                                                                                            \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
    VectorXc a_tmp = a;                                                                                                              \
                                                                                                                                     \
    backend->scale(complex(1, 1), a);                                                                                                \
    backend->to_host(a);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) {                                                                                           \
      const auto expected = complex(1, 1) * a_tmp(i);                                                                                \
      ASSERT_NEAR_COMPLEX(expected, a(i), EPS);                                                                                      \
    }                                                                                                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, scale_real) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd a = VectorXd::Random(n);                                                                                                \
    VectorXd a_tmp = a;                                                                                                              \
                                                                                                                                     \
    backend->scale(2, a);                                                                                                            \
    backend->to_host(a);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) {                                                                                           \
      const auto expected = 2 * a_tmp(i);                                                                                            \
      ASSERT_NEAR(expected, a(i), EPS);                                                                                              \
    }                                                                                                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, dot) {                                                                                                              \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
    VectorXc b = VectorXc::Random(n);                                                                                                \
                                                                                                                                     \
    auto expected = complex(0, 0);                                                                                                   \
    for (Eigen::Index i = 0; i < n; i++) expected += std::conj(a(i)) * b(i);                                                         \
                                                                                                                                     \
    ASSERT_NEAR_COMPLEX(backend->dot(a, b), expected, EPS);                                                                          \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, dot_real) {                                                                                                         \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd a = VectorXd::Random(n);                                                                                                \
    VectorXd b = VectorXd::Random(n);                                                                                                \
                                                                                                                                     \
    autd3::driver::autd3_float_t expected = 0;                                                                                       \
    for (Eigen::Index i = 0; i < n; i++) expected += a(i) * b(i);                                                                    \
                                                                                                                                     \
    ASSERT_NEAR(backend->dot(a, b), expected, EPS);                                                                                  \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, add_vector) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXc b = VectorXc::Zero(m);                                                                                                  \
    backend->add(ONE, a, b);                                                                                                         \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    VectorXc expected = a;                                                                                                           \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_EQ(b(i), expected(i));                                                               \
                                                                                                                                     \
    VectorXc aa = VectorXc::Random(m);                                                                                               \
    backend->add(complex(2, 0), aa, b);                                                                                              \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    expected += complex(2, 0) * aa;                                                                                                  \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_NEAR_COMPLEX(b(i), expected(i), EPS);                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, add_vector_real) {                                                                                                  \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXd a = VectorXd::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXd b = VectorXd::Zero(m);                                                                                                  \
    backend->add(1, a, b);                                                                                                           \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    VectorXd expected = a;                                                                                                           \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_EQ(b(i), expected(i));                                                               \
                                                                                                                                     \
    VectorXd aa = VectorXd::Random(m);                                                                                               \
    backend->add(2, aa, b);                                                                                                          \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    expected += 2 * aa;                                                                                                              \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_NEAR(b(i), expected(i), EPS);                                                        \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, add_matrix) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXc b = MatrixXc::Zero(m, n);                                                                                               \
    backend->add(ONE, a, b);                                                                                                         \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    MatrixXc expected = a;                                                                                                           \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_EQ(b(i, j), expected(i, j));                                                       \
                                                                                                                                     \
    MatrixXc aa = MatrixXc::Random(m, n);                                                                                            \
    backend->add(complex(2, 0), aa, b);                                                                                              \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    expected += complex(2, 0) * aa;                                                                                                  \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_NEAR_COMPLEX(b(i, j), expected(i, j), EPS);                                        \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, add_matrix_real) {                                                                                                  \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXd a = MatrixXd::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXd b = MatrixXd::Zero(m, n);                                                                                               \
    backend->add(1, a, b);                                                                                                           \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    MatrixXd expected = a;                                                                                                           \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_EQ(b(i, j), expected(i, j));                                                       \
                                                                                                                                     \
    MatrixXd aa = MatrixXd::Random(m, n);                                                                                            \
    backend->add(2, aa, b);                                                                                                          \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    expected += 2 * aa;                                                                                                              \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_NEAR(b(i, j), expected(i, j), EPS);                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, mul_matrix) {                                                                                                       \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index k = 3 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(n, m);                                                                                             \
    MatrixXc b = MatrixXc::Random(m, m);                                                                                             \
                                                                                                                                     \
    MatrixXc c = MatrixXc::Zero(n, m);                                                                                               \
    backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, a, b, ZERO, c);                                                        \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    MatrixXc expected = a * b;                                                                                                       \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++)                                                                                             \
      for (Eigen::Index j = 0; j < m; j++) ASSERT_NEAR_COMPLEX(c(i, j), expected(i, j), EPS);                                        \
                                                                                                                                     \
    MatrixXc aa = MatrixXc::Random(k, n);                                                                                            \
    MatrixXc bb = MatrixXc::Random(m, k);                                                                                            \
    backend->mul(Transpose::ConjTrans, Transpose::Trans, ONE, aa, bb, ONE, c);                                                       \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    expected += aa.adjoint() * bb.transpose();                                                                                       \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++)                                                                                             \
      for (Eigen::Index j = 0; j < m; j++) ASSERT_NEAR_COMPLEX(c(i, j), expected(i, j), EPS);                                        \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, mul_vec) {                                                                                                          \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(n, m);                                                                                             \
    VectorXc b = VectorXc::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXc c = VectorXc::Zero(n);                                                                                                  \
    backend->mul(Transpose::NoTrans, ONE, a, b, ZERO, c);                                                                            \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    VectorXc expected = a * b;                                                                                                       \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR_COMPLEX(c(i), expected(i), EPS);                                                \
                                                                                                                                     \
    MatrixXc aa = MatrixXc::Random(m, n);                                                                                            \
    backend->mul(Transpose::ConjTrans, ONE, aa, b, ONE, c);                                                                          \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    expected += aa.adjoint() * b;                                                                                                    \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR_COMPLEX(c(i), expected(i), EPS);                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, mul_matrix_real) {                                                                                                  \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index k = 3 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXd a = MatrixXd::Random(n, m);                                                                                             \
    MatrixXd b = MatrixXd::Random(m, m);                                                                                             \
                                                                                                                                     \
    MatrixXd c = MatrixXd::Zero(n, m);                                                                                               \
    backend->mul(Transpose::NoTrans, Transpose::NoTrans, autd3::driver::autd3_float_t{1}, a, b, autd3::driver::autd3_float_t{0}, c); \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    MatrixXd expected = a * b;                                                                                                       \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++)                                                                                             \
      for (Eigen::Index j = 0; j < m; j++) ASSERT_NEAR(c(i, j), expected(i, j), EPS);                                                \
                                                                                                                                     \
    MatrixXd aa = MatrixXd::Random(k, n);                                                                                            \
    MatrixXd bb = MatrixXd::Random(m, k);                                                                                            \
    backend->mul(Transpose::Trans, Transpose::Trans, autd3::driver::autd3_float_t{2}, aa, bb, autd3::driver::autd3_float_t{1}, c);   \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    expected += 2 * (aa.transpose() * bb.transpose());                                                                               \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++)                                                                                             \
      for (Eigen::Index j = 0; j < m; j++) ASSERT_NEAR(c(i, j), expected(i, j), EPS);                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, mul_vec_real) {                                                                                                     \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXd a = MatrixXd::Random(n, m);                                                                                             \
    VectorXd b = VectorXd::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXd c = VectorXd::Zero(n);                                                                                                  \
    backend->mul(Transpose::NoTrans, autd3::driver::autd3_float_t{1}, a, b, autd3::driver::autd3_float_t{0}, c);                     \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    VectorXd expected = a * b;                                                                                                       \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(c(i), expected(i), EPS);                                                        \
                                                                                                                                     \
    MatrixXd aa = MatrixXd::Random(m, n);                                                                                            \
    backend->mul(Transpose::Trans, autd3::driver::autd3_float_t{3}, aa, b, autd3::driver::autd3_float_t{1}, c);                      \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    expected += 3 * (aa.transpose() * b);                                                                                            \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR(c(i), expected(i), EPS);                                                        \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, hadamard_product) {                                                                                                 \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    VectorXc a = VectorXc::Random(n);                                                                                                \
    VectorXc b = VectorXc::Random(n);                                                                                                \
                                                                                                                                     \
    VectorXc c(n);                                                                                                                   \
    backend->hadamard_product(a, b, c);                                                                                              \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) {                                                                                           \
      const auto expected = a(i) * b(i);                                                                                             \
      ASSERT_NEAR_COMPLEX(c(i), expected, EPS);                                                                                      \
    }                                                                                                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, hadamard_product_mat) {                                                                                             \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
    constexpr Eigen::Index m = 2 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
    MatrixXc b = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXc c(m, n);                                                                                                                \
    backend->hadamard_product(a, b, c);                                                                                              \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < n; j++) ASSERT_NEAR_COMPLEX(c(i, j), (a(i, j) * b(i, j)), EPS);                                   \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, solvet) {                                                                                                           \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    MatrixXd tmp = MatrixXd::Random(m, m);                                                                                           \
    MatrixXd a = tmp * tmp.transpose();                                                                                              \
                                                                                                                                     \
    VectorXd x = VectorXd::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXd b = a * x;                                                                                                              \
                                                                                                                                     \
    backend->solvet(a, b);                                                                                                           \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_NEAR(b(i), x(i), EPS);                                                               \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, solveh) {                                                                                                           \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index m = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    const MatrixXc tmp = MatrixXc::Random(m, m);                                                                                     \
    MatrixXc a = tmp * tmp.adjoint();                                                                                                \
                                                                                                                                     \
    VectorXc x = VectorXc::Random(m);                                                                                                \
                                                                                                                                     \
    VectorXc b = a * x;                                                                                                              \
                                                                                                                                     \
    backend->solveh(a, b);                                                                                                           \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++) ASSERT_NEAR_COMPLEX(b(i), x(i), EPS);                                                       \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, max_eigen_vector) {                                                                                                 \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr Eigen::Index n = 1 * TEST_SIZE;                                                                                        \
                                                                                                                                     \
    auto gen_unitary = [](const Eigen::Index size) -> MatrixXc {                                                                     \
      const MatrixXc tmp = MatrixXc::Random(size, size);                                                                             \
      const MatrixXc hermite = tmp.adjoint() * tmp;                                                                                  \
      return (complex(0, 1) * hermite).exp();                                                                                        \
    };                                                                                                                               \
                                                                                                                                     \
    MatrixXc u = gen_unitary(n);                                                                                                     \
    std::random_device seed_gen;                                                                                                     \
    std::mt19937 engine(seed_gen());                                                                                                 \
    std::uniform_real_distribution<autd3::driver::autd3_float_t> dist(0, 1);                                                         \
    std::vector<autd3::driver::autd3_float_t> lambda_vals;                                                                           \
    for (Eigen::Index i = 0; i < n; i++) lambda_vals.emplace_back(dist(engine));                                                     \
    std::sort(lambda_vals.begin(), lambda_vals.end());                                                                               \
    MatrixXc lambda = MatrixXc::Zero(n, n);                                                                                          \
    for (Eigen::Index i = 0; i < n; i++) lambda(i, i) = lambda_vals[i];                                                              \
    MatrixXc a = u * lambda * u.adjoint();                                                                                           \
                                                                                                                                     \
    VectorXc b(n);                                                                                                                   \
    backend->max_eigen_vector(a, b);                                                                                                 \
    backend->to_host(b);                                                                                                             \
                                                                                                                                     \
    Eigen::MatrixXf::Index max_idx;                                                                                                  \
    u.col(n - 1).cwiseAbs2().maxCoeff(&max_idx);                                                                                     \
    const auto k = b(max_idx) / u.col(n - 1)(max_idx);                                                                               \
    const MatrixXc expected = u.col(n - 1) * k;                                                                                      \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < n; i++) ASSERT_NEAR_COMPLEX(b(i), expected(i), EPS);                                                \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, pseudo_inverse_svd) {                                                                                               \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr auto n = 5 * TEST_SIZE;                                                                                                \
    constexpr auto m = 1 * TEST_SIZE;                                                                                                \
    MatrixXc a = MatrixXc::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXc b = MatrixXc::Zero(n, m);                                                                                               \
    MatrixXc u(m, m);                                                                                                                \
    MatrixXc s(n, m);                                                                                                                \
    MatrixXc vt(n, n);                                                                                                               \
    MatrixXc buf = MatrixXc::Zero(n, m);                                                                                             \
    MatrixXc tmp = a;                                                                                                                \
    backend->pseudo_inverse_svd(tmp, 0, u, s, vt, buf, b);                                                                           \
                                                                                                                                     \
    MatrixXc c = MatrixXc::Zero(m, m);                                                                                               \
    backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, a, b, ZERO, c);                                                        \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < m; j++)                                                                                           \
        if (i == j)                                                                                                                  \
          ASSERT_NEAR_COMPLEX(c(i, j), ONE, 0.1);                                                                                    \
        else                                                                                                                         \
          ASSERT_NEAR_COMPLEX(c(i, j), ZERO, 0.1);                                                                                   \
  }                                                                                                                                  \
                                                                                                                                     \
  TEST(TestName, pseudo_inverse_svd_real) {                                                                                          \
    auto backend = BackendType::create();                                                                                            \
    backend->init();                                                                                                                 \
                                                                                                                                     \
    constexpr auto n = 5 * TEST_SIZE;                                                                                                \
    constexpr auto m = 1 * TEST_SIZE;                                                                                                \
    MatrixXd a = MatrixXd::Random(m, n);                                                                                             \
                                                                                                                                     \
    MatrixXd b = MatrixXd::Zero(n, m);                                                                                               \
    MatrixXd u(m, m);                                                                                                                \
    MatrixXd s(n, m);                                                                                                                \
    MatrixXd vt(n, n);                                                                                                               \
    MatrixXd buf = MatrixXd::Zero(n, m);                                                                                             \
    MatrixXd tmp = a;                                                                                                                \
    backend->pseudo_inverse_svd(tmp, 0, u, s, vt, buf, b);                                                                           \
                                                                                                                                     \
    MatrixXd c = MatrixXd::Zero(m, m);                                                                                               \
    backend->mul(Transpose::NoTrans, Transpose::NoTrans, autd3::driver::autd3_float_t{1}, a, b, autd3::driver::autd3_float_t{0}, c); \
    backend->to_host(c);                                                                                                             \
                                                                                                                                     \
    for (Eigen::Index i = 0; i < m; i++)                                                                                             \
      for (Eigen::Index j = 0; j < m; j++)                                                                                           \
        if (i == j)                                                                                                                  \
          ASSERT_NEAR(c(i, j), 1, 0.1);                                                                                              \
                                                                                                                                     \
        else                                                                                                                         \
          ASSERT_NEAR(c(i, j), 0, 0.1);                                                                                              \
  }
