// File: modulation.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 01/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <utility>
#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct Modulation final : Operation {
  Modulation(std::vector<autd3_float_t> data, const uint32_t freq_div) : _mod_data(std::move(data)), _freq_div(freq_div) {}

  static uint8_t to_duty(const autd3_float_t amp) { return static_cast<uint8_t>(std::round(std::asin(std::clamp<autd3_float_t>(amp, 0, 1)) / pi * 510)); }

  void init() override { _sent = 0; }

  void pack(TxDatagram& tx) override {
    if (_mod_data.size() > MOD_BUF_SIZE_MAX) throw std::runtime_error("Modulation buffer overflow");
    if (_freq_div < MOD_SAMPLING_FREQ_DIV_MIN)
      throw std::runtime_error("Modulation frequency division is out of range. Minimum is " + std::to_string(MOD_SAMPLING_FREQ_DIV_MIN) +
                               " but you use " + std::to_string(_freq_div));

    const auto is_first_frame = _sent == 0;
    const auto max_size = is_first_frame ? MOD_HEADER_INITIAL_DATA_SIZE : MOD_HEADER_SUBSEQUENT_DATA_SIZE;
    const auto mod_size = (std::min)(_mod_data.size() - _sent, max_size);
    if (mod_size == 0) return;
    const auto is_last_frame = _sent + mod_size == _mod_data.size();

    tx.header().cpu_flag.set(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ModBegin);
    tx.header().cpu_flag.remove(CPUControlFlags::ModEnd);
    tx.header().size = static_cast<uint8_t>(mod_size);

    if (is_first_frame) {
      tx.header().cpu_flag.set(CPUControlFlags::ModBegin);
      tx.header().mod_initial().freq_div = _freq_div;
      std::transform(&_mod_data[_sent], &_mod_data[_sent] + mod_size, &tx.header().mod_initial().data[0], to_duty);
    } else {
      std::transform(&_mod_data[_sent], &_mod_data[_sent] + mod_size, &tx.header().mod_subsequent().data[0], to_duty);
    }

    if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::ModEnd);

    _sent += mod_size;
  }

  [[nodiscard]] bool is_finished() const override { return _sent == _mod_data.size(); }

 private:
  size_t _sent{0};
  std::vector<autd3_float_t> _mod_data{};
  uint32_t _freq_div{40960};
};

}  // namespace autd3::driver
