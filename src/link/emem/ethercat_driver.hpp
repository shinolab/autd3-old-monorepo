// File: ethercat_driver.hpp
// Project: emem
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

#include "ethercat/command.hpp"
#include "ethercat/datagram_addr.hpp"
#include "ethercat/registers.hpp"
#include "ethercat/status.hpp"
#include "idx_stack.hpp"
#include "network_driver.hpp"

namespace autd3::link {

using Duration = std::chrono::high_resolution_clock::duration;

constexpr size_t MAX_FPRD_MULTI = 64;

using EcAlStatusBytes = std::array<uint8_t, sizeof(ethercat::EcAlStatus)>;

template <class I>
class EtherCATDriver {
 public:
  explicit EtherCATDriver(I interf) : _net_driver(interf), _idx_stack() {}

  void close() { _net_driver.close(); }

  [[nodiscard]] Result<uint16_t> bwr(const ethercat::BroadcastAddress addr, const uint8_t* data, const size_t size, const Duration timeout) {
    return write_cmd(ethercat::Command::Bwr, ethercat::DatagramAddr(addr), data, size, timeout);
  }

  [[nodiscard]] Result<uint16_t> bwr_word(const ethercat::BroadcastAddress addr, const uint16_t data, const Duration timeout) {
    const auto data_le = to_le_bytes(data);
    return bwr(addr, reinterpret_cast<const uint8_t*>(&data_le), sizeof(uint16_t), timeout);
  }

  [[nodiscard]] Result<uint16_t> brd(const ethercat::BroadcastAddress addr, uint8_t* data, const size_t size, const Duration timeout) {
    return read_cmd(ethercat::Command::Brd, ethercat::DatagramAddr(addr), data, size, timeout);
  }

  [[nodiscard]] Result<uint16_t> brd_word(const ethercat::BroadcastAddress addr, const Duration timeout) {
    uint8_t data[sizeof(uint16_t)];
    if (const auto res = brd(addr, data, sizeof(uint16_t), timeout); res.is_err()) return res;
    return Result(u16_from_le_bytes(data[0], data[1]));
  }

  [[nodiscard]] Result<uint16_t> apwr(const ethercat::PositionAddr addr, const uint8_t* data, const size_t size, const Duration timeout) {
    return write_cmd(ethercat::Command::Apwr, ethercat::DatagramAddr(addr), data, size, timeout);
  }

  [[nodiscard]] Result<uint16_t> aprd(const ethercat::PositionAddr addr, uint8_t* data, const size_t size, const Duration timeout) {
    return read_cmd(ethercat::Command::Aprd, ethercat::DatagramAddr(addr), data, size, timeout);
  }

  [[nodiscard]] Result<uint16_t> aprd_word(const ethercat::PositionAddr addr, const Duration timeout) {
    uint8_t data[sizeof(uint16_t)];
    if (const auto res = aprd(addr, data, sizeof(uint16_t), timeout); res.is_err()) return res;
    return Result(u16_from_le_bytes(data[0], data[1]));
  }

  [[nodiscard]] Result<uint16_t> fpwr(const ethercat::NodeAddress addr, const uint8_t* data, const size_t size, const Duration timeout) {
    return write_cmd(ethercat::Command::Fpwr, ethercat::DatagramAddr(addr), data, size, timeout);
  }

  [[nodiscard]] Result<uint16_t> fpwr_word(const ethercat::NodeAddress addr, const uint16_t data, const Duration timeout) {
    const auto data_le = to_le_bytes(data);
    return fpwr(addr, reinterpret_cast<const uint8_t*>(&data_le), sizeof(uint16_t), timeout);
  }

  template <typename T>
  [[nodiscard]] Result<uint16_t> fpwr_struct(const ethercat::NodeAddress addr, const T* data, const Duration timeout) {
    return fpwr(addr, reinterpret_cast<const uint8_t*>(data), sizeof(T), timeout);
  }

  [[nodiscard]] Result<uint16_t> fprd(const ethercat::NodeAddress addr, uint8_t* data, const size_t size, const Duration timeout) {
    return read_cmd(ethercat::Command::Fprd, ethercat::DatagramAddr(addr), data, size, timeout);
  }

  [[nodiscard]] Result<uint16_t> fprd_word(const ethercat::NodeAddress addr, const Duration timeout) {
    uint8_t data[sizeof(uint16_t)];
    if (const auto res = fprd(addr, data, sizeof(uint16_t), timeout); res.is_err()) return res;
    return Result(u16_from_le_bytes(data[0], data[1]));
  }

  [[nodiscard]] Result<uint16_t> fprd_multi(const int32_t n, const uint16_t* config_list, EcAlStatusBytes* al_status_list, const Duration timeout) {
    const auto idx = _net_driver.get_index();
    {
      auto& data = al_status_list[0];
      ethercat::setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), ethercat::Command::Fprd, idx,
                               ethercat::DatagramAddr(ethercat::NodeAddress{config_list[0], ethercat::registers::ALSTAT}), data.data(),
                               static_cast<uint16_t>(data.size()));
      _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + data.size() + 2);
    }

    size_t sl_data_pos[MAX_FPRD_MULTI];
    sl_data_pos[0] = 2 + sizeof(ethercat::DatagramHeader);

    if (n > 1) {
      for (int32_t sl_cnt = 1; sl_cnt < n - 1; sl_cnt++) {
        auto& data = al_status_list[sl_cnt];
        sl_data_pos[sl_cnt] = ethercat::add_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader),
                                                     _net_driver.buffer(idx).len(), ethercat::Command::Fprd, idx, true,
                                                     ethercat::DatagramAddr(ethercat::NodeAddress{config_list[sl_cnt], ethercat::registers::ALSTAT}),
                                                     data.data(), static_cast<uint16_t>(data.size()));
        _net_driver.buffer(idx).add_len(sizeof(ethercat::DatagramHeader) + data.size() + 2);
      }
      {
        auto& data = al_status_list[n - 1];
        sl_data_pos[n - 1] = ethercat::add_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader),
                                                    _net_driver.buffer(idx).len(), ethercat::Command::Fprd, idx, false,
                                                    ethercat::DatagramAddr(ethercat::NodeAddress{config_list[n - 1], ethercat::registers::ALSTAT}),
                                                    data.data(), static_cast<uint16_t>(data.size()));
        _net_driver.buffer(idx).add_len(sizeof(ethercat::DatagramHeader) + data.size() + 2);
      }
    }

    const auto res = _net_driver.sr_blocking(idx, timeout);
    if (res.is_ok() && res.value() > 0)
      for (int32_t i = 0; i < n; i++) {
        auto& data = al_status_list[i];
        std::memcpy(data.data(), _net_driver.buffer(idx).rx_data() + sl_data_pos[i], data.size());
      }

    _net_driver.setup_buf_state(idx, BufState::Empty);
    return res;
  }

  void process_data_segment_trans_lrd(uint8_t* data, const uint32_t log_addr, const uint32_t len, const bool first, const uint16_t config_addr,
                                      const int64_t dc_time) {
    const auto idx = _net_driver.get_index();
    ethercat::setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), ethercat::Command::Lrd, idx,
                             ethercat::DatagramAddr(ethercat::LogicalAddress{log_addr}), data, static_cast<uint16_t>(len));
    _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + len + 2);

    size_t dc_offset = 0;
    if (first) {
      const auto dc_time_le = to_le_bytes(dc_time);
      dc_offset =
          add_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), _net_driver.buffer(idx).len(), ethercat::Command::Frmw,
                       idx, false, ethercat::DatagramAddr(ethercat::NodeAddress{config_addr, ethercat::registers::DCSYSTIME}),
                       reinterpret_cast<const uint8_t*>(&dc_time_le), sizeof(int64_t));
      _net_driver.buffer(idx).add_len(sizeof(ethercat::DatagramHeader) + sizeof(int64_t) + 2);
    }

    _net_driver.send_frame(idx);
    _idx_stack.push_index(idx, data, static_cast<uint16_t>(len), static_cast<uint16_t>(dc_offset));
  }

  void process_data_segment_trans_lwr(uint8_t* data, const uint32_t log_addr, const uint32_t len) {
    const auto idx = _net_driver.get_index();
    setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), ethercat::Command::Lwr, idx,
                   ethercat::DatagramAddr(ethercat::LogicalAddress{log_addr}), data, static_cast<uint16_t>(len));
    _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + len + 2);
    _net_driver.send_frame(idx);
    _idx_stack.push_index(idx, data, static_cast<uint16_t>(len), 0);
  }

  [[nodiscard]] Result<uint16_t> receive_process_data(const Duration timeout, int64_t& dc_time) {
    auto pos = _idx_stack.pull_index();
    uint16_t wkc = 0;
    auto valid_wkc = false;

    while (pos >= 0) {
      const auto idx = _idx_stack.idx(pos);
      if (const auto res = _net_driver.wait_inframe(idx, timeout); res.is_ok()) {
        const auto wkc2 = res.value();
        if (const auto cmd = static_cast<ethercat::Command>(_net_driver.buffer(idx).rx_data()[2]);
            cmd == ethercat::Command::Lrd || cmd == ethercat::Command::Lrw) {
          if (_idx_stack.dc_offset(pos) > 0) {
            std::memcpy(_idx_stack.data(pos), _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader), _idx_stack.length(pos));
            uint8_t le_wkc[sizeof(uint16_t)];
            std::memcpy(&le_wkc, _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader) + _idx_stack.length(pos), sizeof(uint16_t));
            wkc = u16_from_le_bytes(le_wkc[0], le_wkc[1]) * 2;

            uint8_t le_dc_time[sizeof(int64_t)];
            std::memcpy(&le_dc_time, _net_driver.buffer(idx).rx_data() + _idx_stack.dc_offset(pos), sizeof(int64_t));
            dc_time = i64_from_le_bytes(le_dc_time);
          } else {
            std::memcpy(_idx_stack.data(pos), _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader), _idx_stack.length(pos));
            wkc += wkc2;
          }
          valid_wkc = true;
        } else if (cmd == ethercat::Command::Lwr) {
          if (_idx_stack.dc_offset(pos) > 0) {
            uint8_t le_wkc[sizeof(uint16_t)];
            std::memcpy(&le_wkc, _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader) + _idx_stack.length(pos), sizeof(uint16_t));
            wkc = u16_from_le_bytes(le_wkc[0], le_wkc[1]) * 2;

            uint8_t le_dc_time[sizeof(int64_t)];
            std::memcpy(&le_dc_time, _net_driver.buffer(idx).rx_data() + _idx_stack.dc_offset(pos), sizeof(int64_t));
            dc_time = i64_from_le_bytes(le_dc_time);
          } else {
            wkc += wkc2 * 2;
          }
          valid_wkc = true;
        }
      }

      _net_driver.setup_buf_state(idx, BufState::Empty);
      pos = _idx_stack.pull_index();
    }

    _idx_stack.clear_index();

    return valid_wkc ? Result(wkc) : Result<uint16_t>(EmemError::NoFrame);
  }

 private:
  NetworkDriver<I> _net_driver;
  IdxStack _idx_stack;

  [[nodiscard]] Result<uint16_t> write_cmd(const ethercat::Command cmd, const ethercat::DatagramAddr addr, const uint8_t* data, const size_t data_len,
                                           const std::chrono::high_resolution_clock::duration timeout) {
    const auto idx = _net_driver.get_index();
    setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), cmd, idx, addr, data, static_cast<uint16_t>(data_len));
    _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + data_len + 2);
    const auto res = _net_driver.sr_blocking(idx, timeout);
    _net_driver.setup_buf_state(idx, BufState::Empty);
    return res;
  }

  [[nodiscard]] Result<uint16_t> read_cmd(const ethercat::Command cmd, const ethercat::DatagramAddr addr, uint8_t* data, const size_t data_len,
                                          const std::chrono::high_resolution_clock::duration timeout) {
    const auto idx = _net_driver.get_index();
    setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), cmd, idx, addr, data, static_cast<uint16_t>(data_len));
    _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + data_len + 2);
    const auto res = _net_driver.sr_blocking(idx, timeout);
    if (res.is_ok() && res.value() > 0) std::memcpy(data, _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader), data_len);
    _net_driver.setup_buf_state(idx, BufState::Empty);
    return res;
  }
};

}  // namespace autd3::link
