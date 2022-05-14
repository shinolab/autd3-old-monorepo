// File: test_utils.hpp
// Project: tests
// Created Date: 14/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#define ASSERT_NEAR_COMPLEX(a, b, eps)    \
  do {                                    \
    ASSERT_NEAR(a.real(), b.real(), eps); \
    ASSERT_NEAR(a.imag(), b.imag(), eps); \
  } while (0)
