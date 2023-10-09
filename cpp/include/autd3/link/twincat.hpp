// File: twincat.hpp
// Project: link
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
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

/**
 * @brief Link using TwinCAT3
 */
class TwinCAT final {
 public:
  class Builder final : public internal::LinkBuilder {
    friend class TwinCAT;

    internal::native_methods::LinkTwinCATBuilderPtr _ptr;

    Builder() : LinkBuilder(), _ptr(internal::native_methods::AUTDLinkTwinCAT()) {}

   public:
    internal::native_methods::LinkBuilderPtr ptr() const override { return internal::native_methods::AUTDLinkTwinCATIntoBuilder(_ptr); }

    template <typename Rep, typename Period>
    Builder with_timeout(const std::chrono::duration<Rep, Period> timeout) {
      const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
      _ptr = AUTDLinkTwinCATWithTimeout(_ptr, static_cast<uint64_t>(ns));
      return *this;
    }
  };

  static Builder builder() { return Builder(); }

  TwinCAT() = delete;
};

/**
 * @brief Link for remote TwinCAT3 server via [ADS](https://github.com/Beckhoff/ADS) library
 */
class RemoteTwinCAT final {
 public:
  class Builder final : public internal::LinkBuilder {
    friend class RemoteTwinCAT;

    internal::native_methods::LinkRemoteTwinCATBuilderPtr _ptr;

    explicit Builder(const std::string& server_ams_net_id) : internal::LinkBuilder() {
      char err[256];
      _ptr = internal::native_methods::AUTDLinkRemoteTwinCAT(server_ams_net_id.c_str(), err);
      if (_ptr._0 == nullptr) throw internal::AUTDException(err);
    }

   public:
    internal::native_methods::LinkBuilderPtr ptr() const override { return internal::native_methods::AUTDLinkRemoteTwinCATIntoBuilder(_ptr); }

    /**
     * @brief Set server IP address
     *
     * @param ip Server IP address
     * @return RemoteTwinCAT
     */
    Builder with_server_ip(const std::string& ip) {
      _ptr = AUTDLinkRemoteTwinCATWithServerIP(_ptr, ip.c_str());
      return *this;
    }

    /**
     * @brief Set client AMS Net ID
     *
     * @param id AMS Net ID
     * @return RemoteTwinCAT
     */
    Builder with_client_ams_net_id(const std::string& id) {
      _ptr = AUTDLinkRemoteTwinCATWithClientAmsNetId(_ptr, id.c_str());
      return *this;
    }

    template <typename Rep, typename Period>
    Builder with_timeout(const std::chrono::duration<Rep, Period> timeout) {
      const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
      _ptr = AUTDLinkRemoteTwinCATWithTimeout(_ptr, static_cast<uint64_t>(ns));
      return *this;
    }
  };

  /**
   * @brief Constructor
   *
   * @param server_ams_net_id Server AMS Net ID
   */
  static Builder builder(const std::string& server_ams_net_id) { return Builder(server_ams_net_id); }

  RemoteTwinCAT() = delete;
};

}  // namespace autd3::link
