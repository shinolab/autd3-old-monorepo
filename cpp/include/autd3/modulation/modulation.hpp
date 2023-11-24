// File: primitive.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

/**
 * @brief Base class for custom modulation
 */
class Modulation : public internal::Modulation {
 public:
  explicit Modulation(const internal::SamplingConfiguration config) : _config(config) {}

  [[nodiscard]] virtual std::vector<internal::EmitIntensity> calc() const = 0;

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    const auto buffer = calc();
    const auto size = buffer.size();
    return AUTDModulationCustom(static_cast<internal::native_methods::SamplingConfiguration>(_config),
                                reinterpret_cast<const uint8_t*>(buffer.data()), size);
  }

 private:
  internal::SamplingConfiguration _config;
};

}  // namespace autd3::modulation
