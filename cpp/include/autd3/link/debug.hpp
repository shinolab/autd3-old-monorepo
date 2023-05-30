// File: debug.hpp
// Project: link
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>

#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

class Debug {
 public:
  Debug() : _builder(internal::native_methods::AUTDLinkDebug()) {}

  Debug& log_level(const internal::native_methods::Level level) {
    _builder = AUTDLinkDebugLogLevel(_builder, level);
    return *this;
  }

  Debug& log_func(const internal::native_methods::Level level, const internal::LogOutCallback out, const internal::LogFlushCallback flush) {
    _builder = AUTDLinkDebugLogFunc(_builder, level, reinterpret_cast<void*>(out), reinterpret_cast<void*>(flush));
    return *this;
  }

  template <typename Rep, typename Period>
  Debug& timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _builder = internal::native_methods::AUTDLinkDebugTimeout(_builder, static_cast<uint64_t>(ns));
    return *this;
  }

  [[nodiscard]] internal::Link build() const { return internal::Link{internal::native_methods::AUTDLinkDebugBuild(_builder)}; }

 private:
  void* _builder;
};

}  // namespace autd3::link
