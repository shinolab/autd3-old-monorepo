// File: test_utils.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#define ASSERT_NEAR_COMPLEX(a, b, eps)    \
  do {                                    \
    ASSERT_NEAR(a.real(), b.real(), eps); \
    ASSERT_NEAR(a.imag(), b.imag(), eps); \
  } while (0)

#define ASSERT_NEAR_VECTOR3(a, b, eps) \
  do {                                 \
    ASSERT_NEAR(a.x(), b.x(), eps);    \
    ASSERT_NEAR(a.y(), b.y(), eps);    \
    ASSERT_NEAR(a.z(), b.z(), eps);    \
  } while (0)
