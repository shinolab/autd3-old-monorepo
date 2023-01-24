// File: lpf.hpp
// Project: modulation
// Created Date: 08/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/modulation.hpp"

namespace autd3::modulation {

/**
 * @brief LPF Modulation to reduce noise
 */
class LPF final : public core::Modulation {
 public:
  /**
   * \param modulation Modulation which passes through LPF to reduce noise
   */
  explicit LPF(Modulation& modulation);

  std::vector<driver::Amp> calc() override;

  ~LPF() override = default;
  LPF(const LPF& v) noexcept = default;
  LPF& operator=(const LPF& obj) = delete;
  LPF(LPF&& obj) = default;
  LPF& operator=(LPF&& obj) = delete;

 private:
  Modulation& _modulation;
  std::vector<driver::autd3_float_t> _coefficients;
};

}  // namespace autd3::modulation
