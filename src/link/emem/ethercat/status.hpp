// File: status.hpp
// Project: ethercat
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::link::ethercat {

constexpr uint8_t EcStateNone = 0x00;
constexpr uint8_t EcStateInit = 0x01;
constexpr uint8_t EcStatePreOp = 0x02;
constexpr uint8_t EcStateSafeOp = 0x04;
constexpr uint8_t EcStateOperational = 0x08;
constexpr uint8_t EcStateAck = 0x10;
constexpr uint8_t EcStateError = 0x10;

#pragma pack(push)
#pragma pack(1)
struct EcAlStatus {
 private:
  uint16_t _al_status{};
  [[maybe_unused]] uint16_t _unused{};
  uint16_t _al_status_code{};
};
#pragma pack(pop)

}  // namespace autd3::link::ethercat
