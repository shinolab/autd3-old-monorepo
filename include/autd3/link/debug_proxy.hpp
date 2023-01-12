// File: debug_proxy.hpp
// Project: link
// Created Date: 11/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"

namespace autd3::link {
/**
 * @brief Link for debug
 */
class DebugProxy {
 public:
  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build();

  /**
   * @brief Constructor
   */
  explicit DebugProxy(core::LinkPtr link) : _link(std::move(link)) {}

  ~DebugProxy() = default;
  DebugProxy(const DebugProxy& v) noexcept = delete;
  DebugProxy& operator=(const DebugProxy& obj) = delete;
  DebugProxy(DebugProxy&& obj) = delete;
  DebugProxy& operator=(DebugProxy&& obj) = delete;

 private:
  core::LinkPtr _link;
};
}  // namespace autd3::link
