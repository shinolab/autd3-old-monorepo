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

class TwinCAT : public internal::Link {
 public:
  TwinCAT() : Link(internal::native_methods::LinkPtr{nullptr}) {
    char err[256];
    _ptr = internal::native_methods::AUTDLinkTwinCAT(err);
    if (_ptr._0 == nullptr) throw internal::AUTDException(err);
  }

  template <typename Rep, typename Period>
  TwinCAT with_timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _ptr = AUTDLinkTwinCATTimeout(_ptr, static_cast<uint64_t>(ns));
    return std::move(*this);
  }
};

class RemoteTwinCAT : public internal::Link {
 public:
  explicit RemoteTwinCAT(const std::string& server_ams_net_id) : Link(internal::native_methods::LinkPtr{nullptr}) {
    char err[256];
    _ptr = internal::native_methods::AUTDLinkRemoteTwinCAT(server_ams_net_id.c_str(), err);
    if (_ptr._0 == nullptr) throw internal::AUTDException(err);
  }

  RemoteTwinCAT with_server_ip(const std::string& ip) {
    _ptr = AUTDLinkRemoteTwinCATServerIP(_ptr, ip.c_str());
    return *this;
  }

  RemoteTwinCAT with_client_ams_net_id(const std::string& id) {
    _ptr = AUTDLinkRemoteTwinCATClientAmsNetId(_ptr, id.c_str());
    return *this;
  }

  template <typename Rep, typename Period>
  RemoteTwinCAT with_timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _ptr = AUTDLinkRemoteTwinCATTimeout(_ptr, static_cast<uint64_t>(ns));
    return std::move(*this);
  }
};

}  // namespace autd3::link
