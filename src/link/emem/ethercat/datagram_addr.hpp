// File: datagram_addr.hpp
// Project: ethercat
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "../utils.hpp"

namespace autd3::link::ethercat {

struct PositionAddr {
  uint16_t position;
  uint16_t offset;
};

struct NodeAddress {
  uint16_t addr;
  uint16_t offset;
};

struct BroadcastAddress {
  uint16_t addr;
  uint16_t offset;
};

struct LogicalAddress {
  uint32_t addr;
};

union DatagramAddr {
  PositionAddr position_addr;
  NodeAddress node_addr;
  BroadcastAddress broad_addr;
  LogicalAddress logical_addr;
  uint32_t bytes;

  explicit DatagramAddr(const PositionAddr addr) : position_addr(addr) {}
  explicit DatagramAddr(const BroadcastAddress addr) : broad_addr(addr) {}
  explicit DatagramAddr(const LogicalAddress addr) : logical_addr(addr) {}
  explicit DatagramAddr(const NodeAddress addr) : node_addr(addr) {}

  [[nodiscard]] constexpr uint32_t to_addr_bytes() const noexcept { return to_le_bytes(bytes); }
};

}  // namespace autd3::link::ethercat
