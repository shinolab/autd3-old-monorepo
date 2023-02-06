// File: datagram.hpp
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

#include <algorithm>
#include <array>
#include <cstdint>

#include "command.hpp"
#include "datagram_addr.hpp"
#include "header.hpp"

namespace autd3::link::ethercat {

constexpr uint16_t EC_HEADER_TYPE = 0x1000;
constexpr uint16_t DATAGRAM_FOLLOWS = 1 << 15;

#pragma pack(push)
#pragma pack(1)
struct DatagramHeader {
  constexpr void set_cmd(const Command cmd) { _cmd = static_cast<uint8_t>(cmd); }
  constexpr void set_idx(const uint8_t idx) { _idx = static_cast<uint8_t>(idx); }

  void set_addr(const DatagramAddr addr) {
    const auto bytes = addr.to_addr_bytes();
    std::memcpy(_addr, &bytes, 4);
  }

  constexpr void set_len(const uint16_t len) { _len = len; }

  constexpr void set_follows() { _len |= DATAGRAM_FOLLOWS; }

  [[nodiscard]] constexpr uint8_t idx() const noexcept { return _idx; }

 private:
  uint8_t _cmd{};
  uint8_t _idx{};
  uint8_t _addr[4]{};
  uint16_t _len{};
  [[maybe_unused]] uint16_t _irq{};
};
#pragma pack(pop)

inline void write_datagram_data(uint8_t* datagram_data, const Command cmd, const uint8_t* data, const uint16_t data_len) {
  switch (cmd) {
    case Command::Nop:
    case Command::Aprd:
    case Command::Fprd:
    case Command::Brd:
    case Command::Lrd:
      std::fill_n(datagram_data, data_len, 0);
      break;
    case Command::Apwr:
    case Command::Aprw:
    case Command::Fpwr:
    case Command::Fprw:
    case Command::Bwr:
    case Command::Brw:
    case Command::Lwr:
    case Command::Lrw:
    case Command::Armw:
    case Command::Frmw:
      std::memcpy(datagram_data, data, data_len);
      break;
  }
}

inline void setup_datagram(uint8_t* tx_data, const Command cmd, const uint8_t idx, const DatagramAddr addr, const uint8_t* data,
                           const uint16_t data_len) {
  const auto len = to_le_bytes(static_cast<uint16_t>(EC_HEADER_TYPE + sizeof(DatagramHeader) + data_len + 2));
  std::memcpy(tx_data, &len, 2);

  auto* p_datagram_header = reinterpret_cast<DatagramHeader*>(tx_data + 2);
  p_datagram_header->set_cmd(cmd);
  p_datagram_header->set_idx(idx);
  p_datagram_header->set_addr(addr);
  p_datagram_header->set_len(data_len);
  write_datagram_data(tx_data + 2 + sizeof(DatagramHeader), cmd, data, data_len);
  tx_data[sizeof(DatagramHeader) + data_len] = 0x00;
  tx_data[sizeof(DatagramHeader) + data_len + 1] = 0x00;
}

inline size_t add_datagram(uint8_t* tx_data, const size_t prev_len, const Command cmd, const uint8_t idx, const bool more, const DatagramAddr addr,
                           const uint8_t* data, const uint16_t data_len) {
  const auto e_len = u16_from_le_bytes(tx_data[0], tx_data[1]);

  const auto len = to_le_bytes(static_cast<uint16_t>(e_len + sizeof(DatagramHeader) + data_len + 2));
  std::memcpy(tx_data, &len, 2);

  {
    auto* p_datagram_header = reinterpret_cast<DatagramHeader*>(tx_data + 2);
    p_datagram_header->set_follows();
  }

  {
    auto* p_datagram_header = reinterpret_cast<DatagramHeader*>(tx_data + prev_len - sizeof(EthernetHeader));
    p_datagram_header->set_cmd(cmd);
    p_datagram_header->set_idx(idx);
    p_datagram_header->set_addr(addr);
    p_datagram_header->set_len(data_len);
    if (more) p_datagram_header->set_follows();
  }

  write_datagram_data(tx_data + prev_len - sizeof(EthernetHeader) + sizeof(DatagramHeader), cmd, data, data_len);
  tx_data[prev_len - sizeof(EthernetHeader) + sizeof(DatagramHeader) + data_len] = 0x00;
  tx_data[prev_len - sizeof(EthernetHeader) + sizeof(DatagramHeader) + data_len + 1] = 0x00;

  return prev_len + sizeof(DatagramHeader) - sizeof(EthernetHeader);
}

}  // namespace autd3::link::ethercat
