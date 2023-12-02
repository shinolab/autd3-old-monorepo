// File: utils.hpp
// Project: tests
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/internal/controller.hpp>
#include <autd3/link/audit.hpp>

static inline autd3::internal::Controller<autd3::link::Audit> create_controller() {
  return autd3::internal::ControllerBuilder()
      .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
      .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
      .open_with_async(autd3::link::Audit::builder())
      .get();
}

#define ASSERT_NEAR_VECTOR3(val1, val2, abs_error) \
  do {                                             \
    ASSERT_NEAR(val1.x(), val2.x(), abs_error);    \
    ASSERT_NEAR(val1.y(), val2.y(), abs_error);    \
    ASSERT_NEAR(val1.z(), val2.z(), abs_error);    \
  } while (false)
