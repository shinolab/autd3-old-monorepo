// File: registers.hpp
// Project: ethercat
// Created Date: 07/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::link::ethercat::registers {

constexpr uint16_t TYPE = 0x0000;
constexpr uint16_t ESCSUP = 0x0008;
constexpr uint16_t STADR = 0x0010;
constexpr uint16_t ALIAS = 0x0012;
constexpr uint16_t DLCTL = 0x0100;
constexpr uint16_t DLPORT = 0x0101;
constexpr uint16_t DLALIAS = 0x0103;
constexpr uint16_t DLSTAT = 0x0110;
constexpr uint16_t ALCTL = 0x0120;
constexpr uint16_t ALSTAT = 0x0130;
constexpr uint16_t PDICTL = 0x0140;
constexpr uint16_t IRQMASK = 0x0200;
constexpr uint16_t RXERR = 0x0300;
constexpr uint16_t EEPCFG = 0x0500;
constexpr uint16_t FMMU0 = 0x0600;
constexpr uint16_t SM0 = 0x0800;
constexpr uint16_t DCTIME0 = 0x0900;
constexpr uint16_t DCTIME1 = 0x0904;
constexpr uint16_t DCSYSTIME = 0x0910;
constexpr uint16_t DCSOF = 0x0918;
constexpr uint16_t DCSYSOFFSET = 0x0920;
constexpr uint16_t DCSYSDELAY = 0x0928;
constexpr uint16_t DCSPEEDCNT = 0x0930;
constexpr uint16_t DCTIMEFILT = 0x0934;
constexpr uint16_t DCCUC = 0x0980;
constexpr uint16_t DCSYNCACT = 0x0981;
constexpr uint16_t DCSTART0 = 0x0990;
constexpr uint16_t DCCYCLE0 = 0x09A0;

}  // namespace autd3::link::ethercat::registers
