// File: command.hpp
// Project: ethercat
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 18/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::link::ethercat {

enum class Command : uint8_t {
  Nop = 0x00,
  Aprd = 0x01,
  Apwr = 0x02,
  Aprw = 0x03,
  Fprd = 0x04,
  Fpwr = 0x05,
  Fprw = 0x06,
  Brd = 0x07,
  Bwr = 0x08,
  Brw = 0x09,
  Lrd = 0x0A,
  Lwr = 0x0B,
  Lrw = 0x0C,
  Armw = 0x0D,
  Frmw = 0x0E,
};

}  // namespace autd3::link::ethercat
