// File: buffer.hpp
// Project: emem
// Created Date: 05/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <array>
#include <cstdint>

#include "consts.hpp"
#include "ethercat/header.hpp"

namespace autd3::link {

enum class BufState {
  Empty = 0x00,
  Alloc = 0x01,
  Tx = 0x02,
  Received = 0x03,
  Complete = 0x04,
};

struct Buffer {
  Buffer() : tx_buf(), rx_buf(), len(0), state(BufState::Empty) {
    tx_buf.fill(0);
    rx_buf.fill(0);
    std::memcpy(tx_buf.data(), &ethercat::ETH_ECAT_HEADER, ethercat::ETH_HEADER_SIZE);
  }

  std::array<uint8_t, EC_MAX_FRAME_SIZE> tx_buf;
  std::array<uint8_t, EC_MAX_FRAME_SIZE> rx_buf;
  size_t len;
  BufState state;
};

}  // namespace autd3::link
