// File: gain.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
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
template <typename T>
struct GainSTM final : STM {
  explicit GainSTM(const Geometry& geometry) : STM(), _geometry(geometry) {}

  /**
   * @brief Set frequency of the STM
   * @param[in] freq Frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return driver::autd3_float_t Actual frequency of STM
   */
  driver::autd3_float_t set_frequency(const driver::autd3_float_t freq) override {
    const auto sample_freq = static_cast<driver::autd3_float_t>(size()) * freq;
    _op.freq_div = static_cast<uint32_t>(std::round(static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / sample_freq));
    return frequency();
  }

  /**
   * @brief Sampling frequency.
   */
  [[nodiscard]] driver::autd3_float_t sampling_frequency() const noexcept override {
    return static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / static_cast<driver::autd3_float_t>(_op.freq_div);
  }

  /**
   * @brief Sampling frequency division.
   */
  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept override { return _op.freq_div; }

  /**
   * @brief Sampling frequency division.
   */
  uint32_t& sampling_frequency_division() noexcept override { return _op.freq_div; }

  std::optional<uint16_t>& start_idx() { return _op.start_idx; }
  std::optional<uint16_t> start_idx() const { return _op.start_idx; }
  std::optional<uint16_t>& finish_idx() { return _op.finish_idx; }
  std::optional<uint16_t> finish_idx() const { return _op.finish_idx; }

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G, std::enable_if_t<std::is_base_of_v<Gain<T>, G>, nullptr_t> = nullptr>
  void add(G& gain) {
    gain.build(_geometry);
    _op.drives.emplace_back(gain.drives());
  }

  driver::GainSTMMode& mode() noexcept { return _op.mode; }

  [[nodiscard]] size_t size() const override { return _op.drives.size(); }

  bool init(const Geometry& geometry) override {
    _op.init();
    return true;
  }

  void pack(driver::TxDatagram& tx) override { _op.pack(tx); }

  [[nodiscard]] bool is_finished() const override { return _op.is_finished(); }

 private:
  const Geometry& _geometry;
  driver::GainSTM<T> _op;
};

}  // namespace autd3::core
