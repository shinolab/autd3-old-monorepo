// File: remote_twincat.hpp
// Project: link
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <string>
#include <utility>

#include "autd3/core/link.hpp"

namespace autd3::link {

/**
 * @brief Link using TwinCAT via Beckhoff ADS
 */
class RemoteTwinCAT {
 public:
  /**
   * @brief Create RemoteTwinCAT
   */
  core::LinkPtr build();

  RemoteTwinCAT& local_ams_net_id(const std::string& local_ams_net_id) {
    _local_ams_net_id = local_ams_net_id;
    return *this;
  }

  /**
   * @brief Constructor
   * @param ipv4_addr remote server ip address
   * @param remote_ams_net_id remote server ams net id
   */
  RemoteTwinCAT(std::string ipv4_addr, std::string remote_ams_net_id)
      : _ipv4_addr(std::move(ipv4_addr)), _remote_ams_net_id(std::move(remote_ams_net_id)) {}
  ~RemoteTwinCAT() = default;
  RemoteTwinCAT(const RemoteTwinCAT& v) noexcept = delete;
  RemoteTwinCAT& operator=(const RemoteTwinCAT& obj) = delete;
  RemoteTwinCAT(RemoteTwinCAT&& obj) = delete;
  RemoteTwinCAT& operator=(RemoteTwinCAT&& obj) = delete;

 private:
  std::string _ipv4_addr;
  std::string _remote_ams_net_id;
  std::string _local_ams_net_id;
};
}  // namespace autd3::link
