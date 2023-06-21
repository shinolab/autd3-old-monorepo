// File: log.hpp
// Project: link
// Created Date: 22/06/2023
// Author: Shun Suzuki
// -----
// Last Modified: 22/06/2023
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

class Log : public internal::Link {
 public:
  template <class L>
  Log(L&& link) : Link(internal::native_methods::AUTDLinkLog(link.ptr())) {
    static_assert(std::is_base_of_v<Link, std::remove_reference_t<L>>, "This is not a Link");
  }

  Log with_log_level(const internal::native_methods::Level level) {
    _ptr = AUTDLinkLogWithLogLevel(_ptr, level);
    return *this;
  }

  Log with_log_func(const internal::LogOutCallback out, const internal::LogFlushCallback flush) {
    _ptr = AUTDLinkLogWithLogFunc(_ptr, reinterpret_cast<void*>(out), reinterpret_cast<void*>(flush));
    return *this;
  }
};

}  // namespace autd3::link
