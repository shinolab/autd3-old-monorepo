// File: buffer.hpp
// Project: emem
// Created Date: 05/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
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

using EcBuf = std::array<uint8_t, EC_MAX_FRAME_SIZE>;

enum class BufState {
  Empty = 0x00,
  Alloc = 0x01,
  Tx = 0x02,
  Received = 0x03,
  Complete = 0x04,
};

struct Buffer {
  Buffer() : _tx_buf(), _rx_buf(), _len(0), _state(BufState::Empty) {
    _tx_buf.fill(0);
    _rx_buf.fill(0);

    const auto ecat_header = ethercat::EthernetHeader::ecat_header();
    std::memcpy(_tx_buf.data(), &ecat_header, sizeof(ethercat::EthernetHeader));
  }

  [[nodiscard]] BufState state() const noexcept { return _state; }
  void set_state(const BufState state) noexcept { _state = state; }

  [[nodiscard]] const uint8_t* tx_data() const noexcept { return _tx_buf.data(); }
  [[nodiscard]] uint8_t* tx_data() noexcept { return _tx_buf.data(); }
  [[nodiscard]] const uint8_t* rx_data() const noexcept { return _rx_buf.data(); }
  [[nodiscard]] uint8_t* rx_data() noexcept { return _rx_buf.data(); }
  [[nodiscard]] size_t len() const noexcept { return _len; }

  void set_len(const size_t len) { _len = len; }

  void add_len(const size_t len) { _len = +len; }

 private:
  EcBuf _tx_buf;
  EcBuf _rx_buf;
  size_t _len;
  BufState _state;
};

}  // namespace autd3::link
