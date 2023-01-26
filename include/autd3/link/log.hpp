// File: log.hpp
// Project: link
// Created Date: 27/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/01/2023
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

namespace autd3::link {

/**
 * @brief Link for Logging
 */
class Log {
 public:
  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build();

  /**
   * @brief Constructor
   */
  explicit Log(core::LinkPtr link) : _link(std::move(link)) {}

  Log& level(const driver::DebugLevel level) {
    _level = level;
    return *this;
  }

  Log& log_func(std::function<void(std::string)> out, std::function<void()> flush) {
    _out = std::move(out);
    _flush = std::move(flush);
    return *this;
  }

  ~Log() = default;
  Log(const Log& v) noexcept = delete;
  Log& operator=(const Log& obj) = delete;
  Log(Log&& obj) = default;
  Log& operator=(Log&& obj) = default;

 private:
  core::LinkPtr _link{nullptr};
  driver::DebugLevel _level{driver::DebugLevel::Info};
  std::function<void(std::string)> _out{nullptr};
  std::function<void()> _flush{nullptr};
};
}  // namespace autd3::link
