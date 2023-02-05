// File: header.hpp
// Project: ethercat
// Created Date: 05/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#ifdef WIN32
#include "winsock.h"
#endif

namespace autd3::link::ethercat {

const uint16_t ETH_TYPE_ECAT = htons(0x88A4);

struct EthernetHeader {
  uint16_t _da0;
  uint16_t _da1;
  uint16_t _da2;
  uint16_t _sa0;
  uint16_t _sa1;
  uint16_t _sa2;
  uint16_t etype;

  [[nodiscard]] bool is_ecat_frame() const noexcept { return etype == ETH_TYPE_ECAT; }
};

const EthernetHeader ETH_ECAT_HEADER = EthernetHeader{
    0xFFFF, 0xFFFF, 0xFFFF, 0x0101, 0x0101, 0x0101, ETH_TYPE_ECAT,
};

constexpr size_t ETH_HEADER_SIZE = sizeof(EthernetHeader);

}  // namespace autd3::link::ethercat
