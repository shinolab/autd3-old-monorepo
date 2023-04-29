// File: remote_simulator.hpp
// Project: link
// Created Date: 28/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <string>

#include "autd3/core/link.hpp"
#include "autd3/link/builder.hpp"

namespace autd3::link {

/**
 * @brief Link using Remote Simulator
 */
class RemoteSimulator : public LinkBuilder<RemoteSimulator> {
 public:
  /**
   * @brief Constructor
   */
  RemoteSimulator(std::string ip, const uint16_t port) : LinkBuilder(core::Milliseconds(20)), _ip(std::move(ip)), _port(port) {}
  ~RemoteSimulator() override = default;
  RemoteSimulator(const RemoteSimulator& v) noexcept = delete;
  RemoteSimulator& operator=(const RemoteSimulator& obj) = delete;
  RemoteSimulator(RemoteSimulator&& obj) = delete;
  RemoteSimulator& operator=(RemoteSimulator&& obj) = delete;

 protected:
  core::LinkPtr build_() override;

 private:
  std::string _ip;
  uint16_t _port;
};
}  // namespace autd3::link
