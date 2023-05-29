// File: twincat.hpp
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

class Simulator {
 public:
  Simulator(const uint16_t port) : _builder(internal::native_methods::AUTDLinkSimulator(port)) {}

  Simulator& addr(const std::string& ip) {
    _builder = internal::native_methods::AUTDLinkSimulatorAddr(_builder, ip.c_str());
    return *this;
  }

  template <typename Rep, typename Period>
  Simulator& timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _builder = internal::native_methods::AUTDLinkSimulatorTimeout(_builder, static_cast<uint64_t>(ns));
    return *this;
  }

  internal::Link build() { return internal::Link(internal::native_methods::AUTDLinkSimulatorBuild(_builder)); }

 private:
  void* _builder;
};

}  // namespace autd3::link
