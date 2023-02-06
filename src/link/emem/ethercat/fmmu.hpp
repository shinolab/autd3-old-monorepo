// File: fmmu.hpp
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

#pragma pack(push)
#pragma pack(1)
struct FMMU {
  uint32_t log_start;
  uint16_t log_length;
  uint8_t log_start_bit;
  uint8_t log_end_bit;
  uint16_t phys_start;
  uint8_t phys_start_bit;
  uint8_t fmmu_type;
  uint8_t fmmu_active;
  [[maybe_unsed]] uint8_t unused1;
  [[maybe_unsed]] uint16_t unused2;
};
#pragma pack(pop)

}  // namespace autd3::link::ethercat
