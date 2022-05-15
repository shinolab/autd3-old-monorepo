// File: remote_twincat.hpp
// Project: link
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
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

  RemoteTwinCAT(const uint16_t cycle_ticks, std::string ipv4_addr, std::string remote_ams_net_id)
      : _cycle_ticks(cycle_ticks), _ipv4_addr(std::move(ipv4_addr)), _remote_ams_net_id(std::move(remote_ams_net_id)) {}
  ~RemoteTwinCAT() = default;
  RemoteTwinCAT(const RemoteTwinCAT& v) noexcept = delete;
  RemoteTwinCAT& operator=(const RemoteTwinCAT& obj) = delete;
  RemoteTwinCAT(RemoteTwinCAT&& obj) = delete;
  RemoteTwinCAT& operator=(RemoteTwinCAT&& obj) = delete;

 private:
  uint16_t _cycle_ticks;
  std::string _ipv4_addr;
  std::string _remote_ams_net_id;
  std::string _local_ams_net_id;
};
}  // namespace autd3::link
