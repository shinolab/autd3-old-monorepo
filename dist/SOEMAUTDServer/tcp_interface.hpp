// File: tcp_interface.hpp
// Project: SOEMAUTDServer
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "interface.hpp"

namespace autd3::publish {

class TcpInterface final : public Interface {
 public:
  TcpInterface() noexcept = default;
  ~TcpInterface() override = default;
  TcpInterface(const TcpInterface& v) noexcept = delete;
  TcpInterface& operator=(const TcpInterface& obj) = delete;
  TcpInterface(TcpInterface&& obj) = default;
  TcpInterface& operator=(TcpInterface&& obj) = default;

  void connect() override {}

  void close() override {}

  bool tx(driver::TxDatagram& tx) override { return true; }

  void rx(driver::RxDatagram& rx) override {}
};

}  // namespace autd3::publish
