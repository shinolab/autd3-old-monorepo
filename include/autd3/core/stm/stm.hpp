// File: stm.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cmath>
#include <limits>

#include "autd3/driver/fpga/defined.hpp"

namespace autd3::core {

/**
 * @brief STM provide hardware Spatio-Temporal Modulation or Lateral Modulation function.
 */
template <typename T = LegacyTransducer, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct STM : DatagramBody<T> {
  STM() noexcept : DatagramBody<T>(), _freq_div(4096) {}
  virtual ~STM() = default;
  STM(const STM& v) = default;
  STM& operator=(const STM& obj) = default;
  STM(STM&& obj) = default;
  STM& operator=(STM&& obj) = default;

  [[nodiscard]] virtual size_t size() const = 0;

  /**
   * @brief Set frequency of the STM
   * @param[in] freq Frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return double Actual frequency of STM
   */
  double set_frequency(const double freq) {
    const auto sample_freq = static_cast<double>(size()) * freq;
    const auto div = std::clamp(static_cast<uint32_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / sample_freq)),
                                driver::STM_SAMPLING_FREQ_DIV_MIN, std::numeric_limits<uint32_t>::max());
    _freq_div = div;
    return frequency();
  }

  /**
   * @return frequency of STM
   */
  [[nodiscard]] double frequency() const { return sampling_frequency() / static_cast<double>(size()); }

  /**
   * @brief Sampling frequency.
   * @details Sampling frequency is driver::FPGA_CLK_FREQ/sampling_frequency_division()
   */
  [[nodiscard]] double sampling_frequency() const noexcept { return static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(_freq_div); }

  /**
   * @brief Sampling frequency division.
   */
  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept { return _freq_div; }

  /**
   * @brief Sampling frequency division.
   * @details The value must be larget than driver::STM_SAMPLING_FREQ_DIV_MIN.
   */
  uint32_t& sampling_frequency_division() noexcept { return _freq_div; }

 protected:
  uint32_t _freq_div;
};
}  // namespace autd3::core
