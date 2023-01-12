// File: debug.hpp
// Project: link
// Created Date: 11/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <functional>
#include <utility>

#include "autd3/core/link.hpp"

namespace autd3::link {

enum class DebugLevel : int { Trace = 0, Debug = 1, Info = 2, Warn = 3, Err = 4, Critical = 5, Off = 6 };

/**
 * @brief Link for debug
 */
class Debug {
 public:
  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build();

  /**
   * @brief Constructor
   */
  Debug() = default;

  Debug& link(core::LinkPtr link) {
    _link = std::move(link);
    return *this;
  }

  Debug& level(const DebugLevel level) {
    _level = level;
    return *this;
  }

  Debug& log_func(std::function<void(std::string)> out, std::function<void()> flush) {
    _out = std::move(out);
    _flush = std::move(flush);
    return *this;
  }

  ~Debug() = default;
  Debug(const Debug& v) noexcept = delete;
  Debug& operator=(const Debug& obj) = delete;
  Debug(Debug&& obj) = default;
  Debug& operator=(Debug&& obj) = default;

 private:
  core::LinkPtr _link{nullptr};
  DebugLevel _level{DebugLevel::Debug};
  std::function<void(std::string)> _out{nullptr};
  std::function<void()> _flush{nullptr};
};
}  // namespace autd3::link
