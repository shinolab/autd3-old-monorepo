// File: debug.hpp
// Project: link
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <utility>

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/link/log.hpp"

namespace autd3::link {

/**
 * @brief Link for debugging
 */
class Debug : public internal::Link {
 public:
  Debug() : Link(internal::native_methods::AUTDLinkDebug()) {}

  /**
   * @brief Set log level
   *
   * @param level log level
   * @return Debug
   */
  Debug with_log_level(const internal::native_methods::Level level) {
    _ptr = AUTDLinkDebugWithLogLevel(_ptr, level);
    return *this;
  }

  /**
   * @brief Set log function
   * @details By default, the logger will display log messages on the console.
   *
   * @param out output callback
   * @param flush flush callback
   * @return Debug
   */
  Debug with_log_func(const internal::LogOutCallback out, const internal::LogFlushCallback flush) {
    _ptr = AUTDLinkDebugWithLogFunc(_ptr, reinterpret_cast<void*>(out), reinterpret_cast<void*>(flush));
    return *this;
  }

  template <typename Rep, typename Period>
  Debug with_timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _ptr = AUTDLinkDebugWithTimeout(_ptr, static_cast<uint64_t>(ns));
    return *this;
  }

  AUTD3_IMPL_WITH_LOG
};

}  // namespace autd3::link
