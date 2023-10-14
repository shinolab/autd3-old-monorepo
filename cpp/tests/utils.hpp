// File: utils.hpp
// Project: tests
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/internal/controller.hpp>
#include <autd3/link/audit.hpp>

static inline autd3::internal::Controller create_controller() {
  return autd3::internal::Controller::builder()
      .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
      .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
      .open_with(autd3::link::Audit::builder());
}
