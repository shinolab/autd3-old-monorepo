// File: soem.hpp
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
// include Level
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

class SOEM {
 public:
  SOEM() : _builder(internal::native_methods::AUTDLinkSOEM()) {}

  SOEM& ifname(const std::string& ifname) {
    _builder = internal::native_methods::AUTDLinkSOEMIfname(_builder, ifname.c_str());
    return *this;
  }

  SOEM& buf_size(const size_t value) {
    _builder = internal::native_methods::AUTDLinkSOEMBufSize(_builder, value);
    return *this;
  }

  SOEM& send_cycle(const uint16_t value) {
    _builder = internal::native_methods::AUTDLinkSOEMSendCycle(_builder, value);
    return *this;
  }

  SOEM& sync0_cycle(const uint16_t value) {
    _builder = internal::native_methods::AUTDLinkSOEMSync0Cycle(_builder, value);
    return *this;
  }

  SOEM& on_lost(internal::LogOutCallback value) {
    _builder = internal::native_methods::AUTDLinkSOEMOnLost(_builder, reinterpret_cast<void*>(value));
    return *this;
  }

  SOEM& timer_strategy(const internal::native_methods::TimerStrategy value) {
    _builder = internal::native_methods::AUTDLinkSOEMTimerStrategy(_builder, value);
    return *this;
  }

  SOEM& sync_mode(const internal::native_methods::SyncMode value) {
    _builder = internal::native_methods::AUTDLinkSOEMSyncMode(_builder, value);
    return *this;
  }

  template <typename Rep, typename Period>
  SOEM& state_check_interval(const std::chrono::duration<Rep, Period> value) {
    const auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(value).count();
    _builder = internal::native_methods::AUTDLinkSOEMStateCheckInterval(_builder, static_cast<uint64_t>(ms));
    return *this;
  }

  SOEM& log_level(const internal::native_methods::Level value) {
    _builder = internal::native_methods::AUTDLinkSOEMLogLevel(_builder, value);
    return *this;
  }

  SOEM& log_func(const internal::native_methods::Level level, internal::LogOutCallback out, internal::LogFlushCallback flush) {
    _builder = internal::native_methods::AUTDLinkSOEMLogFunc(_builder, level, reinterpret_cast<void*>(out), reinterpret_cast<void*>(flush));
    return *this;
  }

  template <typename Rep, typename Period>
  SOEM& timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _builder = internal::native_methods::AUTDLinkSOEMTimeout(_builder, static_cast<uint64_t>(ns));
    return *this;
  }

  internal::Link build() { return internal::Link(internal::native_methods::AUTDLinkSOEMBuild(_builder)); }

 private:
  void* _builder;
};

class RemoteSOEM {
 public:
  RemoteSOEM(const std::string& ip, const uint16_t port) : _builder(internal::native_methods::AUTDLinkRemoteSOEM(ip.c_str(), port)) {}

  template <typename Rep, typename Period>
  RemoteSOEM& timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _builder = internal::native_methods::AUTDLinkRemoteSOEMTimeout(_builder, static_cast<uint64_t>(ns));
    return *this;
  }

  internal::Link build() { return internal::Link(internal::native_methods::AUTDLinkSOEMBuild(_builder)); }

 private:
  void* _builder;
};

}  // namespace autd3::link
