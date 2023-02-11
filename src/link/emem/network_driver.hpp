// File: network_driver.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <optional>

#include "buffer.hpp"
#include "error.hpp"
#include "ethercat/datagram.hpp"

namespace autd3::link {

template <typename I>
class NetworkDriver {
 public:
  explicit NetworkDriver(I interf) : _interf(std::move(interf)), _rx_tmp_buf(), _last_idx(0) { _rx_tmp_buf.fill(0); }

  uint8_t get_index() {
    uint8_t idx = _last_idx + 1;
    if (idx >= static_cast<uint8_t>(EC_BUF_SIZE)) idx = 0;

    for (size_t i = 0; i < EC_BUF_SIZE; i++) {
      if (_buffers[idx].state() == BufState::Empty) break;
      if (++idx >= static_cast<uint8_t>(EC_BUF_SIZE)) idx = 0;
    }

    setup_buf_state(idx, BufState::Alloc);
    return _last_idx = idx;
  }

  void send_frame(const uint8_t idx) {
    const auto& buf = _buffers[idx];
    if (const auto res = _interf.send(buf.tx_data(), buf.len()); res.is_err()) setup_buf_state(idx, BufState::Empty);
  }

  Result<uint16_t> wait_inframe(const uint8_t idx, const std::chrono::high_resolution_clock::duration timeout) {
    const auto expire_time = std::chrono::high_resolution_clock::now() + timeout;
    for (;;) {
      if (const auto res = receive_frame(idx); res.is_ok()) return res;
      if (std::chrono::high_resolution_clock::now() > expire_time) break;
    }
    return Result<uint16_t>(EmemError::NoFrame);
  }

  Result<uint16_t> sr_blocking(const uint8_t idx, const std::chrono::high_resolution_clock::duration timeout) {
    const auto expire_time = std::chrono::high_resolution_clock::now() + timeout;
    for (;;) {
      send_frame(idx);
      if (const auto res = wait_inframe(idx, std::min(timeout, EC_TIMEOUT)); res.is_ok()) return res;
      if (std::chrono::high_resolution_clock::now() > expire_time) break;
    }
    return Result<uint16_t>(EmemError::NoFrame);
  }

  void setup_buf_state(const uint8_t idx, const BufState state) { _buffers[idx].set_state(state); }

  void close() { _interf.close(); }

  Buffer& buffer(const size_t idx) { return _buffers[idx]; }

 private:
  Result<uint16_t> receive_frame(const uint8_t idx) {
    auto& buffer = _buffers[idx];
    auto* rx_buf = buffer.rx_data();
    if (buffer.state() == BufState::Received) {
      const auto len = u16_from_le_bytes(rx_buf[0], rx_buf[1]);
      const auto wkc = u16_from_le_bytes(rx_buf[len], rx_buf[len + 1]);
      buffer.set_state(BufState::Complete);
      return Result(wkc);
    }

    if (const auto res = _interf.read(_rx_tmp_buf.data(), _rx_tmp_buf.size()); res.is_err()) return Result<uint16_t>(res.err());

    if (const auto* p_eth_header = reinterpret_cast<ethercat::EthernetHeader*>(_rx_tmp_buf.data()); !p_eth_header->is_ecat_frame())
      return Result<uint16_t>(EmemError::NoFrame);

    const auto d_len = u16_from_le_bytes(_rx_tmp_buf[sizeof(ethercat::EthernetHeader)], _rx_tmp_buf[sizeof(ethercat::EthernetHeader) + 1]) & 0x07FF;
    const auto* p_datagram_header = reinterpret_cast<ethercat::DatagramHeader*>(&_rx_tmp_buf[sizeof(ethercat::EthernetHeader) + 2]);

    const auto idx_recv = p_datagram_header->idx();
    if (idx == idx_recv) {
      std::memcpy(rx_buf, _rx_tmp_buf.data() + sizeof(ethercat::EthernetHeader), buffer.len() - sizeof(ethercat::EthernetHeader));
      const auto wkc = u16_from_le_bytes(rx_buf[d_len], rx_buf[d_len + 1]);
      buffer.set_state(BufState::Complete);
      return Result(wkc);
    }

    auto& buffer_recv = _buffers[idx_recv];
    auto* rx_buf_recv = buffer_recv.rx_data();
    if (buffer_recv.state() == BufState::Tx) {
      std::memcpy(rx_buf_recv, _rx_tmp_buf.data() + sizeof(ethercat::EthernetHeader), buffer.len() - sizeof(ethercat::EthernetHeader));
      buffer_recv.set_state(BufState::Received);
      return Result<uint16_t>(EmemError::UnknownFrame);
    }
    return Result<uint16_t>(EmemError::UndefinedBehavior);
  }

  I _interf;
  EcBuf _rx_tmp_buf;
  uint8_t _last_idx;
  std::array<Buffer, EC_BUF_SIZE> _buffers;
};

}  // namespace autd3::link
