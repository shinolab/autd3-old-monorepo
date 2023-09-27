// File: primitive.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

/**
 * @brief Base class for custom modulation
 */
class Modulation : public internal::Modulation {
 public:
  explicit Modulation(const uint32_t freq_div) : _freq_div(freq_div) {}

  [[nodiscard]] virtual std::vector<double> calc() const = 0;

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    const auto buffer = calc();
    const auto size = buffer.size();
    return internal::native_methods::AUTDModulationCustom(_freq_div, buffer.data(), size);
  }

 private:
  uint32_t _freq_div;
};

}  // namespace autd3::modulation
