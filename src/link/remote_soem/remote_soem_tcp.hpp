// File: remote_soem_tcp.hpp
// Project: remote_soem
// Created Date: 02/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/core/link.hpp>

namespace autd3::link {

class RemoteSOEMTcp final : public core::Link {
 public:
  RemoteSOEMTcp() : Link() {}
  ~RemoteSOEMTcp() override = default;
  RemoteSOEMTcp(const RemoteSOEMTcp& v) noexcept = delete;
  RemoteSOEMTcp& operator=(const RemoteSOEMTcp& obj) = delete;
  RemoteSOEMTcp(RemoteSOEMTcp&& obj) = delete;
  RemoteSOEMTcp& operator=(RemoteSOEMTcp&& obj) = delete;

  void open(const core::Geometry& geometry) override {}

  void close() override {}

  bool send(const driver::TxDatagram& tx) override { return true; }

  bool receive(driver::RxDatagram& rx) override { return true; }

  bool is_open() override { return false; }
};

}  // namespace autd3::link
