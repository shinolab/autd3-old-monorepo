// File: debug.hpp
// Project: link
// Created Date: 11/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/04/2023
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
  Debug() : LinkBuilder<Debug>(core::Milliseconds(0)){};

  Debug& debug_level(const driver::DebugLevel level) {
    _debug_level = level;
    return *this;
  }

  Debug& debug_log_func(std::function<void(std::string)> out, std::function<void()> flush) {
    _debug_out = std::move(out);
    _debug_flush = std::move(flush);
    return *this;
  }

  ~Debug() = default;
  Debug(const Debug& v) noexcept = delete;
  Debug& operator=(const Debug& obj) = delete;
  Debug(Debug&& obj) = default;
  Debug& operator=(Debug&& obj) = default;

 protected:
  core::LinkPtr build_() override;

 private:
  driver::DebugLevel _debug_level{driver::DebugLevel::Debug};
  std::function<void(std::string)> _debug_out{nullptr};
  std::function<void()> _debug_flush{nullptr};
};
}  // namespace autd3::link
