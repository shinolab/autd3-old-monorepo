// File: simulator.hpp
// Project: link
// Created Date: 27/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <future>
#include <string>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

/**
 * @brief Link for AUTD Simulator
 *
 */
class Simulator final {
  internal::native_methods::LinkPtr _ptr;

 public:
  class Builder final : public internal::LinkBuilder {
    friend class Simulator;

    internal::native_methods::LinkSimulatorBuilderPtr _ptr;

    explicit Builder(const uint16_t port) : LinkBuilder(), _ptr(internal::native_methods::AUTDLinkSimulator(port)) {}

   public:
    [[nodiscard]] internal::native_methods::LinkBuilderPtr ptr() const override { return AUTDLinkSimulatorIntoBuilder(_ptr); }

    /**
     * @brief Set server IP address
     *
     * @param ip Server IP address
     * @return Simulator
     */
    Builder with_server_ip(const std::string& ip) {
      auto [result, err_len, err] = AUTDLinkSimulatorWithAddr(_ptr, ip.c_str());
      if (result._0 == nullptr) {
        const std::string err_str(err_len, ' ');
        internal::native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
        throw internal::AUTDException(err_str);
      }
      _ptr = result;
      return *this;
    }

    template <typename Rep, typename Period>
    Builder with_timeout(const std::chrono::duration<Rep, Period> timeout) {
      const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
      _ptr = AUTDLinkSimulatorWithTimeout(_ptr, static_cast<uint64_t>(ns));
      return *this;
    }
  };

  static Builder builder(const uint16_t port) { return Builder(port); }

  Simulator() = delete;

  explicit Simulator(const internal::native_methods::LinkPtr ptr, const std::shared_ptr<void>&) : _ptr(ptr) {}

  [[nodiscard]] std::future<void> update_geometry_async(const internal::Geometry& geometry) const {
    return std::async(std::launch::async, [this, geometry]() {
      if (const auto [result, err_len, err] = AUTDLinkSimulatorUpdateGeometry(_ptr, geometry.ptr()); result == internal::native_methods::AUTD3_ERR) {
        const std::string err_str(err_len, ' ');
        internal::native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
        throw internal::AUTDException(err_str);
      }
    });
  }
};

}  // namespace autd3::link
