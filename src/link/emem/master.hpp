// File: master.hpp
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

#include "ethercat/slave.hpp"
#include "ethercat_driver.hpp"
#include "interface.hpp"

namespace autd3::link {

constexpr uint64_t SYNC_DELAY = 100000000;

template <typename I>
class Master {
 public:
  explicit Master(I interf) : _ethercat_driver(interf) {
    _slaves.fill(ethercat::Slave{});
    _io_segment.fill(0);
  }

  void close() { _ethercat_driver.close(); }

  void send_process_data() {
    {
      auto current_segment = _input_segment;
      auto* data = _p_input;
      auto length = static_cast<int32_t>(_input_bytes);
      auto log_addr = _output_bytes;
      auto first = true;
      for (;;) {
        const auto sub_len = current_segment == _input_segment ? _io_segment[current_segment] - _input_offset : _io_segment[current_segment];
        current_segment++;
        _ethercat_driver.process_data_segment_trans_lrd(data, log_addr, sub_len, first, _slaves[1].config_addr, _dc_time);
        first = false;
        length -= static_cast<int32_t>(sub_len);
        log_addr += sub_len;
        data += sub_len;
        if (length <= 0 || current_segment >= _num_segment) break;
      }
    }

    {
      auto current_segment = _input_segment;
      auto* data = _p_output;
      auto length = static_cast<int32_t>(_output_bytes);
      auto log_addr = _output_bytes;
      for (;;) {
        const auto sub_len = std::min(_io_segment[current_segment], static_cast<uint32_t>(length));
        current_segment++;
        _ethercat_driver.process_data_segment_trans_lwr(data, log_addr, sub_len);
        length -= static_cast<int32_t>(sub_len);
        log_addr += sub_len;
        data += sub_len;
        if (length <= 0 || current_segment >= _num_segment) break;
      }
    }
  }

  Result<uint16_t> receive_process_data(Duration timeout) { return _ethercat_driver.receive_process_data(timeout, _dc_time); }

 private:
  EtherCATDriver<I> _ethercat_driver;
  uint16_t _slave_num{0};
  std::array<ethercat::Slave, EC_SLAVE_MAX> _slaves{};
  std::array<uint32_t, MAX_IO_SEGMENT> _io_segment{};
  uint8_t* _p_output{nullptr};
  uint8_t* _p_input{nullptr};
  uint32_t _output_bytes{};
  uint32_t _input_bytes{};
  uint16_t _num_segment{};
  uint16_t _input_segment{};
  uint32_t _input_offset{};
  uint16_t _output_wkc{};
  uint16_t _input_wkc{};
  int64_t _dc_time{};
};
}  // namespace autd3::link
