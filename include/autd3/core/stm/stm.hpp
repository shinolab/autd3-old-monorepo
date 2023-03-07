// File: stm.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <optional>

namespace autd3::core {

/**
 * @brief STM provide hardware Spatio-Temporal Modulation or Lateral Modulation function.
 */
struct STM : DatagramBody {
  STM() noexcept : DatagramBody() {}
  ~STM() override = default;
  STM(const STM& v) = default;
  STM& operator=(const STM& obj) = default;
  STM(STM&& obj) = default;
  STM& operator=(STM&& obj) = default;

  [[nodiscard]] virtual size_t size() const = 0;

  /**
   * @brief Set frequency of the STM
   * @param[in] freq Frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return driver::autd3_float_t Actual frequency of STM
   */
  driver::autd3_float_t set_frequency(driver::autd3_float_t freq) {
    const auto sample_freq = static_cast<driver::autd3_float_t>(size()) * freq;
    sampling_frequency_division = static_cast<uint32_t>(std::round(static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / sample_freq));
    return frequency();
  }

  /**
   * @brief Set sampling frequency of the STM
   * @param[in] sample_freq Sampling frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return driver::autd3_float_t Actual sampling frequency of STM
   */
  driver::autd3_float_t set_sampling_frequency(driver::autd3_float_t sample_freq) {
    sampling_frequency_division = static_cast<uint32_t>(std::round(static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / sample_freq));
    return sampling_frequency();
  }

  /**
   * @return frequency of STM
   */
  [[nodiscard]] driver::autd3_float_t frequency() const { return sampling_frequency() / static_cast<driver::autd3_float_t>(size()); }

  /**
   * @brief Sampling frequency.
   */
  [[nodiscard]] driver::autd3_float_t sampling_frequency() const {
    return static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / static_cast<driver::autd3_float_t>(sampling_frequency_division);
  }

  /**
   * @brief Sampling frequency division.
   */
  uint32_t sampling_frequency_division;

  std::optional<uint16_t> start_idx;
  std::optional<uint16_t> finish_idx;
};
}  // namespace autd3::core
