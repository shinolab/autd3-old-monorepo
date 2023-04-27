// File: remote_soem.hpp
// Project: link
// Created Date: 26/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 27/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <string>

#include "autd3/core/link.hpp"
#include "autd3/link/builder.hpp"

namespace autd3::link {

/**
 * @brief Link using SOEMAUTDServer
 */
class RemoteSOEM : public LinkBuilder<RemoteSOEM> {
 public:
  /**
   * @brief Constructor
   */
  RemoteSOEM(std::string ip, const uint16_t port) : LinkBuilder(core::Milliseconds(20)), _ip(std::move(ip)), _port(port) {}
  ~RemoteSOEM() = default;
  RemoteSOEM(const RemoteSOEM& v) noexcept = delete;
  RemoteSOEM& operator=(const RemoteSOEM& obj) = delete;
  RemoteSOEM(RemoteSOEM&& obj) = delete;
  RemoteSOEM& operator=(RemoteSOEM&& obj) = delete;

 protected:
  core::LinkPtr build_() override;

 private:
  std::string _ip;
  uint16_t _port;
};
}  // namespace autd3::link
