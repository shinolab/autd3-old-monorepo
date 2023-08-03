// File: log.hpp
// Project: link
// Created Date: 22/06/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <utility>

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

/**
 * @brief Link for logging
 */
class Log : public internal::Link {
 public:
  template <class L>
  Log(L&& link) : Link(internal::native_methods::AUTDLinkLog(link.ptr())) {
    static_assert(std::is_base_of_v<Link, std::remove_reference_t<L>>, "This is not a Link");
  }

  /**
   * @brief Set log level
   *
   * @param level log level
   * @return Log
   */
  Log with_log_level(const internal::native_methods::Level level) {
    _ptr = AUTDLinkLogWithLogLevel(_ptr, level);
    return *this;
  }

  /**
   * @brief Set log function
   * @details By default, the logger will display log messages on the console.
   *
   * @param out output callback
   * @param flush flush callback
   * @return Log
   */
  Log with_log_func(const internal::LogOutCallback out, const internal::LogFlushCallback flush) {
    _ptr = AUTDLinkLogWithLogFunc(_ptr, reinterpret_cast<void*>(out), reinterpret_cast<void*>(flush));
    return *this;
  }
};

}  // namespace autd3::link
