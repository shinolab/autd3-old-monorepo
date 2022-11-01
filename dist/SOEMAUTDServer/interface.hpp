// File: interface.hpp
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

#include <autd3/driver/cpu/datagram.hpp>

namespace autd3::publish {
class Interface {
 public:
  Interface() noexcept = default;
  virtual ~Interface() = default;
  Interface(const Interface& v) noexcept = delete;
  Interface& operator=(const Interface& obj) = delete;
  Interface(Interface&& obj) = default;
  Interface& operator=(Interface&& obj) = default;

  virtual void connect() = 0;
  virtual void close() = 0;
  virtual bool tx(driver::TxDatagram& tx) = 0;
  virtual void rx(driver::RxDatagram& rx) = 0;
};
}  // namespace autd3::publish
