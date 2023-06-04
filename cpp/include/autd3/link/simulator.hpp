// File: twincat.hpp
// Project: link
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <string>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

class Simulator : public internal::Link {
 public:
  explicit Simulator(const uint16_t port) : Link(internal::native_methods::AUTDLinkSimulator(port)) {}

  Simulator with_addr(const std::string& ip) {
    char err[256];
    _ptr = AUTDLinkSimulatorAddr(_ptr, ip.c_str(), err);
    if (_ptr._0 == nullptr) throw internal::AUTDException(err);
    return *this;
  }

  template <typename Rep, typename Period>
  Simulator with_timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _ptr = AUTDLinkSimulatorTimeout(_ptr, static_cast<uint64_t>(ns));
    return std::move(*this);
  }
};

}  // namespace autd3::link
