// File: twincat.hpp
// Project: link
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/10/2023
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
#include "autd3/internal/geometry/geometry.hpp"

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
      char err[256];
      _ptr = AUTDLinkSimulatorWithAddr(_ptr, ip.c_str(), err);
      if (_ptr._0 == nullptr) throw internal::AUTDException(err);
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

  explicit Simulator(const internal::native_methods::LinkPtr ptr) : _ptr(ptr) {}

  void update_geometry(internal::Geometry& geometry) const {
      if (char err[256]; AUTDLinkSimulatorUpdateGeometry(_ptr, geometry.ptr(), err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }
};

}  // namespace autd3::link
