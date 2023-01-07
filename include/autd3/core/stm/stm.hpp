// File: stm.hpp
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
  virtual driver::autd3_float_t set_frequency(driver::autd3_float_t freq) = 0;

  /**
   * @return frequency of STM
   */
  [[nodiscard]] driver::autd3_float_t frequency() const { return sampling_frequency() / static_cast<driver::autd3_float_t>(size()); }

  /**
   * @brief Sampling frequency.
   */
  [[nodiscard]] virtual driver::autd3_float_t sampling_frequency() const = 0;

  /**
   * @brief Sampling frequency division.
   */
  [[nodiscard]] virtual uint32_t sampling_frequency_division() const = 0;

  /**
   * @brief Sampling frequency division.
   */
  virtual uint32_t& sampling_frequency_division() = 0;
};
}  // namespace autd3::core
