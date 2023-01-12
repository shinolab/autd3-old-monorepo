// File: focus_stm.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/cpu/datagram.hpp"

namespace autd3::driver {

struct FocusSTM final {
  void init() {
    _sent = 0;
    points.clear();
  }

  void pack(TxDatagram& tx) {
    assert(points.size() == tx.num_devices());
    assert(device_map.size() == tx.num_devices());

    const auto total_size = points[0].size();

    if (total_size > FOCUS_STM_BUF_SIZE_MAX) throw std::runtime_error("FocusSTM out of buffer");
    if (freq_div < FOCUS_STM_SAMPLING_FREQ_DIV_MIN)
      throw std::runtime_error("STM frequency division is out of range. Minimum is " + std::to_string(FOCUS_STM_SAMPLING_FREQ_DIV_MIN) +
                               " but you use " + std::to_string(freq_div));

    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().cpu_flag.remove(CPUControlFlags::STMBegin);
    tx.header().cpu_flag.remove(CPUControlFlags::STMEnd);

    tx.header().fpga_flag.set(FPGAControlFlags::STMMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMGainMode);

    tx.num_bodies = 0;

    if (is_finished()) return;

    if (start_idx) {
      if (static_cast<size_t>(start_idx.value()) >= total_size) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    }
    if (finish_idx) {
      if (static_cast<size_t>(finish_idx.value()) >= total_size) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);
    }

    const auto send_size = get_send_size(total_size, _sent, device_map);
    if (_sent == 0) {
      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
#ifdef AUTD3_USE_METER
      const auto sound_speed_internal = static_cast<uint32_t>(std::round(sound_speed * 1024));
#else
      const auto sound_speed_internal = static_cast<uint32_t>(std::round(sound_speed / 1000 * 1024));
#endif
      std::for_each(tx.begin(), tx.end(), [this, sound_speed_internal, send_size](const auto& body) {
        const auto& [idx, d] = body;
        d.focus_stm_initial().size = static_cast<uint16_t>(send_size);
        d.focus_stm_initial().freq_div = freq_div;
        d.focus_stm_initial().sound_speed = sound_speed_internal;
        d.focus_stm_initial().stm_start_idx = start_idx.value_or(0);
        d.focus_stm_initial().stm_finish_idx = finish_idx.value_or(0);
        d.focus_stm_initial().set_point(&points[idx][_sent], send_size);
      });
    } else {
      std::for_each(tx.begin(), tx.end(), [this, send_size](const auto& body) {
        const auto& [idx, d] = body;
        d.focus_stm_subsequent().size = static_cast<uint16_t>(send_size);
        d.focus_stm_subsequent().set_point(&points[idx][_sent], send_size);
      });
    }

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    if (_sent + send_size == total_size) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.num_bodies = tx.num_devices();

    _sent += send_size;
  }

  [[nodiscard]] bool is_finished() const { return _sent == points[0].size(); }

  std::vector<std::vector<STMFocus>> points{};
  std::vector<size_t> device_map{};
  uint32_t freq_div{4096};
  autd3_float_t sound_speed{};
  std::optional<uint16_t> start_idx{std::nullopt};
  std::optional<uint16_t> finish_idx{std::nullopt};

 private:
  size_t _sent{};

  [[nodiscard]] static size_t get_send_size(const size_t total_size, const size_t sent, const std::vector<size_t>& device_map) noexcept {
    const size_t tr_num = *std::min_element(device_map.begin(), device_map.end());
    const size_t data_len = tr_num * sizeof(uint16_t);
    const auto max_size =
        sent == 0 ? (data_len - sizeof(FocusSTMBodyInitial)) / sizeof(STMFocus) : (data_len - sizeof(FocusSTMBodySubsequent)) / sizeof(STMFocus);
    return (std::min)(total_size - sent, max_size);
  }
};
}  // namespace autd3::driver
