// File: debug.hpp
// Project: link
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"

namespace autd3::link {
/**
 * @brief Link for debug
 */
class Debug {
 public:
  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build() const;

  /**
   * @brief Constructor
   */
  Debug() = default;

  ~Debug() = default;
  Debug(const Debug& v) noexcept = delete;
  Debug& operator=(const Debug& obj) = delete;
  Debug(Debug&& obj) = delete;
  Debug& operator=(Debug&& obj) = delete;
};
}  // namespace autd3::link
