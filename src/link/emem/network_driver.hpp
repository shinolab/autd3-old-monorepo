// File: network_driver.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <mutex>
#include <optional>

#include "buffer.hpp"
#include "ethercat/datagram.hpp"
#include "pcap/pcap_interface.hpp"
#include "result.hpp"

namespace autd3::link {

using Clock = std::chrono::high_resolution_clock;
using Duration = Clock::duration;

class NetworkDriver {
 public:
  explicit NetworkDriver() : _rx_tmp_buf(), _last_idx(0) { _rx_tmp_buf.fill(0); }

  uint8_t get_index() {
    std::lock_guard lock(_idx_mtx);

    uint8_t idx = _last_idx + 1;
    if (idx >= static_cast<uint8_t>(EC_BUF_SIZE)) idx = 0;

    for (size_t i = 0; i < EC_BUF_SIZE; i++) {
      if (_buffers[idx].state() == BufState::Empty) break;
      if (++idx >= static_cast<uint8_t>(EC_BUF_SIZE)) idx = 0;
    }

    setup_buf_state(idx, BufState::Alloc);
    return _last_idx = idx;
  }

  EmemResult send_frame(const uint8_t idx) {
    const auto& buf = _buffers[idx];
    const auto res = _interf.send(buf.tx_data(), buf.len());
    if (res != EmemResult::Ok) setup_buf_state(idx, BufState::Empty);
    return res;
  }

  EmemResult wait_inframe(const uint8_t idx, const Duration timeout, uint16_t* wkc) {
    const auto expire_time = Clock::now() + timeout;
    for (;;) {
      if (receive_frame(idx, wkc) == EmemResult::Ok) return EmemResult::Ok;
      if (Clock::now() > expire_time) break;
    }
    return EmemResult::NoFrame;
  }

  EmemResult sr_blocking(const uint8_t idx, const Duration timeout, uint16_t* wkc) {
    const auto expire_time = Clock::now() + timeout;
    for (;;) {
      send_frame(idx);
      if (wait_inframe(idx, std::min(timeout, EC_TIMEOUT), wkc) == EmemResult::Ok) return EmemResult::Ok;
      if (Clock::now() > expire_time) break;
    }
    return EmemResult::NoFrame;
  }

  void setup_buf_state(const uint8_t idx, const BufState state) { _buffers[idx].set_state(state); }

  void open(const std::string& ifname) { _interf.open(ifname); }

  void close() { _interf.close(); }

  Buffer& buffer(const size_t idx) { return _buffers[idx]; }

 private:
  EmemResult receive_frame(const uint8_t idx, uint16_t* wkc) {
    auto& buffer = _buffers[idx];
    auto* rx_buf = buffer.rx_data();
    if (buffer.state() == BufState::Received) {
      const auto len = u16_from_le_bytes(rx_buf[0], rx_buf[1]);
      *wkc = u16_from_le_bytes(rx_buf[len], rx_buf[len + 1]);
      buffer.set_state(BufState::Complete);
      return EmemResult::Ok;
    }

    std::lock_guard lock(_rx_mtx);

    EMEM_CHECK_RESULT(_interf.read(_rx_tmp_buf.data(), _rx_tmp_buf.size()));

    if (const auto* p_eth_header = reinterpret_cast<ethercat::EthernetHeader*>(_rx_tmp_buf.data()); !p_eth_header->is_ecat_frame())
      return EmemResult::NoFrame;

    const auto d_len = u16_from_le_bytes(_rx_tmp_buf[sizeof(ethercat::EthernetHeader)], _rx_tmp_buf[sizeof(ethercat::EthernetHeader) + 1]) & 0x07FF;
    const auto* p_datagram_header = reinterpret_cast<ethercat::DatagramHeader*>(&_rx_tmp_buf[sizeof(ethercat::EthernetHeader) + 2]);

    const auto idx_recv = p_datagram_header->idx();
    if (idx == idx_recv) {
      std::memcpy(rx_buf, _rx_tmp_buf.data() + sizeof(ethercat::EthernetHeader), buffer.len() - sizeof(ethercat::EthernetHeader));
      *wkc = u16_from_le_bytes(rx_buf[d_len], rx_buf[d_len + 1]);
      buffer.set_state(BufState::Complete);
      return EmemResult::Ok;
    }

    auto& buffer_recv = _buffers[idx_recv];
    auto* rx_buf_recv = buffer_recv.rx_data();
    if (buffer_recv.state() == BufState::Tx) {
      std::memcpy(rx_buf_recv, _rx_tmp_buf.data() + sizeof(ethercat::EthernetHeader), buffer.len() - sizeof(ethercat::EthernetHeader));
      buffer_recv.set_state(BufState::Received);
      return EmemResult::UnknownFrame;
    }

    return EmemResult::UndefinedBehavior;
  }

  pcap::PcapInterface _interf;
  EcBuf _rx_tmp_buf;
  uint8_t _last_idx;
  std::array<Buffer, EC_BUF_SIZE> _buffers;
  std::mutex _idx_mtx;
  std::mutex _rx_mtx;
};

}  // namespace autd3::link
