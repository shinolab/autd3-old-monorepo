// File: sync_manager.hpp
// Project: ethercat
// Created Date: 07/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::link::ethercat {

constexpr uint32_t EC_DEFAULTMBXSM0 = 0x00010026;
constexpr uint32_t EC_DEFAULTMBXSM1 = 0x00010022;

enum class SMType : uint8_t {
  Unused = 0x00,
  MbxWr = 0x01,
  MbxRd = 0x02,
  Output = 0x03,
  Input = 0x04,
};

#pragma pack(push)
#pragma pack(1)
struct SM {
  uint16_t start_addr{0};
  uint16_t sm_length{0};
  uint32_t sm_flags{0};
};
#pragma pack(pop)

}  // namespace autd3::link::ethercat
