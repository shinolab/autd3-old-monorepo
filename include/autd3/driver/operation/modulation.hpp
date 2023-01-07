// File: modulation.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct Modulation final : Operation {
  void init() override {
    sent = 0;
    mod_data.clear();
  }

  void pack(TxDatagram& tx) override {
    if (mod_data.size() > MOD_BUF_SIZE_MAX) throw std::runtime_error("Modulation buffer overflow");
    if (freq_div < MOD_SAMPLING_FREQ_DIV_MIN)
      throw std::runtime_error("Modulation frequency division is out of range. Minimum is " + std::to_string(MOD_SAMPLING_FREQ_DIV_MIN) +
                               " but you use " + std::to_string(freq_div));

    const auto is_first_frame = sent == 0;
    const auto max_size = is_first_frame ? MOD_HEADER_INITIAL_DATA_SIZE : MOD_HEADER_SUBSEQUENT_DATA_SIZE;
    const auto mod_size = (std::min)(mod_data.size() - sent, max_size);
    if (mod_size == 0) return;
    const auto is_last_frame = sent + mod_size == mod_data.size();
    const auto* buf = &mod_data[sent];

    tx.header().cpu_flag.set(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ModBegin);
    tx.header().cpu_flag.remove(CPUControlFlags::ModEnd);
    tx.header().size = static_cast<uint8_t>(mod_size);

    if (is_first_frame) {
      tx.header().cpu_flag.set(CPUControlFlags::ModBegin);
      tx.header().mod_initial().freq_div = freq_div;
      std::memcpy(&tx.header().mod_initial().data[0], buf, mod_size);
    } else {
      std::memcpy(&tx.header().mod_subsequent().data[0], buf, mod_size);
    }

    if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::ModEnd);

    sent += mod_size;
  }

  [[nodiscard]] bool is_finished() const override { return sent == mod_data.size(); }

  std::vector<uint8_t> mod_data{};
  size_t sent{};
  uint32_t freq_div{40960};
};

}  // namespace autd3::driver
