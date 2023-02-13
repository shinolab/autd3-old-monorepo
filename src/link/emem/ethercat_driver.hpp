// File: ethercat_driver.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
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

constexpr uint16_t MAX_FPRD_MULTI = 64;

class EtherCATDriver {
 public:
  explicit EtherCATDriver() = default;

  void open(const std::string& ifname) { _net_driver.open(ifname); }

  void close() { _net_driver.close(); }

  EmemResult bwr(const ethercat::BroadcastAddress addr, const uint8_t* data, const size_t size, const Duration timeout, uint16_t* wkc) {
    return write_cmd(ethercat::Command::Bwr, ethercat::DatagramAddr(addr), data, size, timeout, wkc);
  }

  EmemResult bwr_word(const ethercat::BroadcastAddress addr, const uint16_t data, const Duration timeout, uint16_t* wkc) {
    const auto data_le = to_le_bytes(data);
    return bwr(addr, reinterpret_cast<const uint8_t*>(&data_le), sizeof(uint16_t), timeout, wkc);
  }

  EmemResult brd(const ethercat::BroadcastAddress addr, uint8_t* data, const size_t size, const Duration timeout, uint16_t* wkc) {
    return read_cmd(ethercat::Command::Brd, ethercat::DatagramAddr(addr), data, size, timeout, wkc);
  }

  EmemResult brd_word(const ethercat::BroadcastAddress addr, const Duration timeout, uint16_t* wkc, uint16_t* res) {
    uint8_t data[sizeof(uint16_t)]{};
    EMEM_CHECK_RESULT(brd(addr, data, sizeof(uint16_t), timeout, wkc));
    *res = u16_from_le_bytes(data[0], data[1]);
    return EmemResult::Ok;
  }

  EmemResult apwr(const ethercat::PositionAddr addr, const uint8_t* data, const size_t size, const Duration timeout, uint16_t* wkc) {
    return write_cmd(ethercat::Command::Apwr, ethercat::DatagramAddr(addr), data, size, timeout, wkc);
  }

  EmemResult apwr_word(const ethercat::PositionAddr addr, const uint16_t value, const Duration timeout, uint16_t* wkc) {
    const auto value_le = to_le_bytes(value);
    return apwr(addr, reinterpret_cast<const uint8_t*>(&value_le), sizeof(uint16_t), timeout, wkc);
  }

  EmemResult aprd(const ethercat::PositionAddr addr, uint8_t* data, const size_t size, const Duration timeout, uint16_t* wkc) {
    return read_cmd(ethercat::Command::Aprd, ethercat::DatagramAddr(addr), data, size, timeout, wkc);
  }

  EmemResult aprd_word(const ethercat::PositionAddr addr, const Duration timeout, uint16_t* wkc, uint16_t* res) {
    uint8_t data[sizeof(uint16_t)]{};
    EMEM_CHECK_RESULT(aprd(addr, data, sizeof(uint16_t), timeout, wkc));
    *res = u16_from_le_bytes(data[0], data[1]);
    return EmemResult::Ok;
  }

  EmemResult fpwr(const ethercat::NodeAddress addr, const uint8_t* data, const size_t size, const Duration timeout, uint16_t* wkc) {
    return write_cmd(ethercat::Command::Fpwr, ethercat::DatagramAddr(addr), data, size, timeout, wkc);
  }

  EmemResult fpwr_word(const ethercat::NodeAddress addr, const uint16_t data, const Duration timeout, uint16_t* wkc) {
    const auto data_le = to_le_bytes(data);
    return fpwr(addr, reinterpret_cast<const uint8_t*>(&data_le), sizeof(uint16_t), timeout, wkc);
  }

  template <typename T>
  EmemResult fpwr_struct(const ethercat::NodeAddress addr, const T* data, const Duration timeout, uint16_t* wkc) {
    return fpwr(addr, reinterpret_cast<const uint8_t*>(data), sizeof(T), timeout, wkc);
  }

  EmemResult fprd(const ethercat::NodeAddress addr, uint8_t* data, const size_t size, const Duration timeout, uint16_t* wkc) {
    return read_cmd(ethercat::Command::Fprd, ethercat::DatagramAddr(addr), data, size, timeout, wkc);
  }

  EmemResult fprd_word(const ethercat::NodeAddress addr, const Duration timeout, uint16_t* wkc, uint16_t* res) {
    uint8_t data[sizeof(uint16_t)]{};
    EMEM_CHECK_RESULT(fprd(addr, data, sizeof(uint16_t), timeout, wkc));
    *res = u16_from_le_bytes(data[0], data[1]);
    return EmemResult::Ok;
  }

  EmemResult fprd_multi(const int32_t n, const uint16_t* config_list, ethercat::EcAlStatus* al_status_list, const Duration timeout, uint16_t* wkc) {
    const auto idx = _net_driver.get_index();
    int32_t sl_cnt = 0;

    setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), ethercat::Command::Fprd, idx,
                   ethercat::DatagramAddr(ethercat::NodeAddress{config_list[0], ethercat::registers::ALSTAT}),
                   reinterpret_cast<const uint8_t*>(al_status_list + sl_cnt), sizeof(ethercat::EcAlStatus));
    _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + sizeof(ethercat::EcAlStatus) + 2);

    size_t sl_data_pos[MAX_FPRD_MULTI]{};
    sl_data_pos[sl_cnt] = sizeof(ethercat::EthernetHeader);

    while (++sl_cnt < n - 1) {
      sl_data_pos[sl_cnt] =
          add_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), _net_driver.buffer(idx).len(), ethercat::Command::Fprd,
                       idx, true, ethercat::DatagramAddr(ethercat::NodeAddress{config_list[sl_cnt], ethercat::registers::ALSTAT}),
                       reinterpret_cast<const uint8_t*>(al_status_list + sl_cnt), sizeof(ethercat::EcAlStatus));
      _net_driver.buffer(idx).add_len(sizeof(ethercat::DatagramHeader) + sizeof(ethercat::EcAlStatus) + 2);
    }
    if (sl_cnt < n) {
      sl_data_pos[n - 1] =
          add_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), _net_driver.buffer(idx).len(), ethercat::Command::Fprd,
                       idx, false, ethercat::DatagramAddr(ethercat::NodeAddress{config_list[sl_cnt], ethercat::registers::ALSTAT}),
                       reinterpret_cast<const uint8_t*>(al_status_list + sl_cnt), sizeof(ethercat::EcAlStatus));
      _net_driver.buffer(idx).add_len(sizeof(ethercat::DatagramHeader) + sizeof(ethercat::EcAlStatus) + 2);
    }

    const auto res = _net_driver.sr_blocking(idx, timeout, wkc);
    if (res == EmemResult::Ok)
      for (int32_t i = 0; i < n; i++)
        std::memcpy(reinterpret_cast<uint8_t*>(al_status_list + sl_cnt), _net_driver.buffer(idx).rx_data() + sl_data_pos[i],
                    sizeof(ethercat::EcAlStatus));

    _net_driver.setup_buf_state(idx, BufState::Empty);
    return res;
  }

  void process_data_segment_trans_lrd(uint8_t* data, const uint32_t log_addr, const uint32_t len, const bool first, const uint16_t config_addr,
                                      const int64_t dc_time) {
    const auto idx = _net_driver.get_index();
    setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), ethercat::Command::Lrd, idx,
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

  EmemResult receive_process_data(const Duration timeout, int64_t* dc_time, uint16_t* wkc) {
    auto pos = _idx_stack.pull_index();
    *wkc = 0;
    auto valid_wkc = false;

    while (pos >= 0) {
      const auto idx = _idx_stack.idx(pos);
      if (uint16_t wkc2{}; _net_driver.wait_inframe(idx, timeout, &wkc2) == EmemResult::Ok) {
        if (const auto cmd = static_cast<ethercat::Command>(_net_driver.buffer(idx).rx_data()[2]);
            cmd == ethercat::Command::Lrd || cmd == ethercat::Command::Lrw) {
          if (_idx_stack.dc_offset(pos) > 0) {
            std::memcpy(_idx_stack.data(pos), _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader), _idx_stack.length(pos));
            uint8_t le_wkc[sizeof(uint16_t)];
            std::memcpy(le_wkc, _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader) + _idx_stack.length(pos), sizeof(uint16_t));
            *wkc = u16_from_le_bytes(le_wkc[0], le_wkc[1]);

            uint8_t le_dc_time[sizeof(int64_t)];
            std::memcpy(le_dc_time, _net_driver.buffer(idx).rx_data() + _idx_stack.dc_offset(pos), sizeof(int64_t));
            *dc_time = i64_from_le_bytes(le_dc_time);
          } else {
            std::memcpy(_idx_stack.data(pos), _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader), _idx_stack.length(pos));
            *wkc += wkc2;
          }
          valid_wkc = true;
        } else if (cmd == ethercat::Command::Lwr) {
          if (_idx_stack.dc_offset(pos) > 0) {
            uint8_t le_wkc[sizeof(uint16_t)];
            std::memcpy(le_wkc, _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader) + _idx_stack.length(pos), sizeof(uint16_t));
            *wkc = u16_from_le_bytes(le_wkc[0], le_wkc[1]) * 2;

            uint8_t le_dc_time[sizeof(int64_t)];
            std::memcpy(le_dc_time, _net_driver.buffer(idx).rx_data() + _idx_stack.dc_offset(pos), sizeof(int64_t));
            *dc_time = i64_from_le_bytes(le_dc_time);
          } else {
            *wkc += wkc2 * 2;
          }
          valid_wkc = true;
        }
      }

      _net_driver.setup_buf_state(idx, BufState::Empty);
      pos = _idx_stack.pull_index();
    }

    _idx_stack.clear_index();

    return valid_wkc ? EmemResult::Ok : EmemResult::NoFrame;
  }

 private:
  NetworkDriver _net_driver;
  IdxStack _idx_stack;

  EmemResult write_cmd(const ethercat::Command cmd, const ethercat::DatagramAddr addr, const uint8_t* data, const size_t data_len,
                       const std::chrono::high_resolution_clock::duration timeout, uint16_t* wkc) {
    const auto idx = _net_driver.get_index();
    setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), cmd, idx, addr, data, static_cast<uint16_t>(data_len));
    _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + data_len + 2);
    const auto res = _net_driver.sr_blocking(idx, timeout, wkc);
    _net_driver.setup_buf_state(idx, BufState::Empty);
    return res;
  }

  EmemResult read_cmd(const ethercat::Command cmd, const ethercat::DatagramAddr addr, uint8_t* data, const size_t data_len,
                      const std::chrono::high_resolution_clock::duration timeout, uint16_t* wkc) {
    const auto idx = _net_driver.get_index();
    setup_datagram(_net_driver.buffer(idx).tx_data() + sizeof(ethercat::EthernetHeader), cmd, idx, addr, data, static_cast<uint16_t>(data_len));
    _net_driver.buffer(idx).set_len(sizeof(ethercat::EthernetHeader) + 2 + sizeof(ethercat::DatagramHeader) + data_len + 2);
    const auto res = _net_driver.sr_blocking(idx, timeout, wkc);
    if (res == EmemResult::Ok && *wkc > 0) std::memcpy(data, _net_driver.buffer(idx).rx_data() + 2 + sizeof(ethercat::DatagramHeader), data_len);
    _net_driver.setup_buf_state(idx, BufState::Empty);
    return res;
  }
};

}  // namespace autd3::link
