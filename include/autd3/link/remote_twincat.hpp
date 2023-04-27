// File: remote_twincat.hpp
// Project: link
// Created Date: 12/05/2022
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
#include <utility>

#include "autd3/core/link.hpp"
#include "autd3/link/builder.hpp"

namespace autd3::link {

/**
 * @brief Link using TwinCAT via Beckhoff ADS
 */
class RemoteTwinCAT : public LinkBuilder<RemoteTwinCAT> {
 public:
  RemoteTwinCAT& server_ip_address(const std::string& server_ip_address) {
    _server_ip_address = server_ip_address;
    return *this;
  }

  RemoteTwinCAT& client_ams_net_id(const std::string& client_ams_net_id) {
    _client_ams_net_id = client_ams_net_id;
    return *this;
  }

  /**
   * @brief Constructor
   * @param server_ams_net_id server ams net id
   */
  explicit RemoteTwinCAT(std::string server_ams_net_id) : LinkBuilder(core::Milliseconds(0)), _server_ams_net_id(std::move(server_ams_net_id)) {}
  ~RemoteTwinCAT() = default;
  RemoteTwinCAT(const RemoteTwinCAT& v) noexcept = delete;
  RemoteTwinCAT& operator=(const RemoteTwinCAT& obj) = delete;
  RemoteTwinCAT(RemoteTwinCAT&& obj) = delete;
  RemoteTwinCAT& operator=(RemoteTwinCAT&& obj) = delete;

 protected:
  core::LinkPtr build_() override;

 private:
  std::string _server_ip_address;
  std::string _server_ams_net_id;
  std::string _client_ams_net_id;
};
}  // namespace autd3::link
