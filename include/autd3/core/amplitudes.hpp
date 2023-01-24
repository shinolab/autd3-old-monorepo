// File: amplitudes.hpp
// Project: core
// Created Date: 28/06/2022
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
#include "autd3/core/geometry.hpp"
#include "autd3/driver/operation/gain.hpp"

namespace autd3::core {
/**
 * @brief Amplitude configuration for NormalPhaseMode
 */
class Amplitudes final : public DatagramBody {
 public:
  explicit Amplitudes(const driver::autd3_float_t amp = 1.0) : _amp(amp) {}
  ~Amplitudes() override = default;
  Amplitudes(const Amplitudes& v) = default;
  Amplitudes& operator=(const Amplitudes& obj) = default;
  Amplitudes(Amplitudes&& obj) = default;
  Amplitudes& operator=(Amplitudes&& obj) = default;

  std::unique_ptr<driver::Operation> operation(const Geometry& geometry) override {
    return std::make_unique<driver::Amplitude>(std::vector(geometry.num_transducers(), driver::Drive{driver::Phase(0), driver::Amp(_amp)}),
                                               geometry.cycles());
  }

 private:
  driver::autd3_float_t _amp;
};

}  // namespace autd3::core
