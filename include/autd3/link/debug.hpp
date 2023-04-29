// File: debug.hpp
// Project: link
// Created Date: 11/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <functional>
#include <string>
#include <utility>

#include "autd3/core/link.hpp"
#include "autd3/driver/debug_level.hpp"
#include "autd3/link/builder.hpp"

namespace autd3::link {

/**
 * @brief Link for debug
 */
class Debug : public LinkBuilder<Debug> {
 public:
  /**
   * @brief Constructor
   */
  Debug() : LinkBuilder<Debug>(core::Milliseconds(0)) { _level = driver::DebugLevel::Debug; };

  ~Debug() override = default;
  Debug(const Debug& v) noexcept = delete;
  Debug& operator=(const Debug& obj) = delete;
  Debug(Debug&& obj) = default;
  Debug& operator=(Debug&& obj) = default;

 protected:
  core::LinkPtr build_() override;
};
}  // namespace autd3::link
