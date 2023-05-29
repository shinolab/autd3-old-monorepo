// File: debug.hpp
// Project: link
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <functional>
#include <string>

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

class Debug {
 public:
  Debug() : _builder(internal::native_methods::AUTDLinkDebug()) {}

  Debug& log_level(const internal::native_methods::Level level) {
    _builder = internal::native_methods::AUTDLinkDebugLogLevel(_builder, level);
    return *this;
  }

  Debug& log_func(const internal::native_methods::Level level, internal::LogOutCallback out, internal::LogFlushCallback flush) {
    _builder = internal::native_methods::AUTDLinkDebugLogFunc(_builder, level, out, flush);
    return *this;
  }

  template <typename Rep, typename Period>
  Debug& timeout(const std::chrono::duration<Rep, Period> timeout) {
    const ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _builder = internal::native_methods::AUTDLinkDebugTimeout(_builder, static_cast<uint64_t>(ns));
    return *this;
  }

  internal::Link build() { return internal::Link(internal::native_methods::AUTDLinkDebugBuild(_builder)); }

 private:
  void* _builder;
};

}  // namespace autd3::link
