// File: twincat.hpp
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
#include <string>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

class TwinCAT {
 public:
  TwinCAT() : _builder(internal::native_methods::AUTDLinkTwinCAT()) {}

  template <typename Rep, typename Period>
  TwinCAT& timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _builder = internal::native_methods::AUTDLinkTwinCATTimeout(_builder, static_cast<uint64_t>(ns));
    return *this;
  }

  [[nodiscard]] internal::Link build() const {
    char err[256]{};
    auto* ptr = internal::native_methods::AUTDLinkTwinCATBuild(_builder, err);
    if (ptr == nullptr) throw internal::AUTDException(err);
    return internal::Link{ptr};
  }

 private:
  void* _builder;
};

class RemoteTwinCAT {
 public:
  explicit RemoteTwinCAT(const std::string& server_ams_net_id)
      : _builder(internal::native_methods::AUTDLinkRemoteTwinCAT(server_ams_net_id.c_str())) {}

  RemoteTwinCAT& server_ip(const std::string& ip) {
    _builder = internal::native_methods::AUTDLinkRemoteTwinCATServerIP(_builder, ip.c_str());
    return *this;
  }

  RemoteTwinCAT& client_ams_net_id(const std::string& id) {
    _builder = internal::native_methods::AUTDLinkRemoteTwinCATClientAmsNetId(_builder, id.c_str());
    return *this;
  }

  template <typename Rep, typename Period>
  RemoteTwinCAT& timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _builder = internal::native_methods::AUTDLinkRemoteTwinCATTimeout(_builder, static_cast<uint64_t>(ns));
    return *this;
  }

  [[nodiscard]] internal::Link build() const {
    char err[256]{};
    auto* ptr = internal::native_methods::AUTDLinkRemoteTwinCATBuild(_builder, err);
    if (ptr == nullptr) throw internal::AUTDException(err);
    return internal::Link{ptr};
  }

 private:
  void* _builder;
};

}  // namespace autd3::link
