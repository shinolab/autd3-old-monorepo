// File: gain_stm.hpp
// Project: base
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <vector>

#include "autd3.hpp"

/**
 * @brief GainSTM for capi
 */
struct GainSTM4CAPI final : autd3::core::STM {
  explicit GainSTM4CAPI() : autd3::core::STM() {}

  autd3::driver::autd3_float_t set_frequency(const autd3::driver::autd3_float_t freq) override {
    const auto sample_freq = static_cast<autd3::driver::autd3_float_t>(size()) * freq;
    _freq_div = static_cast<uint32_t>(std::round(static_cast<autd3::driver::autd3_float_t>(autd3::driver::FPGA_CLK_FREQ) / sample_freq));
    return frequency();
  }

  [[nodiscard]] autd3::driver::autd3_float_t sampling_frequency() const noexcept override {
    return static_cast<autd3::driver::autd3_float_t>(autd3::driver::FPGA_CLK_FREQ) / static_cast<autd3::driver::autd3_float_t>(_freq_div);
  }

  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept override { return _freq_div; }

  uint32_t& sampling_frequency_division() noexcept override { return _freq_div; }

  std::optional<uint16_t>& start_idx() override { return _start_idx; }
  [[nodiscard]] std::optional<uint16_t> start_idx() const override { return _start_idx; }
  std::optional<uint16_t>& finish_idx() override { return _finish_idx; }
  [[nodiscard]] std::optional<uint16_t> finish_idx() const override { return _finish_idx; }

  void add(autd3::core::Gain* gain) { _gains.emplace_back(gain); }

  autd3::driver::GainSTMMode mode{autd3::driver::GainSTMMode::PhaseDutyFull};

  [[nodiscard]] size_t size() const override { return _gains.size(); }

  void init(const autd3::core::Mode mode_, const autd3::core::Geometry& geometry) override {
    switch (mode_) {
      case autd3::core::Mode::Legacy: {
        auto op = std::make_unique<autd3::driver::GainSTM<autd3::driver::Legacy>>();
        op->init();
        _op = std::move(op);
      } break;
      case autd3::core::Mode::Normal: {
        auto op = std::make_unique<autd3::driver::GainSTM<autd3::driver::Normal>>();
        op->init();
        op->cycles = geometry.cycles();
        _op = std::move(op);
      } break;
      case autd3::core::Mode::NormalPhase: {
        auto op = std::make_unique<autd3::driver::GainSTM<autd3::driver::NormalPhase>>();
        op->init();
        op->cycles = geometry.cycles();
        _op = std::move(op);
      } break;
    }

    _op->start_idx = _start_idx;
    _op->finish_idx = _finish_idx;
    _op->freq_div = _freq_div;
    _op->mode = mode;

    std::transform(_gains.begin(), _gains.end(), std::back_inserter(_op->drives), [mode_, geometry](const auto& gain) {
      gain->init(mode_, geometry);
      return gain->drives();
    });
  }

  void pack(autd3::driver::TxDatagram& tx) override { _op->pack(tx); }

  [[nodiscard]] bool is_finished() const override { return _op->is_finished(); }

 private:
  std::optional<uint16_t> _start_idx{std::nullopt};
  std::optional<uint16_t> _finish_idx{std::nullopt};
  uint32_t _freq_div{4096};
  std::vector<autd3::core::Gain*> _gains;
  std::unique_ptr<autd3::driver::GainSTMBase> _op;
};
