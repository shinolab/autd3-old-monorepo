// File: modulation.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/modulation.hpp"

namespace autd3::core {

/**
 * @brief Modulation controls the amplitude modulation
 */
class Modulation : public DatagramHeader {
 public:
  Modulation() = default;
  ~Modulation() override = default;
  Modulation(const Modulation& v) noexcept = default;
  Modulation& operator=(const Modulation& obj) = default;
  Modulation(Modulation&& obj) = default;
  Modulation& operator=(Modulation&& obj) = default;

  /**
   * \brief Calculate modulation data
   */
  virtual std::vector<driver::Amp> calc() = 0;

  /**
   * \brief sampling frequency division ratio
   */
  uint32_t& sampling_frequency_division() noexcept { return _freq_div; }

  /**
   * \brief sampling frequency division ratio
   */
  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept { return _freq_div; }

  /**
   * \brief modulation sampling frequency
   */
  [[nodiscard]] driver::autd3_float_t sampling_frequency() const noexcept {
    return static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / static_cast<driver::autd3_float_t>(_freq_div);
  }

  /**
   * \brief Set modulation sampling frequency
   */
  [[nodiscard]] driver::autd3_float_t set_sampling_frequency(const driver::autd3_float_t freq) {
    _freq_div = static_cast<uint32_t>(std::round(static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / freq));
    return sampling_frequency();
  }

  std::unique_ptr<driver::Operation> operation() override { return std::make_unique<driver::Modulation>(calc(), _freq_div); }

  template <class Fn>
  static std::vector<driver::Amp> generate_iota(size_t first, const size_t last, Fn func) {
    assert(first < last);
    std::vector<driver::Amp> buffer;
    buffer.reserve(last - first);
    for (size_t i = 0; first != last; ++first) buffer.emplace_back(func(i++));
    return buffer;
  }

 protected:
  uint32_t _freq_div{40960};
};

}  // namespace autd3::core
