// File: stm.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
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
struct STM {
  STM() noexcept : _freq_div(4096) {}
  virtual ~STM() = default;
  STM(const STM& v) = default;
  STM& operator=(const STM& obj) = default;
  STM(STM&& obj) = default;
  STM& operator=(STM&& obj) = default;

  virtual size_t size() = 0;
  double set_frequency(const double freq) {
    const auto sample_freq = static_cast<double>(size()) * freq;
    const auto div = std::clamp(static_cast<uint32_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / sample_freq)),
                                driver::STM_SAMPLING_FREQ_DIV_MIN, std::numeric_limits<uint32_t>::max());
    _freq_div = div;
    return frequency();
  }
  double frequency() { return sampling_frequency() / static_cast<double>(size()); }
  [[nodiscard]] double sampling_frequency() const noexcept { return static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(_freq_div); }
  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept { return _freq_div; }
  uint32_t& sampling_frequency_division() noexcept { return _freq_div; }

 protected:
  uint32_t _freq_div;
};
}  // namespace autd3::core
