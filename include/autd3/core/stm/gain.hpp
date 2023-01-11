// File: gain.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/gain.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/core/stm/stm.hpp"
#include "autd3/driver/operation/gain_stm.hpp"

namespace autd3::core {

/**
 * @brief GainSTM provides a function to display Gain sequentially and periodically.
 * @details GainSTM uses a timer on the FPGA to ensure that Gain is precisely timed.
 */
struct GainSTM final : STM {
  explicit GainSTM() : STM() {}

  /**
   * @brief Set frequency of the STM
   * @param[in] freq Frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return driver::autd3_float_t Actual frequency of STM
   */
  driver::autd3_float_t set_frequency(const driver::autd3_float_t freq) override {
    const auto sample_freq = static_cast<driver::autd3_float_t>(size()) * freq;
    _props.freq_div = static_cast<uint32_t>(std::round(static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / sample_freq));
    return frequency();
  }

  /**
   * @brief Sampling frequency.
   */
  [[nodiscard]] driver::autd3_float_t sampling_frequency() const noexcept override {
    return static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / static_cast<driver::autd3_float_t>(_props.freq_div);
  }

  /**
   * @brief Sampling frequency division.
   */
  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept override { return _props.freq_div; }

  /**
   * @brief Sampling frequency division.
   */
  uint32_t& sampling_frequency_division() noexcept override { return _props.freq_div; }

  std::optional<uint16_t>& start_idx() override { return _props.start_idx; }

  [[nodiscard]] std::optional<uint16_t> start_idx() const override { return _props.start_idx; }

  std::optional<uint16_t>& finish_idx() override { return _props.finish_idx; }

  [[nodiscard]] std::optional<uint16_t> finish_idx() const override { return _props.finish_idx; }

  driver::GainSTMMode& mode() { return _props.mode; }

  [[nodiscard]] driver::GainSTMMode mode() const { return _props.mode; }

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G>
  void add(G&& gain) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
  }

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  void add(std::shared_ptr<core::Gain> gain) { _gains.emplace_back(std::move(gain)); }

  [[nodiscard]] size_t size() const override { return _gains.size(); }

  void init(const Mode mode_, const Geometry& geometry) override {
    switch (mode_) {
      case Mode::Legacy: {
        auto op = std::make_unique<driver::GainSTM<driver::Legacy>>(_props);
        op->init();
        _op = std::move(op);
      } break;
      case Mode::Normal: {
        auto op = std::make_unique<driver::GainSTM<driver::Normal>>(_props);
        op->init();
        op->cycles = geometry.cycles();
        _op = std::move(op);
      } break;
      case Mode::NormalPhase: {
        auto op = std::make_unique<driver::GainSTM<driver::NormalPhase>>(_props);
        op->init();
        op->cycles = geometry.cycles();
        _op = std::move(op);
      } break;
    }

    std::transform(_gains.begin(), _gains.end(), std::back_inserter(_props.drives), [mode_, geometry](const auto& gain) {
      gain->init(mode_, geometry);
      return gain->drives();
    });
  }

  void pack(driver::TxDatagram& tx) override { _op->pack(tx); }

  [[nodiscard]] bool is_finished() const override { return _op->is_finished(); }

 private:
  std::vector<std::shared_ptr<Gain>> _gains;
  driver::GainSTMProps _props;
  std::unique_ptr<driver::GainSTMBase> _op;
};

}  // namespace autd3::core
