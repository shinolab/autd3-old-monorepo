// File: interface.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::link {

class Interface {
  virtual void send(uint8_t* data, size_t size) = 0;
  virtual void read(const uint8_t* data) = 0;
  virtual void close() = 0;
  virtual ~Interface() = default;
  Interface(const Interface& v) = default;
  Interface& operator=(const Interface& obj) = default;
  Interface(Interface&& obj) = default;
  Interface& operator=(Interface&& obj) = default;
};

}  // namespace autd3::link
