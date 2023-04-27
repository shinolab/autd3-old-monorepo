// File: builder.hpp
// Project: link
// Created Date: 27/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>

#include "autd3/core/link.hpp"
#include "autd3/driver/debug_level.hpp"

namespace autd3::link {

/**
 * @brief Builder for Link
 */
template <typename T>
class LinkBuilder {
 public:
  virtual core::LinkPtr build() = 0;

  /**
   * @brief Set default timeout.
   */
  template <typename Rep, typename Period>
  T& timeout(const std::chrono::duration<Rep, Period> timeout) {
    _timeout = timeout;
    return *this;
  }

  /**
   * @brief Enable log
   */
  T& log(const bool enable) {
    _log_enable = enable;
    return static_cast<T&>(*this);
  }

  T& log_level(const driver::DebugLevel level) {
    _level = level;
    return static_cast<T&>(*this);
  }

  T& log_func(std::function<void(std::string)> out, std::function<void()> flush) {
    _out = std::move(out);
    _flush = std::move(flush);
    return static_cast<T&>(*this);
  }

  ~LinkBuilder() = default;
  LinkBuilder(const LinkBuilder& v) noexcept = default;
  LinkBuilder& operator=(const LinkBuilder& obj) = default;
  LinkBuilder(LinkBuilder&& obj) = default;
  LinkBuilder& operator=(LinkBuilder&& obj) = default;

 private:
  driver::DebugLevel _level{driver::DebugLevel::Info};
  bool _log_enable{false};
  std::function<void(std::string)> _out{nullptr};
  std::function<void()> _flush{nullptr};
  core::Duration _timeout{core::Milliseconds(0)};
};
}  // namespace autd3::link
