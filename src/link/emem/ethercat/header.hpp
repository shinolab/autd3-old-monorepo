// File: header.hpp
// Project: ethercat
// Created Date: 05/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "../utils.hpp"

namespace autd3::link::ethercat {

const uint16_t ETH_TYPE_ECAT = to_be(0x88A4);

struct EthernetHeader {
  static EthernetHeader ecat_header() noexcept { return {0xFFFF, 0xFFFF, 0xFFFF, 0x0101, 0x0101, 0x0101, ETH_TYPE_ECAT}; }

  [[nodiscard]] bool is_ecat_frame() const noexcept { return _etype == ETH_TYPE_ECAT; }

 private:
  EthernetHeader(const uint16_t da0, const uint16_t da1, const uint16_t da2, const uint16_t sa0, const uint16_t sa1, const uint16_t sa2,
                 const uint16_t etype)
      : _da0(da0), _da1(da1), _da2(da2), _sa0(sa0), _sa1(sa1), _sa2(sa2), _etype(etype) {}

  uint16_t _da0;  // NOLINT
  uint16_t _da1;  // NOLINT
  uint16_t _da2;  // NOLINT
  uint16_t _sa0;  // NOLINT
  uint16_t _sa1;  // NOLINT
  uint16_t _sa2;  // NOLINT
  uint16_t _etype;
};

}  // namespace autd3::link::ethercat
